# Kế hoạch triển khai cho Issue #36 - Backspace xóa nhầm autocomplete (xóa cả suggestion + ký tự)

## Mô tả

Fix lỗi Backspace trong browser address bars/search fields: khi có autocomplete suggestion đang được highlight, Backspace phải xóa ký tự trước cursor **và** bỏ hẳn phần suggestion.

## Problem Issue: #36

### Current Issues

1. **Backspace chưa xóa sạch suggestion đang highlight:**
   - Suggestion vẫn xuất hiện sau khi Backspace, người dùng không xóa được ký tự vừa gõ kèm suggestion.
2. **Suffix logic sai khi có selection:**
   - Cắt suffix từ `userText` thay vì `fullText`, dẫn đến mất phần còn lại của chuỗi.
   - Dùng Swift `String` (grapheme) cho range UTF-16 từ AX → lệch index.

### Root Causes

1. **Range unit mismatch:** AX `kAXSelectedTextRangeAttribute` dùng UTF-16, nhưng code thao tác theo grapheme → sai chỉ số khi có emoji/ký tự đa byte.
2. **Không tách rõ autocomplete:** Suffix phải lấy từ `fullText` sau vùng selection (autocomplete) thay vì từ `userText` đã bị cắt.
3. **Cursor placement không theo UTF-16:** Vị trí cursor mới tính bằng `.count` thay vì `.utf16.count` → đặt cursor sai sau khi thay text.

## Các bước triển khai

### Bước 1: Chuẩn hóa range theo UTF-16

```text
- Đọc fullText thành NSString
- Clamp cursor và selection trong [0, length]
- Tính autocompleteEnd = cursor + selection
```

### Bước 2: Tách phần text và bỏ suggestion

```swift
let nsText = fullText as NSString
let length = nsText.length
let cursor = min(range.location, length)
let selLen = min(range.length, length - cursor)
let autocompleteEnd = cursor + selLen

let userText = nsText.substring(to: cursor)
let suffix = (selLen > 0 && autocompleteEnd < length)
    ? nsText.substring(from: autocompleteEnd) // bỏ toàn bộ suggestion
    : nsText.substring(from: cursor)
```

### Bước 3: Xóa `bs` ký tự trước cursor và dựng newText

```swift
let deleteStart = max(0, cursor - bs)
let prefix = nsText.substring(to: deleteStart)
let newText = (prefix + text + suffix).precomposedStringWithCanonicalMapping
```

### Bước 4: Đặt lại cursor theo UTF-16

```swift
let newCursorLocation = deleteStart + text.utf16.count
```

### Bước 5: Logging phục vụ debug

```swift
Log.info("AX autocomplete: cursor=\(cursor), selection=\(selLen)")
Log.info("  fullText: '\(fullText)'")
Log.info("  userText: '\(userText)'")
Log.info("  suffix: '\(suffix)'")
Log.info("  deleteStart: \(deleteStart), bs: \(bs)")
Log.info("  newText: '\(newText)'")
```

### Bước 6: Test cases

1. **Arc/Chrome address bar có autocomplete:** gõ "d" → gợi ý "google.com" được highlight, Backspace phải xóa "d" và toàn bộ gợi ý.
2. **Tiếng Việt có autocomplete:** gõ "dien" → Backspace xóa "n" và gợi ý.
3. **Không autocomplete:** gõ "hello" → Backspace xóa "o" bình thường.
4. **Manual selection:** chọn "llo" trong "hello" → Backspace xóa selection (giữ hành vi hệ thống).

## Proposed Changes

- File: platforms/macos/goxviet/goxviet/TextInjectionHelper.swift
- Function: injectViaAX(bs: Int, text: String) -> Bool
- Nội dung:
  1. Sử dụng NSString (UTF-16) để tính prefix/suffix và selection.
  2. Bỏ toàn bộ autocomplete suggestion khi selection > 0.
  3. Đặt cursor bằng UTF-16 count, thêm logging chi tiết.

## Thời gian dự kiến

- Phân tích & code: 30 phút
- Testing: 30 phút
- Documentation: 15 phút
- **Tổng:** ~1.5 giờ

## Tài nguyên cần thiết

- macOS với Arc/Chrome/Safari để test
- Quyền Accessibility
- Terminal để xem logs

## Implementation Order

1. Chuẩn hóa range và tách suffix bằng UTF-16.
2. Xóa suggestion + ký tự, dựng newText.
3. Đặt cursor, thêm logging.
4. Test trên Arc/Chrome/Safari.
