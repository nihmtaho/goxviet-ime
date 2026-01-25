# Detailed Action Sequence: Check for Updates → Close Window

## Scenario: No New Version Found (upToDate State)

This document traces EXACTLY what happens at each step when:
1. User clicks "Check for Updates" from About screen
2. No new version is found
3. User clicks "Close" to close the Update window

---

## Action 1: Click "Check for Updates"

```
User Action: Click menu item or About → "Check for Updates"
     ↓
AboutView.checkForUpdatesButton.action
     ↓
WindowManager.shared.showUpdateWindow()
     ↓
Creates NSWindow with UpdateWindowView
     ↓
@EnvironmentObject UpdateManager injected via environment
     ↓
UpdateManager.shared.resumeChecking() called
     ↓
Timer re-scheduled (if was previously paused)
     ↓
Window appears on screen
```

### UpdateManager State After Step 1

```
isRunning = true
timer = Timer (scheduled, firing every 6 hours)
updateState = .idle
isChecking = false
```

---

## Action 2: UpdateManager Checks for Updates

```
Timer fires or user clicks "Check" button
     ↓
UpdateManager.checkForUpdates(userInitiated: true)
     ↓
updateState = .checking
statusMessage = "Checking for updates..."
     ↓
URLSession.dataTask to GitHub API
     ↓
Gets latest release info
     ↓
handleRelease(release, userInitiated: true)
```

### In handleRelease() - NO NEW VERSION case

```
Get latestVersion from release.tag_name
Get currentVersion from Bundle.main
     ↓
if !isNewerVersion(latestVersion, currentVersion):
    ↓
    updateState = .upToDate
    statusMessage = "You are up to date"
    lastChecked = Date()
    ↓
    (NO download, NO installation)
    ↓
    Loop back to waiting for next timer fire
```

### UpdateManager State After Step 2

```
isRunning = true
timer = Timer (continues firing)
updateState = .upToDate ← KEY STATE
statusMessage = "You are up to date"
isChecking = false
```

---

## Action 3: UI Renders "You're Up to Date"

```
updateState = .upToDate triggers UI update
     ↓
UpdateWindowView.mainContentView observes updateState
     ↓
case .upToDate:
    upToDateView() renders:
    ├─ Green checkmark icon
    ├─ "You're Up to Date" title
    ├─ Current version number
    ├─ "You have the latest version of GoxViet"
    ├─ Last checked timestamp
    └─ "Close" button
```

### User Sees On Screen

```
┌─────────────────────────────────────┐
│ Check for Updates                   │
├─────────────────────────────────────┤
│                                     │
│          ✅ Green Checkmark        │
│                                     │
│     You're Up to Date              │
│     Version 1.5.2                  │
│                                     │
│  You have the latest version of    │
│           GoxViet                   │
│                                     │
│   Last checked: just now           │
│                                     │
│          [Close]                   │
│                                     │
└─────────────────────────────────────┘
```

---

## Action 4: User Clicks "Close" Button

```
User Action: Click "Close" button on Update window
     ↓
UpdateWindowView.actionButtonsView
     case .upToDate:
         Button("Close") {
             closeWindow()
         }
     ↓
UpdateWindowView.closeWindow()
```

### In closeWindow()

```
DispatchQueue.main.async {
    WindowManager.shared.closeUpdateWindow()
    Log.info("Update window close initiated safely")
}
     ↓
Dispatched to main thread (async)
```

---

## Action 5: WindowManager Closes Window

```
WindowManager.shared.closeUpdateWindow()
     ↓
║ CRITICAL FIX ║
║ UpdateManager.shared.pauseChecking() ← PAUSE TIMER
║
This is where the FIX prevents the crash!
     ↓
Inside pauseChecking():
    DispatchQueue.main.async {
        ResourceManager.shared.unregister(
            timerIdentifier: "UpdateManager.checkTimer"
        )
        self.timer = nil
    }
     ↓
Timer is STOPPED - no more periodic checks
     ↓
updateWindow?.close()
     ↓
updateWindow = nil
     ↓
handleLastWindowClosed()
```

### UpdateManager State After Step 5

```
isRunning = true (still true!)
timer = nil ← TIMER REMOVED
updateState = .upToDate (unchanged)
statusMessage = "You are up to date" (unchanged)
isChecking = false
```

---

## Action 6: Window Deallocates Safely

```
NSWindow.close() called
     ↓
windowWillClose delegate callback
     ↓
NSHostingView deallocates
     ↓
UpdateWindowView deallocates
     ↓
All SwiftUI state cleaned up
     ↓
@EnvironmentObject UpdateManager DETACHED from view
(but singleton still exists)
     ↓
Window memory freed
```

**KEY:** UpdateManager.shared is still ALIVE, but timer is PAUSED

---

## Action 7: Timer Does NOT Fire (Because Paused)

```
Normally, timer would fire every 6 hours
     ↓
But we called pauseChecking(), so:
    ResourceManager.unregister(...) removed the timer
    self.timer = nil cleared the reference
     ↓
No timer callback = NO attempt to update UI on deallocated view
     ↓
No EXC_BAD_ACCESS ✅
```

---

## Action 8: App Continues Normally

```
User can:
├─ Type Vietnamese ✅
├─ Click "Check for Updates" again ✅
│  (Will call resumeChecking() and restart timer)
├─ Use all other app features ✅
└─ Close app normally ✅
```

---

## What Changed Before vs After

### BEFORE (Crashes)

```
Close window
    ↓
window.close()
    ↓
updateWindow = nil
    ↓
View deallocates
    ↓
Timer STILL FIRING every 6 hours!
    ↓
Timer calls maybeCheck()
    ↓
Tries to update @Published properties
    ↓
Deallocated view observers
    ↓
EXC_BAD_ACCESS ❌
    ↓
App crashes
```

### AFTER (Fixed)

```
Close window
    ↓
pauseChecking() ← FIX: Unregister timer
    ↓
window.close()
    ↓
updateWindow = nil
    ↓
View deallocates
    ↓
Timer NOT FIRING (it's paused)
    ↓
No timer callback
    ↓
No attempt to update deallocated view
    ↓
No EXC_BAD_ACCESS ✅
    ↓
App continues normally
```

---

## Timeline Summary

| Step | Action | UpdateManager State | Timer Status |
|------|--------|---------------------|--------------|
| 1 | Show Update Window | isRunning=true | Resume (or start) |
| 2 | Check for Updates | .upToDate found | Still firing |
| 3 | Render UI | .upToDate | Still firing |
| 4 | User clicks Close | .upToDate | Still firing |
| 5 | Call pauseChecking() | isRunning=true | **PAUSED (unregistered)** |
| 5 | Close window | .upToDate | Paused |
| 6 | Window deallocates | .upToDate | Paused (safe!) |
| 7 | Time passes | .upToDate | Paused (no fires!) |
| 8 | User clicks Check again | isRunning=true | **RESUMED** |

---

## Why No More Crash

**Root Cause:** Timer firing after window deallocate
**Previous Attempt:** Fixed observer binding issue (but timer still fired!)
**Real Fix:** PAUSE the timer before closing window

When timer is paused:
- ✅ No callbacks attempt to fire
- ✅ No references to deallocated objects
- ✅ Window can close safely
- ✅ App continues normally
- ✅ When window reopens, timer resumes

**The key insight:** The timer is UpdateManager's responsibility, not the window's. Window must tell UpdateManager "I'm closing, pause your timer" before deallocating.
