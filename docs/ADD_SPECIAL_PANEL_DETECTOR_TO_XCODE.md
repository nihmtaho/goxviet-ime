# Hướng dẫn thêm SpecialPanelAppDetector.swift vào Xcode Project

## Bước 1: Mở Xcode Project
```bash
open platforms/macos/goxviet/goxviet.xcodeproj
```

## Bước 2: Thêm file vào project

1. Trong Xcode, click phải vào folder `goxviet` trong Project Navigator
2. Chọn "Add Files to goxviet..."
3. Navigate đến: `platforms/macos/goxviet/goxviet/SpecialPanelAppDetector.swift`
4. Đảm bảo các option sau được chọn:
   - ✅ "Copy items if needed" (NÊN BỎ chọn vì file đã ở đúng vị trí)
   - ✅ "Create groups" (không phải "Create folder references")
   - ✅ Target: "goxviet" được chọn
5. Click "Add"

## Bước 3: Verify trong Build Phases

1. Select project "goxviet" ở đầu Project Navigator
2. Select target "goxviet"
3. Chọn tab "Build Phases"
4. Expand "Compile Sources"
5. Verify `SpecialPanelAppDetector.swift` có trong danh sách

## Bước 4: Build và test

```bash
# Clean build folder
cd platforms/macos/goxviet
xcodebuild clean -project goxviet.xcodeproj -scheme goxviet

# Build
xcodebuild -project goxviet.xcodeproj -scheme goxviet -configuration Debug
```

## File Structure Expected

```
platforms/macos/goxviet/goxviet/
├── SpecialPanelAppDetector.swift  ← NEW FILE
├── PerAppModeManager.swift        ← MODIFIED
├── InputManager.swift
├── AppDelegate.swift
└── ... (other files)
```

## Testing Checklist

Sau khi build thành công:

- [ ] Open Spotlight (Cmd+Space) và gõ tiếng Việt
- [ ] Verify buffer được clear khi switch từ Spotlight về app khác
- [ ] Check logs: `tail -f ~/Library/Logs/GoxViet/keyboard.log`
- [ ] Verify không có crash hoặc warning
- [ ] Test với Raycast nếu có cài đặt
- [ ] Verify performance không giảm (<16ms latency)

## Troubleshooting

### Nếu file không xuất hiện trong Project Navigator:
1. Quit Xcode
2. Delete derived data: `rm -rf ~/Library/Developer/Xcode/DerivedData/goxviet-*`
3. Reopen Xcode
4. Try adding file again

### Nếu compile error:
- Verify import statements đúng
- Verify file được add vào correct target
- Check Build Settings → Swift Language Version

### Nếu runtime error:
- Check Accessibility permissions
- Verify logging is working
- Check Console.app for crash logs
