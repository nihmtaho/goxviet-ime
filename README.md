# Gõ Việt (GoxViet)

Bộ gõ tiếng Việt hiệu suất cao với Core engine bằng Rust, hỗ trợ đa nền tảng (macOS/Windows).

[![Performance](https://img.shields.io/badge/latency-<3ms-brightgreen)]()
[![Memory Safe](https://img.shields.io/badge/memory-safe-blue)]()
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey)]()

---

## ✨ Tính năng nổi bật

- **⚡ Hiệu suất cao:** Độ trễ < 3ms, nhanh hơn 47× so với các giải pháp thông thường
- **🛡️ An toàn bộ nhớ:** 100% memory-safe với Rust Core
- **🎯 Trải nghiệm native:** Hoạt động mượt mà như bộ gõ gốc
- **⌨️ Toggle nhanh:** Control+Space để chuyển đổi, priority cao nhất (không bị override)
- **🔧 Đa nền tảng:** macOS (Swift/IMKit) và Windows (C++/TSF)

---

## 🚀 Cài đặt nhanh

### Yêu cầu hệ thống

- **Rust:** 1.70+ (`rustup install stable`)
- **macOS:** Xcode 14+, Swift 5.7+, macOS 11+
- **Windows:** Visual Studio 2022, C++20 *(đang phát triển)*

### Build và chạy

```bash
# 1. Build Rust Core
cd core
cargo build --release

# 2. Build macOS App
cd ../platforms/macos/goxviet
open goxviet.xcodeproj
# Nhấn ⌘R để Build & Run

# 3. Cấp quyền Accessibility
# System Settings → Privacy & Security → Accessibility
# Bật "GoxViet"
```

📖 **Hướng dẫn chi tiết:** [`docs/QUICK_START.md`](docs/QUICK_START.md)

---

## 📁 Cấu trúc dự án & tài liệu

```
goxviet/
├── core/                     # Rust Engine (IME logic)
│   ├── src/engine/           # Core processing
│   ├── src/input/            # Input method handlers
│   └── tests/                # Unit & integration tests
│
├── platforms/                # Platform implementations
│   ├── macos/goxviet/        # Swift/CGEvent & Accessibility API
│   └── windows/goxviet/      # C++/TSF (in development)
│
├── docs/                     # 📚 Tài liệu (theo chủ đề, master file)
│   ├── README.md                 # Danh mục tài liệu & hướng dẫn tra cứu
│   ├── STRUCTURE_VISUAL.md       # Sơ đồ visual cấu trúc docs
│   ├── DOCUMENTATION_STRUCTURE.md# Hướng dẫn cấu trúc & migration
│   ├── getting-started/          # Hướng dẫn bắt đầu, testing
│   ├── shortcuts/                # Phím tắt, roadmap, testing shortcut
│   ├── fixes/                    # Tổng hợp fix (backspace, arrow, telex, menubar, ...)
│   ├── performance/              # Tối ưu hiệu năng, benchmark, guides
│   ├── project/                  # Quản lý dự án, trạng thái, changelog
│   ├── release-note/             # Ghi chú phát hành
│   └── archive/                  # Lưu trữ tài liệu cũ, tổng hợp lịch sử
│
└── .github/instructions/         # Project guidelines
```

---

## 📚 Tài liệu & Tra cứu

### Danh mục tài liệu chính (theo chủ đề)

| Chủ đề                | File chính (master)                              | Mô tả ngắn                    |
|-----------------------|--------------------------------------------------|-------------------------------|
| Bắt đầu nhanh         | [getting-started/QUICK_START.md](docs/getting-started/QUICK_START.md) | Hướng dẫn cài đặt, build, test|
| Phím tắt & Shortcut   | [shortcuts/SHORTCUT_GUIDE.md](docs/shortcuts/SHORTCUT_GUIDE.md)      | Cấu hình & ưu tiên phím tắt   |
| Sửa lỗi & Fixes       | [fixes/BACKSPACE_FIX.md](docs/fixes/backspace/BACKSPACE_FIX.md)      | Tổng hợp fix backspace, arrow, telex, v.v. |
| Tối ưu hiệu năng      | [performance/PERFORMANCE_OPTIMIZATION_GUIDE.md](docs/performance/guides/PERFORMANCE_OPTIMIZATION_GUIDE.md) | Hướng dẫn tối ưu, benchmark   |
| Quản lý dự án         | [project/PROJECT_STATUS.md](docs/project/PROJECT_STATUS.md)          | Trạng thái, lộ trình, branding|
| Ghi chú phát hành     | [release-note/RELEASE_NOTE_1.3.2.md](docs/release-note/RELEASE_NOTE_1.3.2.md) | Thông tin các bản phát hành   |
| Lưu trữ (archive)     | [archive/FIX_SUMMARY.md](docs/archive/FIX_SUMMARY.md)                | Tài liệu lịch sử, tổng hợp    |

- 📖 **Tổng mục lục & hướng dẫn:** [`docs/README.md`](docs/README.md)
- 📊 **Sơ đồ cấu trúc & visual:** [`docs/STRUCTURE_VISUAL.md`](docs/STRUCTURE_VISUAL.md)
- 🗂️ **Chi tiết cấu trúc & migration:** [`docs/DOCUMENTATION_STRUCTURE.md`](docs/DOCUMENTATION_STRUCTURE.md)

---

### Hướng dẫn tra cứu nhanh

- **Bắt đầu:** `docs/getting-started/QUICK_START.md`
- **Tìm shortcut:** `docs/shortcuts/SHORTCUT_GUIDE.md`
- **Tìm fix lỗi:** `docs/fixes/backspace/BACKSPACE_FIX.md` hoặc các thư mục con trong `fixes/`
- **Tối ưu hiệu năng:** `docs/performance/guides/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Kiểm tra trạng thái dự án:** `docs/project/PROJECT_STATUS.md`
- **Xem lịch sử phát hành:** `docs/release-note/`
- **Tìm tài liệu cũ:** `docs/archive/`

> Khi tham chiếu tài liệu, luôn dùng đường dẫn tương đối theo chuẩn mới (xem ví dụ trong `docs/DOCUMENTATION_STRUCTURE.md`).

---

---

## ⚡ Hiệu suất

### Metrics đạt được

| Chỉ số | Mục tiêu | Đạt được | Cải thiện |
|--------|----------|----------|-----------|
| Single keystroke | < 16ms | 1-3ms | ✅ 5-16× nhanh hơn |
| Backspace (10 ký tự) | < 160ms | < 3ms | ✅ 50× nhanh hơn |
| Editor deletion | < 16ms | < 1ms | ✅ 47× nhanh hơn |
| Memory safety | 100% | 100% | ✅ Zero leaks |

### Tối ưu hóa chính

- **Editor Optimization (VSCode/Zed):** Instant injection với zero delays → 47× nhanh hơn
- **Backspace Optimization:** O(1) cho ký tự thường, O(syllable) cho phức tạp → 3-15× nhanh hơn
- **Smart App Detection:** Tự động chọn injection method phù hợp

**Chi tiết:** [`docs/PERFORMANCE_COMPARISON.md`](docs/PERFORMANCE_COMPARISON.md)

**Chạy benchmark:**
```bash
./test-performance.sh
```

---

## 🎨 Tính năng

### Toggle Shortcut

- **Default:** Control+Space (⌃Space) để chuyển đổi gõ Việt ↔ English
- **High Priority:** Sử dụng `.headInsertEventTap` - luôn capture trước tất cả ứng dụng
- **No Conflicts:** Không xung đột với Spotlight hay các system shortcuts khác
- **Customizable:** Có thể đổi sang Control+Shift+Space, Control+Option+Space, etc.
- **Persistent:** Lưu cấu hình qua UserDefaults

📖 **Chi tiết:** [`docs/SHORTCUT_GUIDE.md`](docs/SHORTCUT_GUIDE.md)

### Input Methods

- **Telex:** `aa→â`, `aw→ă`, `oo→ô`, `ow→ơ`, `dd→đ`, `s/f/r/x/j` (dấu thanh)
- **VNI:** `6→mũ`, `7→móc`, `8→trăng`, `9→đ`, `1-5` (dấu thanh)
- **Smart "ươ":** `u`+`o`+`w` → `ươ` (intelligent detection)
- **Undo/Backspace:** Smart restore với syllable-based rebuild

### Tone Placement

- **Modern style:** `hoà`, `thuỷ` *(dấu ở nguyên âm cuối)*
- **Old style:** `hòa`, `thủy` *(dấu ở nguyên âm đầu)*
- Configurable - có thể chuyển đổi

### Platform Support

- ✅ **macOS 11+:** CGEvent & Accessibility API, system-wide support
- 🚧 **Windows 10+:** TSF integration *(đang phát triển)*

---

## 🧪 Testing

```bash
# 1. Rust core tests
cd core
cargo test

# 2. Performance tests
./test-performance.sh

# 3. Manual testing
# Type: "duongw" → "đương" ✅
# Type: "viets" → "việt" ✅
# Type: "duocwj khongf" → "được không" ✅
```

**Hướng dẫn chi tiết:** [`docs/TESTING_GUIDE.md`](docs/TESTING_GUIDE.md)

---

## 🔧 Development

### Build Commands

```bash
# Rust Core
cd core
cargo build --release
cargo test
cargo clippy

# macOS
cd platforms/macos/goxviet
xcodebuild -scheme goxviet build
# Hoặc: open goxviet.xcodeproj

# Windows (in development)
cd platforms/windows/goxviet
msbuild goxviet.sln /p:Configuration=Release
```

---

## 🤝 Contributing

Chúng tôi hoan nghênh mọi đóng góp! Vui lòng:

1. Đọc project guidelines trong [`.github/instructions/`](.github/instructions/)
2. Follow coding style và naming conventions
3. Add tests cho features mới
4. Update documentation (tên IN HOA trong `docs/`)
5. Tuân thủ performance targets (< 16ms)

---

## 🚀 Roadmap

### Phase 1: Core Features ✅ COMPLETE

- [x] Keyboard shortcut toggle (Control+Space)
- [x] High-priority event capture (never overridden)
- [x] Persistent shortcut configuration
- [x] System-wide operation

### Phase 2: Shortcut Customization 🎯 NEXT

- [ ] **Settings UI panel for shortcut customization**
  - Visual shortcut recorder (like macOS System Settings)
  - Live preview of shortcut conflicts
  - Preset shortcuts selector (Control+Shift+Space, Control+Option+Space, etc.)
  - Test shortcut button (verify it works)
  - Reset to default option

- [ ] **Advanced Shortcut Features**
  - Multiple shortcut support (primary + secondary)
  - Modifier-only shortcuts (double-tap Shift, double-tap Control)
  - Per-app shortcut overrides (different shortcut per app)
  - Shortcut profiles (switch between profiles)
  - Import/export shortcut configurations

- [ ] **Conflict Detection & Resolution**
  - Real-time system shortcut conflict warnings
  - App-specific conflict detection (VSCode, Terminal, etc.)
  - Automatic conflict resolution suggestions
  - Disable conflicting app shortcuts option

### Phase 3: Enhanced Features

- [ ] Dictionary/Autocomplete
- [ ] Emoji picker
- [ ] Cloud sync settings (including shortcuts)
- [ ] Windows platform support

---

## 📊 Current Status

**Version:** 0.1.0  
**Status:** ✅ Active Development  
**Last Updated:** 2025-01-20

### Achievements

- 🚀 47× nhanh hơn cho deletion trong modern editors
- ⚡ < 3ms latency (vượt mục tiêu < 16ms)
- 🐛 Tất cả critical bugs đã được fix
- 📚 Tài liệu đầy đủ (25 files, 6,500+ dòng)
- ✅ Production ready cho macOS

---

## 📄 License

[Add license information here]

---

## 🙏 Acknowledgments

Cảm ơn cộng đồng phát triển bộ gõ tiếng Việt đã đóng góp kiến thức và kinh nghiệm!

---

## 📞 Support

- **Documentation:** [`docs/`](docs/) - 25 files, 6,500+ dòng
- **Quick Start:** [`docs/QUICK_START.md`](docs/QUICK_START.md)
- **Issues:** [GitHub Issues](../../issues)

---

---

**Gõ Việt (GoxViet)** - Made with ❤️ for the Vietnamese community