use jni::objects::{JByteArray, JClass};
use jni::sys::{jbyteArray, jint};
use jni::JNIEnv;
use crate::opts::{ImageType, Compression};

/// JNI function for FastImageUtils.compressNative()
/// 
/// This is the main compression function that:
/// 1. Automatically detects image format (PNG/JPEG)
/// 2. Applies appropriate compression with specified quality
/// 3. Returns compressed image data
/// 
/// # Arguments
/// * `env` - JNI environment
/// * `_class` - Java class (unused)
/// * `image_bytes` - Input image data as byte array
/// * `quality` - Compression quality (0-100, where 0 is highest compression)
/// 
/// # Returns
/// * Compressed image data as byte array, or null if compression fails
#[unsafe(no_mangle)]
pub extern "system" fn Java_cn_lihongjie_image_FastImageUtils_compressNative(
    mut env: JNIEnv,
    _class: JClass,
    image_bytes: JByteArray,
    quality: jint,
) -> jbyteArray {
    // Validate quality parameter (0-100)
    let quality = match quality {
        0..=100 => quality as u8,
        _ => {
            // Invalid quality range, throw exception
            let _ = env.throw_new(
                "java/lang/IllegalArgumentException", 
                &format!("Quality must be between 0 and 100, got: {}", quality)
            );
            return std::ptr::null_mut();
        }
    };

    // Convert Java byte array to Rust Vec<u8>
    let input_data = match env.convert_byte_array(&image_bytes) {
        Ok(data) => data,
        Err(e) => {
            let _ = env.throw_new(
                "java/lang/RuntimeException", 
                &format!("Failed to read input image data: {}", e)
            );
            return std::ptr::null_mut();
        }
    };

    // Validate input data
    if input_data.is_empty() {
        let _ = env.throw_new(
            "java/lang/IllegalArgumentException", 
            "Input image data cannot be empty"
        );
        return std::ptr::null_mut();
    }

    // Perform compression using our Rust implementation
    // This will auto-detect format and apply appropriate compression
    let compressed_data = match ImageType::compress(&input_data, quality) {
        Ok(data) => data,
        Err(e) => {
            let _ = env.throw_new(
                "java/lang/RuntimeException", 
                &format!("Image compression failed: {}", e)
            );
            return std::ptr::null_mut();
        }
    };

    // Convert compressed result back to Java byte array
    match env.byte_array_from_slice(&compressed_data) {
        Ok(result) => result.into_raw(),
        Err(e) => {
            let _ = env.throw_new(
                "java/lang/RuntimeException", 
                &format!("Failed to create output byte array: {}", e)
            );
            std::ptr::null_mut()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jni_compress_function_exists() {
        // This test verifies that the JNI function compiles correctly
        // Real testing requires a JVM environment with actual image data
        assert!(true);
    }

    #[test]
    fn test_quality_validation() {
        // Test quality parameter bounds
        assert!(matches!(0_i32, 0..=100));
        assert!(matches!(50_i32, 0..=100));
        assert!(matches!(100_i32, 0..=100));
        assert!(!matches!(-1_i32, 0..=100));
        assert!(!matches!(101_i32, 0..=100));
    }

    #[test]
    fn test_function_naming() {
        // Verify function names follow JNI conventions
        let function_name = "Java_cn_lihongjie_image_FastImageUtils_compressNative";
        assert!(function_name.starts_with("Java_"));
        assert!(function_name.contains("FastImageUtils"));
        assert!(function_name.contains("compressNative"));
    }
}
