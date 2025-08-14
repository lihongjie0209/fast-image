use fast_image::*;
use std::fs;
use std::time::Instant;

#[test]
fn analyze_memory_usage_and_performance() {
    let png_path = "examples/SamplePNGImage_5mbmb.png";
    
    if !std::path::Path::new(png_path).exists() {
        return; // 跳过测试如果文件不存在
    }
    
    let original_data = fs::read(png_path).expect("Failed to read PNG test file");
    
    println!("🧠 PNG压缩性能和内存分析");
    println!("{}", "=".repeat(50));
    
    // 内存使用分析
    let img = image::load_from_memory(&original_data).unwrap();
    let rgba_img = img.to_rgba8();
    let image_data = rgba_img.as_raw();
    
    let original_bytes = image_data.len();
    let pixel_count = original_bytes / 4;
    
    println!("📊 内存占用分析:");
    println!("  原始RGBA数据: {:.2}MB ({} pixels)", 
        original_bytes as f64 / (1024.0 * 1024.0), 
        pixel_count
    );
    
    // 模拟RGBA结构体转换的内存开销
    let rgba_struct_size = std::mem::size_of::<u32>(); // 假设RGBA是4字节结构体
    let converted_bytes = pixel_count * rgba_struct_size;
    
    println!("  RGBA结构体数据: {:.2}MB", 
        converted_bytes as f64 / (1024.0 * 1024.0)
    );
    println!("  峰值内存使用: {:.2}MB (2倍数据)", 
        (original_bytes + converted_bytes) as f64 / (1024.0 * 1024.0)
    );
    
    // 性能测试
    println!("\n⏱️  性能基准测试:");
    
    let quality_levels = [30, 70];
    for quality in quality_levels {
        println!("\n🎯 质量级别 {}:", quality);
        
        // 测试当前方法的性能分解
        let total_start = Instant::now();
        
        let conversion_start = Instant::now();
        
        // 模拟当前的转换过程
        let _rgba_pixels: Vec<_> = image_data
            .chunks_exact(4)
            .map(|chunk| (chunk[0], chunk[1], chunk[2], chunk[3]))
            .collect();
        
        let conversion_time = conversion_start.elapsed();
        
        // 实际压缩
        let compression_start = Instant::now();
        let compressed = do_png_compression(&original_data, quality)
            .expect("Compression failed");
        let compression_time = compression_start.elapsed();
        
        let total_time = total_start.elapsed();
        
        println!("  数据转换耗时: {:?} ({:.1}%)", 
            conversion_time,
            conversion_time.as_millis() as f64 / total_time.as_millis() as f64 * 100.0
        );
        println!("  实际压缩耗时: {:?} ({:.1}%)", 
            compression_time,
            compression_time.as_millis() as f64 / total_time.as_millis() as f64 * 100.0
        );
        println!("  总耗时: {:?}", total_time);
        
        let compression_ratio = (1.0 - compressed.len() as f64 / original_data.len() as f64) * 100.0;
        println!("  压缩比: {:.1}% ({} -> {})", 
            compression_ratio,
            format_bytes(original_data.len()),
            format_bytes(compressed.len())
        );
    }
    
    println!("\n💡 性能优化建议:");
    println!("  1. 🚀 减少数据拷贝: 当前方法创建了完整的RGBA副本");
    println!("  2. 📦 使用引用: 如果库支持，直接传递slice引用");
    println!("  3. � 并行处理: 大图片可以分块并行转换");
    println!("  4. 💾 流式处理: 避免全部数据加载到内存");
    println!("  5. ⚡ SIMD优化: 使用向量指令加速像素转换");
}

fn format_bytes(bytes: usize) -> String {
    if bytes >= 1024 * 1024 {
        format!("{:.2}MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.2}KB", bytes as f64 / 1024.0)
    } else {
        format!("{}B", bytes)
    }
}
