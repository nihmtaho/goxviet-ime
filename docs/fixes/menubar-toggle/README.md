# MENUBAR TOGGLE FIXES - DOCUMENTATION INDEX

**Thư mục:** `docs/fixes/menubar-toggle/`  
**Chủ đề:** Sửa lỗi, cải tiến, testing, và quyết định kiến trúc liên quan đến Toggle (bật/tắt tiếng Việt) trên Menu Bar macOS.

---

## 📑 MỤC LỤC

### 1. Tổng quan & Lịch sử thay đổi

- [CHANGELOG_TOGGLE_FIX.md](CHANGELOG_TOGGLE_FIX.md)  
  **Lịch sử thay đổi, các phiên bản fix, quyết định kiến trúc qua từng giai đoạn.**

- [TOGGLE_FIX_SUMMARY.md](TOGGLE_FIX_SUMMARY.md)  
  **Tóm tắt nhanh các vấn đề, giải pháp, kết quả kiểm thử.**

---

### 2. Phân tích & Giải pháp kỹ thuật

- [MENUBAR_APPEARANCE_FIX.md](MENUBAR_APPEARANCE_FIX.md)  
  **Phân tích và fix warning deprecated API về appearance.**

- [MENUBAR_TOGGLE_CUSTOM_CONTROL.md](MENUBAR_TOGGLE_CUSTOM_CONTROL.md)  
  **Giải pháp custom NSControl để chống dimming khi mất focus (v2.0.0).**

- [MENUBAR_TOGGLE_SWIFTUI_DECISION.md](MENUBAR_TOGGLE_SWIFTUI_DECISION.md)  
  **Phân tích lý do revert về SwiftUI Toggle, chấp nhận behavior chuẩn macOS (v2.1.0).**

---

### 3. Testing & Checklist

- [TOGGLE_TESTING_CHECKLIST.md](TOGGLE_TESTING_CHECKLIST.md)  
  **Checklist kiểm thử toggle/focus/fix.**

- [TESTING_V2_FOCUS_FIX.md](TESTING_V2_FOCUS_FIX.md)  
  **Hướng dẫn kiểm thử chi tiết cho fix focus/dimming v2.0.0 & v2.1.0.**

---

### 4. Tổng kết & So sánh giải pháp

- [TOGGLE_V2_SUMMARY.md](TOGGLE_V2_SUMMARY.md)  
  **Tóm tắt executive cho giải pháp custom control (v2.0.0).**

- [TOGGLE_V2.1_SUMMARY.md](TOGGLE_V2.1_SUMMARY.md)  
  **Tóm tắt executive cho quyết định dùng SwiftUI Toggle (v2.1.0).**

---

## 🗂️ MỤC ĐÍCH THƯ MỤC

Thư mục này tập trung toàn bộ tài liệu liên quan đến:
- Sửa lỗi toggle mất màu khi mất focus
- Quyết định chọn giải pháp SwiftUI hay custom control
- Checklist và hướng dẫn kiểm thử
- Changelog và tổng kết các phiên bản fix
- Đảm bảo tài liệu luôn đúng chuẩn cấu trúc dự án

---

## 📚 LIÊN KẾT LIÊN QUAN

- [../backspace/](../backspace/) — Fix liên quan phím Backspace
- [../arrow-keys/](../arrow-keys/) — Fix liên quan phím mũi tên
- [../telex/](../telex/) — Fix liên quan chuyển đổi Telex

---

## 📝 HƯỚNG DẪN ĐÓNG GÓP

- Khi thêm tài liệu mới về toggle/menu bar, hãy đặt vào thư mục này.
- Đặt tên file IN HOA, rõ ràng, đúng chuẩn (ví dụ: `MENUBAR_TOGGLE_NEW_FIX.md`).
- Cập nhật README này để bổ sung vào mục lục.
- Đảm bảo liên kết nội bộ trỏ đúng path mới.

---

**Cập nhật lần cuối:** 2025-12-20  
**Người quản lý:** Vietnamese IME Documentation Team
