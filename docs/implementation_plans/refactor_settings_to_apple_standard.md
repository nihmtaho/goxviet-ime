# Kế hoạch refactor SettingsUI theo Apple Standard Design

## Mô tả

Refactor lại SettingsUI của GoxViet để sử dụng tiêu chuẩn thiết kế của Apple:

- Sử dụng `Settings` scene của SwiftUI (thay vì tạo NSWindow thủ công)
- Loại bỏ `WindowManager` custom window management
- Tối ưu Menubar để tiết kiệm RAM
- Giao diện chính (GoxVietApp) trở thành một frame rỗng
- Nhấn vào "Settings" trên Menubar sẽ mở Settings scene tiêu chuẩn

## Problem Issue

### Current Issues


1. **Custom Window Management**: Tạo NSWindow thủ công trong `WindowManager` tốn bộ nhớ
2. **Menubar Complexity**: Menubar có quá nhiều option (Input Method, Tone Style submenu) chiếm RAM
3. **Inactive Main Window**: GoxVietApp tạo một main window nhưng không sử dụng (EmptyView)
4. **Memory Overhead**: NSWindow management kèm theo lifecycle complexity

### Root Causes
1. Thiết kế không tuân theo Apple HIG (Human Interface Guidelines) cho settings window
2. Sử dụng `NSWindow` thay vì `Settings` scene (API tiêu chuẩn của Apple)
3. Menubar design không được tối ưu - có quá nhiều nested menu items
4. Không tận dụng built-in macOS Settings pattern

## Các bước triển khai

### Bước 1: Chuẩn bị - Xem lại SettingsRootView
- Xem xét tất cả settings sections hiện tại
- Phân loại thành các tab Settings tiêu chuẩn:
  - General (Input Method, Modern/Traditional Tone)
  - Typing (ESC Restore, Free Tone, Instant Restore)
  - Per-App (Smart Mode, Per-App Settings)
  - About

### Bước 2: Refactor GoxVietApp.swift
- Thay `EmptyView()` bằng `SettingsRootView()` trong `Settings`
- Loại bỏ manual `WindowManager` control
- Apple sẽ quản lý Settings window lifecycle

### Bước 3: Tối ưu Menubar (AppDelegate.swift)
- Loại bỏ submenu Input Method (chuyển vào Settings)
- Loại bỏ submenu Tone Style (chuyển vào Settings)
- Giữ lại: Vietnamese Input toggle, Smart Mode toggle
- Giữ lại: Check for Updates, About, Quit
- Loại bỏ: Shortcut info, Debug "View Log"

### Bước 4: Xóa hoặc simplify SettingsRootView
- Kiểm tra AppKit/SwiftUI Settings pattern
- Có thể cần tạo `SettingsTabView` structure mới
- Sử dụng @AppStorage cho state management

### Bước 5: Xóa WindowManager hoặc simplify
- Chỉ giữ `showUpdateWindow()` (vì Update window không phải Settings)
- Loại bỏ `showSettingsWindow()`

### Bước 6: Update AppDelegate
- Đơn giản hóa menu setup
- Link Settings action đến Apple Settings scene
- Kiểm tra logic accessibility permission

## Proposed Changes

### File: GoxVietApp.swift
```swift
@main
struct GoxVietApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    
    var body: some Scene {
        // Settings scene sẽ được quản lý bởi Apple/macOS
        Settings {
            SettingsTabView()
        }
    }
}
```

### File: AppDelegate.swift


- Loại bỏ submenu Input Method
- Loại bỏ submenu Tone Style
- Giữ: Vietnamese Input toggle, Smart Mode toggle, Settings, Check for Updates, About, Quit
- Link Settings action: `#selector(NSApplication.shared.sendAction(#selector(SettingsWindowController.showSettings(_:)), to: nil, from: nil))`

### File: SettingsRootView hoặc tạo SettingsTabView mới


- Tạo tab structure theo Apple design
- Áp dụng Glass background tính năng tiêu chuẩn
- Sử dụng @AppStorage cho persistence

### File: WindowManager.swift


- Giữ `showUpdateWindow()`, `closeUpdateWindow()`
- Loại bỏ `showSettingsWindow()`, `closeSettingsWindow()`
- Simplify logic

## Thời gian dự kiến


- Bước 1: 15 phút
- Bước 2: 20 phút
- Bước 3: 30 phút
- Bước 4: 45 phút
- Bước 5: 15 phút
- Bước 6: 30 phút
- **Tổng cộng**: ~2.5 giờ

## Tài nguyên cần thiết


- macOS Development skill (kiến thức về Settings scene)
- Xcode (để test)
- Apple HIG documentation

## Implementation Order


1. Chuẩn bị & phân tích (Step 1)
2. Update GoxVietApp (Step 2)
3. Tối ưu Menubar (Step 3)
4. Refactor SettingsUI (Step 4)
5. Simplify WindowManager (Step 5)
6. Update AppDelegate (Step 6)
7. Testing & validation

## Lợi ích


- ✅ Tiết kiệm RAM (không cần NSWindow management)
- ✅ Tuân theo Apple HIG standard
- ✅ Code ít phức tạp hơn
- ✅ Menubar nhẹ hơn
- ✅ Có thể dùng native keyboard shortcut cho Settings (Cmd+,)
