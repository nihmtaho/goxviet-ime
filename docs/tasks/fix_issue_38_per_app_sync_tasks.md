# Nhiệm vụ sửa lỗi Issue #38 - Per-App Mode Synchronization

## Phase 1: Notification Infrastructure
- [ ] Thêm notification name `.smartModeChanged` vào `Notification.Name` extension
- [ ] Update `isSmartModeEnabled` property với `didSet` để post notification
- [ ] Test: Verify notification được post khi property thay đổi

## Phase 2: Status Bar Update  
- [ ] Thêm observer cho `.smartModeChanged` trong `AppDelegate.setupObservers()`
- [ ] Implement handler để update `smartModeToggleView?.updateState()`
- [ ] Test: Toggle từ settings → verify status bar cập nhật

## Phase 3: Settings Update
- [ ] Thêm `@State private var smartModeObserver: NSObjectProtocol?` vào SettingsRootView
- [ ] Implement `setupSmartModeObserver()` method
- [ ] Implement `cleanupSmartModeObserver()` method  
- [ ] Call `setupSmartModeObserver()` trong `.onAppear`
- [ ] Call `cleanupSmartModeObserver()` trong `.onDisappear`
- [ ] Test: Toggle từ status bar → verify settings cập nhật

## Phase 4: Integration Testing
- [ ] Test: Toggle Smart Mode từ status bar
  - [ ] Settings window toggle cập nhật
  - [ ] Danh sách apps reload
  - [ ] Log message hiển thị đúng
- [ ] Test: Toggle Smart Mode từ settings window
  - [ ] Status bar toggle cập nhật
  - [ ] State persist sau khi đóng/mở settings
  - [ ] Log message hiển thị đúng
- [ ] Test: Edge cases
  - [ ] Toggle nhanh liên tục (no lag)
  - [ ] Open/close settings nhiều lần (no memory leak)
  - [ ] Switch apps trong khi settings mở

## Phase 5: Verification & Documentation
- [ ] Chạy full test suite
- [ ] Verify không có memory leak (Instruments)
- [ ] Update CHANGELOG.md
- [ ] Commit với message: "fix(macos): sync per-app mode toggle between status bar and settings (#38)"
