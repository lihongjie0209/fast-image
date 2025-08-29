use fast_image::*;
use std::fs;
use std::path::Path;

/// Compress a transparent-background PNG and write result to target/ for inspection.
///
/// The test searches for the input image at common locations:
/// - fast-image/images/transparent.png
/// - fast-image/examples/transparent.png
/// - fast-image/transparent.png
/// - workspace root: ../transparent.png
///
/// If the file is not found, the test exits early (no failure) to avoid
/// breaking CI; when present, it verifies transparency is preserved.
#[test]
fn compress_transparent_png_and_write_to_target() {
    let candidates = [
        "images/transparent.png",
        "examples/transparent.png",
        "transparent.png",
        "../transparent.png",
    ];

    let input_path = candidates
        .iter()
        .map(Path::new)
        .find(|p| p.exists())
        .map(|p| p.to_path_buf());

    let Some(input_path) = input_path else {
        eprintln!(
            "transparent.png not found; looked in: {:?}. Place your file at one of these paths to run this test.",
            &candidates
        );
        return; // Skip gracefully if the asset isn't available locally
    };

    let data = fs::read(&input_path).expect("Failed to read transparent.png");
    assert!(matches!(ImageType::detect_type(&data), Some(ImageType::PNG)));

    let quality = 75u8;
    let compressed = do_png_compression(&data, quality)
        .expect("PNG compression failed for transparent image");

    // Ensure target directory exists and write output
    let out_dir = Path::new("target");
    let _ = fs::create_dir_all(out_dir);
    let out_path = out_dir.join(format!("transparent_compressed_q{}.png", quality));
    fs::write(&out_path, &compressed).expect("Failed to write compressed transparent PNG");
    println!("Saved compressed transparent PNG: {}", out_path.display());

    // Validate that transparency is preserved (at least one pixel alpha < 255)
    let decoded = image::load_from_memory(&compressed)
        .expect("Failed to decode compressed PNG");
    let rgba = decoded.to_rgba8();
    let has_transparency = rgba.as_raw().chunks_exact(4).any(|px| px[3] < 255);
    assert!(
        has_transparency,
        "Compressed PNG appears to have lost transparency (no alpha < 255 found)"
    );
}
