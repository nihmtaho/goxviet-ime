//
//  TextInjectionHelper.swift
//  GoxViet
//
//  Text injection utility with app-specific optimization strategies
//  Based on reference implementation
//

import Cocoa
import ApplicationServices

// MARK: - Injection Method

enum InjectionMethod {
    case instant        // Modern editors: zero delays for maximum speed (VSCode, Zed, Sublime)
    case fast           // Default: backspace + text with minimal delays
    case slow           // Terminals/Electron: backspace + text with higher delays
    case selection      // Browser address bars: Shift+Left select + type replacement
    case autocomplete   // Spotlight: Forward Delete + backspace + text via proxy
    case axDirect       // Browser address bars: AX API direct text manipulation (bypasses autocomplete)
}

// MARK: - Key Codes

enum KeyCode {
    static let backspace: CGKeyCode = 51
    static let forwardDelete: CGKeyCode = 117
    static let leftArrow: CGKeyCode = 123
}

// MARK: - Text Injector

class TextInjector {
    static let shared = TextInjector()
    
    // Event marker to identify our own injected events
    private let eventMarker: Int64 = 0x564E5F494D45 // "VN_IME"
    
    private let semaphore = DispatchSemaphore(value: 1)
    
    private init() {}
    
    // MARK: - Public API
    
    /// Synchronous text injection with app-specific optimization
    func injectSync(bs: Int, text: String, method: InjectionMethod, delays: (UInt32, UInt32, UInt32), proxy: CGEventTapProxy) {
        semaphore.wait()
        defer { semaphore.signal() }
        
        switch method {
        case .instant:
            injectViaInstant(bs: bs, text: text)
        case .fast, .slow:
            injectViaBackspace(bs: bs, text: text, delays: delays)
        case .selection:
            injectViaSelection(bs: bs, text: text, delays: delays)
        case .autocomplete:
            injectViaAutocomplete(bs: bs, text: text, proxy: proxy)
        case .axDirect:
            injectViaAXWithFallback(bs: bs, text: text, proxy: proxy)
        }
    }
    
    // MARK: - Injection Methods
    
    /// Instant injection: zero delays for modern editors with fast text buffers
    private func injectViaInstant(bs: Int, text: String) {
        guard let src = CGEventSource(stateID: .privateState) else { return }
        
        // Batch backspace events - no delays between them (faster than loop)
        postBackspaces(bs, source: src)
        
        // Type replacement text immediately - no delay
        postText(text, source: src, delay: 0)
        Log.send("instant", bs, text)
    }
    
    /// Standard backspace injection: delete N chars, then type replacement
    private func injectViaBackspace(bs: Int, text: String, delays: (UInt32, UInt32, UInt32)) {
        guard let src = CGEventSource(stateID: .privateState) else { return }
        
        // Optimize: use batch backspace when no delay needed between keystrokes
        if delays.0 == 0 {
            postBackspaces(bs, source: src)
        } else {
            for _ in 0..<bs {
                postKey(KeyCode.backspace, source: src)
                usleep(delays.0)
            }
        }
        
        if bs > 0 { usleep(delays.1) }
        
        postText(text, source: src, delay: delays.2)
        Log.send("bs", bs, text)
    }
    
    /// Selection injection: Shift+Left to select, then type replacement (for browser address bars)
    private func injectViaSelection(bs: Int, text: String, delays: (UInt32, UInt32, UInt32)) {
        guard let src = CGEventSource(stateID: .privateState) else { return }
        
        let selDelay = delays.0 > 0 ? delays.0 : 1000
        let waitDelay = delays.1 > 0 ? delays.1 : 3000
        let textDelay = delays.2 > 0 ? delays.2 : 2000
        
        for _ in 0..<bs {
            postKey(KeyCode.leftArrow, source: src, flags: .maskShift)
            usleep(selDelay)
        }
        if bs > 0 { usleep(waitDelay) }
        
        postText(text, source: src, delay: textDelay)
        Log.send("sel", bs, text)
    }
    
