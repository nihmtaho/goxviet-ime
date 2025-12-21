# 🧪 Quick Test: Backspace Fix cho VSCode và Zed

## ⚡ Test trong 2 phút

### Bước 1: Build & Run
```bash
# Terminal 1: Build
cd vietnamese-ime/core && cargo build --release
cd ../platforms/macos/VietnameseIMEFast
xcodebuild -scheme VietnameseIMEFast -configuration Release build

# Terminal 2: Run app
open ~/Library/Developer/Xcode/DerivedData/VietnameseIMEFast-*/Build/Products/Release/VietnameseIMEFast.app
```

### Bước 2: Enable IME
- Click vào icon VietnameseIMEFast trong menu bar (nếu có)
- Hoặc app sẽ tự enable sau khi mở

### Bước 3: CRITICAL TEST (VSCode/Zed)

#### Test trên VSCode:
1. Mở VSCode
2. Tạo file mới (Cmd+N)
3. Gõ theo sequence:

```
Gõ:  g → õ → SPACE → BACKSPACE → BACKSPACE → BACKSPACE
```

#### ✅ Expected Result (PASS):
```
g → gõ → "gõ " → "gõ" → "g" → ""
```

Mỗi lần nhấn Backspace phải xóa được 1 ký tự!

#### ❌ Wrong Result (FAIL):
```
g → gõ → "gõ " → "gõ" → "gõ" → "gõ"  (STUCK!)
```

Nếu "gõ" không xóa được sau backspace thứ 2 → **FAIL!**

---

### Bước 3b: CRITICAL TEST 2 - Fix "được kkhôn" bug

#### Test trên VSCode:
1. Mở VSCode
2. Tạo file mới (Cmd+N)
3. Gõ theo sequence:

```
Gõ:  đ → ư → ợ → c → SPACE → k → h → ô → n → g → BACKSPACE
```

#### ✅ Expected Result (PASS):
```
đ → ư → được → được → "được " → ... → "được không" → "được khôn"
```

Xóa "g" phải về "được khôn" (không phải "được kkhôn")!

#### ❌ Wrong Result (FAIL):
```
"được không" → BACKSPACE → "được kkhôn"  (backspace count sai!)
```

Nếu thấy "kk" thay vì "k" → **FAIL!** → Engine đếm buffer sau pop thay vì old_length trước pop

---

### Bước 4: Test trên Zed (nếu có)

Same test như VSCode:
```
Gõ:  g → õ → SPACE → BACKSPACE × 3
```

Expected: `"gõ " → "gõ" → "g" → ""`

---

## 🎯 Additional Quick Tests

### Test 1: Xóa dấu thanh
```
Input:  a → a → s → BACKSPACE
Expect: a → â → á → â ✅
```

### Test 2: Xóa liên tiếp
```
Input:  v → i → e → e → s → t → BACKSPACE × 4
Expect: việt → viê → vi → v → "" ✅
```

### Test 3: Xóa transform
```
Input:  d → d → BACKSPACE
Expect: d → đ → d ✅
```

### Test 4: Xóa từ dài - Backspace count chính xác
```
Input:  t → h → a → n → h → SPACE → p → h → o → w → BACKSPACE
Expect: "thanh phơ" (không phải "thanh pphơ")
```

### Test 5: Xóa "không"
```
Input:  k → h → o → n → g → BACKSPACE × 5
Expect: không → khôn → khô → kh → k → ""
        (Mỗi bước phải đúng, không được xuất hiện "kk", "hh", etc.)
```

---

## 🐛 Debug: Nếu test FAIL

### 1. Check log
```bash
tail -f /tmp/vietnameseime.log
```

Tìm dòng khi nhấn Backspace:
```
KEY[51] → Processing          (Backspace key detected)
TRANSFORM bs=2 chars=g         (Engine rebuild và return text mới)
SEND[fast] bs=2 chars=g        (Inject backspace + text)
```

### 2. Kiểm tra fixes đã được apply chưa?

#### Fix 1: Swift inject manually
```bash
grep -A 10 "currentCompositionLength > 0" \
  platforms/macos/VietnameseIMEFast/VietnameseIMEFast/InputManager.swift
```

Phải thấy:
```swift
TextInjector.shared.injectSync(
    bs: 1,
    text: "",
    ...
)
return true  // ← QUAN TRỌNG!
```

#### Fix 2: Rust rebuild buffer
```bash
grep -A 5 "Pop the last character" core/src/engine/mod.rs
```

Phải thấy:
```rust
self.buf.pop();
self.raw_input.pop();
self.last_transform = None;
return self.rebuild_from(0);  // ← QUAN TRỌNG!
```

### 3. Rebuild nếu cần
```bash
# Clean và rebuild
cd core
cargo clean && cargo build --release

cd ../platforms/macos/VietnameseIMEFast
xcodebuild clean
xcodebuild -scheme VietnameseIMEFast -configuration Release build
```

---

## ✅ Success Criteria

- [ ] Test CRITICAL 1 trên VSCode: PASS (gõ → "" không stuck)
- [ ] Test CRITICAL 2 trên VSCode: PASS ("được không" → "được khôn", không phải "được kkhôn")
- [ ] Test CRITICAL 1 trên Zed: PASS
- [ ] Test CRITICAL 2 trên Zed: PASS
- [ ] Test 1-5: PASS
- [ ] Không bị crash khi xóa liên tiếp
- [ ] Log hiển thị `TRANSFORM` và `SEND` khi backspace
- [ ] Không xuất hiện ký tự double (kk, hh, pp, etc.) khi xóa

**Nếu tất cả PASS → Fix thành công! 🎉**

**Nếu vẫn thấy "được kkhôn" → Fix 3 chưa được apply (old_length)!**

---

## 📝 Report Issue

Nếu test vẫn FAIL sau khi rebuild:

1. Copy output của:
   ```bash
   tail -50 /tmp/vietnameseime.log
   ```

2. Check version:
   ```bash
   git log --oneline -1
   ```

3. Report với thông tin:
   - App nào fail (VSCode/Zed)
   - Expected vs Actual behavior
   - Log output
   - Git commit hash

---

## 🎯 Quick Summary

**4 Fixes Applied:**
1. ✅ Swift: Call `ime_key()` để thông báo engine
2. ✅ Swift: Inject backspace manually (không dựa vào system)
3. ✅ Rust: Lưu `old_length` trước pop, dùng `rebuild_from_with_backspace()`
4. ✅ Rust: Hàm mới với explicit backspace count

**Critical Bugs Fixed:**
- ✅ Backspace stuck sau commit word
- ✅ Backspace count sai → "được kkhôn" bug

---

**Last Updated:** 2024-01-XX