# HƯỚNG DẪN BUILD VÀ TEST SAU KHI SỬA LỖI PHÍM MŨI TÊN

## 📋 Tổng quan

Sau khi sửa lỗi phím mũi tên trong `InputManager.swift`, bạn cần build lại project và test các tính năng để đảm bảo mọi thứ hoạt động đúng.

## 🔧 Build Project

### Bước 1: Build Rust Core

```bash
cd vietnamese-ime/core
cargo build --release
```

**Kiểm tra:** File `libvietnamese_ime.dylib` được tạo trong `target/release/`

### Bước 2: Copy Library vào Xcode Project

```bash
cp target/release/libvietnamese_ime.dylib \
   ../platforms/macos/VietnameseIMEFast/VietnameseIMEFast/
```

### Bước 3: Build macOS App

```bash
cd ../platforms/macos/VietnameseIMEFast
xcodebuild -project VietnameseIMEFast.xcodeproj \
           -scheme VietnameseIMEFast \
           -configuration Release \
           build
```

**Hoặc:** Mở Xcode và build bằng `Cmd+B`

```bash
open VietnameseIMEFast.xcodeproj
```

### Bước 4: Enable Accessibility Permission

1. Mở **System Settings** → **Privacy & Security** → **Accessibility**
2. Click dấu **+** và thêm ứng dụng `VietnameseIMEFast.app`
3. Enable checkbox cho ứng dụng

## 🧪 Test Cases

### Test 1: Gõ tiếng Việt cơ bản

**Mục tiêu:** Kiểm tra engine vẫn hoạt động đúng

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `v` `i` `e` `e` `t` | `việt` | [ ] |
| `t` `r` `u` `o` `w` `n` `g` | `trường` | [ ] |
| `h` `o` `a` `f` | `hoá` | [ ] |
| `c` `h` `a` `o` `f` | `cháo` | [ ] |

### Test 2: Phím mũi tên (KEY TEST!)

**Mục tiêu:** Phím mũi tên phải hoạt động bình thường

```
1. Gõ: x i n
2. Output: "xin"
3. Nhấn: ← (left arrow) 2 lần
4. Expected: Con trỏ di chuyển về trước 2 ký tự (giữa "x" và "i")
5. Result: [ ] PASS / [ ] FAIL
```

```
1. Gõ: h o a f (ra "hoá")
2. Nhấn: ← (left arrow) 1 lần
3. Expected: Con trỏ ở giữa "ho" và "á"
4. Gõ thêm: l
5. Expected: "holá"
6. Result: [ ] PASS / [ ] FAIL
```

### Test 3: Backspace

**Mục tiêu:** Backspace vẫn xử lý đúng tone marks

```
Test 3.1: Backspace xóa dấu
Input: h o a f → "hoá"
Press: Backspace
Expected: "hoa"
Result: [ ] PASS / [ ] FAIL
```

```
Test 3.2: Backspace nhiều lần
Input: t r u o w n g → "trường"
Press: Backspace 3 lần
Expected: "trư"
Result: [ ] PASS / [ ] FAIL
```

### Test 4: Navigation keys clear buffer

**Mục tiêu:** Navigation keys phải clear buffer nhưng vẫn di chuyển

```
Test 4.1: Enter key
1. Gõ: h o a (chưa có dấu)
2. Press: Enter
3. Expected: Buffer cleared, xuống dòng mới
4. Gõ: f
5. Expected: Chữ "f" thường (không thêm dấu vào "hoa")
6. Result: [ ] PASS / [ ] FAIL
```

```
Test 4.2: Arrow keys
1. Gõ: h o a (chưa có dấu)
2. Press: ↓ (down arrow)
3. Expected: Buffer cleared, con trỏ di chuyển xuống
4. Gõ: f
5. Expected: Chữ "f" thường (không thêm dấu)
6. Result: [ ] PASS / [ ] FAIL
```

### Test 5: ESC restore (nếu được enable)

```
Input: h o a f → "hoá"
Press: ESC
Expected: "hoaf" (restore to original input)
Result: [ ] PASS / [ ] FAIL
```

### Test 6: Modifier keys pass through

**Mục tiêu:** Cmd/Ctrl shortcuts vẫn hoạt động

```
Test 6.1: Copy/Paste
1. Gõ: việt
2. Cmd+A (select all)
3. Cmd+C (copy)
4. Cmd+V (paste)
5. Expected: "việt" được copy/paste đúng
6. Result: [ ] PASS / [ ] FAIL
```

