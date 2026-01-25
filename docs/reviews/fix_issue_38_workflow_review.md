# Workflow Review - Fix Issue #38: Per-App Mode Synchronization

## Mô tả
Đã sửa lỗi đồng bộ hóa giữa status bar và settings window cho tính năng Smart Per-App Mode trong GoxViet v2.0.0.

## Những gì đã làm tốt

### 1. Root Cause Analysis
- Phân tích chính xác nguyên nhân: thiếu notification mechanism để đồng bộ giữa các UI components
- Xác định được 3 phần cần sửa: AppState, AppDelegate, SettingsRootView

### 2. Implementation Strategy  
- Sử dụng Notification pattern (Observer) để decouple components
- Tận dụng SwiftUI `.onReceive()` thay vì manual NotificationCenter observer để tránh retain cycle
- Không thay đổi API hoặc behavior hiện tại, chỉ thêm sync mechanism

### 3. Code Changes
**Tổng cộng 3 files modified:**

#### AppState.swift
- Thêm `Notification.Name.smartModeChanged`
- Update `isSmartModeEnabled` property để post notification khi thay đổi
```swift
NotificationCenter.default.post(
    name: .smartModeChanged,
    object: newValue
)
```

#### AppDelegate.swift
- Thêm observer trong `setupObservers()` để update `smartModeToggleView`
```swift
let smartModeToken = NotificationCenter.default.addObserver(
    forName: .smartModeChanged,
    ...
)
```

#### SettingsRootView.swift
- Sử dụng SwiftUI `.onReceive()` để subscribe notification
- Update `smartModeEnabled` state và reload `perAppModes` list
```swift
.onReceive(NotificationCenter.default.publisher(for: .smartModeChanged)) { notification in
    if let newState = notification.object as? Bool {
        smartModeEnabled = newState
        loadPerAppModes()
    }
}
```

### 4. Testing Approach
- Build thành công trên Debug configuration
- Không phá vỡ bất kỳ code hiện tại nào
- Sử dụng Combine publisher để tự động cleanup (SwiftUI manages lifecycle)

## Những gì cần cải thiện

### 1. Testing
- Cần test thực tế trên app để verify:
  - Toggle từ status bar → settings cập nhật
  - Toggle từ settings → status bar cập nhật
  - Danh sách apps reload đúng cách
  - Không có memory leak

### 2. Documentation
- Cần update docs/ về notification flow
- Cần thêm comment giải thích notification lifecycle

### 3. Edge Cases
- Chưa test với rapid toggles (spam toggle)
- Chưa test với multiple windows mở đồng thời

## Bài học rút ra

### 1. SwiftUI vs UIKit Patterns
- SwiftUI Views (struct) không thể dùng `[weak self]` vì không phải class
- `.onReceive()` là cách tốt nhất để subscribe notifications trong SwiftUI
- Combine publisher tự động cleanup khi view destroyed

### 2. Notification Pattern
- Notification là giải pháp tốt cho cross-component communication
- Cần consistent naming convention cho notification names
- Object parameter cho phép pass data type-safe

### 3. Debugging Build Errors
- Lỗi "weak may only be applied to class" → struct vs class issue
- Nên dùng Combine `.onReceive()` thay vì manual observer cho SwiftUI

## Notes/Important

### Files Changed
1. `platforms/macos/goxviet/goxviet/AppState.swift`
2. `platforms/macos/goxviet/goxviet/AppDelegate.swift`
3. `platforms/macos/goxviet/goxviet/SettingsRootView.swift`

### Next Steps
1. ✅ Build thành công (Debug)
2. ⏳ Test manual trên app
3. ⏳ Build Release và verify
4. ⏳ Update CHANGELOG.md
5. ⏳ Create PR với message: `fix(macos): sync per-app mode toggle between status bar and settings (#38)`

### Success Metrics
- Toggle từ bất kỳ nơi nào sẽ sync tất cả UI
- Không có lag hoặc delay
- Danh sách apps luôn hiển thị đúng
- Không có memory leak

### Related Issue
- GitHub Issue #38: https://github.com/nihmtaho/goxviet-ime/issues/38
- Reporter: @meichengg
- Version affected: v2.0.0