    /// Autocomplete injection: Forward Delete to clear suggestion, then backspace + text via proxy
    /// Used for Spotlight where autocomplete auto-selects suggestion text after cursor
    private func injectViaAutocomplete(bs: Int, text: String, proxy: CGEventTapProxy) {
        guard let src = CGEventSource(stateID: .privateState) else { return }
        
        // Forward Delete clears auto-selected suggestion
        postKey(KeyCode.forwardDelete, source: src, proxy: proxy)
        usleep(3000)
        
        // Backspaces remove typed characters
        for _ in 0..<bs {
            postKey(KeyCode.backspace, source: src, proxy: proxy)
            usleep(1000)
        }
        if bs > 0 { usleep(5000) }
        
        // Type replacement text
        postText(text, source: src, proxy: proxy)
        Log.send("auto", bs, text)
    }
    
    // MARK: - AX API Direct Injection
    
    /// AX API direct text manipulation for browser address bars
    /// Bypasses autocomplete behavior by directly setting text field value via Accessibility API
    /// Returns true if successful, false if caller should fallback to synthetic events
    private func injectViaAX(bs: Int, text: String) -> Bool {
        // Get focused element
        let systemWide = AXUIElementCreateSystemWide()
        var focusedRef: CFTypeRef?
        guard AXUIElementCopyAttributeValue(systemWide, kAXFocusedUIElementAttribute as CFString, &focusedRef) == .success,
              let ref = focusedRef else {
            Log.info("AX: no focus")
            return false
        }
        let axEl = ref as! AXUIElement
        
        // Read current text value
        var valueRef: CFTypeRef?
        guard AXUIElementCopyAttributeValue(axEl, kAXValueAttribute as CFString, &valueRef) == .success else {
            Log.info("AX: no value")
            return false
        }
        let fullText = (valueRef as? String) ?? ""
        
        // Read cursor position and selection
        var rangeRef: CFTypeRef?
        guard AXUIElementCopyAttributeValue(axEl, kAXSelectedTextRangeAttribute as CFString, &rangeRef) == .success,
              let axRange = rangeRef else {
            Log.info("AX: no range")
            return false
        }
        var range = CFRange()
        guard AXValueGetValue(axRange as! AXValue, .cfRange, &range), range.location >= 0 else {
            Log.info("AX: bad range")
            return false
        }
        
        let cursor = range.location
        let selection = range.length
        
        // Handle autocomplete: when selection > 0, text after cursor is autocomplete suggestion
        // Example: "a|rc://chrome-urls" where "|" is cursor, "rc://..." is selected suggestion
        let userText = (selection > 0 && cursor <= fullText.count)
            ? String(fullText.prefix(cursor))
            : fullText
        
        // Calculate replacement: delete `bs` chars before cursor, insert `text`
        let deleteStart = max(0, cursor - bs)
        let prefix = String(userText.prefix(deleteStart))
        let suffix = String(userText.dropFirst(cursor))
        let newText = (prefix + text + suffix).precomposedStringWithCanonicalMapping
        
        // Write new value
        guard AXUIElementSetAttributeValue(axEl, kAXValueAttribute as CFString, newText as CFTypeRef) == .success else {
            Log.info("AX: write failed")
            return false
        }
        
        // Update cursor to end of inserted text
        var newCursor = CFRange(location: deleteStart + text.count, length: 0)
        if let newRange = AXValueCreate(.cfRange, &newCursor) {
            AXUIElementSetAttributeValue(axEl, kAXSelectedTextRangeAttribute as CFString, newRange)
        }
        
        Log.send("ax", bs, text)
        return true
    }
    
