# 📝 Release Notes - Phiên bản 2.0.15

**Ngày phát hành:** 2026-05-01
**Phiên bản:** 2.0.15
**PRs:**
- [#82 – feat(core,macos): US1–US5 feature gap + US2 bracket shortcut fixes](https://github.com/nihmtaho/goxviet-ime/pull/82)
- [#80 – fix(core): block circumflex on V1+tone+V2+V2 + fix doubled Telex tone-marker consonants](https://github.com/nihmtaho/goxviet-ime/pull/80)
- [#83 – refactor(core): wire DI factory functions and remove dead code in SOLID container](https://github.com/nihmtaho/goxviet-ime/pull/83)
- [#84 – fix(macos): macOS input pipeline bug fixes & optimization](https://github.com/nihmtaho/goxviet-ime/pull/84)

---

## ✨ Tính năng mới

### ESC Restore (US1) (PR #82)

Nhấn **ESC** khi đang gõ tiếng Việt sẽ khôi phục lại chuỗi phím gốc (raw keystrokes), hoàn tác mọi biến đổi Telex/VNI.

**Ví dụ:** Gõ `viet` → hiện `việt` → nhấn ESC → trả về `viet` nguyên bản.

**Bật/tắt:** Settings › Editing › ESC Restore.

---

### Bracket Shortcuts (US2) (PR #82)

Gõ `[` và `]` để tạo ngoặc vuông tiêu chuẩn trong khi đang ở chế độ Vietnamese — không còn bị transform thành ký tự khác.

**Bật/tắt:** Settings › Editing › Bracket Shortcuts.

---

### Foreign Consonants Pass-Through (US3) (PR #82)

Phụ âm `f`, `j`, `w`, `z` khi đứng đầu âm tiết sẽ được pass-through nguyên vẹn, hỗ trợ gõ từ nước ngoài và tên riêng (wifi, zoom, javascript…).

**Bật/tắt:** Settings › Editing › Foreign Consonant Pass-Through.

---

### Auto-Capitalise (US4) (PR #82)

Tự động viết hoa chữ cái đầu từ khi bắt đầu câu mới (sau dấu `.`, `!`, `?`). Tương thích với Smart Mode.

**Bật/tắt:** Settings › Editing › Auto-Capitalise.

---

### Word History / Backspace-After-Space (US5) (PR #82)

Nhấn Backspace ngay sau khi Space sẽ khôi phục lại từ vừa commit vào buffer để tiếp tục chỉnh sửa. Lịch sử lưu tối đa 10 từ.

**Bật/tắt:** Settings › Editing › Backspace-After-Space Restore.

---

## 🐛 Sửa lỗi

### Block circumflex trên V1+tone+V2+V2 (PR #80)

Sửa lỗi Telex không chặn circumflex khi một nguyên âm khác trong buffer đã mang dấu:

- `tafoo` → `tàoo` (không phải `tàô`)
- `chaofo` → `chàoo` (không phải `chàô`)
- `mufaa` → `mùaa` (không phải `muầ`)

### Doubled Telex tone-marker consonants (PR #80)

Từ tiếng Anh có phím tone-marker bị gõ đôi sẽ được tự động sửa khi nhấn SPACE:

- `inffer` → `infer `
- `caffe` → `cafe `
- `conffer` → `confer `

Không ảnh hưởng các từ tiếng Anh có double-consonant thực sự (offer, differ, correct).

### Capitalized Vietnamese dictionary lookup (PR #80)

Từ tiếng Việt viết hoa đầu câu (`Trường`, `Không`, `Việt`) không còn bị auto-restore về English khi nhấn SPACE.

### macOS input pipeline optimization (PR #84)

Cải thiện độ ổn định của CGEventTap pipeline, PerAppModeManagerEnhanced, và TextInjectionHelper; tăng cường logging để dễ debug hơn.

---

## ⚠️ Known Issues

- Không có vấn đề đã biết trong phiên bản này.
