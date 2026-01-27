# PHASE 1: Core Engine – Text Expansion, Shift+Backspace, Multi-Encoding

## Mục tiêu
- Hoàn thiện các tính năng cốt lõi của engine: Gõ tắt, Shift+Backspace, Multi-Encoding
- Đảm bảo hiệu suất <16ms, undo/redo chuẩn, không panic qua FFI

## Milestones
- [x] Milestone 1.1: Hoàn thành Text Expansion (Gõ tắt) ✅
- [x] Milestone 1.2: Tích hợp Shift+Backspace (xóa nhanh từ) ✅
- [x] Milestone 1.3: Hỗ trợ Multi-Encoding Output (Unicode, TCVN3, VNI, CP1258) ✅
- [ ] Milestone 1.4: Unit test & benchmark <1ms/keystroke

## Tiến độ: 75% (3/4 milestones)

## Hoàn thành (2026-01-26)
### Milestone 1.1: Text Expansion
- JSON import/export (`to_json()`, `from_json()`)
- FFI functions: `ime_export_shortcuts_json()`, `ime_import_shortcuts_json()`
- 14 unit tests pass

### Milestone 1.2: Shift+Backspace  
- `handle_shift_backspace()` method
- Delete entire word with Shift+DELETE
- 5 unit tests pass

### Milestone 1.3: Multi-Encoding
- `encoding.rs` module with TCVN3, VNI, CP1258 support
- FFI: `ime_set_encoding()`, `ime_get_encoding()`, `ime_convert_encoding()`
- 7 unit tests pass

## Tiêu chí hoàn thành
- Tất cả tính năng hoạt động ổn định, pass unit test, không crash
- Được review và checklist kỹ lưỡng
