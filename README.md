# Fast Image - 高性能图像压缩库

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)

一个用 Rust 编写的高性能图像压缩库，支持 PNG 和 JPEG 格式的智能压缩。

## 🏗️ 自动化构建

本项目使用 GitHub Actions 自动构建多平台原生库：

### 🎯 支持平台（自动构建）
- **Windows x64** (`fast_image-windows-x86_64.dll`)
- **Windows ARM64** (`fast_image-windows-aarch64.dll`)  
- **Linux x64** (`libfast_image-linux-x86_64.so`)
- **Linux ARM64** (`libfast_image-linux-aarch64.so`)
- **macOS Intel** (`libfast_image-macos-x86_64.dylib`)
- **macOS Apple Silicon** (`libfast_image-macos-aarch64.dylib`)

### 📦 获取预构建库

1. **从 GitHub Releases**（推荐）:
   - 访问 [Releases 页面](https://github.com/lihongjie0209/fast-image/releases)
   - 下载最新版本的所有平台库
   - 或下载特定平台的单个库文件

2. **手动触发构建**:
   ```bash
   # 创建新版本发布
   ./release.sh           # Linux/macOS
   ./release.bat          # Windows
   
   # 或推送版本标签
   git tag v0.2.1
   git push origin v0.2.1
   ```

3. **GitHub Actions 构建产物**:
   - 每次提交都会构建所有平台
   - 可从 Actions 标签页下载测试版本

## ✨ 特性

- 🚀 **高性能压缩**: 使用业界领先的压缩算法
- 🎯 **智能格式检测**: 自动识别图片格式
- ⚙️ **质量控制**: 支持0-100的质量参数调节
- 📊 **显著压缩效果**: PNG最高66.4%压缩率，JPEG最高95.5%压缩率
- 🛠️ **易于使用**: 简洁的API设计

## 🔧 技术栈

| 组件 | 库 | 用途 |
|------|----|----- |
| PNG压缩 | `imagequant` + `png` | 颜色量化 + 索引色彩PNG |
| JPEG压缩 | `mozjpeg` | 高质量JPEG压缩 |
| 图像处理 | `image` | 通用图像操作 |

## 📦 安装

将以下内容添加到你的 `Cargo.toml`:

```toml
[dependencies]
fast-image = "0.1.0"
```

## 🚀 快速开始

```rust
use fast_image::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取图片文件
    let image_data = fs::read("input.png")?;
    
    // 压缩图片 (质量: 0-100)
    let compressed = do_png_compression(&image_data, 70)?;
    
    // 保存压缩后的图片
    fs::write("output.png", compressed)?;
    
    Ok(())
}
```

## 📊 压缩效果对比

> 📋 **详细结果**: 查看 [压缩结果对比文档](COMPRESSION_RESULTS.md) 了解完整的图片对比和分析  
> ⚡ **性能分析**: 查看 [性能分析报告](PERFORMANCE_ANALYSIS.md) 了解数据转换开销和优化建议

我们使用5MB的测试图片进行了全面的压缩测试，结果如下：

### PNG 压缩结果

使用 `imagequant` 颜色量化 + PNG 索引色彩模式：

| 质量等级 | 文件大小 | 压缩率 | 文件大小减少 | 适用场景 |
|---------|----------|--------|-------------|----------|
| **原始** | 5.01MB | - | - | - |
| **30** | 1.68MB | **66.4%** | 3.33MB | 最大压缩 |
| **50** | 1.98MB | **60.4%** | 3.03MB | 平衡模式 |
| **70** | 2.02MB | **59.7%** | 2.99MB | 标准质量 |
| **90** | 2.32MB | **53.7%** | 2.69MB | 高质量 |

### JPEG 压缩结果

使用 `mozjpeg` 高性能JPEG压缩：

| 质量等级 | 文件大小 | 压缩率 | 文件大小减少 | 适用场景 |
|---------|----------|--------|-------------|----------|
| **原始** | 5.02MB | - | - | - |
| **30** | 229.50KB | **95.5%** | 4.79MB | 缩略图 |
| **50** | 389.35KB | **92.4%** | 4.63MB | 网页优化 |
| **70** | 745.89KB | **85.5%** | 4.28MB | 标准质量 |
| **90** | 2.58MB | **48.7%** | 2.44MB | 高质量 |

## 📈 压缩效果可视化

### 文件大小对比图

```
原始文件大小: 5MB
┌─────────────────────────────────────────────────────────┐
│ PNG压缩 (质量30): ████████████████░░░░░░░░░░░  66.4%减少  │
│ PNG压缩 (质量50): ███████████████░░░░░░░░░░░░  60.4%减少  │  
│ PNG压缩 (质量70): ███████████████░░░░░░░░░░░░  59.7%减少  │
│ PNG压缩 (质量90): ██████████████░░░░░░░░░░░░░  53.7%减少  │
├─────────────────────────────────────────────────────────┤
│ JPEG压缩(质量30): ████████████████████████████████████░  95.5%减少 │
│ JPEG压缩(质量50): ███████████████████████████████████░░  92.4%减少 │
│ JPEG压缩(质量70): ██████████████████████████████░░░░░░░  85.5%减少 │ 
│ JPEG压缩(质量90): ███████████████████░░░░░░░░░░░░░░░░░░░  48.7%减少 │
└─────────────────────────────────────────────────────────┘
```

## 🎯 使用建议

### PNG压缩建议
- **质量 30-50**: 适用于需要最大压缩的场景，可减少60-66%文件大小
- **质量 70-90**: 适用于需要保持较好质量的场景，可减少54-60%文件大小

### JPEG压缩建议  
- **质量 30-50**: 适用于网页缩略图，可减少92-95%文件大小
- **质量 70-90**: 适用于高质量展示，可减少49-86%文件大小

## 📖 API 文档

### 主要函数

```rust
// PNG压缩
pub fn do_png_compression(data: &[u8], quality: u8) -> Result<Vec<u8>, String>

// JPEG压缩  
pub fn do_jpeg_compression(data: &[u8], quality: u8) -> Result<Vec<u8>, String>

// 自动格式检测
impl ImageType {
    pub fn detect_type(data: &[u8]) -> Option<ImageType>
}

// 通用压缩接口
impl Compression for ImageType {
    fn compress(data: &[u8], quality: u8) -> Result<Vec<u8>, String>
}
```

### 使用示例

```rust
use fast_image::*;
use std::fs;

// 示例1: PNG压缩
fn compress_png() -> Result<(), String> {
    let png_data = fs::read("input.png").map_err(|e| e.to_string())?;
    let compressed = do_png_compression(&png_data, 70)?;
    fs::write("output.png", compressed).map_err(|e| e.to_string())?;
    Ok(())
}

// 示例2: JPEG压缩
fn compress_jpeg() -> Result<(), String> {
    let jpeg_data = fs::read("input.jpg").map_err(|e| e.to_string())?;
    let compressed = do_jpeg_compression(&jpeg_data, 80)?;
    fs::write("output.jpg", compressed).map_err(|e| e.to_string())?;
    Ok(())
}

// 示例3: 自动检测格式并压缩
fn auto_compress() -> Result<(), String> {
    let image_data = fs::read("input.jpg").map_err(|e| e.to_string())?;
    let compressed = ImageType::compress(&image_data, 75)?;
    fs::write("output_compressed", compressed).map_err(|e| e.to_string())?;
    Ok(())
}
```

## 🧪 测试

运行测试套件：

```bash
# 运行所有测试
cargo test

# 运行压缩测试（带详细输出）
cargo test --test compression_tests -- --nocapture

# 运行PNG压缩测试
cargo test test_png_compression_with_sample_file -- --nocapture

# 运行JPEG压缩测试  
cargo test test_jpeg_compression_with_sample_file -- --nocapture

# 运行性能基准测试
cargo test --test performance_benchmark -- --nocapture
```

## 🚀 性能基准

在5MB测试文件上的性能表现：

| 操作 | 平均耗时 | 内存使用 |
|------|----------|----------|
| PNG压缩 | ~13秒 | 低 |
| JPEG压缩 | ~15秒 | 低 |
| 格式检测 | <1ms | 极低 |

## 🔄 示例程序

查看 `examples/` 文件夹中的示例：

```bash
# 运行压缩示例
cargo run --example compress_image
```

## 🤝 贡献

欢迎提交 Pull Request 和 Issue！

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🔗 相关链接

- [imagequant](https://crates.io/crates/imagequant) - PNG颜色量化库
- [mozjpeg](https://crates.io/crates/mozjpeg) - 高性能JPEG压缩库  
- [png](https://crates.io/crates/png) - PNG编码解码库
- [image](https://crates.io/crates/image) - Rust图像处理库

---

**Fast Image** - 让图像压缩更简单、更高效！ 🚀
