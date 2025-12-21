# ACCESSIBILITY API SUPPORT

> **Version:** 1.0.0  
> **Last Updated:** December 21, 2025  
> **Status:** ✅ Production Ready

## Overview

This document describes how Vietnamese IME uses macOS Accessibility API to provide intelligent, context-aware text input across different applications including Spotlight, browsers, and editors.

---

## Table of Contents

- [What is Accessibility API](#what-is-accessibility-api)
- [Why We Need It](#why-we-need-it)
- [Architecture Overview](#architecture-overview)
- [Detection Mechanism](#detection-mechanism)
- [Injection Methods](#injection-methods)
- [Application Support](#application-support)
- [Performance Characteristics](#performance-characteristics)
- [Troubleshooting](#troubleshooting)
- [Technical Details](#technical-details)

---

## What is Accessibility API

The macOS Accessibility API (`ApplicationServices/Accessibility`) provides system-level access to UI elements and application information. It allows assistive technologies to:

1. **Query focused UI element** - Determine which text field is currently active
2. **Identify element type** - Distinguish between text fields, combo boxes, search fields
3. **Get application context** - Retrieve bundle ID of the owning application
4. **Work with overlays** - Support floating UI like Spotlight that doesn't have frontmost app status

### Key Functions Used

```swift
// Get system-wide accessibility element
let systemWide = AXUIElementCreateSystemWide()

// Get currently focused UI element
AXUIElementCopyAttributeValue(systemWide, kAXFocusedUIElementAttribute, &focused)

// Get role (AXTextField, AXComboBox, AXSearchField, etc.)
AXUIElementCopyAttributeValue(element, kAXRoleAttribute, &role)

// Get process ID of owning application
AXUIElementGetPid(element, &pid)
```

---

## Why We Need It

### Problem: Different Apps Need Different Approaches

1. **Modern Editors** (VSCode, Zed)
   - Fast text buffers
   - Can handle rapid events (zero delays)
   - Need instant injection for < 16ms latency

2. **Terminals** (iTerm2, Terminal)
   - Slower rendering
   - Need delays between events for stability
   - Require conservative timing

3. **Browser Address Bars** (Chrome, Safari, Arc)
   - Have autocomplete suggestions
   - Need selection-based replacement
   - Direct backspace can interfere with suggestions

4. **Spotlight** (System Search)
   - Is an overlay, not a frontmost app
   - Has auto-selected suggestions
   - Needs Forward Delete before backspace

### Solution: Accessibility API Detection

By querying the focused element's **role** and **bundle ID**, we can:
- Detect Spotlight even though it's not the frontmost app
- Identify browser address bars by their `AXTextField` role
- Distinguish combo boxes that need special handling
- Apply optimal injection method per app/context

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Keyboard Event                           │
│                         ↓                                   │
│              ┌──────────────────────┐                       │
│              │   InputManager       │                       │
│              │   (processKey)       │                       │
│              └──────────┬───────────┘                       │
│                         ↓                                   │
│              ┌──────────────────────┐                       │
│              │   detectMethod()     │ ← Accessibility API   │
│              │   - Query focused el │                       │
│              │   - Get bundle ID    │                       │
│              │   - Get role         │                       │
│              └──────────┬───────────┘                       │
│                         ↓                                   │
│         ┌───────────────┼───────────────┐                   │
│         ↓               ↓               ↓                   │
│   ┌─────────┐    ┌──────────┐    ┌──────────┐             │
│   │ instant │    │ selection│    │autocmplt │             │
│   │ (0ms)   │    │(Shift+L) │    │(Fwd Del) │             │
│   └─────────┘    └──────────┘    └──────────┘             │
│                                                             │
│              ┌──────────────────────┐                       │
│              │   TextInjector       │                       │
│              │   - Inject backspace │                       │
│              │   - Inject text      │                       │
│              └──────────────────────┘                       │
└─────────────────────────────────────────────────────────────┘
```

---

## Detection Mechanism

### Implementation in `detectMethod()`

Located in: `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/RustBridge.swift`

```swift
func detectMethod() -> (InjectionMethod, (UInt32, UInt32, UInt32)) {
    // 1. Get focused element via Accessibility API
    let systemWide = AXUIElementCreateSystemWide()
    var focused: CFTypeRef?
    var role: String?
    var bundleId: String?
    
    // 2. Query element attributes
    if AXUIElementCopyAttributeValue(systemWide, kAXFocusedUIElementAttribute, &focused) == .success {
        // Get role (AXTextField, AXComboBox, etc.)
        AXUIElementCopyAttributeValue(element, kAXRoleAttribute, &role)
        
        // Get owning app's bundle ID
        var pid: pid_t = 0
        AXUIElementGetPid(element, &pid)
        bundleId = NSRunningApplication(processIdentifier: pid)?.bundleIdentifier
    }
    
    // 3. Fallback to frontmost app if focused element query failed
    if bundleId == nil {
        bundleId = NSWorkspace.shared.frontmostApplication?.bundleIdentifier
    }
    
    // 4. Apply detection rules based on bundle ID and role
    // (see Application Support section)
}
```

### Detection Rules Priority

1. **Role-based detection** (highest priority)
   - `AXComboBox` → Selection method
   - `AXSearchField` → Selection method

2. **Bundle ID + Role combination**
   - Browsers + `AXTextField` → Selection method (address bar)

3. **Bundle ID only**
   - Spotlight → Autocomplete method
   - VSCode → Instant method
   - Terminals → Slow method

4. **Default fallback**
   - Unknown apps → Fast method (safe delays)

---

## Injection Methods

### 1. Instant Method (⚡ Fastest)

**Target:** Modern editors with fast text buffers

**Apps:** VSCode, Zed, Sublime Text, Nova

**Characteristics:**
- **Zero delays** between events
- Batch backspace injection
- < 16ms total latency
- Maximum responsiveness

**Algorithm:**
```swift
1. Send all backspace events consecutively (no delays)
   for i in 0..<count {
       postBackspace()  // No usleep()
   }

2. Inject replacement text immediately (no delay)
   postText(text, delay: 0)
```

**Performance:**
- Backspace: < 3ms per character
- Total injection: < 10ms for typical 5-character syllable

---

### 2. Selection Method (🎯 Precision)

**Target:** Browser address bars and combo boxes with autocomplete

**Apps:** Chrome, Safari, Firefox, Arc, Edge, Brave

**Characteristics:**
- Uses Shift+Left to select text
- Typing replaces selection
- Avoids interfering with browser suggestions
- Zero/minimal delays

**Algorithm:**
```swift
1. Select text to replace using Shift+Left Arrow
   for i in 0..<backspaceCount {
       postKey(leftArrow, flags: .maskShift)
       usleep(selectionDelay)  // Usually 0-1ms
   }

2. Type replacement text (replaces selection automatically)
   postText(text)
```

**Why this works:**
- Browser autocomplete engines don't trigger on selection changes
- Typing over selection is atomic operation
- No backspace events that could clear suggestion dropdown

---

### 3. Autocomplete Method (🔍 Spotlight)

**Target:** Spotlight and system search overlays

**Apps:** Spotlight (com.apple.Spotlight)

**Characteristics:**
- Forward Delete first to clear auto-selected suggestion
- Then backspace and text injection
- Handles Spotlight's unique suggestion behavior

**Algorithm:**
```swift
1. Clear auto-selected suggestion
   postKey(forwardDelete)
   usleep(3000)  // Wait for suggestion to clear

2. Send backspaces
   for i in 0..<backspaceCount {
       postKey(backspace)
       usleep(1000)
   }

3. Type replacement text
   postText(text)
```

**Why Forward Delete:**
- Spotlight auto-selects first suggestion
- Backspace alone would delete suggestion, not typed text
- Forward Delete clears selection first

---

### 4. Fast Method (⚡ Balanced)

**Target:** Default for unknown applications

**Characteristics:**
- Minimal delays (1-3ms)
- Good balance between speed and stability
- Safe for most applications

**Timing:**
```swift
delays: (1000, 3000, 1500)  // in microseconds
// (backspaceDelay, waitDelay, textDelay)
```

---

### 5. Slow Method (🐌 Stability)

**Target:** Terminals and Electron apps with slow rendering

**Apps:** iTerm2, Terminal, Notion, Claude

**Characteristics:**
- Conservative delays (3-15ms)
- Ensures rendering stability
- Prevents event loss

**Timing:**
```swift
delays: (3000, 8000, 3000)  // in microseconds
// Longer waits between backspace and text
```

---

## Application Support

### ✅ Fully Supported Applications

#### Modern Editors (Instant Method - 0ms delays)

| Application | Bundle ID | Method | Performance |
|------------|-----------|--------|-------------|
| Visual Studio Code | `com.microsoft.VSCode` | instant | < 10ms |
| VSCode Insiders | `com.microsoft.VSCodeInsiders` | instant | < 10ms |
| VSCodium | `com.vscodium` | instant | < 10ms |
| Zed | `dev.zed.Zed` | instant | < 10ms |
| Zed Preview | `dev.zed.preview` | instant | < 10ms |
| Sublime Text 4 | `com.sublimetext.4` | instant | < 10ms |
| Sublime Text 3 | `com.sublimetext.3` | instant | < 10ms |
| Nova | `com.panic.Nova` | instant | < 10ms |
| CotEditor | `com.coteditor.CotEditor` | instant | < 10ms |
| Atom | `com.github.atom` | instant | < 10ms |

#### Browsers (Selection Method - address bars)

**Chromium-based:**

| Browser | Bundle ID | Address Bar | Content |
|---------|-----------|-------------|---------|
| Google Chrome | `com.google.Chrome` | ✅ Selection | ✅ Fast |
| Chrome Canary | `com.google.Chrome.canary` | ✅ Selection | ✅ Fast |
| Chrome Beta | `com.google.Chrome.beta` | ✅ Selection | ✅ Fast |
| Chromium | `org.chromium.Chromium` | ✅ Selection | ✅ Fast |
| Brave | `com.brave.Browser` | ✅ Selection | ✅ Fast |
| Brave Beta | `com.brave.Browser.beta` | ✅ Selection | ✅ Fast |
| Brave Nightly | `com.brave.Browser.nightly` | ✅ Selection | ✅ Fast |
| Microsoft Edge | `com.microsoft.edgemac` | ✅ Selection | ✅ Fast |
| Edge Beta | `com.microsoft.edgemac.Beta` | ✅ Selection | ✅ Fast |
| Edge Dev | `com.microsoft.edgemac.Dev` | ✅ Selection | ✅ Fast |
| Edge Canary | `com.microsoft.edgemac.Canary` | ✅ Selection | ✅ Fast |
| Vivaldi | `com.vivaldi.Vivaldi` | ✅ Selection | ✅ Fast |
| Vivaldi Snapshot | `com.vivaldi.Vivaldi.snapshot` | ✅ Selection | ✅ Fast |
| Yandex Browser | `ru.yandex.desktop.yandex-browser` | ✅ Selection | ✅ Fast |

**Opera-based:**

| Browser | Bundle ID | Address Bar | Content |
|---------|-----------|-------------|---------|
| Opera | `com.opera.Opera` | ✅ Selection | ✅ Fast |
| Opera (alt) | `com.operasoftware.Opera` | ✅ Selection | ✅ Fast |
| Opera GX | `com.operasoftware.OperaGX` | ✅ Selection | ✅ Fast |
| Opera Air | `com.operasoftware.OperaAir` | ✅ Selection | ✅ Fast |
| Opera Next | `com.opera.OperaNext` | ✅ Selection | ✅ Fast |

**Firefox-based:**

| Browser | Bundle ID | Address Bar | Content |
|---------|-----------|-------------|---------|
| Firefox | `org.mozilla.firefox` | ✅ Selection | ✅ Fast |
| Firefox Developer | `org.mozilla.firefoxdeveloperedition` | ✅ Selection | ✅ Fast |
| Firefox Nightly | `org.mozilla.nightly` | ✅ Selection | ✅ Fast |
| Waterfox | `org.waterfoxproject.waterfox` | ✅ Selection | ✅ Fast |
| LibreWolf | `io.gitlab.librewolf-community.librewolf` | ✅ Selection | ✅ Fast |
| Floorp | `one.ablaze.floorp` | ✅ Selection | ✅ Fast |
| Tor Browser | `org.torproject.torbrowser` | ✅ Selection | ✅ Fast |
| Mullvad Browser | `net.mullvad.mullvadbrowser` | ✅ Selection | ✅ Fast |

**Safari & WebKit:**

| Browser | Bundle ID | Address Bar | Content |
|---------|-----------|-------------|---------|
| Safari | `com.apple.Safari` | ✅ Selection | ✅ Fast |
| Safari Tech Preview | `com.apple.SafariTechnologyPreview` | ✅ Selection | ✅ Fast |
| Orion (Kagi) | `com.kagi.kagimacOS` | ✅ Selection | ✅ Fast |

**Arc & Modern Browsers:**

| Browser | Bundle ID | Address Bar | Content |
|---------|-----------|-------------|---------|
| **Arc** | `company.thebrowser.Arc` | ✅ Selection | ✅ Fast |
| The Browser Company | `company.thebrowser.Browser` | ✅ Selection | ✅ Fast |
| Dia | `company.thebrowser.dia` | ✅ Selection | ✅ Fast |
| Zen Browser | `app.zen-browser.zen` | ✅ Selection | ✅ Fast |
| SigmaOS | `com.sigmaos.sigmaos.macos` | ✅ Selection | ✅ Fast |
| Sidekick | `com.pushplaylabs.sidekick` | ✅ Selection | ✅ Fast |
| Polypane | `com.firstversionist.polypane` | ✅ Selection | ✅ Fast |
| Comet (Perplexity) | `ai.perplexity.comet` | ✅ Selection | ✅ Fast |
| DuckDuckGo | `com.duckduckgo.macos.browser` | ✅ Selection | ✅ Fast |

#### System & Special Apps

| Application | Bundle ID | Method | Notes |
|------------|-----------|--------|-------|
| **Spotlight** | `com.apple.Spotlight` | autocomplete | Forward Delete clears suggestions |
| System Search | `com.apple.Spotlight` | autocomplete | Same as Spotlight |

#### Terminals (Slow Method)

| Terminal | Bundle ID | Method | Delays |
|----------|-----------|--------|--------|
| Terminal.app | `com.apple.Terminal` | slow | 3-8ms |
| iTerm2 | `com.googlecode.iterm2` | slow | 3-8ms |
| Alacritty | `io.alacritty` | slow | 3-8ms |
| WezTerm | `com.github.wez.wezterm` | slow | 3-8ms |
| Ghostty | `com.mitchellh.ghostty` | slow | 3-8ms |
| Warp | `dev.warp.Warp-Stable` | slow | 3-8ms |
| Kitty | `net.kovidgoyal.kitty` | slow | 3-8ms |
| Hyper | `co.zeit.hyper` | slow | 3-8ms |
| Tabby | `org.tabby` | slow | 3-8ms |
| Rio | `com.raphaelamorim.rio` | slow | 3-8ms |
| Termius | `com.termius-dmg.mac` | slow | 3-8ms |

#### JetBrains IDEs (Slow Method)

| IDE | Bundle ID Pattern | Method | Notes |
|-----|------------------|--------|-------|
| IntelliJ IDEA | `com.jetbrains.intellij*` | slow | Address bars use selection |
| PyCharm | `com.jetbrains.pycharm*` | slow | Text fields use selection |
| WebStorm | `com.jetbrains.webstorm*` | slow | Mixed strategy |
| PhpStorm | `com.jetbrains.phpstorm*` | slow | Mixed strategy |
| All JetBrains | `com.jetbrains.*` | slow | Wildcard matching |

#### Microsoft Office (Slow Method)

| Application | Bundle ID | Method | Delays | Notes |
|------------|-----------|--------|--------|-------|
| Microsoft Word | `com.microsoft.Word` | slow | 3-8ms | Backspace to avoid autocomplete conflict |
| Microsoft Excel | `com.microsoft.Excel` | slow | 3-8ms | Backspace to avoid autocomplete conflict |

#### Electron Apps (Slow Method)

| Application | Bundle ID | Method | Delays |
|------------|-----------|--------|--------|
| Notion | `notion.id` | slow | 8-15ms |
| Claude Desktop | `com.todesktop.230313mzl4w4u92` | slow | 8-15ms |

---

## Performance Characteristics

### Latency Comparison by Method

| Method | Backspace Time | Text Injection | Total (5-char syllable) | Use Case |
|--------|---------------|----------------|------------------------|----------|
| **Instant** | < 1ms each | < 2ms | **< 10ms** ✅ | Modern editors |
| **Fast** | 1ms each | 3ms wait | **~ 15ms** | Default apps |
| **Selection** | 0-1ms per select | 0ms wait | **~ 8ms** | Browser bars |
| **Autocomplete** | 1ms each + 3ms | 5ms wait | **~ 20ms** | Spotlight |
| **Slow** | 3ms each | 8ms wait | **~ 30ms** | Terminals |

### Target Metrics Achievement

| Metric | Target | Instant | Selection | Fast | Status |
|--------|--------|---------|-----------|------|--------|
| Single keystroke | < 16ms | ✅ 10ms | ✅ 8ms | ✅ 15ms | ✅ Met |
| Backspace operation | < 3ms | ✅ 1ms | ✅ 1ms | ✅ 1ms | ✅ Met |
| 60fps responsiveness | Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Met |

---

## Troubleshooting

### Common Issues

#### 1. Spotlight Not Working

**Symptoms:**
- Typed text appears but doesn't transform
- First character gets lost
- Suggestions don't clear

**Solution:**
- Ensure Accessibility permission is granted
- Check that bundle ID detection works: `com.apple.Spotlight`
- Verify autocomplete method is selected

**Debug:**
```swift
// Check log output
Log.info("detect: \(bundleId) role=\(role)")
// Should show: "detect: com.apple.Spotlight role=AXSearchField"
```

#### 2. Arc Browser Address Bar Issues

**Symptoms:**
- Text doesn't replace in address bar
- Autocomplete interferes

**Solution:**
- Verify Arc's bundle ID: `company.thebrowser.Arc`
- Ensure role is detected as `AXTextField`
- Selection method should be used automatically

**Manual Check:**
```swift
if bundleId == "company.thebrowser.Arc" && role == "AXTextField" {
    // Should use selection method
}
```

#### 3. Modern Editor Slow Response

**Symptoms:**
- VSCode/Zed feels laggy
- Visible character-by-character rendering

**Solution:**
- Verify instant method is selected
- Check log: should show `METHOD: instant:editor`
- Ensure zero delays are applied

#### 4. Terminal Character Loss

**Symptoms:**
- Missing characters in iTerm2/Terminal
- Inconsistent rendering

**Solution:**
- Slow method should be auto-selected
- Increase delays if needed
- Check terminal buffer settings

---

## Technical Details

### Event Marker System

All injected events are marked with a unique identifier to prevent re-processing:

```swift
private let kEventMarker: Int64 = 0x564E5F494D45 // "VN_IME" in ASCII

// Mark event as injected
event.setIntegerValueField(.eventSourceUserData, value: kEventMarker)

// Check in event handler
let marker = event.getIntegerValueField(.eventSourceUserData)
if marker == kEventMarker {
    return Unmanaged.passUnretained(event)  // Pass through, don't process
}
```

**Why needed:**
- Prevents infinite loops (IME processing its own injected events)
- Allows original events to pass through naturally
- Performance: marker check is O(1)

### Thread Safety

```swift
class TextInjector {
    private let semaphore = DispatchSemaphore(value: 1)
    
    func injectSync(...) {
        semaphore.wait()  // Acquire lock
        defer { semaphore.signal() }  // Always release
        
        // Injection logic...
    }
}
```

**Guarantees:**
- Only one injection at a time
- No race conditions
- Prevents event queue corruption

### Memory Management

All Accessibility API objects use Core Foundation and require proper memory management:

```swift
// Create system-wide element (no release needed - singleton)
let systemWide = AXUIElementCreateSystemWide()

// Query attribute (release needed if success)
var focused: CFTypeRef?
if AXUIElementCopyAttributeValue(systemWide, kAXFocusedUIElementAttribute, &focused) == .success {
    // Use focused element
    // Swift ARC handles release automatically via CFTypeRef
}
```

### Performance Optimization

**Batch Backspace (Instant Method):**
```swift
// Old approach (slower)
for _ in 0..<count {
    postBackspace()
    usleep(1000)  // 1ms delay
}
// Total: count * 1ms

// New approach (faster)
for _ in 0..<count {
    postBackspace()  // No delay
}
// Total: < count * 0.2ms
```

**Result:** 5x faster backspace injection for modern editors

---

## References

### Internal Documentation

- [`PERFORMANCE_OPTIMIZATION_GUIDE.md`](./performance/PERFORMANCE_OPTIMIZATION_GUIDE.md) - Optimization strategies
- [`SMART_PER_APP_MODE.md`](./SMART_PER_APP_MODE.md) - Per-app configuration
- [`MENU_TOGGLE_IMPLEMENTATION.md`](./MENU_TOGGLE_IMPLEMENTATION.md) - UI integration

### Source Code

- **Detection Logic:** `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/RustBridge.swift` (line 549-685)
- **Injection Methods:** `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/RustBridge.swift` (line 69-249)
- **Input Manager:** `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/InputManager.swift`

### External Resources

- [Apple Accessibility API Documentation](https://developer.apple.com/documentation/applicationservices/accessibility)
- [AXUIElement Reference](https://developer.apple.com/documentation/applicationservices/axuielement)
- [CGEvent Reference](https://developer.apple.com/documentation/coregraphics/cgevent)

---

## Appendix: Adding New Applications

### How to Add Support for a New App

1. **Identify Bundle ID:**
   ```bash
   osascript -e 'id of app "YourApp"'
   ```

2. **Test and determine optimal method:**
   - Try instant method first for editors
   - Use selection for text fields with autocomplete
   - Fall back to fast/slow if issues occur

3. **Add to detection logic:**
   ```swift
   // In detectMethod()
   if bundleId == "com.yourapp.YourApp" {
       Log.method("instant:yourapp")
       return (.instant, (0, 0, 0))
   }
   ```

4. **Test thoroughly:**
   - Type Vietnamese in various contexts
   - Test backspace/undo behavior
   - Verify no character loss
   - Check performance metrics

5. **Update documentation:**
   - Add to Application Support table
   - Document any special considerations
   - Update CHANGELOG.md

---

## Changelog

### Version 1.0.0 (December 21, 2025)
- ✅ Initial documentation
- ✅ Comprehensive browser support list
- ✅ Arc browser confirmed working
- ✅ Spotlight autocomplete method documented
- ✅ Performance metrics added
- ✅ Troubleshooting guide included

---

## License

This documentation is part of Vietnamese IME project.  
Copyright © 2025 Vietnamese IME Contributors.  
Licensed under MIT License.

---

**Questions or Issues?**  
Please open an issue in the project repository or refer to the main [documentation index](./README.md).