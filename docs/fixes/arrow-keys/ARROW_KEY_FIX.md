# SỬA LỖI PHÍM MŨI TÊN (ARROW KEYS FIX)

## Vấn đề

Khi gõ tiếng Việt, người dùng không thể sử dụng phím mũi tên (←, →, ↑, ↓) để di chuyển con trỏ. Các phím này bị "nuốt" (swallowed) thay vì được pass through đến ứng dụng.

## Nguyên nhân

Trong implementation ban đầu của `InputManager.swift`, có 2 vấn đề chính:

### 1. **Inject thủ công khi engine không xử lý (action == 0)**

```swift
// ❌ SAI - Code cũ
if r.pointee.action == 0 { 
    // Đang cố inject thủ công ký tự gốc
    if let char = getCharFromEvent(event: event, keyCode: keyCode, caps: caps) {
        TextInjector.shared.injectSync(bs: 0, text: String(char), ...)
        return nil // Swallow event!
    }
}
```

Khi engine trả về `action == 0` (không xử lý), code đang cố inject thủ công ký tự gốc và swallow event. Điều này khiến **TẤT CẢ các phím** (kể cả phím mũi tên) đều bị chặn.

### 2. **Tracking composition length không cần thiết**

Code đang cố theo dõi `currentCompositionLength` trong Swift layer, trong khi Rust engine đã tự quản lý buffer thông qua field `backspace` trong `ImeResult`.

## Giải pháp

### Nguyên tắc từ Project Mẫu (gonhanh.org)

Sau khi phân tích project mẫu, ta thấy pattern đơn giản và đúng đắn:

1. **Gọi Rust engine cho MỌI keystroke**
2. **Chỉ inject khi engine trả về transformation (action == 1)**
3. **Pass through khi engine không xử lý (action == 0 hoặc nil)**
4. **KHÔNG theo dõi composition length** - để engine tự quản lý

### Implementation Đúng

```swift
// ✅ ĐÚNG - Code mới
private func processKeyWithEngine(...) -> Unmanaged<CGEvent>? {
    let result = ime_key(keyCode, caps, ctrl)
    
    guard let r = result else {
        // Engine chưa init -> pass through
        return Unmanaged.passUnretained(event)
    }
    
    defer { ime_free(r) }
    
    // Action == 0: Engine không xử lý -> pass through
    if r.pointee.action == 0 {
        return Unmanaged.passUnretained(event)
    }
    
    // Action == 1: Có transformation -> inject
    if r.pointee.action == 1 {
        let backspaceCount = Int(r.pointee.backspace)
        let chars = extractChars(from: r.pointee)
        
        TextInjector.shared.injectSync(
            bs: backspaceCount,
            text: String(chars),
            method: method,
            delays: delays,
            proxy: proxy
        )
        
        return nil // Swallow event
    }
    
    // Action == 2: Restore (ESC key)
    if r.pointee.action == 2 {
        // ... xử lý restore
        return nil
    }
    
    // Unknown action -> pass through
    return Unmanaged.passUnretained(event)
}
```

## Thay đổi Chi tiết

### 1. Loại bỏ `currentCompositionLength`

```diff
- private var currentCompositionLength: Int = 0

- currentCompositionLength = 0
- currentCompositionLength = chars.count
- currentCompositionLength -= 1
```

**Lý do:** Rust engine tự quản lý buffer và trả về `backspace` count chính xác.

### 2. Đơn giản hóa xử lý Backspace

```diff
- // Handle backspace - must inform Rust engine
- if keyCode == KeyCode.backspace {
-     // ... 60+ dòng code phức tạp
- }

+ // Backspace is handled in processKeyWithEngine
+ // No special treatment needed here
```

**Lý do:** Backspace được xử lý như mọi phím khác thông qua engine.

### 3. Pass through khi action == 0

```diff
- if r.pointee.action == 0 {
-     // Inject thủ công
-     TextInjector.shared.injectSync(bs: 0, text: String(char), ...)
-     return nil // Swallow!
- }

+ if r.pointee.action == 0 {
+     // Pass through - let system handle
+     return Unmanaged.passUnretained(event)
+ }
```

**Lý do:** Phím mũi tên (và các phím non-Vietnamese) sẽ được system xử lý tự nhiên.

### 4. Clear buffer khi gặp navigation keys

```swift
let navigationKeys: Set<UInt16> = [
    36,  // Return
    76,  // Enter
    48,  // Tab
    123, // Left arrow
    124, // Right arrow
    125, // Down arrow
    126  // Up arrow
]

if navigationKeys.contains(keyCode) {
    ime_clear() // Clear engine buffer
    return false // Don't swallow, let system handle
}
```

**Lý do:** Navigation keys sẽ ngắt quá trình composition, nên cần clear buffer. Nhưng vẫn phải **pass through** để user có thể di chuyển con trỏ.

## Kết quả

✅ **Phím mũi tên hoạt động bình thường** - Pass through đến ứng dụng  
✅ **Gõ tiếng Việt vẫn chính xác** - Engine Rust quản lý buffer  
✅ **Code đơn giản hơn** - Loại bỏ 100+ dòng code phức tạp  
✅ **Tuân thủ pattern của project mẫu** - Proven solution  

## Testing

### Test Cases

1. **Gõ từ tiếng Việt đơn giản:**
   - Input: `v` `i` `e` `e` `t` → Output: `việt` ✅

2. **Sử dụng phím mũi tên giữa chừng:**
   - Input: `x` `i` `n` `←` `←` `[space]` → Con trỏ di chuyển về trước 2 ký tự ✅

3. **Backspace trong từ có dấu:**
   - Input: `h` `o` `a` `f` → `hoá`
   - Backspace → `hoa` ✅

4. **ESC restore (nếu được enable):**
   - Input: `h` `o` `a` `f` → `hoá`
   - ESC → `hoaf` ✅

5. **Navigation keys clear buffer:**
   - Input: `h` `o` `a` (chưa có dấu)
   - `↓` (down arrow) → Buffer cleared, con trỏ di chuyển xuống ✅

## Reference

- **Project mẫu:** `example-project/gonhanh.org-main/platforms/macos/RustBridge.swift`
- **Key function:** `keyboardCallback()` line 606-720
- **Pattern:** Chỉ inject khi có transformation, pass through khi không xử lý

## Lessons Learned

1. **Đừng over-engineer:** Rust engine đã quản lý buffer tốt rồi, không cần duplicate logic trong Swift layer.

2. **Trust the engine:** Khi engine trả về action == 0, nghĩa là "tôi không xử lý phím này", hãy pass through thay vì cố xử lý thủ công.

3. **Keep it simple:** 100+ dòng code phức tạp có thể được thay bằng 10 dòng đơn giản nếu thiết kế đúng.

4. **Learn from proven solutions:** Project mẫu đã hoạt động tốt, nên học pattern của họ thay vì tự phát minh lại.

---

**Date:** 2024  
**Based on:** gonhanh.org reference implementation  
**Status:** ✅ Fixed and tested