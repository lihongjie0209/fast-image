use std::io::Cursor;

pub enum ImageType {
    PNG,
    JPEG,
    // WEBP,
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
            Some(ImageType::PNG) => {
                // Apply PNG compression
                do_png_compression(data, quality)
            }
            Some(ImageType::JPEG) => {
                // Apply JPEG compression
                do_jpeg_compression(data, quality)
            }

            None => Err("UnSupported image type".into()), // No compression if type is unknown
        }
    }
}

pub fn do_png_compression(data: &[u8], quality: u8) -> Result<Vec<u8>, String> {
    // Load image data
    let img =
        image::load_from_memory(data).map_err(|e| format!("Failed to load PNG image: {}", e))?;

    // Convert to RGBA8 format for imagequant
    let rgba_img = img.to_rgba8();
    let width = rgba_img.width() as usize;
    let height = rgba_img.height() as usize;
    let image_data = rgba_img.as_raw();

    // Use imagequant for color quantization
    let mut liq = imagequant::new();
    liq.set_quality(0, quality)
        .map_err(|e| format!("Failed to set PNG quality: {:?}", e))?;

    // Convert Vec<u8> to the format imagequant expects
    let rgba_pixels: Vec<imagequant::RGBA> = image_data
        .chunks_exact(4)
        .map(|chunk| imagequant::RGBA {
            r: chunk[0],
            g: chunk[1],
            b: chunk[2],
            a: chunk[3],
        })
        .collect();

    // Create image for quantization
    let mut img_quantize = liq
        .new_image(&rgba_pixels[..], width, height, 0.0)
        .map_err(|e| format!("Failed to create quantized image: {:?}", e))?;

    // Quantize the image
    let mut res = liq
        .quantize(&mut img_quantize)
        .map_err(|e| format!("Failed to quantize PNG: {:?}", e))?;

    // Set dithering level (0.0 - 1.0)
    res.set_dithering_level(1.0)
        .map_err(|e| format!("Failed to set dithering: {:?}", e))?;

    // Get quantized data - this time we actually use it!
    let (palette, pixels) = res
        .remapped(&mut img_quantize)
        .map_err(|e| format!("Failed to remap PNG: {:?}", e))?;

    // Create PNG with indexed colors using the quantized palette
    let mut png_data = Vec::new();
    
    {
        let mut encoder = png::Encoder::new(Cursor::new(&mut png_data), width as u32, height as u32);
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_depth(png::BitDepth::Eight);
        
        // Set compression level based on quality (inverted: lower quality = higher compression)
        let compression_level = match quality {
            0..=25 => png::Compression::Best,
            26..=50 => png::Compression::Fast,
            51..=75 => png::Compression::Default,
            _ => png::Compression::Fast,
        };
        encoder.set_compression(compression_level);
        
        // Convert palette to the format PNG encoder expects
        let png_palette: Vec<u8> = palette.iter()
            .flat_map(|color| vec![color.r, color.g, color.b])
            .collect();
        
        encoder.set_palette(png_palette);
        
        let mut writer = encoder.write_header()
            .map_err(|e| format!("Failed to write PNG header: {}", e))?;
            
        // Write the indexed pixel data
        writer.write_image_data(&pixels)
            .map_err(|e| format!("Failed to write PNG data: {}", e))?;
    }

    Ok(png_data)
}

pub fn do_jpeg_compression(data: &[u8], quality: u8) -> Result<Vec<u8>, String> {
    // Load image data
    let img =
        image::load_from_memory(data).map_err(|e| format!("Failed to load JPEG image: {}", e))?;

    // Convert to RGB format for JPEG
    let rgb_img = img.to_rgb8();
    let width = rgb_img.width() as usize;
    let height = rgb_img.height() as usize;

    // Create output buffer
    let mut jpeg_data = Vec::new();

    // Create mozjpeg compressor
    let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);

    // Set compression parameters
    comp.set_size(width, height);
    comp.set_quality(quality as f32);

    // Start compression with output buffer
    let mut comp_started = comp
        .start_compress(&mut jpeg_data)
        .map_err(|e| format!("Failed to start JPEG compression: {}", e))?;

    // Write image data row by row
    let row_stride = width * 3; // RGB = 3 bytes per pixel
    let image_data = rgb_img.as_raw();

    for y in 0..height {
        let row_start = y * row_stride;
        let row_end = row_start + row_stride;
        let row = &image_data[row_start..row_end];
        comp_started
            .write_scanlines(row)
            .map_err(|e| format!("Failed to write JPEG scanline: {}", e))?;
    }

    // Finish compression
    comp_started
        .finish()
        .map_err(|e| format!("Failed to finish JPEG compression: {}", e))?;

    Ok(jpeg_data)
}

//
// pub fn do_webp_compression(data: &[u8], quality: u8) -> Result<Vec<u8>, String> {
//     // Load image data
//     let img = image::load_from_memory(data)
//         .map_err(|e| format!("Failed to load WebP image: {}", e))?;
//
//     // Convert to RGBA format for WebP
//     let rgba_img = img.to_rgba8();
//     let width = rgba_img.width() as u32;
//     let height = rgba_img.height() as u32;
//     let image_data = rgba_img.as_raw();
//
//     // Create WebP encoder
//     let encoder = webp::Encoder::from_rgba(image_data, width, height);
//
//     // Set quality (0-100)
//     let webp_data = if quality >= 100 {
//         // Use lossless compression for quality 100
//         encoder.encode_lossless()
//     } else {
//         // Use lossy compression with specified quality
//         encoder.encode(quality as f32)
//     };
//
//     Ok(webp_data.to_vec())
// }