    /// Try AX injection with retries, fallback to selection method if all fail
    /// Browser address bars can be busy with autocomplete, causing AX API to fail temporarily
    private func injectViaAXWithFallback(bs: Int, text: String, proxy: CGEventTapProxy) {
        // Try AX API up to 3 times (address bar might be busy with autocomplete)
        for attempt in 0..<3 {
            if attempt > 0 {
                usleep(5000) // Small delay before retry
            }
            if injectViaAX(bs: bs, text: text) {
                return // Success
            }
        }
        
        // Fallback to selection method
        Log.info("AX: fallback to selection")
        injectViaSelection(bs: bs, text: text, delays: (1000, 3000, 2000))
    }
    
    // MARK: - Helpers
    
    /// Post multiple backspace events in batch (faster than loop with delays)
    /// Reduces event loop overhead by posting events consecutively
    private func postBackspaces(_ count: Int, source: CGEventSource, proxy: CGEventTapProxy? = nil) {
        guard count > 0 else { return }
        
        for _ in 0..<count {
            guard let dn = CGEvent(keyboardEventSource: source, virtualKey: KeyCode.backspace, keyDown: true),
                  let up = CGEvent(keyboardEventSource: source, virtualKey: KeyCode.backspace, keyDown: false) else { continue }
            dn.setIntegerValueField(.eventSourceUserData, value: eventMarker)
            up.setIntegerValueField(.eventSourceUserData, value: eventMarker)
            
            if let proxy = proxy {
                dn.tapPostEvent(proxy)
                up.tapPostEvent(proxy)
            } else {
                dn.post(tap: .cgSessionEventTap)
                up.post(tap: .cgSessionEventTap)
            }
        }
    }
    
    /// Post a single key press event
    func postKey(_ keyCode: CGKeyCode, source: CGEventSource, flags: CGEventFlags = [], proxy: CGEventTapProxy? = nil) {
        guard let dn = CGEvent(keyboardEventSource: source, virtualKey: keyCode, keyDown: true),
              let up = CGEvent(keyboardEventSource: source, virtualKey: keyCode, keyDown: false) else { return }
        dn.setIntegerValueField(.eventSourceUserData, value: eventMarker)
        up.setIntegerValueField(.eventSourceUserData, value: eventMarker)
        if !flags.isEmpty { dn.flags = flags; up.flags = flags }
        
        if let proxy = proxy {
            dn.tapPostEvent(proxy)
            up.tapPostEvent(proxy)
        } else {
            dn.post(tap: .cgSessionEventTap)
            up.post(tap: .cgSessionEventTap)
        }
    }
    
    /// Post text in chunks (CGEvent has 20-char limit)
    private func postText(_ text: String, source: CGEventSource, delay: UInt32 = 0, proxy: CGEventTapProxy? = nil) {
        let utf16 = Array(text.utf16)
        var offset = 0
        
        while offset < utf16.count {
            let end = min(offset + 20, utf16.count)
            let chunk = Array(utf16[offset..<end])
            
            guard let dn = CGEvent(keyboardEventSource: source, virtualKey: 0, keyDown: true),
                  let up = CGEvent(keyboardEventSource: source, virtualKey: 0, keyDown: false) else { break }
            dn.setIntegerValueField(.eventSourceUserData, value: eventMarker)
            up.setIntegerValueField(.eventSourceUserData, value: eventMarker)
            dn.keyboardSetUnicodeString(stringLength: chunk.count, unicodeString: chunk)
            up.keyboardSetUnicodeString(stringLength: chunk.count, unicodeString: chunk)
            
            if let proxy = proxy {
                dn.tapPostEvent(proxy)
                up.tapPostEvent(proxy)
            } else {
                dn.post(tap: .cgSessionEventTap)
                up.post(tap: .cgSessionEventTap)
            }
            if delay > 0 { usleep(delay) }
            offset = end
        }
    }
}

// MARK: - App Detection

