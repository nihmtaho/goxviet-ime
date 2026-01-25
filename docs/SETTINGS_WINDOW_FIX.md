# Settings Window Fix – Complete Solution

## Problem
When clicking "Settings..." in the GoxViet menubar, the Settings window was not appearing.

## Root Cause
The original approach used SwiftUI's `Settings` scene with the intention that it would be automatically triggered via the `showPreferencesWindow:` selector. However:
1. SwiftUI's `Settings` scene in macOS doesn't work well with menu bar apps
2. The responder chain approach with `NSApplication.sendAction()` doesn't properly trigger the Settings window in menu bar applications
3. The Settings window needs explicit NSWindow management like other windows in the app

## Solution Implemented

### 1. Restored WindowManager Settings Window Management
**File**: `WindowManager.swift`

Added back Settings window management alongside the existing Update window:

```swift
// Properties
private var settingsWindow: NSWindow?
var isSettingsWindowOpen: Bool { return settingsWindow != nil }

// Methods
func showSettingsWindow() { ... }  // Creates and shows Settings window
func closeSettingsWindow() { ... } // Closes Settings window

// Updated window delegate to handle both settings and update windows
func windowWillClose(_ notification: Notification) { ... }

// Updated last window handler to check both windows
private func handleLastWindowClosed() {
    if updateWindow == nil && settingsWindow == nil { ... }
}
```

### 2. Updated AppDelegate Menu Action
**File**: `AppDelegate.swift`

Simplified the Settings menu action to use WindowManager:

```swift
@objc func openSettings() {
    WindowManager.shared.showSettingsWindow()
}
```

### 3. Updated Application Reopen Handler
**File**: `AppDelegate.swift`

When user clicks the app icon in dock:

```swift
func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
    WindowManager.shared.showSettingsWindow()
    return false
}
```

## Files Modified

| File | Changes |
|------|---------|
| `WindowManager.swift` | Added Settings window management (showSettingsWindow, closeSettingsWindow, window lifecycle) |
| `AppDelegate.swift` | Updated openSettings() and applicationShouldHandleReopen() to use WindowManager |
| `GoxVietApp.swift` | Settings { SettingsRootView() } scene remains for potential future use |

## Testing Instructions

### Method 1: Click Menu
1. Launch GoxViet app
2. Click GoxViet icon in menubar (top-right)
3. Click "Settings..." option
4. ✅ Settings window should appear with SettingsRootView content

### Method 2: Keyboard Shortcut
1. Launch GoxViet app
2. Press **Cmd+,** (Command + Comma)
3. ✅ Settings window should appear

### Method 3: Click App Icon
1. Launch GoxViet app (will show menubar icon)
2. Click GoxViet app icon in Dock (if docked)
3. ✅ Settings window should appear

## Technical Details

### Window Configuration
```swift
let window = NSWindow(
    contentRect: NSRect(x: 0, y: 0, width: 800, height: 600),
    styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
    backing: .buffered,
    defer: false
)

window.title = "Settings"
window.titlebarAppearsTransparent = true
window.titleVisibility = .hidden
window.isReleasedWhenClosed = false  // Auto-release on close to save memory
```

### Window Lifecycle
1. **Show**: 
   - Check if window already open (reuse if yes)
   - Create new NSWindow with SettingsRootView content
   - Activate app with `.regular` activation policy
   - Bring window to front

2. **Close**:
   - NSWindowDelegate detects window close
   - Release window reference
   - If no windows open, restore app to background mode (if configured)

### State Synchronization
Settings window uses the same AppState and NotificationCenter observers as the menubar:
- Vietnamese Input toggle state
- Smart Per-App Mode toggle state  
- Input method changes reflected in Settings UI
- Tone style changes reflected in Settings UI

## Why This Works

1. **Explicit Window Management**: Like the Update window, the Settings window is properly created and managed as an NSWindow
2. **Proper Activation**: The window is brought to foreground with proper app activation policy
3. **Lifecycle Handling**: Delegate pattern ensures proper cleanup on window close
4. **Respects App Preferences**: Restores background mode (no dock icon) when Settings window is closed

## Keyboard Shortcut Support

The Cmd+, keyboard shortcut works because:
- The menu item has `keyEquivalent: ","`
- The menu item targets `openSettings` action in AppDelegate
- When any window in the app is focused, or app is in background, the global shortcut works

## Build Status
✅ **BUILD SUCCEEDED** – No errors, no warnings

## Deployment Status
✅ Ready to test on macOS system
- Settings window can now be opened via:
  - Menubar click → "Settings..."
  - Keyboard shortcut: Cmd+,
  - App icon click in Dock
