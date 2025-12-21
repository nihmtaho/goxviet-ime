# Hướng Dẫn Kiểm Tra Vietnamese IME

## 1. Chuẩn Bị

### 1.1. Build Rust Core
```bash
cd core
cargo build --release
```

### 1.2. Build macOS App
```bash
cd platforms/macos/GoxViet
xcodebuild -scheme GoxViet -configuration Release clean build
```

Hoặc mở `GoxViet.xcodeproj` trong Xcode và nhấn Cmd+B để build.

### 1.3. Cấp Quyền Accessibility
1. Mở **System Settings** → **Privacy & Security** → **Accessibility**
2. Thêm **GoxViet.app** vào danh sách
3. Bật toggle để cho phép app kiểm soát máy tính

## 2. Chạy Ứng Dụng

### 2.1. Từ Xcode
- Nhấn **Cmd+R** để chạy
- App sẽ hiển thị icon trên menu bar

### 2.2. Từ Finder
```bash
open ~/Library/Developer/Xcode/DerivedData/GoxViet-*/Build/Products/Release/GoxViet.app
```

## 3. Kiểm Tra Chức Năng

### 3.1. Kiểm Tra FFI (Rust ↔ Swift)

Chạy test đơn giản để xác nhận Rust core hoạt động:

```bash
cd platforms/macos
swiftc -I ../../core/target/release -L ../../core/target/release -lvietnamese_ime_core test_ffi.swift -o test_ffi
./test_ffi
```

**Kết quả mong đợi:**
- ✓ Engine initialized
- ✓ Method set
- Test các keystroke có thể fail nếu chưa implement composition tracking

### 3.2. Kiểm Tra Input trong Ứng Dụng Thực

#### Test Case 1: Gõ Chữ Đơn Giản
1. Mở **TextEdit** hoặc **Notes**
2. Gõ: `v` `i` `e` `t`
   - **Kỳ vọng:** Hiển thị "viet"
3. Gõ: `s` (thêm dấu sắc)
   - **Kỳ vọng:** Chữ "e" thay đổi thành "ế" → "viết"

#### Test Case 2: Gõ Từ Phức Tạp
```
Input:  v i e e j t   n a m
Output: việt          nam

Input:  c h a o   b a n
Output: chào          bạn
        ↑ gõ f        ↑ gõ j
```

