use std::io::Cursor;
use std::time::Instant;

pub enum ImageType {
    PNG,
    JPEG,
}

pub trait Compression {
    fn compress(data: &[u8], quality: u8) -> Result<Vec<u8>, String>;
}

impl ImageType {
    pub fn detect_type(data: &[u8]) -> Option<ImageType> {
        if data.starts_with(b"\x89PNG\r\n\x1a\n") {
            Some(ImageType::PNG)
        } else if data.starts_with(b"\xff\xd8") {
            Some(ImageType::JPEG)
        } else {
            None
        }
    }
}

impl Compression for ImageType {
    fn compress(data: &[u8], quality: u8) -> Result<Vec<u8>, String> {
        match Self::detect_type(data) {
            Some(ImageType::PNG) => do_png_compression(data, quality),
            Some(ImageType::JPEG) => do_jpeg_compression(data, quality),
            None => Err("UnSupported image type".into()),
        }
    }
}

/// 优化版本：减少内存分配和数据复制
pub fn do_png_compression_optimized(data: &[u8], quality: u8) -> Result<Vec<u8>, String> {
    let start_time = Instant::now();
    
    // Load image data
    let img = image::load_from_memory(data)
        .map_err(|e| format!("Failed to load PNG image: {}", e))?;
    
    let load_time = start_time.elapsed();
    let convert_start = Instant::now();
    
    // Convert to RGBA8 format
    let rgba_img = img.to_rgba8();
    let width = rgba_img.width() as usize;
    let height = rgba_img.height() as usize;
    let image_data = rgba_img.as_raw();
    
    let convert_time = convert_start.elapsed();
    let quant_start = Instant::now();
    
    // Use imagequant for color quantization
    let mut liq = imagequant::new();
    liq.set_quality(0, quality)
        .map_err(|e| format!("Failed to set PNG quality: {:?}", e))?;
    
    // 优化：尝试使用slice直接创建，避免Vec分配
    // 检查imagequant是否支持直接从u8 slice创建
    let mut img_quantize = match liq.new_image_stride(
        image_data.as_ptr() as *const imagequant::RGBA,
        width,
        height,
        width * 4,
        0.0
    ) {
        Ok(img) => img,
        Err(_) => {
            // 回退到原来的方法，但优化结构体构造
            let rgba_pixels: Vec<imagequant::RGBA> = image_data
                .chunks_exact(4)
                .map(|chunk| unsafe {
                    // 使用unsafe优化：直接读取4字节作为RGBA
                    std::mem::transmute::<[u8; 4], imagequant::RGBA>([
                        chunk[0], chunk[1], chunk[2], chunk[3]
                    ])
                })
                .collect();
            
            liq.new_image(&rgba_pixels[..], width, height, 0.0)
                .map_err(|e| format!("Failed to create quantized image: {:?}", e))?
        }
    };
    
    // Quantize the image
    let mut res = liq.quantize(&mut img_quantize)
        .map_err(|e| format!("Failed to quantize PNG: {:?}", e))?;
    
    // Set dithering level
    res.set_dithering_level(1.0)
        .map_err(|e| format!("Failed to set dithering: {:?}", e))?;
    
    let quant_time = quant_start.elapsed();
    let encode_start = Instant::now();
    
    // Get quantized data
    let (palette, pixels) = res.remapped(&mut img_quantize)
        .map_err(|e| format!("Failed to remap PNG: {:?}", e))?;
    
    // Create PNG with indexed colors
    let mut png_data = Vec::new();
    {
        let mut encoder = png::Encoder::new(
            Cursor::new(&mut png_data), 
            width as u32, 
            height as u32
        );
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_depth(png::BitDepth::Eight);
        
        let compression_level = match quality {
            0..=25 => png::Compression::Best,
            26..=50 => png::Compression::Fast,
            51..=75 => png::Compression::Default,
            _ => png::Compression::Fast,
        };
        encoder.set_compression(compression_level);
        
        // 优化：预分配palette容量
        let mut png_palette = Vec::with_capacity(palette.len() * 3);
        for color in palette.iter() {
            png_palette.extend_from_slice(&[color.r, color.g, color.b]);
        }
        
        encoder.set_palette(png_palette);
        
        let mut writer = encoder.write_header()
            .map_err(|e| format!("Failed to write PNG header: {}", e))?;
        
        writer.write_image_data(&pixels)
            .map_err(|e| format!("Failed to write PNG data: {}", e))?;
    }
    
    let encode_time = encode_start.elapsed();
    let total_time = start_time.elapsed();
    
    println!("🔍 PNG压缩性能分析:");
    println!("  加载图片: {:?}", load_time);
    println!("  格式转换: {:?}", convert_time);
    println!("  颜色量化: {:?}", quant_time);
    println!("  PNG编码: {:?}", encode_time);
    println!("  总耗时: {:?}", total_time);
    
    Ok(png_data)
}

