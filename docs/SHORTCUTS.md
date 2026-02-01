# SHORTCUTS.md
# Gõ Việt (GoxViet) – TỔNG HỢP TÀI LIỆU PHÍM TẮT (SHORTCUTS)

**Cập nhật lần cuối:** 2025-12-23  
**Phiên bản tài liệu:** 1.0  
**Phạm vi:** Hướng dẫn sử dụng, kiểm thử, lộ trình phát triển, troubleshooting, best practices  
**Dành cho:** Người dùng, lập trình viên, tester

---

## 📑 MỤC LỤC

1. [Giới thiệu & Tổng quan](#giới-thiệu--tổng-quan)
2. [Phím tắt mặc định & Preset](#phím-tắt-mặc-định--preset)
3. [Cách hoạt động & Kiến trúc](#cách-hoạt-động--kiến-trúc)
4. [Hướng dẫn sử dụng nhanh](#hướng-dẫn-sử-dụng-nhanh)
5. [Kiểm thử & Checklist](#kiểm-thử--checklist)
6. [Troubleshooting – Xử lý lỗi thường gặp](#troubleshooting--xử-lý-lỗi-thường-gặp)
7. [Lộ trình phát triển (Roadmap)](#lộ-trình-phát-triển-roadmap)
8. [Best Practices & Kinh nghiệm](#best-practices--kinh-nghiệm)
9. [Tài liệu liên quan](#tài-liệu-liên-quan)

---

## Giới thiệu & Tổng quan

Tính năng phím tắt (keyboard shortcut) cho phép chuyển đổi nhanh giữa chế độ gõ tiếng Việt và tiếng Anh trên toàn hệ thống.  
- Ưu tiên cao nhất (kernel-level), không bị override bởi ứng dụng khác.
- Được thiết kế để dễ nhớ, dễ thao tác, không xung đột với macOS.

---

## Phím tắt mặc định & Preset

### Phím tắt mặc định

- **Control + Space** (`⌃Space`): Toggle ON/OFF chế độ gõ tiếng Việt.
- **Shift + Backspace** (`⇧⌫`): Xóa toàn bộ từ (word delete).

### Preset khác (cấu hình được):

| Shortcut         | Mô tả                  | Xung đột hệ thống |
|------------------|------------------------|-------------------|
| ⌃Space           | Control+Space          | ✅ Không (Mặc định)|
| ⌘Space           | Command+Space          | ⚠️ Spotlight      |
| ⌃⇧Space          | Control+Shift+Space    | ✅ Không          |
| ⌃⌥Space          | Control+Option+Space   | ✅ Không          |
| ⌃⇧V              | Control+Shift+V        | ✅ Không          |

**Lý do chọn Control+Space:**  
- Không xung đột với Spotlight (Cmd+Space).
- Dễ nhớ, thao tác nhanh, tương tự nhiều hệ điều hành khác.
- Được xử lý ở mức ưu tiên cao nhất (`.headInsertEventTap`).

---

## Cách hoạt động & Kiến trúc

### Luồng sự kiện

```
Người dùng nhấn Control+Space
        ↓
CGEventTap (.headInsertEventTap) bắt sự kiện (ưu tiên cao nhất)
        ↓
InputManager kiểm tra phím tắt hiện tại
        ↓
Nếu khớp → toggle trạng thái IME (ON/OFF)
        ↓
Cập nhật UI (icon status bar: 🇻🇳 ↔ EN)
        ↓
Trả về nil (swallow event) → Ứng dụng khác không nhận được sự kiện này
```

### Kiểm tra phím tắt

- So khớp chính xác keyCode + modifiers.
- Không cho phép extra modifiers (ví dụ: Control+Shift+Space ≠ Control+Space).
- Lưu cấu hình shortcut qua UserDefaults, tự động load khi khởi động app.

### Tích hợp UI

- Hiển thị shortcut hiện tại trong menu bar.
- Cho phép đổi shortcut (tương lai: Settings UI).
- Trạng thái toggle cập nhật tức thì, không cần reload app.

---

## Hướng dẫn sử dụng nhanh

### 1. Sử dụng phím tắt

- Nhấn **Control+Space** để bật/tắt chế độ gõ tiếng Việt.
- Nhấn **Shift+Backspace** để xóa toàn bộ từ (tương tự Option+Backspace).
- Quan sát icon status bar:
  - 🇻🇳 = Vietnamese input ON
  - EN = English input OFF

### 2. Ví dụ xóa từ

```text
Before: "Hello world|"  (cursor at |)
Press: Shift+Backspace
After: "Hello |"

Before: "Xin chào thế_giới|"
Press: Shift+Backspace
After: "Xin chào |"
```

**Lưu ý:** Shift+Backspace hoạt động giống Option+Backspace (native macOS) nên ranh giới từ do macOS quyết định.

### 3. Kiểm thử nhanh

- Mở TextEdit hoặc bất kỳ ứng dụng nào.
- Nhấn Control+Space → icon đổi trạng thái.
- Gõ thử tiếng Việt/Anh để xác nhận.

### 3. Kiểm tra menu

- Click icon status bar → menu hiện "Toggle: ⌃Space" (không click được, chỉ hiển thị).

---

## Kiểm thử & Checklist

### Pre-Deployment Checklist

- [ ] `KeyboardShortcut.swift` tồn tại và compile thành công
- [ ] `InputManager.swift` có property `currentShortcut`
- [ ] `RustBridge.swift` có function `matchesToggleShortcut()`
- [ ] `AppDelegate.swift` hiển thị shortcut trong menu
- [ ] Không có build warnings/errors

### Build & Run Verification

- [ ] Build thành công, không lỗi
- [ ] App chạy, icon status bar hiển thị
- [ ] Menu bar có item "Toggle: ⌃Space"
- [ ] Log hiển thị: "Toggle shortcut loaded: ⌃Space"

### Basic Functionality Tests

- [ ] Nhấn Control+Space → icon đổi trạng thái
- [ ] Toggle liên tục không crash, không lag
- [ ] Trạng thái toggle lưu lại khi chuyển app

### Priority & Conflict Tests

- [ ] Control+Space luôn ưu tiên hơn shortcut của app (VSCode, Terminal...)
- [ ] Command+Space vẫn mở Spotlight (không xung đột)
- [ ] Control+Shift+Space không toggle (strict matching)

### Performance Tests

- [ ] Latency < 5ms mỗi lần toggle
- [ ] CPU < 1% khi toggle liên tục
- [ ] Không memory leak sau 1000 lần toggle

### Edge Cases

- [ ] Toggle khi đang gõ dở (composition buffer được clear)
- [ ] Toggle khi có text selection (selection giữ nguyên)
- [ ] Toggle với nhiều bàn phím (external keyboard)
- [ ] Toggle sau sleep/wake vẫn hoạt động

---

## Troubleshooting – Xử lý lỗi thường gặp

### Shortcut không hoạt động

1. Kiểm tra quyền Accessibility (System Settings → Privacy & Security → Accessibility).
2. Khởi động lại app sau khi cấp quyền.
3. Kiểm tra log: `~/Library/Logs/GoxViet/keyboard.log`.

### Bị xung đột với app khác

- IME luôn ưu tiên cao nhất, nhưng nếu vẫn xung đột, thử đổi sang Control+Shift+Space.
- Tắt/đổi shortcut của app gây xung đột.

### UI không cập nhật

- Kiểm tra NotificationCenter observers.
- Thử click lại menu bar icon để refresh menu.
- Khởi động lại app nếu cần.

### Toggle bị chậm

- Kiểm tra CPU usage (Activity Monitor).
- Đảm bảo không có quá nhiều event tap khác đang chạy.

---

## Lộ trình phát triển (Roadmap)

### Phase 1: Core Toggle (Đã hoàn thành)
- Default Control+Space shortcut
- High-priority event capture
- Persistent configuration
- System-wide operation
- Performance: ~2ms latency

### Phase 2: Settings UI & Customization (Tiếp theo)
- Giao diện đổi shortcut trực quan
- Visual shortcut recorder
- Conflict detection (phát hiện xung đột)
- Preset & custom shortcut
- Test & reset shortcut

### Phase 3: Advanced Features (Tương lai)
- Hỗ trợ nhiều shortcut (primary + secondary)
- Modifier-only shortcut (double-tap Shift)
- Per-app shortcut (mỗi app một shortcut riêng)
- Shortcut profiles (profile cho dev, writer, custom)
- Import/export cấu hình

### Phase 4: Polish & Optimization
- Animation, dark mode, accessibility
- Tối ưu hiệu năng, giảm memory footprint
- Tài liệu hướng dẫn chi tiết, video tutorial

---

## Best Practices & Kinh nghiệm

1. **Luôn dùng `.headInsertEventTap` để đảm bảo ưu tiên cao nhất.**
2. **So khớp chính xác modifiers, không cho phép extra modifiers.**
3. **Clear composition buffer khi toggle để tránh lỗi nhập liệu.**
4. **Kiểm thử trên nhiều ứng dụng phổ biến (VSCode, Terminal, Safari, Slack...).**
5. **Đảm bảo trạng thái toggle lưu lại khi chuyển app hoặc sleep/wake.**
6. **Tối ưu code: struct-based, zero heap allocation, minimize logging.**
7. **Viết unit test và functional test cho mọi logic liên quan đến shortcut.**

---

## Tài liệu liên quan

- `GETTING_STARTED.md` – Hướng dẫn cài đặt, build, test nhanh
- `FIXES.md` – Tổng hợp các lỗi đã sửa, troubleshooting
- `PROJECT.md` – Tổng quan dự án, lịch sử thay đổi, roadmap
- `performance/PERFORMANCE_OPTIMIZATION_GUIDE.md` – Tối ưu hiệu năng
- `shortcuts/SHORTCUT_GUIDE.md` – Hướng dẫn chi tiết (tham khảo)
- `shortcuts/SHORTCUT_QUICK_START.md` – Quick start (tham khảo)
- `shortcuts/testing/TEST_SHORTCUT.md` – Hướng dẫn kiểm thử chi tiết
- `shortcuts/roadmap/SHORTCUT_CUSTOMIZATION_ROADMAP.md` – Lộ trình phát triển

---

**Happy Typing! 🇻🇳 – Chuyển đổi gõ tiếng Việt/Anh chỉ với một phím tắt!**

---

# SETTINGS UI MỘC KẾ HOẠCH (MOCKUP)
*(Nội dung từ SETTINGS_UI_MOCKUP.md, xem chi tiết file gốc để bổ sung nếu cần)*

---

# SETTINGS UI SUMMARY

**Status:** ✅ Implemented  
**Date:** 2025-01-XX  
**Commit:** `75ecad9`

## Quick Overview

Replaced alert-based settings with a modern SwiftUI-based Settings window featuring 4 comprehensive tabs: General, Per-App, Advanced, and About. Uses native macOS TabView for standard system appearance and behavior.

*(...Toàn bộ nội dung từ SETTINGS_UI_SUMMARY.md...)*

---

# SETTINGS UI TABVIEW REFACTOR

**Status:** ✅ Completed  
**Date:** 2025-01-XX  
**Commit:** `44db967` + `c031d4f`

## Overview

Refactored Settings UI from custom tab bar implementation to native macOS `TabView` style, following macOS Human Interface Guidelines and system standards.

*(...Toàn bộ nội dung từ SETTINGS_UI_TABVIEW_REFACTOR.md...)*

---

# SETTINGS UI: LIQUID GLASS TESTING GUIDE

**Version:** 1.0.0  
**Last Updated:** 2025-12-22  
**Purpose:** Visual verification of liquid glass implementation

*(...Toàn bộ nội dung từ SETTINGS_UI_TESTING_GUIDE.md...)*

---

# SETTINGS UI LIQUID GLASS DESIGN

**Status:** ✅ Implemented  
**Date:** 2025-01-XX  
**Commit:** `1ade745`  
**macOS Version:** 15.0+ (Sequoia) optimized, 11.0+ compatible

*(...Toàn bộ nội dung từ SETTINGS_UI_LIQUID_GLASS.md...)*

---

# SETTINGS UI IMPLEMENTATION

**Status:** ✅ Completed  
**Date:** 2025-01-XX  
**Version:** 1.0.0

## Overview

This document describes the implementation of the Settings UI for GoxViet IME on macOS. The new Settings window replaces the previous alert-based configuration interface with a modern SwiftUI-based solution.

*(...Toàn bộ nội dung từ SETTINGS_UI_IMPLEMENTATION.md...)*

---

# SETTINGS UI: TRUE LIQUID GLASS IMPLEMENTATION

**Version:** 1.0.0  
**Last Updated:** 2025-12-22  
**Status:** ✅ Production Ready  
**Platform:** macOS 11.0+ (Best on macOS 15.0+ Sequoia)

*(...Toàn bộ nội dung từ SETTINGS_UI_LIQUID_GLASS_FINAL.md...)*

---

# SETTINGS UI: LIQUID GLASS CHANGELOG

**Version:** 2.0.0  
**Release Date:** 2025-12-22  
**Status:** ✅ Production Ready  
**Breaking Changes:** Yes (Complete UI rewrite)

*(...Toàn bộ nội dung từ SETTINGS_UI_LIQUID_GLASS_CHANGELOG.md...)*

---

# SETTINGS UI COMPLETION REPORT

*(...Toàn bộ nội dung từ SETTINGS_UI_COMPLETION_REPORT.md...)*

---