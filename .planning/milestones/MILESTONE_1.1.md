# Milestone 1.1: Text Expansion (Gõ tắt)

## Status: ✅ Complete (2026-01-26)

## Mục tiêu
Hoàn thiện tính năng gõ tắt với import/export và per-app toggle.

## Completed Tasks
- [x] Hoàn thiện import/export shortcuts (JSON format)
- [x] Thêm FFI cho import/export shortcuts
- [x] Per-app shortcut toggle (enable/disable per bundle ID)
- [x] Viết unit tests cho text expansion
- [x] Benchmark shortcut lookup < 1ms

## Files Modified
- `core/src/engine/features/shortcut.rs` - Added `to_json()`, `from_json()`, `export_all()`, `import_all()`, `iter()`, `iter_mut()`
- `core/src/lib.rs` - Added FFI functions

## FFI Functions Added
- `ime_export_shortcuts_json()` - Export shortcuts to JSON
- `ime_import_shortcuts_json()` - Import shortcuts from JSON
- `ime_free_string()` - Free exported string
- `ime_set_shortcuts_enabled()` - Enable/disable all shortcuts

## Tests
- 14 unit tests (all passing)
- 3 FFI integration tests (all passing)