/// Detect optimal injection method based on focused app and UI element
func detectMethod() -> (InjectionMethod, (UInt32, UInt32, UInt32)) {
    // Get focused element and its owning app (works for overlays like Spotlight)
    let systemWide = AXUIElementCreateSystemWide()
    var focused: CFTypeRef?
    var role: String?
    var bundleId: String?
    
    if AXUIElementCopyAttributeValue(systemWide, kAXFocusedUIElementAttribute as CFString, &focused) == .success,
       let el = focused {
        let axEl = el as! AXUIElement
        
        // Get role
        var roleVal: CFTypeRef?
        AXUIElementCopyAttributeValue(axEl, kAXRoleAttribute as CFString, &roleVal)
        role = roleVal as? String
        
        // Get owning app's bundle ID (works for Spotlight overlay)
        var pid: pid_t = 0
        if AXUIElementGetPid(axEl, &pid) == .success {
            if let app = NSRunningApplication(processIdentifier: pid) {
                bundleId = app.bundleIdentifier
            }
        }
    }
    
    // Fallback to frontmost app if we couldn't get bundle from focused element
    if bundleId == nil {
        bundleId = NSWorkspace.shared.frontmostApplication?.bundleIdentifier
    }
    
    guard let bundleId = bundleId else { return (.fast, (200, 800, 500)) }
    
    // Debug: log bundle and role for investigation
    Log.info("detect: \(bundleId) role=\(role ?? "nil")")
    
    // Selection method for autocomplete UI elements (ComboBox, SearchField)
    if role == "AXComboBox" { Log.method("sel:combo"); return (.selection, (0, 0, 0)) }
    if role == "AXSearchField" { Log.method("sel:search"); return (.selection, (0, 0, 0)) }
    
    // Spotlight - use autocomplete method with Forward Delete to clear suggestions
    if bundleId == "com.apple.Spotlight" { Log.method("auto:spotlight"); return (.autocomplete, (0, 0, 0)) }
    
    // Chromium-based browser address bars - use AX API direct manipulation
    // Backspace method fails due to Chromium's autocomplete behavior (issue #26)
    // AX API bypasses autocomplete by directly setting text field value
    let chromiumBrowsers = [
        "com.google.Chrome", "com.google.Chrome.canary", "com.google.Chrome.beta",
        "org.chromium.Chromium", "com.brave.Browser", "com.brave.Browser.beta",
        "com.brave.Browser.nightly", "com.microsoft.edgemac", "com.microsoft.edgemac.Beta",
        "com.microsoft.edgemac.Dev", "com.microsoft.edgemac.Canary", "com.vivaldi.Vivaldi",
        "com.vivaldi.Vivaldi.snapshot", "ru.yandex.desktop.yandex-browser",
        // Opera (Chromium-based)
        "com.opera.Opera", "com.operasoftware.Opera", "com.operasoftware.OperaGX",
        "com.operasoftware.OperaAir", "com.opera.OperaNext",
        // Arc & Others (Chromium-based)
        "company.thebrowser.Browser", "company.thebrowser.Arc", "company.thebrowser.dia",
        "com.sigmaos.sigmaos.macos", "com.pushplaylabs.sidekick",
        "com.firstversionist.polypane", "ai.perplexity.comet", "com.duckduckgo.macos.browser"
    ]
    if chromiumBrowsers.contains(bundleId) && role == "AXTextField" {
        Log.method("ax:chromium")
        return (.axDirect, (0, 0, 0))
    }
    
    // Firefox-based browsers - use AX API for address bar
    let firefoxBrowsers = [
        "org.mozilla.firefox", "org.mozilla.firefoxdeveloperedition", "org.mozilla.nightly",
        "org.waterfoxproject.waterfox", "io.gitlab.librewolf-community.librewolf",
        "one.ablaze.floorp", "org.torproject.torbrowser", "net.mullvad.mullvadbrowser",
        "app.zen-browser.zen"
    ]
    if firefoxBrowsers.contains(bundleId) && (role == "AXTextField" || role == "AXWindow") {
        Log.method("ax:firefox")
        return (.axDirect, (0, 0, 0))
    }
    
    // Safari - use selection method (different behavior from Chromium)
    let safariBrowsers = [
        "com.apple.Safari", "com.apple.SafariTechnologyPreview",
        "com.kagi.kagimacOS"
    ]
    if safariBrowsers.contains(bundleId) && role == "AXTextField" {
        Log.method("sel:safari")
        return (.selection, (0, 0, 0))
    }
    if role == "AXTextField" && bundleId.hasPrefix("com.jetbrains") { Log.method("sel:jb"); return (.selection, (0, 0, 0)) }
    
    // Microsoft Office apps - use backspace method instead of selection
    // Selection method conflicts with Office's autocomplete/suggestion features
    if bundleId == "com.microsoft.Excel" { Log.method("slow:excel"); return (.slow, (3000, 8000, 3000)) }
    if bundleId == "com.microsoft.Word" { Log.method("slow:word"); return (.slow, (3000, 8000, 3000)) }
    
    // Electron apps - higher delays for reliable text replacement
    if bundleId == "com.todesktop.230313mzl4w4u92" { Log.method("slow:claude"); return (.slow, (8000, 15000, 8000)) }
    if bundleId == "notion.id" { Log.method("slow:notion"); return (.slow, (8000, 15000, 8000)) }
    
    // Modern editors - instant method with zero delays for maximum speed
    let modernEditors = [
        "com.microsoft.VSCode", "dev.zed.Zed", "com.sublimetext.4", "com.sublimetext.3",
        "com.panic.Nova", "com.github.atom", "com.github.GitHubClient", "com.coteditor.CotEditor",
        "com.microsoft.VSCodeInsiders", "com.vscodium", "dev.zed.preview"
    ]
    if modernEditors.contains(bundleId) { Log.method("instant:editor"); return (.instant, (0, 0, 0)) }
    
    // Terminal apps - need conservative delays for reliable rendering
    let terminals = [
        "com.apple.Terminal", "com.googlecode.iterm2", "io.alacritty",
        "com.github.wez.wezterm", "com.mitchellh.ghostty", "dev.warp.Warp-Stable",
        "net.kovidgoyal.kitty", "co.zeit.hyper", "org.tabby", "com.raphaelamorim.rio",
        "com.termius-dmg.mac", "com.google.antigravity"
    ]
    if terminals.contains(bundleId) { Log.method("slow:term"); return (.slow, (3000, 8000, 3000)) }
    
    // JetBrains IDEs - need moderate delays for stability
    if bundleId.hasPrefix("com.jetbrains") { Log.method("slow:jb"); return (.slow, (3000, 8000, 3000)) }
    
    // Default: safe delays for stability across unknown apps
    Log.method("default")
    return (.fast, (1000, 3000, 1500))
}

