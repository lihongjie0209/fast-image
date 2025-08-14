use fast_image::*;
use std::fs;
use std::path::Path;

/// 计算压缩比例的辅助函数
fn calculate_compression_ratio(original_size: usize, compressed_size: usize) -> f64 {
    (1.0 - compressed_size as f64 / original_size as f64) * 100.0
}

/// 格式化文件大小显示
fn format_size(bytes: usize) -> String {
    if bytes >= 1024 * 1024 {
        format!("{:.2}MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.2}KB", bytes as f64 / 1024.0)
    } else {
        format!("{}B", bytes)
    }
}

#[test]
fn test_png_compression_with_sample_file() {
    let png_path = "examples/SamplePNGImage_5mbmb.png";
    
    // 检查文件是否存在
    if !Path::new(png_path).exists() {
        panic!("Test file not found: {}", png_path);
    }
    
    // 读取原始PNG文件
    let original_data = fs::read(png_path).expect("Failed to read PNG test file");
    let original_size = original_data.len();
    
    println!("Testing PNG compression with {}", png_path);
    println!("Original size: {}", format_size(original_size));
    
    // 验证文件类型检测
    assert!(matches!(ImageType::detect_type(&original_data), Some(ImageType::PNG)));
    
    // 测试不同质量级别的压缩
    let quality_levels = [30, 50, 70, 90];
    
    for quality in quality_levels {
        match do_png_compression(&original_data, quality) {
            Ok(compressed_data) => {
                let compressed_size = compressed_data.len();
                let compression_ratio = calculate_compression_ratio(original_size, compressed_size);
                
                println!("Quality {}: {} -> {} ({:.1}% {})", 
                    quality,
                    format_size(original_size),
                    format_size(compressed_size),
                    compression_ratio.abs(),
                    if compression_ratio > 0.0 { "reduction" } else { "increase" }
                );
                
                // 验证压缩后的数据仍然是有效的PNG
                assert!(ImageType::detect_type(&compressed_data).is_some());
                
                // 保存压缩后的文件用于手动验证
                let output_path = format!("test_png_compressed_q{}.png", quality);
                fs::write(&output_path, &compressed_data)
                    .expect("Failed to write compressed PNG file");
                println!("Saved compressed file: {}", output_path);
            }
            Err(e) => {
                panic!("PNG compression failed at quality {}: {}", quality, e);
            }
        }
    }
    
    println!("PNG compression test completed successfully!\n");
}

#[test]
fn test_jpeg_compression_with_sample_file() {
    let jpeg_path = "examples/SampleJPGImage_5mbmb.jpg";
    
    // 检查文件是否存在
    if !Path::new(jpeg_path).exists() {
        panic!("Test file not found: {}", jpeg_path);
    }
    
    // 读取原始JPEG文件
    let original_data = fs::read(jpeg_path).expect("Failed to read JPEG test file");
    let original_size = original_data.len();
    
    println!("Testing JPEG compression with {}", jpeg_path);
    println!("Original size: {}", format_size(original_size));
    
    // 验证文件类型检测
    assert!(matches!(ImageType::detect_type(&original_data), Some(ImageType::JPEG)));
    
    // 测试不同质量级别的压缩
    let quality_levels = [30, 50, 70, 90];
    
    for quality in quality_levels {
        match do_jpeg_compression(&original_data, quality) {
            Ok(compressed_data) => {
                let compressed_size = compressed_data.len();
                let compression_ratio = calculate_compression_ratio(original_size, compressed_size);
                
                println!("Quality {}: {} -> {} ({:.1}% {})", 
                    quality,
                    format_size(original_size),
                    format_size(compressed_size),
                    compression_ratio.abs(),
                    if compression_ratio > 0.0 { "reduction" } else { "increase" }
                );
                
                // 验证压缩后的数据仍然是有效的JPEG
                assert!(matches!(ImageType::detect_type(&compressed_data), Some(ImageType::JPEG)));
                
                // 保存压缩后的文件用于手动验证
                let output_path = format!("test_jpeg_compressed_q{}.jpg", quality);
                fs::write(&output_path, &compressed_data)
                    .expect("Failed to write compressed JPEG file");
                println!("Saved compressed file: {}", output_path);
            }
            Err(e) => {
                panic!("JPEG compression failed at quality {}: {}", quality, e);
            }
        }
    }
    
    println!("JPEG compression test completed successfully!\n");
}