#### Test Case 3: Telex Mode
| Input Sequence | Expected Output | Description |
|----------------|-----------------|-------------|
| `a` `a`        | â               | Double vowel → circumflex |
| `a` `w`        | ă               | w → breve |
| `o` `o`        | ô               | Double vowel → circumflex |
| `o` `w`        | ơ               | w → horn |
| `u` `w`        | ư               | w → horn |
| `d` `d`        | đ               | Double d → đ |
| `s`            | Sắc (´)         | Tone mark |
| `f`            | Huyền (`)       | Tone mark |
| `r`            | Hỏi (?)         | Tone mark |
| `x`            | Ngã (~)         | Tone mark |
| `j`            | Nặng (.)        | Tone mark |
| `z`            | Remove tone     | Clear tone mark |

#### Test Case 4: VNI Mode
| Input Sequence | Expected Output | Description |
|----------------|-----------------|-------------|
| `a` `6`        | â               | 6 → circumflex |
| `a` `8`        | ă               | 8 → breve |
| `o` `6`        | ô               | 6 → circumflex |
| `o` `7`        | ơ               | 7 → horn |
| `u` `7`        | ư               | 7 → horn |
| `d` `9`        | đ               | 9 → đ |
| `a` `1`        | á               | 1 → Sắc |
| `a` `2`        | à               | 2 → Huyền |
| `a` `3`        | ả               | 3 → Hỏi |
| `a` `4`        | ã               | 4 → Ngã |
| `a` `5`        | ạ               | 5 → Nặng |
| `a` `0`        | a               | 0 → Remove tone |

### 3.3. Kiểm Tra Toggle IME
1. Nhấn **Cmd+Shift+V** (hoặc shortcut tùy chỉnh)
2. Gõ: `viet`
   - **IME enabled:** → "viet" (chờ biến đổi)
   - **IME disabled:** → "viet" (tiếng Anh thuần)

### 3.4. Kiểm Tra ESC Restore
1. Gõ: `v` `i` `e` `e` `s` `t`
   - Kết quả: "viết"
2. Nhấn **ESC**
   - **Kỳ vọng:** Khôi phục về "vieest" (raw keystrokes)

### 3.5. Kiểm Tra Backspace
1. Gõ: `v` `i` `e` `e` `s` `t`
   - Kết quả: "viết"
2. Nhấn **Backspace** 1 lần
   - **Kỳ vọng:** "việ" (xóa 1 ký tự)
3. Nhấn **Backspace** thêm 2 lần
   - **Kỳ vọng:** "vi" (xóa tiếp)

## 4. Kiểm Tra Log

### 4.1. Xem Log File
```bash
tail -f ~/Library/Logs/GoxViet/keyboard.log
```

### 4.2. Log Format
```
[2024-12-19 16:45:23] KEY: 9 (v) -> Processing
[2024-12-19 16:45:23] TRANSFORM: BS=0, CHARS='v'
[2024-12-19 16:45:24] KEY: 34 (i) -> Processing
[2024-12-19 16:45:24] TRANSFORM: BS=0, CHARS='i'
[2024-12-19 16:45:25] KEY: 14 (e) -> Processing
[2024-12-19 16:45:25] TRANSFORM: BS=0, CHARS='e'
[2024-12-19 16:45:26] KEY: 1 (s) -> Processing
[2024-12-19 16:45:26] TRANSFORM: BS=1, CHARS='é'
```

## 5. Debug Thường Gặp

### 5.1. App Không Phản Hồi Phím
**Nguyên nhân:** Accessibility permission chưa được cấp hoặc event tap không start.

**Giải pháp:**
1. Kiểm tra quyền trong System Settings
2. Khởi động lại app
3. Kiểm tra log: `~/Library/Logs/GoxViet/keyboard.log`

### 5.2. Ký Tự Không Được Biến Đổi
**Nguyên nhân:** 
- Rust engine trả về `action=0` (None)
- Composition tracking bị mất

**Giải pháp:**
1. Kiểm tra log để xem engine response
2. Verify Rust library đã được link: `otool -L GoxViet.app/Contents/MacOS/GoxViet`
3. Chạy test FFI để isolate vấn đề

### 5.3. Ký Tự Bị Duplicate
**Nguyên nhân:** 
- Event được pass through thay vì swallow
- Injection và pass-through xảy ra đồng thời

**Giải pháp:**
- Đảm bảo `processKeyWithEngine` luôn return `nil` (swallow) hoặc inject manual
- KHÔNG bao giờ pass through event khi đã inject

### 5.4. Rust Library Not Found
**Error:** `dyld: Library not loaded: libvietnamese_ime_core.dylib`

**Giải pháp:**
```bash
# Rebuild Rust core
cd core
cargo clean
cargo build --release

# Verify library exists
ls -lh target/release/libvietnamese_ime_core.a
```

## 6. Performance Testing

### 6.1. Latency Test
1. Gõ nhanh: `v` `i` `e` `e` `s` `t` liên tục
2. Kiểm tra độ trễ giữa keystroke và hiển thị
3. **Mục tiêu:** < 16ms (60fps)

### 6.2. Memory Test
```bash
# Monitor memory usage
top -pid $(pgrep GoxViet)
```

**Mục tiêu:** < 50MB RAM usage

### 6.3. Stress Test
Gõ liên tục trong 5 phút với các từ phức tạp:
```
trường   đại   học   bách   khoa   hà   nội
việt   nam   công   nghệ   thông   tin
```

Kiểm tra:
- [ ] Không crash
- [ ] Không memory leak
- [ ] Response time ổn định

## 7. Test Coverage Checklist

### Core Functions
- [ ] Telex input (aa, aw, oo, ow, uw, dd)
- [ ] VNI input (6, 7, 8, 9, 0-5)
- [ ] Tone marks (sắc, huyền, hỏi, ngã, nặng)
- [ ] ESC restore
- [ ] Backspace handling
- [ ] Space clears composition
- [ ] Toggle IME on/off

### Edge Cases
- [ ] Gõ số khi Shift (Shift+2 → @, không phải dấu huyền)
- [ ] Modifier keys (Cmd+C không trigger IME)
- [ ] Arrow keys clear composition
- [ ] Multiple spaces
- [ ] Punctuation handling

### Cross-App Testing
- [ ] TextEdit
- [ ] Notes
- [ ] Safari (URL bar, text fields)
- [ ] Terminal
- [ ] VS Code
- [ ] Slack/Discord
- [ ] Spotlight Search

## 8. Báo Lỗi

Khi phát hiện lỗi, vui lòng cung cấp:

1. **Môi trường:**
   - macOS version
   - Xcode version
   - Rust version (`rustc --version`)

2. **Tái hiện lỗi:**
   - Input sequence chính xác
   - Expected vs Actual output
   - Log file (`~/Library/Logs/GoxViet/keyboard.log`)

3. **Screenshots/Videos** nếu có

## 9. Next Steps

Sau khi test thành công:
1. Đóng gói app cho distribution
2. Code signing
3. Notarization (cho macOS Gatekeeper)
4. Tạo installer DMG
5. Release notes

---

**Lưu ý:** Đây là phiên bản đang phát triển. Một số tính năng có thể chưa hoàn thiện.