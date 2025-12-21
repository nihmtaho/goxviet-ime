# Test Checklist: Backspace Fix

## 🎯 Mục đích

Kiểm tra xem phím **Backspace** đã hoạt động chính xác sau khi fix vấn đề đồng bộ giữa Engine và Screen.

---

## ✅ Test Cases

### 0a. CRITICAL: VSCode/Zed - Xóa sau khi commit word (MUST PASS!)

**App:** Bất kỳ (TextEdit, VSCode, Zed...)

**Input:**
```
đ → ư → ợ → c → SPACE → k → h → ô → n → g → BACKSPACE
```

**Expected:**
```
đ → ư → được → được → "được " → ... → "được không" → "được khôn"
```

**NOT:**
~~"được kkhôn"~~ ❌ (backspace count sai, chỉ xóa 9/10 ký tự)

**Actual:** ___________

**Status:** [ ] PASS  [ ] FAIL

**Note:** Test này kiểm tra backspace count có chính xác không. Nếu fail → Engine đếm buffer.len() sau khi pop thay vì old_length trước khi pop!

---

### 0b. CRITICAL: Backspace count sai - Fix "được kkhôn" bug (MUST PASS!)

**App:** VSCode hoặc Zed

**Input:**
```
g → õ → SPACE → BACKSPACE → BACKSPACE → BACKSPACE
```

**Expected:**
```
g → gõ → "gõ " (committed) → gõ (editable) → g → (empty)
```

**NOT:**
~~gõ → gõ → gõ (STUCK!)~~ ❌

**Actual:** ___________

**Status:** [ ] PASS  [ ] FAIL

**Note:** Đây là test case QUAN TRỌNG NHẤT! Nếu fail, backspace không hoạt động trên VSCode/Zed.

---

### 1. Basic Backspace - Xóa dấu thanh

**Input:**
```
a → a → s → BACKSPACE
```

**Expected:**
```
a → â → á → â
```

**Actual:** ___________

**Status:** [ ] PASS  [ ] FAIL

---

### 2. Basic Backspace - Xóa transform

**Input:**
```
d → d → BACKSPACE
```

**Expected:**
```
d → đ → d
```

**Actual:** ___________

**Status:** [ ] PASS  [ ] FAIL

---

### 3. Xóa liên tiếp

**Input:**
```
v → i → e → e → s → t → BACKSPACE → BACKSPACE → BACKSPACE
```

**Expected:**
```
v → vi → vi → viê → viê → việt → viê → vi → v
```

**Actual:** ___________

**Status:** [ ] PASS  [ ] FAIL

---

### 4. Xóa và gõ lại

**Input:**
```
a → a → BACKSPACE → s
```

**Expected:**
```
a → â → a → as
```

**NOT:** ~~âs~~ (Engine không được giữ buffer cũ!)

**Actual:** ___________

**Status:** [ ] PASS  [ ] FAIL

---

### 5. Xóa hết buffer

**Input:**
```
h → o → a → f → BACKSPACE → BACKSPACE → BACKSPACE → BACKSPACE
```

**Expected:**
```
h → ho → hoa → hoà → hoa → ho → h → (empty)
```

**Actual:** ___________

**Status:** [ ] PASS  [ ] FAIL

---

### 6. Backspace-after-space (Restore word)

**Input:**
```
h → o → a → f → SPACE → BACKSPACE
```

**Expected:**
```
h → ho → hoa → hoà → (commit) → hoà (editable!)
```

**Actual:** ___________

**Status:** [ ] PASS  [ ] FAIL

---

### 7. Multiple Backspace-after-space

**Input:**
```
v → i → e → e → t → SPACE → SPACE → BACKSPACE → BACKSPACE
```

**Expected:**
```
việt →  (2 spaces) → việt (1 space) → việt (editable!)
```

**Actual:** ___________

**Status:** [ ] PASS  [ ] FAIL

---

### 8. Xóa trong từ phức tạp

**Input:**
```
t → r → u → o → w → BACKSPACE
```

**Expected:**
```
t → tr → tru → truo → trươ → truo
```

**NOT:** ~~truơ~~ (phải revert đúng thứ tự!)

**Actual:** ___________

**Status:** [ ] PASS  [ ] FAIL

---

### 9. Xóa double letter transform

**Input:**
```
a → a → BACKSPACE → BACKSPACE
```

**Expected:**
```
a → â → a → (empty)
```

**Actual:** ___________

**Status:** [ ] PASS  [ ] FAIL

