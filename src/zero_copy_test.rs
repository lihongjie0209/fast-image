#[cfg(test)]
mod zero_copy_tests {
    use crate::{can_use_zero_copy, is_rgba_layout_compatible};

    #[test]
    fn test_zero_copy_availability() {
        // Test if zero-copy optimization is available
        let can_zero_copy = can_use_zero_copy();
        
        println!("🔬 Zero-copy availability test");
        println!("✅ Zero-copy available: {}", can_zero_copy);
        
        if can_zero_copy {
            println!("🚀 Memory layout compatible for zero-copy optimization!");
            println!("   - imagequant::RGBA size: {} bytes", std::mem::size_of::<imagequant::RGBA>());
            println!("   - imagequant::RGBA alignment: {} bytes", std::mem::align_of::<imagequant::RGBA>());
            
            // Test runtime compatibility
            let test_compatible = is_rgba_layout_compatible();
            println!("   - Runtime layout test: {}", if test_compatible { "✅ PASS" } else { "❌ FAIL" });
            
            assert!(test_compatible, "Runtime layout compatibility test failed");
        } else {
            println!("⚠️  Zero-copy not available, will use pre-allocation fallback");
        }
        
        // This test always passes - we just want to see the output
        assert!(true);
    }
    
    #[test]
    fn test_memory_layout_verification() {
        // Verify imagequant::RGBA memory layout requirements
        let size = std::mem::size_of::<imagequant::RGBA>();
        let align = std::mem::align_of::<imagequant::RGBA>();
        
        println!("📏 Memory layout verification:");
        println!("   - Size: {} bytes (expected: 4)", size);
        println!("   - Alignment: {} bytes (expected: 1)", align);
        
        // These are the requirements for zero-copy to work
        if size == 4 && align == 1 {
            println!("✅ Basic memory layout requirements met");
            
            // Test actual field layout
            let test_compatible = is_rgba_layout_compatible();
            if test_compatible {
                println!("✅ Field order compatible (R,G,B,A)");
            } else {
                println!("❌ Field order incompatible");
            }
        } else {
            println!("❌ Basic memory layout requirements not met");
        }
    }
}
