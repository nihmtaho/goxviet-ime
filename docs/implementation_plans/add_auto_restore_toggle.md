# Kế hoạch triển khai toggle Auto-Restore trong Settings

## Mô tả
Thêm toggle cho tính năng auto-restore (tự động khôi phục từ tiếng Anh) vào phần General Settings, cho phép người dùng bật/tắt tính năng này.

## Problem Issue
### Current Issues
- Tính năng `instant_restore_enabled` đã có trong Rust engine nhưng không có giao diện điều khiển trong Settings
- Người dùng không thể tắt tính năng auto-restore nếu muốn

### Root Causes
- Thiếu FFI function `ime_instant_restore()` để set config từ Swift
- Thiếu UI toggle trong GeneralSettingsView
- Thiếu @AppStorage binding cho setting này

## Proposed Changes

### 1. Rust Core (lib.rs)
Thêm FFI function:
```rust
#[no_mangle]
pub extern "C" fn ime_instant_restore(enabled: bool) {
    let mut guard = lock_engine();
    if let Some(ref mut e) = *guard {
        e.set_instant_restore(enabled);
    }
}
```

### 2. Swift Bridge (RustBridge.swift)
Thêm wrapper function:
```swift
func setInstantRestore(_ enabled: Bool) {
    Log.info("Instant restore: \(enabled)")
    ime_instant_restore(enabled)
}
```

### 3. Settings UI (SettingsRootView.swift)
- Thêm `@AppStorage("instantRestoreEnabled")` trong SettingsRootView
- Thêm binding parameter trong GeneralSettingsView
- Thêm Toggle control trong Smart Features section

### 4. AppState Integration
- Thêm property `instantRestoreEnabled` vào AppState.swift
- Đồng bộ với InputManager khi thay đổi

## Các bước triển khai

1. **Bước 1:** Thêm FFI function `ime_instant_restore()` vào lib.rs
2. **Bước 2:** Thêm Swift wrapper `setInstantRestore()` vào RustBridge.swift
3. **Bước 3:** Thêm @AppStorage binding vào SettingsRootView
4. **Bước 4:** Thêm Toggle UI vào GeneralSettingsView trong Smart Features section
5. **Bước 5:** Update AppState với property mới
6. **Bước 6:** Update InputManager để sync config khi khởi động

## Thời gian dự kiến
- Rust FFI: 5 phút
- Swift Bridge: 5 phút  
- UI Integration: 10 phút
- Testing: 10 phút
**Tổng:** ~30 phút

## Tài nguyên cần thiết
- Rust compiler (cargo)
- Xcode
- Access to macOS testing device

## Implementation Order
1. FFI layer (Rust) - foundation
2. Bridge layer (Swift wrapper) - interface
3. UI layer (SwiftUI) - user-facing
4. State management (AppState) - persistence
5. Testing & validation
