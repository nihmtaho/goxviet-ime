# Gõ Việt (GoxViet)

Bộ gõ tiếng Việt hiệu suất cao, đa nền tảng (macOS/Windows), core engine viết bằng Rust.

[![Latency <3ms](https://img.shields.io/badge/latency-<3ms-brightgreen)]()
[![Memory Safe](https://img.shields.io/badge/memory-safe-blue)]()
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey)]()

---

## 🚀 Tính năng chính

- Độ trễ < 3ms, memory-safe tuyệt đối (Rust)
- Trải nghiệm native, toggle nhanh (Control+Space)
- Hỗ trợ Telex, VNI, smart "ươ", undo/backspace thông minh
- Đa nền tảng: macOS (Swift), Windows (TSF, đang phát triển)

---

## 🖥️ Platform Support

- **macOS 15+**: Hỗ trợ đầy đủ, sử dụng Swift/CGEvent & Accessibility API.
- **Windows**: Đang phát triển (TSF, Visual Studio 2022).

## ⚡ Cài đặt & Build nhanh

**Yêu cầu:**  
- Rust 1.70+, macOS 11+ (Xcode 14+), hoặc Windows 10+ (Visual Studio 2022)

**Build & chạy:**
```sh
cd core && cargo build --release
cd ../platforms/macos/goxviet && open goxviet.xcodeproj
# Build & Run (⌘R), cấp quyền Accessibility cho "GoxViet"
```

---

## 📄 License

[Add license information here]

---

**Gõ Việt (GoxViet)** – Made with ❤️ for the Vietnamese community