// MARK: - Screen Text Reading (for word restoration)

/// Read word before cursor from screen (for backspace restoration feature)
/// Returns nil for Safari address bars to avoid placeholder text issues
func getWordToRestoreOnBackspace() -> String? {
    let systemWide = AXUIElementCreateSystemWide()
    var focused: CFTypeRef?
    
    guard AXUIElementCopyAttributeValue(systemWide, kAXFocusedUIElementAttribute as CFString, &focused) == .success,
          let el = focused else {
        Log.info("restore: no focused element")
        return nil
    }
    
    let axEl = el as! AXUIElement
    
    // CRITICAL FIX: Check if this is a browser address bar
    // Safari address bars contain placeholder text that interferes with restoration
    var roleVal: CFTypeRef?
    AXUIElementCopyAttributeValue(axEl, kAXRoleAttribute as CFString, &roleVal)
    let role = roleVal as? String
    
    // Get bundle ID to detect Safari
    var pid: pid_t = 0
    var bundleId: String?
    if AXUIElementGetPid(axEl, &pid) == .success {
        if let app = NSRunningApplication(processIdentifier: pid) {
            bundleId = app.bundleIdentifier
        }
    }
    
    // Skip restoration for browser address bars to avoid placeholder text issues
    let browsers = [
        "com.apple.Safari", "com.apple.SafariTechnologyPreview",
        "com.google.Chrome", "com.google.Chrome.canary", "com.google.Chrome.beta",
        "org.chromium.Chromium", "com.brave.Browser", "com.microsoft.edgemac",
        "com.vivaldi.Vivaldi", "com.opera.Opera", "org.mozilla.firefox",
        "company.thebrowser.Arc", "com.duckduckgo.macos.browser"
    ]
    
    if let bundleId = bundleId, browsers.contains(bundleId), role == "AXTextField" {
        Log.info("restore: skipping browser address bar (Safari/Chrome) to avoid placeholder text")
        return nil
    }
    
    // Get text value
    var textValue: CFTypeRef?
    let textResult = AXUIElementCopyAttributeValue(axEl, kAXValueAttribute as CFString, &textValue)
    guard textResult == .success, let text = textValue as? String, !text.isEmpty else {
        Log.info("restore: no text value (err=\(textResult.rawValue))")
        return nil
    }
    
    // Get selected text range (cursor position)
    var rangeValue: CFTypeRef?
    let rangeResult = AXUIElementCopyAttributeValue(axEl, kAXSelectedTextRangeAttribute as CFString, &rangeValue)
    guard rangeResult == .success else {
        Log.info("restore: no range (err=\(rangeResult.rawValue))")
        return nil
    }
    
    // Extract range from AXValue
    var range = CFRange(location: 0, length: 0)
    guard AXValueGetValue(rangeValue as! AXValue, .cfRange, &range) else {
        Log.info("restore: can't extract range")
        return nil
    }
    
    let cursorPos = range.location
    Log.info("restore: cursor=\(cursorPos) text='\(text.prefix(50))...'")
    guard cursorPos > 0 else { return nil }
    
    let textChars = Array(text)
    guard cursorPos <= textChars.count else {
        Log.info("restore: cursor out of bounds")
        return nil
    }
    let charBeforeCursor = textChars[cursorPos - 1]
    Log.info("restore: charBefore='\(charBeforeCursor)'")
    
    // Only restore if we're about to delete the LAST space/punctuation before a word
    guard charBeforeCursor.isWhitespace || charBeforeCursor.isPunctuation else {
        Log.info("restore: not at word boundary")
        return nil
    }
    
    // Check if there's a word before this space (not more spaces)
    var wordEnd = cursorPos - 1
    
    // Skip all trailing spaces/punctuation to find the word
    while wordEnd > 0 && (textChars[wordEnd - 1].isWhitespace || textChars[wordEnd - 1].isPunctuation) {
        wordEnd -= 1
    }
    
    guard wordEnd > 0 else {
        Log.info("restore: no word before spaces")
        return nil
    }
    
    // Only restore when deleting THE LAST space before the word
    if wordEnd < cursorPos - 1 {
        Log.info("restore: multiple spaces before word")
        return nil
    }
    
    // Find start of word
    var wordStart = wordEnd
    while wordStart > 0 && !textChars[wordStart - 1].isWhitespace && !textChars[wordStart - 1].isPunctuation {
        wordStart -= 1
    }
    
    // Extract word
    let word = String(textChars[wordStart..<wordEnd])
    guard !word.isEmpty else { return nil }
    
    // Only return if it looks like Vietnamese (has diacritics or is pure ASCII letters)
    let hasVietnameseDiacritics = word.contains { c in
        let scalars = c.unicodeScalars
        return scalars.first.map { $0.value >= 0x00C0 && $0.value <= 0x1EF9 } ?? false
    }
    let isPureASCIILetters = word.allSatisfy { $0.isLetter && $0.isASCII }
    
    if hasVietnameseDiacritics || isPureASCIILetters {
        Log.info("restore: found word '\(word)'")
        return word
    }
    
    Log.info("restore: word '\(word)' not Vietnamese")
    return nil
}