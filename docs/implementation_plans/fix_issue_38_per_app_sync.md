# Kế hoạch triển khai sửa lỗi Issue #38 - Per-App Mode Synchronization

## Mô tả
Sửa lỗi tính năng per-app không đồng bộ giữa status bar và settings window trong v2.0.0.

## Problem Issue: 
### Current Issues
1. **Toggle từ Status Bar:**
   - Khi bật/tắt "Smart Per-App Mode" từ status bar menu
   - Settings window không cập nhật toggle state
   - Danh sách apps trong settings cũng không refresh

2. **Toggle từ Settings Window:**
   - Khi bật/tắt "Smart Per-App Mode" trong settings
   - Status bar menu không cập nhật toggle view state
   - Người dùng thấy hai nơi hiển thị khác nhau

3. **Không có observer pattern:**
   - Không có notification để thông báo khi Smart Mode state thay đổi
   - Mỗi component tự quản lý state riêng, không đồng bộ

### Root Causes
1. **Missing Notification:**
   - `AppState.isSmartModeEnabled` được update nhưng không post notification
   - Không có notification name cho smart mode state change

2. **One-way Data Flow:**
   - `SettingsRootView` chỉ load data khi `.onAppear`
   - Không subscribe để nhận updates từ external sources

3. **No Refresh Trigger:**
   - Status bar menu toggle view không được cập nhật khi state thay đổi từ settings
   - Settings view không reload danh sách apps khi toggle từ menu bar

## Các bước triển khai

### Bước 1: Thêm Notification cho Smart Mode
**File:** `platforms/macos/goxviet/goxviet/AppState.swift`

1. Thêm notification name mới: `.smartModeChanged`
2. Modify property observer cho `isSmartModeEnabled`:
   ```swift
   @Published var isSmartModeEnabled: Bool = true {
       didSet {
           UserDefaults.standard.set(isSmartModeEnabled, forKey: Keys.smartModeEnabled)
           // Post notification
           NotificationCenter.default.post(
               name: .smartModeChanged,
               object: isSmartModeEnabled
           )
       }
   }
   ```

### Bước 2: Update AppDelegate để handle notification
**File:** `platforms/macos/goxviet/goxviet/AppDelegate.swift`

1. Trong `setupObservers()`, thêm observer cho `.smartModeChanged`
2. Khi nhận notification, update `smartModeToggleView.updateState()`

### Bước 3: Update SettingsRootView để subscribe
**File:** `platforms/macos/goxviet/goxviet/SettingsRootView.swift`

1. Trong `.onAppear`, register observer cho `.smartModeChanged`
2. Khi nhận notification:
   - Update `smartModeEnabled` @State
   - Reload `perAppModes` list
3. Cleanup observer trong `.onDisappear`

### Bước 4: Ensure PerAppModeManager refresh triggers update
**File:** `platforms/macos/goxviet/goxviet/PerAppModeManager.swift`

Không cần thay đổi - đã hoạt động tốt, chỉ cần đảm bảo notification được post từ AppState.

### Bước 5: Update MenuToggleView (nếu cần)
**File:** `platforms/macos/goxviet/goxviet/MenuToggleView.swift`

Kiểm tra method `updateState()` đã tồn tại và hoạt động đúng.

## Proposed Changes:

### 1. AppState.swift
```swift
// Thêm notification name
extension Notification.Name {
    static let toggleVietnamese = Notification.Name("toggleVietnamese")
    static let updateStateChanged = Notification.Name("updateStateChanged")
    static let shortcutChanged = Notification.Name("shortcutChanged")
    static let smartModeChanged = Notification.Name("smartModeChanged")  // NEW
    static let openUpdateWindow = Notification.Name("openUpdateWindow")
}

// Modify property
@Published var isSmartModeEnabled: Bool = true {
    didSet {
        UserDefaults.standard.set(isSmartModeEnabled, forKey: Keys.smartModeEnabled)
        NotificationCenter.default.post(
            name: .smartModeChanged,
            object: isSmartModeEnabled
        )
    }
}
```

### 2. AppDelegate.swift
```swift
// Trong setupObservers(), thêm:
let smartModeToken = NotificationCenter.default.addObserver(
    forName: .smartModeChanged,
    object: nil,
    queue: .main
) { [weak self] notification in
    if let newState = notification.object as? Bool {
        self?.smartModeToggleView?.updateState(newState)
        Log.info("Status bar smart mode updated: \(newState)")
    }
}
observerTokens.append(smartModeToken)
```

### 3. SettingsRootView.swift
```swift
// Thêm observer property
@State private var smartModeObserver: NSObjectProtocol?

// Trong .onAppear
.onAppear {
    loadPerAppModes()
    syncToAppState()
    setupSmartModeObserver()  // NEW
}

// Trong .onDisappear
.onDisappear {
    cleanupSmartModeObserver()  // NEW
}

// Helper methods
private func setupSmartModeObserver() {
    smartModeObserver = NotificationCenter.default.addObserver(
        forName: .smartModeChanged,
        object: nil,
        queue: .main
    ) { [weak self] notification in
        if let newState = notification.object as? Bool {
            self?.smartModeEnabled = newState
            self?.loadPerAppModes()  // Refresh list
            Log.info("Settings smart mode updated: \(newState)")
        }
    }
}

private func cleanupSmartModeObserver() {
    if let observer = smartModeObserver {
        NotificationCenter.default.removeObserver(observer)
        smartModeObserver = nil
    }
}
```

## Thời gian dự kiến
- Bước 1: 10 phút
- Bước 2: 10 phút
- Bước 3: 15 phút
- Bước 4: 5 phút (verification only)
- Bước 5: 5 phút (verification only)
- Testing: 15 phút
**Tổng: ~60 phút**

## Tài nguyên cần thiết
- Xcode
- macOS testing environment
- Không cần rebuild Rust core

## Implementation Order
1. **Phase 1 - Notification Infrastructure (Bước 1)**
   - Thêm notification name
   - Update AppState property observer
   
2. **Phase 2 - Status Bar Update (Bước 2)**
   - AppDelegate observe notification
   - Update toggle view
   
3. **Phase 3 - Settings Update (Bước 3)**
   - SettingsRootView observe notification
   - Reload data when notified
   
4. **Phase 4 - Testing (Bước 4-5)**
   - Verify two-way sync
   - Test edge cases

## Testing Checklist
- [ ] Toggle Smart Mode từ status bar → Settings cập nhật
- [ ] Toggle Smart Mode từ settings → Status bar cập nhật
- [ ] Khi Smart Mode ON, danh sách apps hiển thị đúng
- [ ] Khi Smart Mode OFF, clear button vẫn hoạt động
- [ ] Không có memory leak (observer cleanup)
- [ ] Log messages đúng

## Success Criteria
✅ Toggle từ bất kỳ nơi nào sẽ cập nhật tất cả UI components
✅ Không có lag hoặc delay trong sync
✅ Danh sách apps luôn hiển thị đúng state
✅ Không có memory leak
