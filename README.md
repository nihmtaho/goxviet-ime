# 🇻🇳 GoxViet (Gõ Việt)

**Bộ gõ tiếng Việt hiện đại, hiệu suất cực cao và an toàn với Rust.**

GoxViet là một bộ gõ tiếng Việt (IME) thế hệ mới, được thiết kế để mang lại trải nghiệm gõ phím mượt mà như native trên nhiều nền tảng (macOS và Windows). Với triết lý ưu tiên tốc độ, sự ổn định và hỗ trợ song ngữ thông minh.

[![Latency <3ms](https://img.shields.io/badge/latency-<3ms-brightgreen?style=for-the-badge)]()
[![Memory Safe](https://img.shields.io/badge/memory-safe-blue?style=for-the-badge)]()
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey?style=for-the-badge)]()

---

## ✨ Tại sao chọn GoxViet?

*   🚀 **Hiệu năng vượt trội**: Core engine được viết bằng Rust, đảm bảo độ trễ (latency) cực thấp (< 3ms) và quản lý bộ nhớ an toàn tuyệt đối.
*   🧠 **Auto-Restore thông minh**: Tự động nhận diện và khôi phục từ tiếng Anh khi gõ trong chế độ Telex/VNI, giúp gõ song ngữ không cần chuyển chế độ.
*   🎨 **Giao diện hiện đại**: Settings UI được thiết kế theo phong cách Liquid Glass, tích hợp sâu vào hệ điều hành.
*   ⌨️ **Hỗ trợ đầy đủ**: Đầy đủ các bảng mã và kiểu gõ phổ biến (Telex, VNI), cùng các tính năng thông minh như gõ "ươ" nhanh, sửa lỗi dấu tự động.

---

## 🛠️ Trạng thái dự án

| Nền tảng | Trạng thái | Tính năng chính |
| :--- | :--- | :--- |
| **macOS** | ✅ Sẵn sàng | Swift, Accessibility API, Hỗ trợ full app, Auto-update |
| **Windows** | 🚧 Đang phát triển | TSF Provider, Visual Studio 2022 |
| **Core** | ✅ Hoàn thiện | Rust, FSM-based, 100% Vietnamese logic accuracy |

---

## 🚀 Cài đặt & Sử dụng nhanh

### Build từ source (Dành cho Dev)

1.  **Yêu cầu**: Rust 1.70+, Xcode 14+ (macOS).
2.  **Build Core**:
    ```bash
    cd core && cargo build --release
    ```
3.  **Build macOS App**:
    Mở `platforms/macos/goxviet/goxviet.xcodeproj` bằng Xcode, chọn Scheme **GoxViet** và nhấn `Cmd + R`.

### Cấp quyền (macOS)
Để GoxViet có thể xử lý phím, bạn cần cấp quyền **Accessibility** trong:
`System Settings` → `Privacy & Security` → `Accessibility`.

---

## 🤝 Đóng góp & Phản hồi

Chúng tôi luôn hoan nghênh mọi đóng góp từ cộng đồng!

*   🐛 **Báo lỗi**: Phát hiện lỗi gõ hoặc lỗi ứng dụng? [Gửi Bug Report](https://github.com/nihmtaho/goxviet-ime/issues/new?template=bug_report.md)
*   💡 **Yêu cầu tính năng**: Bạn muốn GoxViet có thêm tính năng gì? [Gửi Feature Request](https://github.com/nihmtaho/goxviet-ime/issues/new?template=feature_request.md)
*   🔡 **Thêm từ tiếng Anh**: Từ tiếng Anh bạn gõ hay bị biến thành tiếng Việt? [Yêu cầu thêm từ vào Auto-Restore](https://github.com/nihmtaho/goxviet-ime/issues/new?template=english_word_request.md)

---

## 📁 Project Structure

Dự án được tổ chức theo kiến trúc hybrid với Core Engine (Rust) và Platform Layers (Native):

```
goxviet/
├── core/           # Rust core engine (logic, state, transform)
├── platforms/      # Platform implementations (macOS, Windows)
├── docs/           # Public documentation
├── .docs/          # Internal/developer documentation
├── scripts/        # Build scripts and utilities
└── .agent/         # AI agent skills
```

📋 Xem chi tiết tại [STRUCTURE.md](STRUCTURE.md) để hiểu rõ cấu trúc và biết nên đặt file ở đâu.

---

## 📚 Tài liệu tham khảo

Hệ thống tài liệu chi tiết giúp bạn bắt đầu nhanh chóng:

*   📖 [Hướng dẫn bắt đầu (Vietnamese)](docs/GETTING_STARTED.md)
*   🚀 [Hướng dẫn Release & Deployment](docs/RELEASE_AND_DEPLOYMENT.md)
*   ⌨️ [Danh sách phím tắt](docs/SHORTCUTS.md)
*   🛠️ [Hướng dẫn cho Developer (English Words)](docs/ADDING_ENGLISH_WORDS.md)
*   📝 [Release Notes](docs/release-note/)
*   📁 [Project Structure](STRUCTURE.md)

---

## 📄 License

Dự án này được phát triển cho cộng đồng người Việt. Thông tin chi tiết về License sẽ được cập nhật sớm.

---

**GoxViet** – Developed with ❤️ by Vietnamese Developers
