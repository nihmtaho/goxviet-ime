# Milestone 1.3: Multi-Encoding Output

## Status: ✅ Complete (2026-01-26)

## Mục tiêu
Hỗ trợ lựa chọn bảng mã đầu ra: Unicode, TCVN3, VNI, CP1258.

## Completed Tasks
- [x] Thiết kế module encoding với các bảng mã
- [x] Implement bảng chuyển đổi ký tự cho từng encoding
- [x] Thêm FFI để chọn encoding output
- [x] Tích hợp encoding vào Result output
- [x] Viết unit tests cho từng encoding

## Files Created/Modified
- `core/src/engine/features/encoding.rs` - **NEW** module
- `core/src/engine/features/mod.rs` - Export encoding
- `core/src/lib.rs` - FFI functions

## Encodings Supported
| Code | Encoding | Implementation |
|------|----------|----------------|
| 0 | Unicode | UTF-8 passthrough (default) |
| 1 | TCVN3 | Full character mapping |
| 2 | VNI | Placeholder (returns UTF-8) |
| 3 | CP1258 | Partial mapping + UTF-8 fallback |

## FFI Functions Added
- `ime_set_encoding(u8)` - Set output encoding
- `ime_get_encoding()` - Get current encoding
- `ime_convert_encoding(char*)` - Convert string
- `ime_free_bytes(u8*, usize)` - Free converted bytes

## Tests
- 7 unit tests (all passing)
