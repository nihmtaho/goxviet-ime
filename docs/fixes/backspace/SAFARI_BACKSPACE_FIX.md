# Safari Backspace Fix Documentation

**Date:** 2025-12-21  
**Issue:** Safari address bar displays garbled text when deleting characters  
**Status:** ✅ FIXED

---

## Problem Description

### Symptoms
When typing Vietnamese text in Safari's address bar (e.g., "gõ tiếng việt") and then deleting characters with backspace, the display becomes corrupted with random characters appearing, especially when deleting down to "gõ ".

### Root Cause
The `getWordToRestoreOnBackspace()` function was reading text content from the Accessibility API (`AXValueAttribute`) to enable the "word restoration" feature. However, Safari's address bar (`AXTextField` role) contains:

1. **Placeholder text** - Default suggestion text shown in the address bar
2. **Autocomplete text** - Auto-suggested URLs/search results
3. **User-entered text** - Actual typed content

When the function attempted to restore a "word" before the cursor, it incorrectly included placeholder or autocomplete text, causing the garbled display.

### Technical Details

```swift
// BEFORE: Function would read ALL text from Safari address bar
var textValue: CFTypeRef?
AXUIElementCopyAttributeValue(axEl, kAXValueAttribute as CFString, &textValue)
let text = textValue as? String // ⚠️ Includes placeholder text!
```

In Safari specifically:
- Text field shows: `"gõ " + [placeholder: "search or enter website"]`
- When backspacing at position after "gõ ", the function would try to restore text that includes the placeholder
- This caused Vietnamese IME to inject incorrect replacement text

---

## Solution

### Fix Strategy
**Skip word restoration for browser address bars** to avoid placeholder text interference.

### Implementation

Modified `getWordToRestoreOnBackspace()` in `TextInjectionHelper.swift`:

```swift
// Get UI element role and bundle ID
var roleVal: CFTypeRef?
AXUIElementCopyAttributeValue(axEl, kAXRoleAttribute as CFString, &roleVal)
let role = roleVal as? String

var pid: pid_t = 0
var bundleId: String?
if AXUIElementGetPid(axEl, &pid) == .success {
    if let app = NSRunningApplication(processIdentifier: pid) {
        bundleId = app.bundleIdentifier
    }
}

// Define browser list
let browsers = [
    "com.apple.Safari", "com.apple.SafariTechnologyPreview",
    "com.google.Chrome", "com.brave.Browser", 
    "org.mozilla.firefox", "company.thebrowser.Arc",
    // ... (full list in code)
]

// Skip restoration for browser address bars
if let bundleId = bundleId, browsers.contains(bundleId), role == "AXTextField" {
    Log.info("restore: skipping browser address bar to avoid placeholder text")
    return nil  // ✅ Don't restore in browser address bars
}
```

### Why This Works

1. **Browser Detection:** Identifies Safari and other browsers via bundle ID
2. **Role Detection:** Confirms the focused element is an address bar (`AXTextField`)
3. **Early Exit:** Returns `nil` to skip word restoration entirely for these cases
4. **Selection Method:** Browsers already use the `.selection` injection method which handles autocomplete correctly

---

## App-Specific Injection Methods

The fix integrates with our existing app-specific injection strategy:

### Safari Address Bar Flow

```
User types: "g" "o" "~" → "gõ"
              ↓
[detectMethod() detects Safari + AXTextField]
              ↓
[Uses .selection injection method]
              ↓
[Text injected via Shift+Left selection]
              ↓
User presses Backspace
              ↓
[getWordToRestoreOnBackspace() detects Safari]
              ↓
[Returns nil - NO restoration attempted] ✅
              ↓
[Native backspace deletes character normally]
```

### Comparison with Other Apps

| App Type | Injection Method | Word Restoration | Reason |
|----------|-----------------|------------------|---------|
| **Safari Address Bar** | `.selection` | ❌ Disabled | Placeholder text interference |
| **VSCode Editor** | `.instant` | ✅ Enabled | Clean text buffer, no placeholders |
| **Terminal** | `.slow` | ✅ Enabled | No autocomplete interference |
| **Spotlight** | `.autocomplete` | ❌ Disabled | System handles autocomplete |

---

## Testing

### Test Cases

1. **✅ Safari Address Bar**
   - Type: "gõ tiếng việt"
   - Delete back to "gõ "
   - Expected: Clean deletion, no garbled text

2. **✅ Chrome Address Bar**
   - Same as Safari test
   - Expected: Works correctly

3. **✅ VSCode (Control Test)**
   - Type: "gõ tiếng việt "
   - Press backspace to delete space
   - Expected: Word restoration SHOULD work (re-enters "việt" for editing)

4. **✅ Safari Regular Text Fields**
   - Type in Google search box (not address bar)
   - Expected: Normal Vietnamese input works

### Verification

```bash
# 1. Build and run the app
cd platforms/macos/VietnameseIMEFast
xcodebuild

# 2. Enable debug logging
# Set Log.isEnabled = true in AppDelegate.swift

# 3. Test in Safari address bar
# Monitor: ~/Library/Logs/VietnameseIME/keyboard.log

# Expected log output:
# detect: com.apple.Safari role=AXTextField
# METHOD: sel:browser
# restore: skipping browser address bar to avoid placeholder text
```

---

## Files Modified

### Created Files
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/TextInjectionHelper.swift`
  - New file containing all text injection logic
  - Includes `TextInjector` class with app-specific methods
  - Contains fixed `getWordToRestoreOnBackspace()` function
  - Implements `detectMethod()` for app detection

### Modified Files
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/InputManager.swift`
  - Updated to use `TextInjector.shared.injectSync()`
  - Added call to `getWordToRestoreOnBackspace()` for backspace handling
  - Integrated with `detectMethod()` for app-specific injection

---

## Performance Impact

- **No performance degradation:** The fix adds a simple bundle ID check (O(1) operation)
- **Improved user experience:** Eliminates garbled text issue in Safari
- **Maintained functionality:** Word restoration still works in editors and terminals

---

## Future Improvements

### Potential Enhancements

1. **Smart Placeholder Detection**
   ```swift
   // Could detect if text contains common placeholder patterns
   if text.contains("search or enter") || text.contains("Type a URL") {
       return nil // Skip restoration
   }
   ```

2. **Cursor Position Validation**
   ```swift
   // Verify cursor is within user-entered text, not placeholder region
   if cursorPos > actualUserTextLength {
       return nil
   }
   ```

3. **AXSelectedText Attribute**
   ```swift
   // Use AXSelectedTextAttribute to get only actual selected text
   // This might avoid placeholder text entirely
   ```

---

## Related Issues

- Similar issues might occur in other apps with autocomplete/placeholder text
- Chrome, Firefox, Arc browsers also addressed with same fix
- Spotlight already uses `.autocomplete` method which avoids this issue

---

## References

- Based on reference implementation from `example-project/gonhanh.org-main/`
- Accessibility API documentation: https://developer.apple.com/documentation/applicationservices/axuielement
- Safari bundle IDs: `com.apple.Safari`, `com.apple.SafariTechnologyPreview`

---

## Checklist

- [x] Root cause identified
- [x] Fix implemented in `TextInjectionHelper.swift`
- [x] InputManager updated to use new injection system
- [x] Tested in Safari address bar
- [x] Tested in Chrome address bar
- [x] Verified word restoration still works in VSCode
- [x] Documentation created
- [x] No reference to "GoNhanh" or example project branding

---

**Contributors:** Vietnamese IME Team  
**Last Updated:** 2025-12-21