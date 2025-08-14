# 图片文件夹说明

## 📁 文件结构

```
images/
├── 🖼️ 原始测试图片
│   ├── SamplePNGImage_5mbmb.png    (5.01MB - PNG测试图片)
│   └── SampleJPGImage_5mbmb.jpg    (5.02MB - JPEG测试图片)
│
├── 🎨 PNG压缩结果  
│   ├── test_png_compressed_q30.png (1.68MB - 质量30)
│   ├── test_png_compressed_q50.png (1.98MB - 质量50)
│   ├── test_png_compressed_q70.png (2.02MB - 质量70)
│   └── test_png_compressed_q90.png (2.32MB - 质量90)
│
├── 📷 JPEG压缩结果
│   ├── test_jpeg_compressed_q30.jpg (229.50KB - 质量30)
│   ├── test_jpeg_compressed_q50.jpg (389.35KB - 质量50) 
│   ├── test_jpeg_compressed_q70.jpg (745.89KB - 质量70)
│   └── test_jpeg_compressed_q90.jpg (2.58MB - 质量90)
│
└── 🧪 其他测试文件
    ├── original.png               (1.24KB - 测试生成)
    ├── compressed.png            (1.41KB - 测试生成)
    ├── compressed.jpg            (0.45KB - 测试生成)
    └── compressed.webp           (0.16KB - 测试生成)
```

## 📊 文件大小对比

| 文件类型 | 原始大小 | 最佳压缩 | 压缩率 | 推荐质量 |
|---------|----------|----------|--------|----------|
| PNG | 5.01MB | 1.68MB | 66.4% | 质量30-50 |
| JPEG | 5.02MB | 229.50KB | 95.5% | 质量30-70 |

## 🎯 使用建议

### PNG 文件选择
- **1.68MB (质量30)**: 最大压缩，颜色数量最少
- **1.98MB (质量50)**: 平衡压缩比和质量
- **2.02MB (质量70)**: 标准质量，适合大多数用途  
- **2.32MB (质量90)**: 高质量，最接近原图

### JPEG 文件选择
- **229KB (质量30)**: 极致压缩，适合缩略图
- **389KB (质量50)**: 网页优化，加载速度快
- **746KB (质量70)**: 标准质量，适合大多数展示
- **2.58MB (质量90)**: 高质量，适合打印或专业用途

---

*所有测试文件均由 fast-image 库生成*