```
Test 6.2: Cmd+Arrow
1. Gõ: xin chào
2. Press: Cmd+← (move to start of line)
3. Expected: Con trỏ nhảy về đầu dòng
4. Result: [ ] PASS / [ ] FAIL
```

### Test 7: Different apps

**Mục tiêu:** Hoạt động đúng trên nhiều ứng dụng

| App | Gõ "việt" | Arrow keys | Status |
|-----|-----------|------------|--------|
| TextEdit | [ ] | [ ] | [ ] |
| VSCode | [ ] | [ ] | [ ] |
| Terminal | [ ] | [ ] | [ ] |
| Chrome/Safari | [ ] | [ ] | [ ] |
| Notes.app | [ ] | [ ] | [ ] |

## 🐛 Debug Tips

### Enable Debug Logging

```bash
# Enable
touch /tmp/vietnamese_ime_debug.log

# View logs
tail -f /tmp/vietnamese_ime_debug.log

# Disable
rm /tmp/vietnamese_ime_debug.log
```

### Check FFI Binding

```bash
cd platforms/macos
swift test_ffi.swift
```

Expected output:
```
IME initialized
Processing 'a': Result(action=0, ...)
Processing 's': Result(action=1, backspace=1, chars=[...])
```

### Common Issues

#### Issue 1: Phím mũi tên vẫn bị chặn

**Nguyên nhân:** Code chưa được compile lại

**Giải pháp:**
```bash
# Clean build
cd platforms/macos/VietnameseIMEFast
xcodebuild clean
xcodebuild -scheme VietnameseIMEFast -configuration Release build
```

#### Issue 2: Gõ tiếng Việt không ra

**Nguyên nhân:** Rust library chưa được copy

**Giải pháp:**
```bash
cd core
cargo build --release
cp target/release/libvietnamese_ime.dylib \
   ../platforms/macos/VietnameseIMEFast/VietnameseIMEFast/
```

#### Issue 3: Accessibility permission

**Nguyên nhân:** Chưa grant quyền Accessibility

**Giải pháp:**
1. System Settings → Privacy & Security → Accessibility
2. Add VietnameseIMEFast.app
3. Restart app

## 📊 Test Report Template

```markdown
## Test Report - Arrow Key Fix

**Date:** YYYY-MM-DD
**Tester:** Your Name
**Build:** Release/Debug

### Summary
- [ ] All tests passed
- [ ] Some tests failed (see details)
- [ ] Critical issues found

### Test Results

#### Basic Vietnamese Input
- Test 1.1: [ ] PASS / [ ] FAIL
- Test 1.2: [ ] PASS / [ ] FAIL
- Test 1.3: [ ] PASS / [ ] FAIL

#### Arrow Keys (CRITICAL)
- Test 2.1: [ ] PASS / [ ] FAIL
- Test 2.2: [ ] PASS / [ ] FAIL

#### Backspace
- Test 3.1: [ ] PASS / [ ] FAIL
- Test 3.2: [ ] PASS / [ ] FAIL

### Issues Found
1. [Issue description]
   - Severity: High/Medium/Low
   - Steps to reproduce: ...
   - Expected: ...
   - Actual: ...

### Notes
- [Any additional observations]
```

## ✅ Success Criteria

Bạn có thể coi việc sửa lỗi là **THÀNH CÔNG** khi:

- ✅ Gõ tiếng Việt hoạt động đúng (Test 1)
- ✅ **Phím mũi tên di chuyển con trỏ được (Test 2)** ← QUAN TRỌNG NHẤT
- ✅ Backspace xóa dấu đúng (Test 3)
- ✅ Navigation keys clear buffer (Test 4)
- ✅ Modifier shortcuts pass through (Test 6)
- ✅ Hoạt động trên nhiều apps (Test 7)

## 🎯 Next Steps

Sau khi test thành công:

1. ✅ Commit changes với message rõ ràng
2. ✅ Update CHANGELOG.md
3. ✅ Create release build
4. ✅ Test trên clean macOS install (optional)
5. ✅ Deploy to users

---

**Good luck testing!** 🚀

Nếu gặp vấn đề, xem lại `docs/ARROW_KEY_FIX.md` để hiểu rõ hơn về cách sửa.