mod opts;

pub use opts::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_type_detection() {
        // Test PNG detection
        let png_header = b"\x89PNG\r\n\x1a\n";
        assert!(matches!(ImageType::detect_type(png_header), Some(ImageType::PNG)));
        
        // Test JPEG detection  
        let jpeg_header = b"\xff\xd8\xff";
        assert!(matches!(ImageType::detect_type(jpeg_header), Some(ImageType::JPEG)));
        
        // Test unknown format
        let unknown = b"unknown format";
        assert!(ImageType::detect_type(unknown).is_none());
    }
}
