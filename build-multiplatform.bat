@echo off
REM Multi-platform build script for Fast Image JNI Library (Windows)

echo 🚀 Building Fast Image JNI Library for Multiple Platforms...

REM Set up variables
set RUST_PROJECT_DIR=c:\Users\lihongjie\RustroverProjects\fast-image
set JAVA_PROJECT_DIR=d:\code\fast-image-java
set RESOURCE_DIR=%JAVA_PROJECT_DIR%\src\main\resources\native

REM Navigate to Rust project
cd /d "%RUST_PROJECT_DIR%"

echo 📦 Installing Rust targets...

REM Install targets
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

echo 🔨 Building for all platforms...

REM Create native resources directory
if not exist "%RESOURCE_DIR%" mkdir "%RESOURCE_DIR%"

REM Build for Windows x64
echo Building for x86_64-pc-windows-msvc...
cargo build --release --target=x86_64-pc-windows-msvc

if %ERRORLEVEL% equ 0 (
    echo ✅ Build successful for Windows x64
    if exist "target\x86_64-pc-windows-msvc\release\fast_image.dll" (
        copy "target\x86_64-pc-windows-msvc\release\fast_image.dll" "%RESOURCE_DIR%\fast_image-windows-x86_64.dll"
        echo   📁 Copied to fast_image-windows-x86_64.dll
    )
) else (
    echo ❌ Build failed for Windows x64
)

REM Build for Linux x64 (cross-compilation may not work on Windows without additional setup)
echo Building for x86_64-unknown-linux-gnu...
cargo build --release --target=x86_64-unknown-linux-gnu

if %ERRORLEVEL% equ 0 (
    echo ✅ Build successful for Linux x64
    if exist "target\x86_64-unknown-linux-gnu\release\libfast_image.so" (
        copy "target\x86_64-unknown-linux-gnu\release\libfast_image.so" "%RESOURCE_DIR%\libfast_image-linux-x86_64.so"
        echo   📁 Copied to libfast_image-linux-x86_64.so
    )
) else (
    echo ⚠️ Build failed for Linux x64 (cross-compilation may require additional setup)
)

REM Build for macOS Intel (cross-compilation may not work on Windows)
echo Building for x86_64-apple-darwin...
cargo build --release --target=x86_64-apple-darwin

if %ERRORLEVEL% equ 0 (
    echo ✅ Build successful for macOS Intel
    if exist "target\x86_64-apple-darwin\release\libfast_image.dylib" (
        copy "target\x86_64-apple-darwin\release\libfast_image.dylib" "%RESOURCE_DIR%\libfast_image-macos-x86_64.dylib"
        echo   📁 Copied to libfast_image-macos-x86_64.dylib
    )
) else (
    echo ⚠️ Build failed for macOS Intel (cross-compilation may require additional setup)
)

REM Build for macOS Apple Silicon (cross-compilation may not work on Windows)
echo Building for aarch64-apple-darwin...
cargo build --release --target=aarch64-apple-darwin

if %ERRORLEVEL% equ 0 (
    echo ✅ Build successful for macOS Apple Silicon
    if exist "target\aarch64-apple-darwin\release\libfast_image.dylib" (
        copy "target\aarch64-apple-darwin\release\libfast_image.dylib" "%RESOURCE_DIR%\libfast_image-macos-aarch64.dylib"
        echo   📁 Copied to libfast_image-macos-aarch64.dylib
    )
) else (
    echo ⚠️ Build failed for macOS Apple Silicon (cross-compilation may require additional setup)
)

echo 📋 Build Summary:
echo Native libraries created in: %RESOURCE_DIR%
dir "%RESOURCE_DIR%"

echo 🎉 Multi-platform build completed!
pause
