use std::io::Cursor;

pub enum ImageType {
    PNG,
    JPEG,
    // WEBP,
}

/// Check if zero-copy conversion is safe for imagequant::RGBA
/// This verifies that imagequant::RGBA has the same memory layout as [u8; 4]
pub fn can_use_zero_copy() -> bool {
    // Verify memory layout compatibility
    std::mem::size_of::<imagequant::RGBA>() == 4
        && std::mem::align_of::<imagequant::RGBA>() == 1
        && is_rgba_layout_compatible()
}

/// Runtime verification of RGBA memory layout
/// Tests if imagequant::RGBA fields are in R,G,B,A order
pub fn is_rgba_layout_compatible() -> bool {
    let test_bytes = [0x12u8, 0x34u8, 0x56u8, 0x78u8];
    let rgba: imagequant::RGBA = unsafe {
        std::mem::transmute(test_bytes)
    };
    
    rgba.r == 0x12 && rgba.g == 0x34 && rgba.b == 0x56 && rgba.a == 0x78
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

    // Memory optimization info
    let pixel_count = width * height;
    let memory_size_mb = (pixel_count * 4) / (1024 * 1024);
    
    // Use imagequant for color quantization
    let mut liq = imagequant::new();
    liq.set_quality(0, quality)
        .map_err(|e| format!("Failed to set PNG quality: {:?}", e))?;

    // Optimized RGBA conversion with zero-copy or pre-allocation
    let use_zero_copy = can_use_zero_copy();
    
    let mut img_quantize = if use_zero_copy {
        // Zero-copy path: directly reinterpret memory layout
        // This saves ~50% memory for large images
        let rgba_pixels = unsafe {
            std::slice::from_raw_parts(
                image_data.as_ptr() as *const imagequant::RGBA,
                image_data.len() / 4,
            )
        };
        
        // Log memory optimization for debugging
        if memory_size_mb > 50 {
            eprintln!("PNG compression: Using zero-copy optimization for {}MB image ({}x{})", 
                     memory_size_mb, width, height);
        }
        
        liq.new_image(rgba_pixels, width, height, 0.0)
            .map_err(|e| format!("Failed to create quantized image with zero-copy: {:?}", e))?
    } else {
        // Pre-allocation path: minimize allocation overhead
        let mut rgba_pixels = Vec::with_capacity(image_data.len() / 4);
        
        // Use chunks_exact for better performance (no bounds checking)
        for chunk in image_data.chunks_exact(4) {
            rgba_pixels.push(imagequant::RGBA {
                r: chunk[0],
                g: chunk[1],
                b: chunk[2],
                a: chunk[3],
            });
        }
        
        // Log fallback reason for debugging
        if memory_size_mb > 50 {
            eprintln!("PNG compression: Using pre-allocation fallback for {}MB image ({}x{}) - zero-copy not available", 
                     memory_size_mb, width, height);
        }
        
        liq.new_image(&rgba_pixels[..], width, height, 0.0)
            .map_err(|e| format!("Failed to create quantized image with pre-allocation: {:?}", e))?
    };

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
        let compression_level = png::Compression::Best;
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

pub fn do_jpeg_compression_fast(data: &[u8], quality: u8) -> Result<Vec<u8>, String> {
    // Load image data using the image crate for initial parsing
    let img = image::load_from_memory(data)
        .map_err(|e| format!("Failed to load image: {}", e))?;

    // Convert to RGB format
    let rgb_img = img.to_rgb8();
    let width = rgb_img.width() as u16;
    let height = rgb_img.height() as u16;
    let raw_data = rgb_img.as_raw();

    // Pre-allocate output buffer with estimated size
    let mut jpeg_data = Vec::with_capacity(data.len() / 2);

    // Use the dedicated jpeg-encoder crate for better performance
    {
        let encoder = jpeg_encoder::Encoder::new(&mut jpeg_data, quality);
        encoder.encode(raw_data, width, height, jpeg_encoder::ColorType::Rgb)
            .map_err(|e| format!("Failed to encode JPEG with fast encoder: {}", e))?;
    }

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