---

### 10. Xóa trong app đặc biệt (VSCode)

**App:** VSCode
**Input:**
```
v → i → e → e → s → t → BACKSPACE
```

**Expected:**
```
v → vi → vi → viê → viê → việt → viê
```

**Actual:** ___________

**Status:** [ ] PASS  [ ] FAIL

---

### 11. Xóa trong app đặc biệt (Zed)

**App:** Zed
**Input:**
```
t → h → a → n → h
```

**Expected:**
```
t → th → tha → than → thanh
```

**Then BACKSPACE x3:**
```
thanh → than → tha → th
```

**Actual:** ___________

**Status:** [ ] PASS  [ ] FAIL

---

### 12. Xóa trong Terminal

**App:** Terminal
**Input:**
```
e → c → h → o → SPACE → " → h → e → l → l → o → w → " → BACKSPACE → BACKSPACE
```

**Expected:**
```
echo "hellow → echo "hellơ → echo "hell
```

**Actual:** ___________

**Status:** [ ] PASS  [ ] FAIL

---

## 🐛 Bug Scenarios (Phải KHÔNG xảy ra)

### Bug 1: Buffer không sync
```
Input:  a → a → s → BACKSPACE → n
Wrong:  ásn  ❌
Right:  ân   ✅
```

**Status:** [ ] NO BUG  [ ] BUG FOUND

---

### Bug 2: Crash khi xóa liên tiếp
```
Input:  h → o → a → BACKSPACE x10
Wrong:  Crash hoặc behavior lạ  ❌
Right:  Xóa hết về empty, không crash  ✅
```

**Status:** [ ] NO BUG  [ ] BUG FOUND

---

### Bug 3: Xóa không restore đúng
```
Input:  d → d → BACKSPACE
Wrong:  (empty) hoặc "dd"  ❌
Right:  "d"  ✅
```

**Status:** [ ] NO BUG  [ ] BUG FOUND

---

## 📊 Summary

Total tests: 14
Passed: _____  
Failed: _____  
Bugs found: _____

**CRITICAL TESTS (Must Pass):**
- [ ] Test 0a: VSCode/Zed backspace sau commit
- [ ] Test 0b: Backspace count chính xác - Fix "được kkhôn"

---

## 🔧 Debugging

Nếu có test FAIL, check log:

```bash
tail -f /tmp/vietnameseime.log
```

Tìm dòng:
```
KEY[51] → Processing     (keycode 51 = backspace)
TRANSFORM bs=1 chars=â   (engine restore)
```

Nếu KHÔNG thấy `KEY[51]` khi nhấn Backspace:
→ Event bị swallow trước khi vào engine!

Nếu thấy `KEY[51]` nhưng không có `TRANSFORM`:
→ Engine return `action=None`, check logic trong `core/src/engine/mod.rs`

---

## ✅ Acceptance Criteria

- [ ] Tất cả 12 test cases PASS
- [ ] Không có bug nào trong Bug Scenarios
- [ ] Hoạt động ổn định trên VSCode, Zed, Terminal, TextEdit
- [ ] Không bị crash khi xóa liên tiếp
- [ ] Backspace-after-space restore đúng từ
- [ ] Backspace count chính xác (không bị "được kkhôn")

---

## 🎯 Priority Testing

### HIGH PRIORITY (Test đầu tiên!)
1. **Test 0a** - VSCode/Zed backspace sau commit
2. **Test 0b** - Backspace count chính xác (Fix "được kkhôn")
3. **Test 4** - Xóa và gõ lại (kiểm tra buffer sync)
4. **Test 10** - Xóa trong VSCode
5. **Test 11** - Xóa trong Zed

### NORMAL PRIORITY
- Tests 1-3, 5-9, 12

Nếu **Test 0a FAIL** → Các fix chưa được apply đúng, cần check lại:
1. Rust engine có rebuild buffer sau khi pop? (`core/src/engine/mod.rs` line 357-375)
2. Swift có inject backspace manually? (`InputManager.swift` line 300-316)

Nếu **Test 0b FAIL** → Backspace count sai, cần check lại:
1. Rust có lưu `old_length` trước khi pop? (`core/src/engine/mod.rs` line 363)
2. Rust có gọi `rebuild_from_with_backspace(0, old_length)`? (line 375)
3. Hàm `rebuild_from_with_backspace()` đã được implement? (line 1334-1357)

---

**Tested by:** _______________  
**Date:** _______________  
**Result:** [ ] PASS  [ ] FAIL