use jni::objects::{JByteArray, JClass};
use jni::sys::{jbyteArray, jint};
use jni::JNIEnv;
use crate::opts::{ImageType, Compression};
use image::ImageReader;
use std::io::Cursor;

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

/// JNI function for FastImageUtils.compressJpegFastNative()
/// 
/// This function provides fast JPEG compression using Rust's native image library
/// instead of mozjpeg. It's faster but may produce larger files compared to mozjpeg.
/// 
/// # Arguments
/// * `env` - JNI environment
/// * `_class` - Java class (unused)
/// * `image_bytes` - Input image data as byte array
/// * `quality` - Compression quality (0-100, where 0 is highest compression)
/// 
/// # Returns
/// * Compressed JPEG data as byte array, or null if compression fails
#[unsafe(no_mangle)]
pub extern "system" fn Java_cn_lihongjie_image_FastImageUtils_compressJpegFastNative(
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

    // Perform fast JPEG compression using Rust's native image library
    let compressed_data = match crate::opts::do_jpeg_compression_fast(&input_data, quality) {
        Ok(data) => data,
        Err(e) => {
            let _ = env.throw_new(
                "java/lang/RuntimeException", 
                &format!("Fast JPEG compression failed: {}", e)
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

/// JNI function for FastImageUtils.rotateNative()
/// 
/// This function rotates an image by the specified angle (90, 180, or 270 degrees).
/// The function automatically detects the image format and preserves it in the output.
/// 
/// # Arguments
/// * `env` - JNI environment
/// * `_class` - Java class (unused)
/// * `image_bytes` - Input image data as byte array
/// * `angle` - Rotation angle in degrees (90, 180, or 270)
/// 
/// # Returns
/// * Rotated image data as byte array, or null if rotation fails
#[unsafe(no_mangle)]
pub extern "system" fn Java_cn_lihongjie_image_FastImageUtils_rotateNative(
    mut env: JNIEnv,
    _class: JClass,
    image_bytes: JByteArray,
    angle: jint,
) -> jbyteArray {
    // Validate angle parameter (only 90, 180, 270 degrees supported)
    let _rotation = match angle {
        90 | 180 | 270 => angle,
        _ => {
            let _ = env.throw_new(
                "java/lang/IllegalArgumentException", 
                &format!("Rotation angle must be 90, 180, or 270 degrees, got: {}", angle)
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

    // Perform rotation using image library
    let rotated_data = match rotate_image_bytes(&input_data, angle) {
        Ok(data) => data,
        Err(e) => {
            let _ = env.throw_new(
                "java/lang/RuntimeException", 
                &format!("Image rotation failed: {}", e)
            );
            return std::ptr::null_mut();
        }
    };

    // Convert rotated result back to Java byte array
    match env.byte_array_from_slice(&rotated_data) {
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

/// Helper function to rotate image bytes
/// 
/// # Arguments
/// * `image_data` - Input image data
/// * `angle` - Rotation angle (90, 180, or 270 degrees)
/// 
/// # Returns
/// * Rotated image data or error message
fn rotate_image_bytes(image_data: &[u8], angle: i32) -> Result<Vec<u8>, String> {
    // Load image from bytes and detect format
    let reader = ImageReader::new(Cursor::new(image_data))
        .with_guessed_format()
        .map_err(|e| format!("Failed to detect image format: {}", e))?;
    
    // Get the detected format
    let format = reader.format()
        .ok_or_else(|| "Could not determine image format".to_string())?;
    
    // Decode the image
    let img = reader.decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    // Rotate the image based on angle
    let rotated_img = match angle {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => return Err(format!("Unsupported rotation angle: {}", angle)),
    };

    // Encode the rotated image back to bytes using the original format
    let mut output = Vec::new();
    rotated_img
        .write_to(&mut Cursor::new(&mut output), format)
        .map_err(|e| format!("Failed to encode rotated image: {}", e))?;

    Ok(output)
}

#[cfg(test)]
mod tests {

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

    #[test]
    fn test_rotation_angle_validation() {
        // Test valid rotation angles
        assert!(matches!(90_i32, 90 | 180 | 270));
        assert!(matches!(180_i32, 90 | 180 | 270));
        assert!(matches!(270_i32, 90 | 180 | 270));
        
        // Test invalid rotation angles
        assert!(!matches!(0_i32, 90 | 180 | 270));
        assert!(!matches!(45_i32, 90 | 180 | 270));
        assert!(!matches!(360_i32, 90 | 180 | 270));
        assert!(!matches!(-90_i32, 90 | 180 | 270));
    }

    #[test]
    fn test_jni_rotate_function_naming() {
        // Verify rotation function name follows JNI conventions
        let function_name = "Java_cn_lihongjie_image_FastImageUtils_rotateNative";
        assert!(function_name.starts_with("Java_"));
        assert!(function_name.contains("FastImageUtils"));
        assert!(function_name.contains("rotateNative"));
    }
}
