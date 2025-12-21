# SMART PER-APP MODE

**Date:** 2025-12-20  
**Version:** 1.0.1  
**Status:** ✅ IMPLEMENTED

---

## 1. OVERVIEW

Smart Per-App Mode is a feature that automatically remembers and restores your Vietnamese input preference (enabled/disabled) for each application. This eliminates the need to manually toggle Vietnamese input every time you switch between applications.

### 1.1. Key Benefits

- **Automatic Mode Switching:** Vietnamese input state is remembered per-application
- **Seamless Workflow:** No manual toggling needed when switching apps
- **User Preference:** Can be enabled/disabled globally via menu toggle
- **Efficient Storage:** Only stores apps where Vietnamese input is disabled (default is enabled)

---

## 2. HOW IT WORKS

### 2.1. Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      User Switches App                       │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              NSWorkspace Notification Observer               │
│        (didActivateApplicationNotification)                  │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                  PerAppModeManager                           │
│  • Detects app bundle ID                                     │
│  • Saves current app's state                                 │
│  • Loads new app's saved state                               │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                      AppState                                │
│  • Retrieves saved mode from UserDefaults                    │
│  • Applies mode to Rust engine (ime_set_enabled)             │
│  • Updates UI status icon                                    │
└─────────────────────────────────────────────────────────────┘
```

### 2.2. Components

#### 2.2.1. AppState (`AppState.swift`)

Central state manager that:
- Stores global Vietnamese input enabled/disabled state
- Manages UserDefaults for persistent storage
- Provides per-app mode getter/setter methods
- Default state: **Enabled** (only disabled apps are stored)

**Key Methods:**
```swift
func getPerAppMode(bundleId: String) -> Bool
func setPerAppMode(bundleId: String, enabled: Bool)
func clearPerAppMode(bundleId: String)
func clearAllPerAppModes()
```

**Storage Strategy:**
- Only apps with **disabled** Vietnamese input are stored in UserDefaults
- Apps not in storage default to **enabled**
- This saves storage space and handles new apps gracefully

#### 2.2.2. PerAppModeManager (`PerAppModeManager.swift`)

Monitors application switching and manages per-app state:
- Observes `NSWorkspace.didActivateApplicationNotification`
- Detects frontmost application changes
- Saves current app's state before switching
- Restores new app's saved state
- Clears composition buffer on app switch

**Lifecycle:**
```swift
// Start monitoring (called on app launch)
PerAppModeManager.shared.start()

// Stop monitoring (called on app quit)
PerAppModeManager.shared.stop()

// Force refresh current app state
PerAppModeManager.shared.refresh()
```

#### 2.2.3. InputManager Integration

The `InputManager` class integrates with Smart Per-App Mode:
- Checks `AppState.shared.isEnabled` instead of local state
- Calls `PerAppModeManager.shared.setStateForCurrentApp()` when user toggles
- Only saves per-app state if Smart Mode is enabled

---

## 3. USER EXPERIENCE

### 3.1. Enabling Smart Per-App Mode

1. Click the Vietnamese IME status icon in menu bar (🇻🇳 or EN)
2. Toggle "Smart Per-App Mode" switch to **ON**
3. The feature is now active

### 3.2. Using Smart Per-App Mode

**Scenario Example:**

1. **Chrome Browser:** User disables Vietnamese input (types in English)
2. **Switch to Notes App:** Vietnamese input automatically **enables**
3. **Switch back to Chrome:** Vietnamese input automatically **disables**
4. **Switch to Slack:** Vietnamese input automatically **enables** (default)

### 3.3. Visual Feedback

- **Menu Bar Icon:** 🇻🇳 (enabled) or EN (disabled)
- **Status Changes:** Icon updates immediately on app switch
- **Menu Toggle:** Reflects current state with visual toggle switch

---

## 4. CONFIGURATION

### 4.1. Enable/Disable Smart Mode

**Via Menu Bar:**
```
Click Menu Bar Icon → Toggle "Smart Per-App Mode"
```

**Via AppState (Programmatic):**
```swift
AppState.shared.isSmartModeEnabled = true  // Enable
AppState.shared.isSmartModeEnabled = false // Disable
```

### 4.2. View Current Settings

**Via Menu Bar:**
```
Click Menu Bar Icon → Settings...
```

Displays:
- Current app name and bundle ID
- Smart Mode status (Enabled/Disabled)
- Number of apps with custom settings
- Current configuration (input method, tone style, etc.)

### 4.3. Clear Per-App Settings

**Via Settings Dialog:**
```
Click Menu Bar Icon → Settings... → Clear Per-App Settings
```

This resets all applications to default (Vietnamese input enabled).

---

## 5. TECHNICAL DETAILS

### 5.1. Data Storage

**UserDefaults Key:**
```swift
"com.vietnamese.ime.perAppModes"
```

**Data Structure:**
```swift
[String: Bool]  // [bundleId: isEnabled]
```

**Example:**
```json
{
  "com.google.Chrome": false,
  "com.microsoft.VSCode": false,
  "com.apple.Terminal": false
}
```

Apps not in this dictionary default to `true` (enabled).

### 5.2. App Identification

Applications are identified by their **Bundle Identifier**:
- `com.google.Chrome` → Google Chrome
- `com.apple.Safari` → Safari
- `com.microsoft.VSCode` → Visual Studio Code
- `com.apple.Terminal` → Terminal

### 5.3. Performance Considerations

- **App Switch Detection:** O(1) - immediate notification
- **State Lookup:** O(1) - dictionary lookup in UserDefaults
- **State Save:** O(1) - dictionary update
- **Memory Usage:** Minimal (only stores exceptions, not all apps)

### 5.4. Thread Safety

- All state changes occur on **main thread** via NotificationCenter observers
- NSWorkspace notifications are posted to main queue
- No race conditions between app switching and manual toggling

---

## 6. TROUBLESHOOTING

### 6.1. Smart Mode Not Working

**Check:**
1. Is Smart Per-App Mode enabled in menu?
2. Does the app have accessibility permissions?
3. Check logs for "PerAppModeManager started" message

**Solution:**
```swift
// Force refresh current app state
PerAppModeManager.shared.refresh()
```

### 6.2. Wrong State After App Switch

**Possible Causes:**
- UserDefaults corruption
- App bundle ID changed (rare)

**Solution:**
1. Clear per-app settings via Settings dialog
2. Restart the IME application
3. Re-configure preferred states

### 6.3. State Not Persisting

**Check:**
1. UserDefaults write permissions
2. App sandbox entitlements

**Debug:**
```swift
// View all saved states
let states = AppState.shared.getAllPerAppModes()
print("Saved states: \(states)")
```

### 6.4. Clearing Stuck State

**Via Code:**
```swift
// Clear specific app
AppState.shared.clearPerAppMode(bundleId: "com.example.app")

