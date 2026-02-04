# Release Notes - v2.0.5

**Released:** 2026-02-04

## Overview

v2.0.5 là một patch release tập trung vào sửa lỗi quan trọng liên quan đến VNI input method. Release này khắc phục vấn đề auto-capitalization khi Caps Lock tắt, cải thiện độ ổn định của logic phát hiện shift key trong macOS InputManager.

## 🐛 Bug Fixes

### Auto-caps VNI Input (Critical)
- **Vấn đề**: VNI input method tự động viết hoa ký tự dù Caps Lock và modifier keys đều tắt
- **Nguyên nhân**: Logic phát hiện trạng thái phím Shift không chính xác khi buffer rỗng
- **Giải pháp**: Hoàn thiện logic phát hiện shift key, thêm guard condition để tránh gọi Shift handler khi không cần thiết
- **Ảnh hưởng**: Sửa trực tiếp lỗi auto-caps, cải thiện user experience khi gõ VNI

### Shift Key Detection Refinement
- Cải thiện tính chính xác của phát hiện trạng thái phím Shift
- Tránh xử lý Shift không cần thiết khi buffer rỗng
- Giảm false positive trong shift key detection logic

## 🔧 Chores & Improvements

### Test Coverage
- Thêm comprehensive edge case tests cho auto-capitalization handling
- Đảm bảo fix ổn định trên các scenario gõ khác nhau:
  - Gõ VNI với Caps Lock tắt
  - Gõ Telex kết hợp Shift key
  - Gõ tiếng Anh trong macOS Input Manager

### Code Signing & Release Workflow
- Cải thiện code signing process trong release workflow
- Bảo đảm macOS app ký đúng chuẩn phát hành
- Tối ưu hóa GitHub Actions release workflow

## 📊 Technical Details

### Commits
- **9dfd445**: `fix: Refine shift key detection to prevent auto-capitalization issues`
- **180ec7f**: `chore(main): merge develop for v2.0.5 release`

### Branch
- Merged from `develop` (bugfix/auto-caps-vni-input)
- Base: v2.0.4
- Target: main

### Compatibility
- **macOS**: 10.15+
- **Windows**: 10+
- **Breaking Changes**: None
- **Migration**: No action required, direct upgrade recommended

## 🙏 Acknowledgments

Cảm ơn tất cả contributors đã giúp phát hiện và fix lỗi auto-caps VNI input!

## 📊 Full Changelog

Xem [CHANGELOG.md](./CHANGELOG.md) để xem lịch sử đầy đủ của tất cả các thay đổi.

## 🔗 Downloads & Assets

Xem [GitHub Releases](https://github.com/nihmtaho/goxviet-ime/releases/tag/v2.0.5) để tải phiên bản mới.

## 📝 Known Issues

- None reported at this time

## 🐛 Bug Reports

Nếu bạn gặp phải bất kỳ issue nào, vui lòng báo cáo tại [GitHub Issues](https://github.com/nihmtaho/goxviet-ime/issues).

---

**Released:** 2026-02-04 (v2.0.5)

For questions or issues, please open a [GitHub Issue](https://github.com/nihmtaho/goxviet-ime/issues).
