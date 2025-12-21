# MENUBAR APPEARANCE FIX - Deprecated API Resolution

**Date:** 2025-12-20  
**Component:** MenuToggleView.swift  
**Platform:** macOS  
**Issue:** Compiler warning for deprecated `NSAppearance.current` API

---

## Problem

When building the Vietnamese IME macOS app, we encountered deprecation warnings:

```
'current' was deprecated in macOS 12.0: Use -performAsCurrentDrawingAppearance: 
to temporarily set the drawing appearance, or +currentDrawingAppearance to access 
the currently drawing appearance.
```

The warning appeared in 3 locations within `MenuToggleView.swift` where we were using:

```swift
let isDarkMode = NSAppearance.current.bestMatch(from: [.darkAqua, .aqua]) == .darkAqua
```

---

## Root Cause

Apple deprecated the static property `NSAppearance.current` in macOS 12.0 in favor of:
- **For reading:** `NSAppearance.currentDrawing()` - Returns the appearance being used for drawing
- **For setting:** `NSView.performAsCurrentDrawingAppearance(_:)` - Temporarily sets the drawing appearance

In our case, we were **reading** the current appearance to determine if dark mode was active, so we needed to migrate to `currentDrawing()`.

---

## Solution

### Changed Code

Replaced all instances of `NSAppearance.current` with `NSAppearance.currentDrawing()`:

**Before (Deprecated):**
```swift
let isDarkMode = NSAppearance.current.bestMatch(from: [.darkAqua, .aqua]) == .darkAqua
```

**After (Modern API):**
```swift
let isDarkMode = NSAppearance.currentDrawing().bestMatch(from: [.darkAqua, .aqua]) == .darkAqua
```

### Affected Functions

1. **`createToggleHostingView()`** (Line 131)
   - Sets initial appearance when creating the SwiftUI hosting view

2. **`refreshHostingView()`** (Line 174)
   - Updates appearance when toggle state changes

3. **`forceActiveAppearance()`** (Line 189)
   - Maintains appearance when app focus changes

---

## Technical Details

### Why `currentDrawing()` is Better

| Aspect | `.current` (Old) | `.currentDrawing()` (New) |
|--------|------------------|---------------------------|
| **Thread Safety** | Limited | Better thread-safe access |
| **Drawing Context** | Global state | Current drawing context |
| **Performance** | Potential overhead | Optimized for drawing operations |
| **API Design** | Static property | Function call (clearer intent) |

### Context Awareness

`currentDrawing()` is specifically designed for views that need to determine their drawing appearance during render cycles. This is exactly our use case in the menu bar toggle view.

---

## Testing

### Verification Steps

1. **Build Verification:**
   ```bash
   cd platforms/macos/VietnameseIMEFast
   xcodebuild -project VietnameseIMEFast.xcodeproj \
              -scheme VietnameseIMEFast \
              -configuration Debug \
              clean build
   ```
   - ✅ Result: `** BUILD SUCCEEDED **`
   - ✅ No deprecation warnings for `MenuToggleView.swift`

2. **Visual Testing:**
   - Toggle continues to maintain correct appearance in light mode
   - Toggle continues to maintain correct appearance in dark mode
   - No dimming when app loses focus (existing fix still works)
   - Smooth animation during state changes (existing feature preserved)

3. **Appearance Switching:**
   - System Preferences → Appearance → Switch between Light/Dark
   - Menu bar toggle adapts correctly
   - Colors remain vibrant in both modes

---

## Related Context

### Previous Fixes (Still Active)

This API update does **not** affect the following existing solutions:

1. **Focus-State Dimming Fix**
   - Still uses `.brightness(0)` in SwiftUI
   - Still uses `forceActiveAppearance()` with Combine observers
   - Document: `MENUBAR_TOGGLE_FOCUS_FIX.md`

2. **Animation Feature**
   - Still uses `.animation(.easeInOut(duration: 0.25), value: toggleState.isOn)`
   - Document: `MENUBAR_TOGGLE_CHANGELOG.md`

3. **SwiftUI + NSHostingView Pattern**
   - Unchanged architecture
   - Document: `MENUBAR_TOGGLE_IMPLEMENTATION.md`

---

## Best Practices for Future Development

### When to Use Each API

**Use `NSAppearance.currentDrawing()`:**
- Inside `draw(_:)`, `viewWillDraw()`, `viewDidMoveToSuperview()`
- When determining appearance for rendering decisions
- When querying appearance in layout/drawing code

**Use `NSView.performAsCurrentDrawingAppearance(_:)`:**
- When you need to temporarily set appearance for a code block
- When drawing with a specific appearance context

### Migration Checklist

When updating appearance-related code in the future:

- [ ] Search for `NSAppearance.current` in codebase
- [ ] Replace with `NSAppearance.currentDrawing()` for reading
- [ ] Build and verify no deprecation warnings
- [ ] Test appearance changes (light/dark mode switching)
- [ ] Test focus state changes (app active/inactive)
- [ ] Update documentation

---

## References

- **Apple Documentation:** [NSAppearance - currentDrawing()](https://developer.apple.com/documentation/appkit/nsappearance/3674777-currentdrawing)
- **Project File:** `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/MenuToggleView.swift`
- **Related Docs:** 
  - `MENUBAR_TOGGLE_FOCUS_FIX.md` (Focus-state dimming solution)
  - `MENUBAR_TOGGLE_IMPLEMENTATION.md` (Overall architecture)
  - `MENUBAR_TOGGLE_CHANGELOG.md` (Version history)

---

## Status

✅ **RESOLVED** - All deprecation warnings fixed  
✅ **TESTED** - Build successful, no behavioral changes  
✅ **DEPLOYED** - Ready for production use  

**Last Updated:** 2025-12-20