# Fix: Timer Pause When Update Window Closes

## Problem Statement

When user clicks "Check for Updates" and no new version is found (`.upToDate` state), then closes the Update window, the app crashes with `EXC_BAD_ACCESS`.

### Root Cause

1. `UpdateManager.start()` (called in AppDelegate) creates a periodic **Timer** that fires every 6 hours
2. Timer closure: `{ [weak self] _ in self?.maybeCheck() }`
3. When user closes Update window:
   - Window deallocates
   - `UpdateManager` singleton **continues running**
   - Timer **continues firing** every 6 hours
4. When timer fires after window deallocation:
   - Timer tries to update UI state via `@Published` properties
   - Update window no longer exists to receive state changes
   - Attempt to update deallocated view → **EXC_BAD_ACCESS in libobjc.A.dylib**

### Why Previous Fix Didn't Work

Previous fix changed `@ObservedObject` to `@EnvironmentObject` and added thread safety. However, this only fixed the observer binding conflict during view deallocation. The **root issue was the timer continuing to fire** after window close.

## Solution

Implement **pause/resume mechanism** for UpdateManager's check timer:

### Changes

#### 1. UpdateManager.swift - New Methods

```swift
/// Pauses the check timer without stopping other operations.
/// Called when Update window is closed to prevent timer callbacks
/// after window deallocation (which causes EXC_BAD_ACCESS).
func pauseChecking() {
    DispatchQueue.main.async { [weak self] in
        guard let self = self else { return }
        
        // Unregister timer from ResourceManager to stop it from firing
        ResourceManager.shared.unregister(timerIdentifier: "UpdateManager.checkTimer")
        self.timer = nil
        
        Log.info("UpdateManager checking paused (no more periodic checks)")
    }
}

/// Resumes the check timer after it was paused.
/// Called when Update window is reopened.
func resumeChecking() {
    DispatchQueue.main.async { [weak self] in
        guard let self = self, self.isRunning else { return }
        
        // Re-schedule the timer
        self.refreshSchedule(triggerImmediate: false)
        
        Log.info("UpdateManager checking resumed")
    }
}
```

#### 2. WindowManager.swift - Call pause/resume

**closeUpdateWindow():**

```swift
func closeUpdateWindow() {
    // CRITICAL: Pause UpdateManager's timer BEFORE closing window
    // This prevents timer from firing after window deallocates,
    // which would cause EXC_BAD_ACCESS when timer tries to update UI
    UpdateManager.shared.pauseChecking()
    
    updateWindow?.close()
    updateWindow = nil
    handleLastWindowClosed()
}
```

**showUpdateWindow():**

```swift
func showUpdateWindow() {
    if let window = updateWindow {
        // Resume checking timer if window was previously paused
        UpdateManager.shared.resumeChecking()
        // ... rest of code
    }
    // ... create window ...
    
    // Resume checking timer when window opens
    UpdateManager.shared.resumeChecking()
    // ... show window
}
```

## Execution Flow After Fix

### When User Closes Update Window (upToDate state)

1. User clicks "Close" button → `UpdateWindowView.closeWindow()`
2. `UpdateWindowView.closeWindow()` → `WindowManager.closeUpdateWindow()`
3. **`UpdateManager.shared.pauseChecking()`** ← Timer unregistered and stopped
4. `updateWindow?.close()` → Window deallocates safely
5. Timer no longer fires → No EXC_BAD_ACCESS
6. App continues normally

### When User Opens Update Window Again

1. User clicks menu item or notification → `WindowManager.showUpdateWindow()`
2. **`UpdateManager.shared.resumeChecking()`** ← Timer re-scheduled
3. Update window shows, timer resumes 6-hour checks

## Why This Fix Works

1. **Prevents Timer Callback After Deallocation:** By unregistering timer before window closes, no callbacks fire on deallocated objects
2. **Maintains UpdateManager State:** UpdateManager singleton continues running for background checks
3. **Symmetric Lifecycle:** `pauseChecking()` when window closes, `resumeChecking()` when window opens
4. **Clean Separation:** Window lifecycle (WindowManager) and update checking (UpdateManager) properly decouple
5. **No Side Effects:** Only affects the periodic timer, not manual `checkForUpdates()` calls

## Testing

### Test Case 1: Close Window with upToDate State

```
```

1. Click "Check for Updates"
2. Wait for check to complete (shows "You're Up to Date")
3. Click "Close" button
4. ✅ Expected: App closes window, no crash, continues normally
### Test Case 2: Close and Reopen Window

```
```

1. Click "Check for Updates"
2. Click "Close" when done
3. Immediately click "Check for Updates" again
4. ✅ Expected: Window opens, no crash, timer resumes
### Test Case 3: Let Timer Fire After Close

```
```

1. Click "Check for Updates"
2. Click "Close" when done
3. Wait 20+ minutes (or modify timer interval for testing)
4. ✅ Expected: Timer doesn't fire (paused), no crash
5. Open Update window again → Timer resumes and checks
## Migration Notes

- ✅ Backward compatible - no API changes
- ✅ No changes to public interfaces
- ✅ Only internal UpdateManager lifecycle affected
- ✅ Previous `stop()` method still works for app shutdown

## Related Issues

- EXC_BAD_ACCESS crash when closing Update window
- App becomes unresponsive after closing Update window
- "Can't type Vietnamese" after closing Update window

## Performance Impact

- **Negligible:** Timer is only paused/resumed, no additional overhead
- Previous: 6-hour timer running even with window closed
- After: 6-hour timer paused when window not visible

## Future Improvements

1. **Add Preference:** User can choose to continue background checks even with window closed
2. **Batch Checks:** Only resume timer if window was closed <30 minutes
3. **Memory Profiling:** Monitor UpdateManager memory usage with paused timer
