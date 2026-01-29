---
name: "🐛 Bug Report"
about: Report a bug, logic error, or platform issue for GoxViet IME
title: "[Bug] Tính năng bỏ dấu tự do hoạt động không ổn định"
labels: ["bug", "core", "telex"]
assignees: []
---

## 📝 Summary

Tính năng bỏ dấu tự do (free tone) hoạt động không ổn định, đôi khi cho phép đặt dấu ở vị trí tùy ý nhưng đôi khi lại không hoạt động hoặc bị reset về chế độ bỏ dấu chuẩn.

## 🕹 Steps to Reproduce

1. Bật tính năng "Bỏ dấu tự do" trong Settings
2. Gõ một từ tiếng Việt với dấu ở vị trí tùy ý (ví dụ: "hoas" thay vì "hoas" -> "hóa")
3. Quan sát kết quả - đôi khi dấu được đặt đúng vị trí mong muốn, đôi khi bị đặt theo quy tắc chuẩn
4. Thử gõ nhiều từ liên tiếp - tính năng có thể bị tắt hoặc reset về chế độ chuẩn

## 📊 Comparison

| 🛑 Actual Result | ✅ Expected Result |
| :--- | :--- |
| Dấu đôi khi được đặt theo quy tắc chuẩn thay vì vị trí tùy ý | Dấu luôn được đặt ở vị trí người dùng chỉ định khi bật chế độ tự do |

### 🔍 Test Cases / Examples

| Input sequence | Actual result | Expected result | Note |
| :--- | :--- | :--- | :--- |
| `h-o-a-s` | `hóa` (đúng) hoặc `hoá` (sai) | `hóa` (dấu ở 'o') | Không ổn định |
| `v-i-e-t-s` | `việt` (đúng) hoặc `viết` (sai) | `việt` (dấu ở 'e') | Không ổn định |
| `t-u-o-i-r` | `tuỏi` (đúng) hoặc `tuổi` (sai) | `tuỏi` (dấu ở 'o') | Không ổn định |

---

## 💻 Environment

- **OS:** macOS 14+
- **Application:** VSCode, TextEdit, Chrome, etc.
- **GoxViet Version:** v2.0.0
- **Input Method:** Telex

## 📁 Additional Context

Vấn đề có thể liên quan đến:
- Logic kiểm tra `free_tone_enabled` flag trong engine
- Xung đột với thuật toán nhận diện tiếng Anh hoặc Vietnamese validator
- Reset flag khi xử lý backspace hoặc các phím đặc biệt
- Điều kiện kiểm tra trong hàm `handle_normal_letter()` hoặc `apply_tone()`

Cần kiểm tra:
- File `core/src/engine/mod.rs` - các hàm liên quan đến tone placement
- Điều kiện `if !self.free_tone_enabled` trong code
- Các điểm có thể reset hoặc thay đổi flag `free_tone_enabled`

---
> [!TIP]
> Suggested labels: `bug`, `core`, `telex`, `free-tone`, `tone-placement`
