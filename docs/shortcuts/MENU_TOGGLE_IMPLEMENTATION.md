# MENU TOGGLE IMPLEMENTATION

**Date:** 2025-12-20  
**Version:** 1.0.2  
**Author:** VietnameseIMEFast Development Team

---

## OVERVIEW

Tài liệu này mô tả việc triển khai SwiftUI Toggle button với NSHostingView trong menu bar của VietnameseIMEFast, thay thế checkbox truyền thống để cung cấp trải nghiệm người dùng hiện đại hơn.

---

## PROBLEM STATEMENT

### Vấn đề ban đầu:
1. **Checkbox truyền thống** - Sử dụng `NSMenuItem.state` (.on/.off) không trực quan
2. **NSSwitch rendering issues** - Switch control mất màu sắc không ổn định
3. **Selection highlight conflict** - Nền xanh xuất hiện khi switch có màu, biến mất khi switch mất màu
4. **Inconsistent state** - Mối liên hệ bất thường giữa highlight và switch appearance

### Root Cause Analysis:

Sau khi nghiên cứu và tham khảo reference implementation, phát hiện:

#### **Vấn đề với NSSwitch trực tiếp:**
- `NSSwitch` là AppKit control phụ thuộc vào parent view's rendering context
- Khi `NSMenuItem` được highlighted (hovered), NSMenu vẽ selection background
- Selection state ảnh hưởng đến child view rendering pipeline
- `NSSwitch` bị re-render trong context của highlighted menu item
- Dẫn đến: **Intermittent color loss** và **highlight conflicts**

#### **Mối liên hệ giữa highlight và switch color:**
```
Menu Item NOT Highlighted → Switch MẤT màu → NO highlight (vì không hover)
Menu Item IS Highlighted → Switch CÓ màu → CÓ highlight (blue background)
```

Điều này cho thấy:
- Switch color được refresh khi menu item re-renders (during highlight)
- Nhưng không stable khi menu đóng/mở lại
- AppKit menu rendering và NSSwitch rendering không tương thích hoàn hảo

### Yêu cầu:
- Sử dụng **stable toggle control** không bị ảnh hưởng bởi menu rendering
- Loại bỏ hoàn toàn selection highlight màu xanh
- Đảm bảo toggle LUÔN giữ màu sắc chính xác
- Trải nghiệm mượt mà, consistent với macOS design

---

## SOLUTION ARCHITECTURE

### Reference Implementation Analysis:

Từ example project (`gonhanh.org-main/platforms/macos/MenuBar.swift`), phát hiện họ sử dụng:

```swift
// SwiftUI Toggle with NSHostingView
let toggleView = NSHostingView(rootView:
    Toggle("", isOn: binding)
    .toggleStyle(.switch)
    .labelsHidden()
    .scaleEffect(0.8)
)
```

**Key Insights:**
1. ✅ **SwiftUI Toggle** thay vì NSSwitch trực tiếp
2. ✅ **NSHostingView** để embed SwiftUI trong AppKit
3. ✅ Tự động handle rendering và state management
4. ✅ Isolated từ menu item's selection behavior

### 1. Custom View Class: `MenuToggleView`

Tạo một custom `NSView` subclass sử dụng **SwiftUI Toggle + NSHostingView**:
- Chứa `NSTextField` (label) và `NSHostingView<Toggle>` (toggle control)
- SwiftUI Toggle với `.toggleStyle(.switch)` và `.scaleEffect(0.8)`
- Override `draw(_:)` để vẽ background trong suốt
- Override `acceptsFirstResponder` để ngăn selection
- Quản lý state qua SwiftUI Binding

**File:** `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/MenuToggleView.swift`

```swift
class MenuToggleView: NSView {
    private var hostingView: NSHostingView<AnyView>?
    private let label: NSTextField
    private var toggleBinding: Binding<Bool>
    
    var isOn: Bool { didSet { /* triggers binding */ } }
    
    init(labelText: String, isOn: Bool, onToggle: @escaping (Bool) -> Void) {
        // Create SwiftUI Toggle with NSHostingView
        let toggleView = Toggle("", isOn: binding)
            .toggleStyle(.switch)
            .labelsHidden()
            .scaleEffect(0.8)
        
        hostingView = NSHostingView(rootView: AnyView(toggleView))
    }
    
    // Override to prevent selection highlighting
    override func draw(_ dirtyRect: NSRect)
    override var acceptsFirstResponder: Bool { return false }
}
```

### 2. Integration trong AppDelegate

**File:** `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/AppDelegate.swift`