// Clear all apps
AppState.shared.clearAllPerAppModes()
```

**Via Terminal:**
```bash
# Reset UserDefaults for Vietnamese IME
defaults delete com.vietnamese.ime.perAppModes
```

---

## 7. IMPLEMENTATION NOTES

### 7.1. Important: NSWorkspace Notification

The app switching detection **MUST** use `NSWorkspace.shared.notificationCenter`, **NOT** `NotificationCenter.default`:

```swift
// ✅ CORRECT
NSWorkspace.shared.notificationCenter.addObserver(
    forName: NSWorkspace.didActivateApplicationNotification,
    object: nil,
    queue: .main
) { notification in
    // Handle app switch
}

// ❌ WRONG - Will not work!
NotificationCenter.default.addObserver(
    forName: NSWorkspace.didActivateApplicationNotification,
    ...
)
```

### 7.2. State Management Best Practices

1. **Always clear composition buffer** on app switch:
   ```swift
   ime_clear()
   ```

2. **Save previous app state** before switching:
   ```swift
   if let previousId = previousBundleId {
       let currentMode = AppState.shared.isEnabled
       AppState.shared.setPerAppMode(bundleId: previousId, enabled: currentMode)
   }
   ```

3. **Use silent state updates** during app switching:
   ```swift
   // Don't post notification during automatic restore
   AppState.shared.setEnabledSilently(savedMode)
   ```

### 7.3. Rust Engine Integration

The Rust engine must be updated with each state change:

```swift
// Set Vietnamese input enabled/disabled
ime_set_enabled(enabled)

// Clear composition buffer
ime_clear()
```

---

## 8. FUTURE ENHANCEMENTS

### 8.1. Potential Features

- **App Whitelist/Blacklist:** Always enable/disable for specific apps
- **Smart Learning:** Automatically learn user patterns
- **Domain-Based Rules:** Different rules for different app categories
- **Profile Management:** Multiple per-app profiles for different workflows
- **Export/Import:** Backup and restore per-app settings

### 8.2. UI Improvements

- **Settings Window:** Dedicated settings UI instead of alerts
- **App List View:** Visual list of all apps with their states
- **Quick Toggle:** Right-click menu for current app state
- **Statistics:** Usage statistics per application

---

## 9. TESTING

### 9.1. Manual Testing Checklist

- [ ] Enable Smart Mode
- [ ] Disable Vietnamese in Chrome
- [ ] Switch to Notes - should auto-enable
- [ ] Switch back to Chrome - should auto-disable
- [ ] Disable Smart Mode
- [ ] Manual toggle should work normally
- [ ] Re-enable Smart Mode
- [ ] Previous states should be restored
- [ ] Clear per-app settings
- [ ] All apps should default to enabled

### 9.2. Edge Cases to Test

- [ ] First time using an app (should default to enabled)
- [ ] App without bundle ID (should be ignored)
- [ ] Rapid app switching
- [ ] Toggle while Smart Mode is disabled
- [ ] Toggle while Smart Mode is enabled
- [ ] App quit and restart
- [ ] System restart

---

## 10. LOGGING

### 10.1. Debug Logs

Enable debug logging to troubleshoot:

```swift
#if DEBUG
Log.isEnabled = true
#endif
```

### 10.2. Key Log Messages

```
PerAppModeManager started (current app: com.google.Chrome)
App switched: Google Chrome (com.google.Chrome)
Mode restored for Google Chrome: enabled
State saved for Google Chrome: disabled
Smart Per-App Mode: ON
Per-app mode saved: com.google.Chrome = disabled
```

### 10.3. Log Location

```
~/Library/Logs/VietnameseIME/keyboard.log
```

View log via menu: **View Log...**

---

## 11. RELATED DOCUMENTATION

- `ARCHITECTURE.md` - Overall system architecture
- `PERFORMANCE_OPTIMIZATION_GUIDE.md` - Performance considerations
- `docs/README.md` - Documentation index

---

## 12. VERSION HISTORY

| Version | Date       | Changes                                    |
|---------|------------|--------------------------------------------|
| 1.0.1   | 2025-12-20 | Initial Smart Per-App Mode implementation  |

---

## 13. CREDITS

Based on reference implementation architecture, rewritten for Vietnamese IME Fast project.

**Implementation:** AppState.swift, PerAppModeManager.swift  
**Integration:** InputManager.swift, AppDelegate.swift  
**UI:** MenuToggleView.swift