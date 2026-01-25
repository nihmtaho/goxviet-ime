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

Implement **pause/resume mechanism** for UpdateManager's check timer.

### Changes Made

#### 1. UpdateManager.swift - New Methods

Added two new methods:

- `pauseChecking()` - Stops the 6-hour periodic check timer (called when window closes)
- `resumeChecking()` - Restarts the check timer (called when window opens)

These methods safely manage the timer lifecycle without affecting other UpdateManager operations.

#### 2. WindowManager.swift - Lifecycle Integration

**closeUpdateWindow():** Now calls `UpdateManager.shared.pauseChecking()` BEFORE closing the window

**showUpdateWindow():** Now calls `UpdateManager.shared.resumeChecking()` when window opens (or reopens)

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

## Testing Checklist

Test Case 1: Close Window with upToDate State

- Click "Check for Updates"
- Wait for check to complete (shows "You're Up to Date")
- Click "Close" button
- ✅ Expected: App closes window, no crash, continues normally

Test Case 2: Close and Reopen Window

- Click "Check for Updates"
- Click "Close" when done
- Immediately click "Check for Updates" again
- ✅ Expected: Window opens, no crash, timer resumes

Test Case 3: Let Timer Fire After Close

- Click "Check for Updates"
- Click "Close" when done
- Wait 20+ minutes (or modify timer interval for testing)
- ✅ Expected: Timer doesn't fire (paused), no crash
- Open Update window again → Timer resumes and checks

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
