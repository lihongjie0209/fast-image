#!/bin/bash

# Multi-platform build script for Fast Image JNI Library
# This script compiles the Rust library for multiple architectures and platforms

echo "🚀 Building Fast Image JNI Library for Multiple Platforms..."

# Set up variables
RUST_PROJECT_DIR="c:/Users/lihongjie/RustroverProjects/fast-image"
JAVA_PROJECT_DIR="d:/code/fast-image-java"
RESOURCE_DIR="$JAVA_PROJECT_DIR/src/main/resources/native"

# Navigate to Rust project
cd "$RUST_PROJECT_DIR"

# Define target platforms
TARGETS=(
    "x86_64-pc-windows-msvc"      # Windows 64-bit MSVC
    "x86_64-unknown-linux-gnu"    # Linux 64-bit
    "x86_64-apple-darwin"         # macOS Intel
    "aarch64-apple-darwin"        # macOS Apple Silicon
)

# Optional targets (require cross-compilation setup)
OPTIONAL_TARGETS=(
    "aarch64-pc-windows-msvc"     # Windows ARM64  
    "aarch64-unknown-linux-gnu"   # Linux ARM64
)

echo "📦 Installing Rust targets..."

# Install targets
for target in "${TARGETS[@]}"; do
    echo "Installing target: $target"
    rustup target add $target
done

echo "🔨 Building for all platforms..."

# Create native resources directory
mkdir -p "$RESOURCE_DIR"

# Build for each target
for target in "${TARGETS[@]}"; do
    echo "Building for $target..."
    
    cargo build --release --target=$target
    
    if [ $? -eq 0 ]; then
        echo "✅ Build successful for $target"
        
        # Copy the built library to Java resources with appropriate naming
        case $target in
            "x86_64-pc-windows-msvc")
                if [ -f "target/$target/release/fast_image.dll" ]; then
                    cp "target/$target/release/fast_image.dll" "$RESOURCE_DIR/fast_image-windows-x86_64.dll"
                    echo "  📁 Copied to fast_image-windows-x86_64.dll"
                fi
                ;;
            "x86_64-unknown-linux-gnu")
                if [ -f "target/$target/release/libfast_image.so" ]; then
                    cp "target/$target/release/libfast_image.so" "$RESOURCE_DIR/libfast_image-linux-x86_64.so"
                    echo "  📁 Copied to libfast_image-linux-x86_64.so"
                fi
                ;;
            "x86_64-apple-darwin")
                if [ -f "target/$target/release/libfast_image.dylib" ]; then
                    cp "target/$target/release/libfast_image.dylib" "$RESOURCE_DIR/libfast_image-macos-x86_64.dylib"
                    echo "  📁 Copied to libfast_image-macos-x86_64.dylib"
                fi
                ;;
            "aarch64-apple-darwin")
                if [ -f "target/$target/release/libfast_image.dylib" ]; then
                    cp "target/$target/release/libfast_image.dylib" "$RESOURCE_DIR/libfast_image-macos-aarch64.dylib"
                    echo "  📁 Copied to libfast_image-macos-aarch64.dylib"
                fi
                ;;
        esac
    else
        echo "❌ Build failed for $target"
    fi
done

echo "📋 Build Summary:"
echo "Native libraries created in: $RESOURCE_DIR"
ls -la "$RESOURCE_DIR"

echo "🎉 Multi-platform build completed!"