/// 原始版本：用于性能对比
pub fn do_png_compression(data: &[u8], quality: u8) -> Result<Vec<u8>, String> {
    let start_time = Instant::now();
    
    let img = image::load_from_memory(data)
        .map_err(|e| format!("Failed to load PNG image: {}", e))?;
    
    let rgba_img = img.to_rgba8();
    let width = rgba_img.width() as usize;
    let height = rgba_img.height() as usize;
    let image_data = rgba_img.as_raw();
    
    let mut liq = imagequant::new();
    liq.set_quality(0, quality)
        .map_err(|e| format!("Failed to set PNG quality: {:?}", e))?;
    
    // 原始的慢速转换方法
    let rgba_pixels: Vec<imagequant::RGBA> = image_data
        .chunks_exact(4)
        .map(|chunk| imagequant::RGBA {
            r: chunk[0],
            g: chunk[1],
            b: chunk[2],
            a: chunk[3],
        })
        .collect();
    
    let mut img_quantize = liq.new_image(&rgba_pixels[..], width, height, 0.0)
        .map_err(|e| format!("Failed to create quantized image: {:?}", e))?;
    
    let mut res = liq.quantize(&mut img_quantize)
        .map_err(|e| format!("Failed to quantize PNG: {:?}", e))?;
    
    res.set_dithering_level(1.0)
        .map_err(|e| format!("Failed to set dithering: {:?}", e))?;
    
    let (palette, pixels) = res.remapped(&mut img_quantize)
        .map_err(|e| format!("Failed to remap PNG: {:?}", e))?;
    
    let mut png_data = Vec::new();
    {
        let mut encoder = png::Encoder::new(Cursor::new(&mut png_data), width as u32, height as u32);
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_depth(png::BitDepth::Eight);
        
        let compression_level = match quality {
            0..=25 => png::Compression::Best,
            26..=50 => png::Compression::Fast,
            51..=75 => png::Compression::Default,
            _ => png::Compression::Fast,
        };
        encoder.set_compression(compression_level);
        
        let png_palette: Vec<u8> = palette.iter()
            .flat_map(|color| vec![color.r, color.g, color.b])
            .collect();
        
        encoder.set_palette(png_palette);
        
        let mut writer = encoder.write_header()
            .map_err(|e| format!("Failed to write PNG header: {}", e))?;
        
        writer.write_image_data(&pixels)
            .map_err(|e| format!("Failed to write PNG data: {}", e))?;
    }
    
    let total_time = start_time.elapsed();
    println!("📊 原始PNG压缩总耗时: {:?}", total_time);
    
    Ok(png_data)
}

pub fn do_jpeg_compression(data: &[u8], quality: u8) -> Result<Vec<u8>, String> {
    let img = image::load_from_memory(data)
        .map_err(|e| format!("Failed to load JPEG image: {}", e))?;
    
    let rgb_img = img.to_rgb8();
    let width = rgb_img.width() as usize;
    let height = rgb_img.height() as usize;
    
    let mut jpeg_data = Vec::new();
    let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    
    comp.set_size(width, height);
    comp.set_quality(quality as f32);
    
    let mut comp_started = comp.start_compress(&mut jpeg_data)
        .map_err(|e| format!("Failed to start JPEG compression: {}", e))?;
    
    let row_stride = width * 3;
    let image_data = rgb_img.as_raw();
    
    for y in 0..height {
        let row_start = y * row_stride;
        let row_end = row_start + row_stride;
        let row = &image_data[row_start..row_end];
        comp_started.write_scanlines(row)
            .map_err(|e| format!("Failed to write JPEG scanline: {}", e))?;
    }
    
    comp_started.finish()
        .map_err(|e| format!("Failed to finish JPEG compression: {}", e))?;
    
    Ok(jpeg_data)
}
