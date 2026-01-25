# Window Independence Fix Verification

## Vấn đề
Khi đóng Update window, Settings window bị đóng theo (không mong muốn).

## Nguyên nhân
1. Trong `UpdateWindowView`, sử dụng `@Environment(\.dismiss)` có thể ảnh hưởng đến tất cả windows
2. Không có cơ chế đóng window độc lập rõ ràng

## Giải pháp

### 1. UpdateManager.swift
- Thêm comments rõ ràng vào các methods để làm rõ:
  - `stop()`: Chỉ cleanup resources, **KHÔNG** close windows hay terminate app
  - `cancelDownload()`: Chỉ cancel download, **KHÔNG** close windows
  - `relaunchWithNewApp()`: Đây là method DUY NHẤT terminate app (khi install update)

### 2. UpdateWindowView.swift  
- Thay thế `dismiss()` bằng `WindowManager.shared.closeUpdateWindow()`
- Thêm method `closeWindow()` để đóng Update window an toàn
- Đảm bảo chỉ đóng Update window, không ảnh hưởng Settings window

### 3. WindowManager.swift
- Thêm comments chi tiết cho `closeUpdateWindow()` và `closeSettingsWindow()`
- Thêm comments cho `windowWillClose(_:)` delegate method
- Làm rõ cơ chế: Mỗi window đóng độc lập, chỉ khi TẤT CẢ windows đóng mới switch về background mode

## Cơ chế hoạt động

### Trước khi fix:
```
User nhấn Close (Update window)
↓
dismiss() được gọi
↓
SwiftUI dismiss environment có thể close nhiều windows
↓  
⚠️ Settings window BỊ ĐÓNG THEO (BUG)
```

### Sau khi fix:
```
User nhấn Close (Update window)  
↓
closeWindow() được gọi
↓
WindowManager.shared.closeUpdateWindow()
↓
Chỉ updateWindow.close() được gọi
↓
windowWillClose delegate: updateWindow = nil
↓
handleLastWindowClosed() kiểm tra:
  - settingsWindow != nil → Vẫn còn window
  - KHÔNG switch về background mode
↓
✅ Settings window VẪN MỞ (FIXED)
```

## Test Cases

### Test 1: Đóng Update window khi Settings đang mở
1. Mở Settings window
2. Mở Update window  
3. Nhấn Close trên Update window
4. **Kết quả mong đợi**: Settings window vẫn mở

### Test 2: Đóng Settings window khi Update đang mở
1. Mở Update window
2. Mở Settings window
3. Nhấn Close trên Settings window
4. **Kết quả mong đợi**: Update window vẫn mở

### Test 3: Đóng cả 2 windows
1. Mở Update window
2. Mở Settings window  
3. Đóng Update window
4. Đóng Settings window
5. **Kết quả mong đợi**: App switch về background mode (.accessory nếu hideFromDock = true)

### Test 4: Cancel download không đóng windows
1. Mở Update window
2. Mở Settings window
3. Bắt đầu download update
4. Nhấn Cancel
5. **Kết quả mong đợi**: CẢ 2 windows vẫn mở, chỉ download bị cancel

## Verification Commands

```bash
# Build project
cd platforms/macos/goxviet
xcodebuild -scheme goxviet -configuration Debug build

# Run app
open platforms/macos/goxviet/build/Debug/goxviet.app

# Check logs
tail -f ~/Library/Logs/GoxViet/keyboard.log | grep -E "window|Update|Settings"
```

## Expected Log Output

Khi đóng Update window (Settings vẫn mở):
```
[INFO] ✅ Update window will close - Settings window unaffected
[INFO] WindowManager: 1 window(s) still open
```

Khi đóng Settings window (Update vẫn mở):  
```
[INFO] ✅ Settings window will close - Update window unaffected
[INFO] WindowManager: 1 window(s) still open
```

Khi đóng window cuối cùng:
```
[INFO] All windows closed. Policy set to: .accessory
```

## Code Changes Summary

**Files Modified:**
1. `UpdateManager.swift` - Thêm comments an toàn
2. `UpdateWindowView.swift` - Thay dismiss() bằng WindowManager
3. `WindowManager.swift` - Thêm comments chi tiết về window independence

**Lines Changed:** ~50 lines
**Risk Level:** Low (chỉ thêm comments và refactor close logic)
**Breaking Changes:** None
