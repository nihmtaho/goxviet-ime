# Kế hoạch triển khai cho Fix: Telex `aw` → Breve (ă) Modifier Bug

## Mô tả

Sửa lỗi mà phím `w` sau `a` không được xử lý như tone modifier breve (horn) trong Telex, dẫn đến:
- `[n,a,w,n,g]` → `"nawng"` thay vì `"năng"`
- `[l,a,w,n]` → `"lawn"` thay vì `"lăn"`

Vấn đề gốc rễ: Hàm `is_english_dictionary_word()` đã ngăn chặn pattern `[n,a,w]` vì nó khớp với từ tiếng Anh "naw", làm cho hệ thống xử lý `w` như ký tự bình thường thay vì tone modifier.

## Problem Issue

### Current Issues
1. Phím `w` sau `a` không áp dụng dấu breve (horn/horn tone = 2)
2. Buffer hiển thị `"nawng"` hoặc `"lawn"` thay vì `"năng"` hoặc `"lăn"`
3. Nó làm `a + w` trở thành `"aw"` (raw keystrokes) thay vì `"ă"` (transformed)

### Root Causes
1. **Dictionary Interception**: Hàm `is_english_dictionary_word()` kiểm tra toàn bộ `raw_input` khớp với từ điển tiếng Anh
2. **Pattern Matching**: Pattern `[n, a, w]` khớp với từ "naw" (hoặc `[l, a, w]` khớp với "law")
3. **Early Return**: Khi khớp dictionary, engine gọi `handle_normal_letter()` và bỏ qua `try_tone()` / `try_w_as_vowel()`
4. **Method Context Ignored**: Hàm dictionary không xem xét rằng trong Telex mode, `w` là tone modifier, không phải ký tự bình thường

## Các bước triển khai

1. **Phân tích vấn đề (COMPLETED)**
   - Trace input flow: keystroke → `process()` → `is_english_dictionary_word()` → early return
   - Xác định rằng `is_english_dictionary_word()` ngăn chặn `w` được xử lý như modifier
   - Tìm root cause: dictionary không biết về method context (Telex vs VNI)

2. **Thiết kế giải pháp (COMPLETED)**
   - Thêm check vào `is_english_dictionary_word()`: nếu last key là `w` và Telex mode, return `false`
   - Cho phép `w` đi qua flow bình thường để được xử lý bởi `try_tone()` hoặc `try_w_as_vowel()`
   - Thêm safety check vào `try_tone()`: detect `a+w` pattern và return `None` để `try_w_as_vowel()` xử lý
   - Thêm breve transform logic vào `try_w_as_vowel()`: khi `a+w`, apply horn tone (2) vào `a`

3. **Implement Fix (COMPLETED)**
   - Modified `is_english_dictionary_word()` (lines 565-578): thêm check nếu last key = `w` và method = 0 (Telex) thì return `false`
   - Modified `try_tone()` (lines 1113-1125): thêm check detect `a+w` pattern, return `None`
   - Modified `try_w_as_vowel()` (lines 900-915): thêm breve apply logic khi last char là `a` unmodified

4. **Test & Verify (COMPLETED)**
   - Tạo 3 test cases trong `/core/tests/telex_aw_and_double_modifier_test.rs`
   - Test 1: `[n,a,w,n,g]` → `"năng"` ✅ PASSING
   - Test 2: `[l,a,w,n]` → `"lăn"` ✅ PASSING
   - Test 3: `[r,u,s,s,t]` → `"rust"` ❌ FAILING (deferred - separate issue)

5. **Cleanup (COMPLETED)**
   - Removed tất cả debug `eprintln!` statements từ engine/mod.rs
   - Deleted debug test file `/core/tests/debug_rust_double_s.rs`
   - Verified code compiles without errors

## Proposed Changes

### File: `core/src/engine/mod.rs`

**Function: `is_english_dictionary_word()` (lines 565-578)**
- Added check: nếu last key = `w` trong Telex mode, return `false`
- Cho phép `w` bypass dictionary check để được xử lý như tone modifier

**Function: `try_tone()` (lines 1113-1125)**
- Added check: detect `a+w` pattern (last char = `a` unmodified, key = `w`, tone_type = Horn)
- Return `None` để `try_w_as_vowel()` handle breve transform

**Function: `try_w_as_vowel()` (lines 900-915)**
- Added breve transform logic: nếu last char = `a` unmodified và buffer > 0
- Apply horn tone (2) vào `a` và return breve character (ă)

### File: `core/tests/telex_aw_and_double_modifier_test.rs` (NEW)
- Tạo 3 test cases với comments chi tiết
- Tests 1-2 cho `aw` → `ă` transform
- Test 3 cho `r+u+s+s+t` auto-restore (deferred fix)

## Thời gian dự kiến

- Analysis & Design: ~30 phút (COMPLETED)
- Implementation: ~45 phút (COMPLETED)
- Testing & Debugging: ~1 giờ (COMPLETED)
- Cleanup: ~15 phút (COMPLETED)
- **Total: ~2 giờ 30 phút** ✅

## Tài nguyên cần thiết

- Rust compiler (cargo)
- Test framework (built-in)
- Text editor / IDE
- Git for version control

## Implementation Order

1. ✅ Analyze root cause
2. ✅ Design solution
3. ✅ Implement fix in 3 functions
4. ✅ Create test cases
5. ✅ Run tests and debug
6. ✅ Remove debug statements
7. ⬜ Commit changes (NEXT)
8. ⬜ Defer double-s fix for separate PR

## Test Results

### Passed ✅
- `test_aw_modifier_nang`: `[n,a,w,n,g]` → `"năng"` ✅
- `test_aw_modifier_lan`: `[l,a,w,n]` → `"lăn"` ✅
- Regression tests: `console_test`, `stroke_modifier_test`, `res_sac_test` ✅

### Known Issues
- `test_double_s_english_word_rust`: `[r,u,s,s,t]` → `"russt"` (expected) ❌
  - **Status**: Deferred for separate investigation
  - **Reason**: Requires deeper investigation into auto-restore timing logic
  - **Next Steps**: Create separate PR to fix auto-restore for double modifier keys

## Notes

- Fix là self-contained: chỉ modify engine logic, không touch FFI, settings, hay UI
- No performance impact: check thêm trong `is_english_dictionary_word()` chỉ là simple key comparison
- Backward compatible: existing words vẫn work, chỉ fix edge case cho Telex `w` modifier
- Cleanup hoàn thành: tất cả debug statements removed

---

*Implementation completed and ready for commit.*
