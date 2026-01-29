# Milestone 1.2: Shift+Backspace

## Status: ✅ Complete (2026-01-26)

## Mục tiêu
Thêm phím tắt Shift+Backspace để xóa nhanh một từ.

## Completed Tasks
- [x] Thêm xử lý Shift+Backspace trong engine
- [x] Xóa toàn bộ từ hiện tại trong buffer
- [x] Cập nhật FFI để hỗ trợ Shift+Backspace
- [x] Viết unit tests cho Shift+Backspace
- [x] Đảm bảo không gây lỗi buffer

## Files Modified
- `core/src/engine/mod.rs` - Added `handle_shift_backspace()`, Shift+DELETE detection in `on_key_ext()`

## Implementation
- `handle_shift_backspace()` calculates displayed word length (including Vietnamese diacritics)
- Clears entire buffer and returns backspace count
- Handles buffer-after-space and buffer-after-break scenarios

## Tests
- 5 unit tests (all passing):
  - `test_shift_backspace_simple_word`
  - `test_shift_backspace_vietnamese_word`
  - `test_shift_backspace_empty_buffer`
  - `test_shift_backspace_after_space`
  - `test_normal_backspace_still_works`