#[test]
fn test_compression_trait_with_sample_files() {
    println!("Testing compression trait with both sample files...\n");
    
    // 测试PNG文件
    let png_path = "examples/SamplePNGImage_5mbmb.png";
    if Path::new(png_path).exists() {
        let png_data = fs::read(png_path).expect("Failed to read PNG test file");
        let original_size = png_data.len();
        
        match ImageType::compress(&png_data, 75) {
            Ok(compressed_data) => {
                let compressed_size = compressed_data.len();
                let compression_ratio = calculate_compression_ratio(original_size, compressed_size);
                
                println!("PNG via trait: {} -> {} ({:.1}% {})", 
                    format_size(original_size),
                    format_size(compressed_size),
                    compression_ratio.abs(),
                    if compression_ratio > 0.0 { "reduction" } else { "increase" }
                );
            }
            Err(e) => {
                println!("PNG compression via trait failed: {}", e);
            }
        }
    }
    
    // 测试JPEG文件
    let jpeg_path = "examples/SampleJPGImage_5mbmb.jpg";
    if Path::new(jpeg_path).exists() {
        let jpeg_data = fs::read(jpeg_path).expect("Failed to read JPEG test file");
        let original_size = jpeg_data.len();
        
        match ImageType::compress(&jpeg_data, 75) {
            Ok(compressed_data) => {
                let compressed_size = compressed_data.len();
                let compression_ratio = calculate_compression_ratio(original_size, compressed_size);
                
                println!("JPEG via trait: {} -> {} ({:.1}% {})", 
                    format_size(original_size),
                    format_size(compressed_size),
                    compression_ratio.abs(),
                    if compression_ratio > 0.0 { "reduction" } else { "increase" }
                );
            }
            Err(e) => {
                println!("JPEG compression via trait failed: {}", e);
            }
        }
    }
    
    println!("Compression trait test completed successfully!\n");
}

#[test]
fn test_image_type_detection() {
    println!("Testing image type detection...\n");
    
    // 测试PNG检测
    let png_path = "examples/SamplePNGImage_5mbmb.png";
    if Path::new(png_path).exists() {
        let png_data = fs::read(png_path).expect("Failed to read PNG test file");
        match ImageType::detect_type(&png_data) {
            Some(ImageType::PNG) => println!("✓ PNG detection successful"),
            Some(ImageType::JPEG) => panic!("PNG file detected as JPEG"),
            None => panic!("PNG file not detected"),
        }
    }
    
    // 测试JPEG检测
    let jpeg_path = "examples/SampleJPGImage_5mbmb.jpg";
    if Path::new(jpeg_path).exists() {
        let jpeg_data = fs::read(jpeg_path).expect("Failed to read JPEG test file");
        match ImageType::detect_type(&jpeg_data) {
            Some(ImageType::JPEG) => println!("✓ JPEG detection successful"),
            Some(ImageType::PNG) => panic!("JPEG file detected as PNG"),
            None => panic!("JPEG file not detected"),
        }
    }
    
    println!("Image type detection test completed successfully!\n");
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn benchmark_compression_performance() {
        println!("Benchmarking compression performance...\n");
        
        // 基准测试PNG压缩
        let png_path = "examples/SamplePNGImage_5mbmb.png";
        if Path::new(png_path).exists() {
            let png_data = fs::read(png_path).expect("Failed to read PNG test file");
            
            let start = Instant::now();
            let _ = do_png_compression(&png_data, 75).expect("PNG compression failed");
            let duration = start.elapsed();
            
            println!("PNG compression took: {:?}", duration);
        }
        
        // 基准测试JPEG压缩
        let jpeg_path = "examples/SampleJPGImage_5mbmb.jpg";
        if Path::new(jpeg_path).exists() {
            let jpeg_data = fs::read(jpeg_path).expect("Failed to read JPEG test file");
            
            let start = Instant::now();
            let _ = do_jpeg_compression(&jpeg_data, 75).expect("JPEG compression failed");
            let duration = start.elapsed();
            
            println!("JPEG compression took: {:?}", duration);
        }
        
        println!("Performance benchmark completed!\n");
    }
}