```swift
var toggleView: MenuToggleView?

func setupMenu() {
    let toggleMenuItem = NSMenuItem()
    
    // Create custom toggle view
    toggleView = MenuToggleView(
        labelText: "Vietnamese Input", 
        isOn: isEnabled
    ) { [weak self] newState in
        self?.handleToggleChanged(newState)
    }
    
    toggleMenuItem.view = toggleView
    menu.addItem(toggleMenuItem)
}
```

---

## KEY IMPLEMENTATION DETAILS

### A. Preventing Selection Highlight

**Problem:** NSMenuItem với custom view vẫn hiển thị nền xanh khi hover/click

**Solution:** Override drawing và responder chain:
```swift
override func draw(_ dirtyRect: NSRect) {
    // Draw clear background to prevent blue selection highlight
    NSColor.clear.setFill()
    dirtyRect.fill()
}

override var acceptsFirstResponder: Bool {
    return false
}

override func menu(for event: NSEvent) -> NSMenu? {
    return nil
}
```

### B. Maintaining Toggle Color State (SwiftUI Approach)

**Problem:** NSSwitch mất màu sắc do conflict với menu rendering

**Root Cause:**
- AppKit NSSwitch rendering phụ thuộc vào parent view state
- Menu item selection state interferes với switch appearance
- NSSwitch không được thiết kế để stable trong menu context

**Solution:** Sử dụng SwiftUI Toggle với NSHostingView:

```swift
import SwiftUI

class MenuToggleView: NSView {
    private var hostingView: NSHostingView<AnyView>?
    
    private func setupView() {
        // Create SwiftUI Toggle
        let toggleView = Toggle("", isOn: Binding(
            get: { [weak self] in self?.isOn ?? false },
            set: { [weak self] newValue in
                self?.isOn = newValue
                self?.onToggleCallback?(newValue)
            }
        ))
        .toggleStyle(.switch)
        .labelsHidden()
        .scaleEffect(0.8)
        
        // Wrap in NSHostingView
        hostingView = NSHostingView(rootView: AnyView(toggleView))
        hostingView?.frame = NSRect(x: 162, y: 2, width: 50, height: 28)
        
        if let hostingView = hostingView {
            addSubview(hostingView)
        }
    }
}
```

**Why SwiftUI Toggle Works:**
1. ✅ **Self-contained rendering** - SwiftUI manages its own display list
2. ✅ **Isolated from AppKit** - NSHostingView creates rendering boundary
3. ✅ **Stable appearance** - Not affected by parent menu item's state
4. ✅ **Automatic dark mode** - SwiftUI handles appearance changes
5. ✅ **Proven solution** - Used successfully in reference implementation

### C. Layout Configuration

```swift
// View frame (increased width for better spacing)
let toggleView = NSView(frame: NSRect(x: 0, y: 0, width: 220, height: 32))

// Label position (left side)
label.frame = NSRect(x: 16, y: 6, width: 140, height: 20)

// SwiftUI Toggle with NSHostingView (right side)
hostingView?.frame = NSRect(x: 162, y: 2, width: 50, height: 28)
```

**Visual Layout:**
```
┌─────────────────────────────────────┐
│   Vietnamese Input         [◯——]    │
│   ↑ 16px margin           ↑ right   │
│   Label 140px (medium)    aligned   │
│                           50px wide │
└─────────────────────────────────────┘
     Total width: 220px
     Height: 32px (matched with reference)
```

**Font Configuration:**
```swift
label.font = NSFont.systemFont(ofSize: 13, weight: .medium)
// Consistent with reference implementation
```

---

## CALLBACK FLOW

### Toggle State Change Flow:

```
User clicks NSSwitch
       ↓
switchToggled(_:) in MenuToggleView
       ↓
onToggle?(newState) callback
       ↓
handleToggleChanged(_:) in AppDelegate
       ↓
1. Update isEnabled
2. InputManager.shared.setEnabled()
3. updateStatusIcon() (🇻🇳 or EN)
4. Log state change
```

### Keyboard Shortcut Flow:

```
User presses Cmd+Shift+V
       ↓
InputManager detects shortcut
       ↓
NotificationCenter posts .toggleVietnamese
       ↓
AppDelegate receives notification
       ↓
1. Toggle isEnabled
2. handleToggleChanged()
3. updateToggleMenuItem() with animation
```

---

## BENEFITS

### ✅ User Experience:
- **Native macOS control** - Consistent with System Settings
- **Visual feedback** - Clear ON/OFF state with SwiftUI animation
- **No distractions** - No unwanted selection highlights
- **Stable appearance** - Toggle ALWAYS maintains proper color
- **Dark mode support** - Automatic with SwiftUI

### ✅ Code Quality:
- **Modern approach** - SwiftUI + AppKit integration
- **Separation of concerns** - MenuToggleView is reusable
- **Type-safe callbacks** - Swift closures with Binding
- **Maintainable** - Based on proven reference implementation
- **Future-proof** - Ready for full SwiftUI migration

### ✅ Performance:
- **Lightweight** - Minimal view hierarchy with NSHostingView
- **Efficient rendering** - SwiftUI's optimized display list
- **No leaks** - Proper weak references in bindings
- **Stable** - No intermittent rendering issues

### ✅ Technical Correctness:
- **Root cause addressed** - Isolated from AppKit menu rendering
- **No workarounds needed** - Proper solution, not hacks
- **Proven in production** - Reference implementation validated

---

## TESTING CHECKLIST

- [x] Toggle changes state correctly via click
- [x] Keyboard shortcut updates toggle state
- [x] Status bar icon syncs with toggle state
- [x] NO blue highlight when clicking/hovering toggle
- [x] Toggle ALWAYS maintains color after menu close/reopen
- [x] Toggle ALWAYS maintains color during hover
- [x] SwiftUI animation plays smoothly on state change
- [x] Toggle state persists correctly
- [x] Works perfectly in Light mode
- [x] Works perfectly in Dark mode
- [x] Toggle appearance consistent across menu interactions
- [x] No rendering glitches or color loss

---

## LESSONS LEARNED

### ❌ What Didn't Work:

**Approach 1: NSSwitch Directly**
- ❌ Intermittent color loss
- ❌ Selection highlight conflicts
- ❌ Rendering dependent on parent view state
- ❌ Not stable in menu context

**Approach 2: Override Drawing Only**
- ❌ Couldn't prevent highlight when switch had color
- ❌ Couldn't maintain color reliably
- ❌ Fighting against AppKit's rendering pipeline

### ✅ What Worked:

**Approach 3: SwiftUI Toggle + NSHostingView**
- ✅ Completely isolated rendering
- ✅ Stable appearance in all scenarios
- ✅ No selection highlight conflicts
- ✅ Automatic dark mode support
- ✅ Proven in reference implementation

### Key Takeaway:
> **When AppKit controls have rendering conflicts, bridge to SwiftUI instead of fighting AppKit's behavior.**

---

## FUTURE IMPROVEMENTS

### Potential Enhancements:
1. **Accessibility** - Add VoiceOver labels and hints for SwiftUI Toggle
2. **Tooltips** - Show helpful tips on hover
3. **Custom Styling** - Use `.tint()` modifier for brand colors
4. **Settings Persistence** - Remember toggle state across launches
5. **Multiple Toggles** - Reuse MenuToggleView for other settings
6. **Full SwiftUI Menu** - Consider SwiftUI MenuBarExtra for macOS 13+

### Code Quality:
- Extract layout constants to struct
- Add unit tests for state management
- Document SwiftUI/AppKit bridging patterns

---

## REFERENCES

### Apple Documentation:
- [NSHostingView](https://developer.apple.com/documentation/swiftui/nshostingview) - Embedding SwiftUI in AppKit
- [SwiftUI Toggle](https://developer.apple.com/documentation/swiftui/toggle) - Toggle control
- [NSMenuItem Custom Views](https://developer.apple.com/documentation/appkit/nsmenuitem/1514845-view)
- [NSView Drawing](https://developer.apple.com/documentation/appkit/nsview/1483686-draw)

### Related Files:
- `MenuToggleView.swift` - SwiftUI Toggle + NSHostingView implementation
- `AppDelegate.swift` - Menu setup and state management
- `InputManager.swift` - Keyboard shortcut handling

### Reference Implementation:
- `example-project/gonhanh.org-main/platforms/macos/MenuBar.swift:154-192`
- Uses identical NSHostingView + SwiftUI Toggle approach
- Proven stable in production environment

---

## NOTES

### ⚠️ Important:
- Do NOT use `item.state = .on/.off` when using custom view
- Do NOT use NSSwitch directly in menu items (rendering issues)
- MUST use SwiftUI Toggle with NSHostingView for stability
- MUST override `draw(_:)` to prevent selection highlight
- ALWAYS use weak self in bindings to prevent retain cycles

### 💡 Tips:
- SwiftUI Toggle automatically handles dark mode
- NSHostingView creates proper rendering isolation
- `.scaleEffect(0.8)` gives better visual proportion
- Test menu close/reopen to verify color stability
- Reference implementation is your friend!

### 🔧 Debugging:
If toggle loses color:
1. Check if NSHostingView is being properly retained
2. Verify binding is capturing self weakly
3. Ensure frame is set before adding to superview
4. Confirm SwiftUI view is wrapped in AnyView

---

**Last Updated:** 2025-12-20  
**Version:** 1.0.2 (SwiftUI Implementation)  
**Status:** ✅ Implemented, Tested, and Stable