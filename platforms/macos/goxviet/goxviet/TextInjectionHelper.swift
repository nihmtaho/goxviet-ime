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
    static let rightArrow: CGKeyCode = 124
}

// MARK: - Text Injector

class TextInjector {
    static let shared = TextInjector()
    
    // Event marker to identify our own injected events
    private let eventMarker: Int64 = 0x564E5F494D45 // "VN_IME"
    
    private let semaphore = DispatchSemaphore(value: 1)
    
    /// Session buffer for tracking full text (used for selectAll method in special apps)
    private var sessionBuffer: String = ""
    
    private init() {}
    
    // MARK: - Public API
    
    /// Synchronous text injection with app-specific optimization
    /// Synchronous text injection with app-specific optimization
    func injectSync(bs: Int, text: String, method: InjectionMethod, delays: (UInt32, UInt32, UInt32), proxy: CGEventTapProxy, bundleId: String? = nil) {
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
            injectViaAutocomplete(bs: bs, text: text, proxy: proxy, bundleId: bundleId)
        case .axDirect:
            injectViaAXWithFallback(bs: bs, text: text, proxy: proxy, bundleId: bundleId)
        }
    }
    
    private enum AXResult {
        case success
        case axFailure
        case browserOverride
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
        
        // Optimized defaults: 200us selection, 500us wait, 200us text
        // Fast enough to feel instant, slow enough for modern apps to process
        let selDelay = delays.0 > 0 ? delays.0 : 200
        let waitDelay = delays.1 > 0 ? delays.1 : 500
        let textDelay = delays.2 > 0 ? delays.2 : 200
        
        if bs > 0 {
            // If text is empty (backspace-only, no replacement), use backspace to properly delete spaces/punctuation
            // This fixes issue where Shift+Left selects space instead of deleting it
            if text.isEmpty {
                // Backspace-only: use backspace for all deletions
                for _ in 0..<bs {
                    postKey(KeyCode.backspace, source: src)
                    usleep(selDelay)
                }
            } else {
                // Text replacement: use Shift+Left to select (normal selection method)
                for _ in 0..<bs {
                    postKey(KeyCode.leftArrow, source: src, flags: .maskShift)
                    usleep(selDelay)
                }
            }
            usleep(waitDelay)
        }
        
        postText(text, source: src, delay: textDelay)
        Log.send("sel", bs, text)
    }
    
    /// Autocomplete injection: Forward Delete to clear suggestion, then backspace + text via proxy
    /// Used for Spotlight and browser address bars where autocomplete auto-selects suggestion text after cursor
    /// Strategy:
    /// - Forward Delete clears auto-selected suggestion
    /// - Backspace removes typed characters
    /// - Type replacement text
    private func injectViaAutocomplete(bs: Int, text: String, proxy: CGEventTapProxy, bundleId: String? = nil) {
        guard let src = CGEventSource(stateID: .privateState) else { return }
        
        // Special case for Zen Browser 'đ' bug (Issue #54)
        // When typing 'd' then 'd', browser often produces "dđ" due to autocomplete race.
        // Fix: Type "đ" (result "dđ") -> Left (betw d,đ) -> Backspace (delete d) -> Right (restore cursor)
        if bundleId == "app.zen-browser.zen" && text == "đ" && bs == 1 {
            Log.info("Zen: Applying special fix for 'đ'")
            
            // 1. Type "đ" (result likely "dđ")
            postText("đ", source: src, proxy: proxy)
            usleep(1000)
            
            // 2. Left Arrow (move cursor between d and đ)
            postKey(KeyCode.leftArrow, source: src, proxy: proxy)
            usleep(1000)
            
            // 3. Backspace (delete the first 'd')
            postKey(KeyCode.backspace, source: src, proxy: proxy)
            usleep(1000)
            
            // 4. Right Arrow (move cursor back to end)
            postKey(KeyCode.rightArrow, source: src, proxy: proxy)
            
            Log.send("auto:zen:dd", bs, text)
            return
        }
        
        // Standard Autocomplete Logic
        
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
    // MARK: - AX API Direct Injection
    
    /// AX API direct text manipulation for browser address bars
    /// Bypasses autocomplete behavior by directly setting text field value via Accessibility API
    /// Returns AXResult indicating success, failure, or browser override
    private func injectViaAX(bs: Int, text: String) -> AXResult {
        // Get focused element
        let systemWide = AXUIElementCreateSystemWide()
        var focusedRef: CFTypeRef?
        guard AXUIElementCopyAttributeValue(systemWide, kAXFocusedUIElementAttribute as CFString, &focusedRef) == .success,
              let ref = focusedRef else {
            Log.info("AX: no focus")
            return .axFailure
        }
        let axEl = ref as! AXUIElement
        
        // Read current text value
        var valueRef: CFTypeRef?
        guard AXUIElementCopyAttributeValue(axEl, kAXValueAttribute as CFString, &valueRef) == .success else {
            Log.info("AX: no value")
            return .axFailure
        }
        let fullText = (valueRef as? String) ?? ""
        
        // Read cursor position and selection
        var rangeRef: CFTypeRef?
        guard AXUIElementCopyAttributeValue(axEl, kAXSelectedTextRangeAttribute as CFString, &rangeRef) == .success,
              let axRange = rangeRef else {
            Log.info("AX: no range")
            return .axFailure
        }
        var range = CFRange()
        guard AXValueGetValue(axRange as! AXValue, .cfRange, &range), range.location >= 0 else {
            Log.info("AX: bad range")
            return .axFailure
        }
        
        // Work in UTF-16 units to align with AX ranges
        let nsText = fullText as NSString
        let length = nsText.length
        let cursor = min(max(range.location, 0), length)
        let selection = min(max(range.length, 0), length - cursor)

        // Handle autocomplete: when selection > 0, text after cursor is autocomplete suggestion
        let hasAutocompleteSuggestion = selection > 0

        // Calculate replacement: delete `bs` chars before cursor, insert `text`
        let deleteStart = max(0, cursor - bs)
        let prefix = nsText.substring(to: deleteStart)
        
        // CRITICAL FIX: When autocomplete suggestion exists, delete it completely
        // Text structure:
        // - prefix: [0, deleteStart)
        // - user typed: [deleteStart, cursor)
        // - SUGGESTION: [cursor, cursor+selection)  <- DELETE THIS
        // - suffix: [cursor+selection, end)
        //
        // Strategy: Only keep prefix + text, never include suggestion or post-suggestion text
        // This prevents browser from re-triggering autocomplete on what follows
        let suffix = !hasAutocompleteSuggestion
            ? nsText.substring(from: cursor)
            : ""
        
        let newText = (prefix + text + suffix).precomposedStringWithCanonicalMapping
        
        // Debug logging
        Log.info("AX: cursor=\(cursor), selection=\(selection), hasAutocomplete=\(hasAutocompleteSuggestion)")
        Log.info("  newText: '\(newText)'")
        
        // Step 1: Write new value (WITHOUT suggestion content)
        // This deletes backspace chars, inserts new text, and OMITS the suggestion entirely
        guard AXUIElementSetAttributeValue(axEl, kAXValueAttribute as CFString, newText as CFTypeRef) == .success else {
            Log.info("AX: write failed")
            return .axFailure
        }
        
        // Step 2: Set cursor position to end of inserted text
        // CRITICAL: Always set selection.length = 0 to clear any re-triggered autocomplete
        let newCursorLocation = deleteStart + text.utf16.count
        var newCursor = CFRange(location: newCursorLocation, length: 0)
        if let newRange = AXValueCreate(.cfRange, &newCursor) {
            let result = AXUIElementSetAttributeValue(axEl, kAXSelectedTextRangeAttribute as CFString, newRange)
            Log.info("AX: cursor set to \(newCursorLocation), selection cleared, result=\(result.rawValue)")
        }
        
        // Step 3: Verify content
        // Read back text to verify no suggestion was auto-added immediately
        var checkValueRef: CFTypeRef?
        if AXUIElementCopyAttributeValue(axEl, kAXValueAttribute as CFString, &checkValueRef) == .success,
           let verifyText = checkValueRef as? String {
            
            // If text got longer than calculated newText, browser re-added suggestion
            if verifyText.count > newText.count {
                Log.info("AX: VERIFY FAILED - browser override (len \(newText.count) -> \(verifyText.count))")
                return .browserOverride
            }
            if verifyText != newText {
                 Log.info("AX: text verification mismatch but not override")
            } else {
                 Log.info("AX: verify OK")
            }
        }
        
        Log.send("ax", bs, text)
        return .success
    }
    
    /// Try AX injection with retries, fallback to selection method if browser re-adds content
    /// Browser address bars trigger autocomplete which overrides our AX writes
    /// Detection: If verified text is longer than expected, browser re-added suggestion
    /// 
    /// IMPROVEMENT: Added retry logic (3 attempts, 5ms delay) to handle temporary AX API failures
    /// when Spotlight is busy with searches. Based on proven pattern from reference implementation.
    /// Try AX injection with optimized retry/fallback logic
    /// Try AX injection with optimized retry/fallback logic
    /// Try AX injection with optimized retry/fallback logic
    private func injectViaAXWithFallback(bs: Int, text: String, proxy: CGEventTapProxy, bundleId: String? = nil) {
        // Retry loop for transient AX failures (busy system/Spotlight)
        for attempt in 0..<3 {
            if attempt > 0 { usleep(5000) } // 5ms backoff
            
            switch injectViaAX(bs: bs, text: text) {
            case .success:
                Log.info("AX: attempt \(attempt) success")
                return // Fast exit on success
                
            case .browserOverride:
                // Immediate fallback if browser interferes (don't retry)
                Log.info("AX: attempt \(attempt) override detected -> fallback")
                injectViaAutocomplete(bs: bs, text: text, proxy: proxy, bundleId: bundleId)
                return
                
            case .axFailure:
                // Transient failure -> retry
                Log.info("AX: attempt \(attempt) ax failure")
                continue
            }
        }
        
        // Retries exhausted -> Fallback
        Log.info("AX: retries exhausted -> fallback")
        injectViaAutocomplete(bs: bs, text: text, proxy: proxy, bundleId: bundleId)
    }
    

    
    /// Perform AX injection and verify result
    /// Returns whether browser honored our text write or re-triggered autocomplete

    
    // MARK: - Session Buffer Management
    
    /// Update session buffer with new composed text
    /// Called before injection to track full session text for selectAll method
    func updateSessionBuffer(backspace: Int, newText: String) {
        if backspace > 0 && sessionBuffer.count >= backspace {
            sessionBuffer.removeLast(backspace)
        }
        sessionBuffer.append(newText)
    }
    
    /// Clear session buffer (call on focus change, submit, mouse click, etc.)
    func clearSessionBuffer() {
        sessionBuffer = ""
    }
    
    /// Set session buffer to specific value (for restoring after paste, etc.)
    func setSessionBuffer(_ text: String) {
        sessionBuffer = text
    }
    
    /// Get current session buffer
    func getSessionBuffer() -> String {
        return sessionBuffer
    }
    
    // MARK: - Helpers
    
    /// Post multiple backspace events in batch (faster than loop with delays)
    /// Reduces event loop overhead by posting events consecutively
    /// CRITICAL: Never use proxy - causes event loop and duplicate keystrokes
    private func postBackspaces(_ count: Int, source: CGEventSource, proxy: CGEventTapProxy? = nil) {
        guard count > 0 else { return }
        
        for _ in 0..<count {
            guard let dn = CGEvent(keyboardEventSource: source, virtualKey: KeyCode.backspace, keyDown: true),
                  let up = CGEvent(keyboardEventSource: source, virtualKey: KeyCode.backspace, keyDown: false) else { continue }
            dn.setIntegerValueField(.eventSourceUserData, value: eventMarker)
            up.setIntegerValueField(.eventSourceUserData, value: eventMarker)
            
            // ALWAYS post via cgSessionEventTap - never via proxy to avoid event loop
            dn.post(tap: .cgSessionEventTap)
            up.post(tap: .cgSessionEventTap)
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

/// Static bundle ID constants for app detection
/// Using Set for O(1) lookup instead of Array O(n) contains()
/// Prevents reallocating these arrays on every keystroke
private struct BundleConstants {
    static let chromiumBrowsers: Set<String> = [
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
    
    static let firefoxBrowsers: Set<String> = [
        "org.mozilla.firefox", "org.mozilla.firefoxdeveloperedition", "org.mozilla.nightly",
        "org.waterfoxproject.waterfox", "io.gitlab.librewolf-community.librewolf",
        "one.ablaze.floorp", "org.torproject.torbrowser", "net.mullvad.mullvadbrowser",
        "app.zen-browser.zen"  // Zen Browser - treated as regular Firefox browser
    ]
    
    static let safariBrowsers: Set<String> = [
        "com.apple.Safari", "com.apple.SafariTechnologyPreview",
        "com.kagi.kagimacOS"
    ]
    
    static let modernEditors: Set<String> = [
        // Code Editors
        "com.microsoft.VSCode", "dev.zed.Zed", "com.sublimetext.4", "com.sublimetext.3",
        "com.panic.Nova", "com.github.atom", "com.github.GitHubClient", "com.coteditor.CotEditor",
        "com.microsoft.VSCodeInsiders", "com.vscodium", "dev.zed.preview", "com.google.antigravity",
        // Text Editors
        "com.apple.TextEdit", "com.apple.Notes", "com.apple.mail",
        // Note-taking apps
        "md.obsidian", "com.bear-writer.Bear", "com.dayoneapp.dayone",
        // Chat & Communication
        "com.tinyspeck.slackmacgap", "com.hnc.Discord", "com.apple.iChat",
        "com.microsoft.teams", "com.microsoft.teams2", "us.zoom.xos",
        // Browsers (content areas, not address bars)
        "com.google.Chrome", "com.apple.Safari", "org.mozilla.firefox",
        "com.brave.Browser", "com.microsoft.edgemac", "com.vivaldi.Vivaldi",
        "company.thebrowser.Arc", "com.opera.Opera"
    ]
    
    static let terminals: Set<String> = [
        "com.apple.Terminal", "com.googlecode.iterm2", "io.alacritty",
        "com.github.wez.wezterm", "com.mitchellh.ghostty", "dev.warp.Warp-Stable",
        "net.kovidgoyal.kitty", "co.zeit.hyper", "org.tabby", "com.raphaelamorim.rio",
        "com.termius-dmg.mac", "com.google.antigravity"
    ]
}

// MARK: - Detection Cache

/// Cache for detectMethod() - avoids expensive AX queries on every keystroke
/// Uses time-based TTL (200ms) + app switch invalidation for safety
/// PERFORMANCE: Uses CFAbsoluteTimeGetCurrent() instead of Date() for faster timestamp
private enum DetectionCache {
    static var result: (method: InjectionMethod, delays: (UInt32, UInt32, UInt32))?
    static var timestamp: CFAbsoluteTime = 0
    static var lastLoggedKey: String = ""  // Only log when method+app changes
    static let ttl: CFAbsoluteTime = 0.2  // 200ms

    static func get() -> (InjectionMethod, (UInt32, UInt32, UInt32))? {
        guard let cached = result,
              CFAbsoluteTimeGetCurrent() - timestamp < ttl else { return nil }
        return (cached.method, cached.delays)
    }

    static func set(_ method: InjectionMethod, _ delays: (UInt32, UInt32, UInt32), logKey: String) {
        result = (method, delays)
        timestamp = CFAbsoluteTimeGetCurrent()
        // Only log when method+app combination changes
        if logKey != lastLoggedKey {
            lastLoggedKey = logKey
            Log.method(logKey)
        }
    }

    static func clear() {
        result = nil
        timestamp = 0
    }
}

/// Clear detection cache (call on app switch)
func clearDetectionCache() {
    DetectionCache.clear()
}

// MARK: - Method Detection

/// Detect optimal injection method based on focused app and UI element
func detectMethod() -> (InjectionMethod, (UInt32, UInt32, UInt32)) {
    // Check cache first (200ms TTL)
    if let cached = DetectionCache.get() {
        return cached
    }
    
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
    
    // Helper to cache and return result (only logs when method+app changes)
    func cached(_ m: InjectionMethod, _ d: (UInt32, UInt32, UInt32), _ methodName: String) -> (InjectionMethod, (UInt32, UInt32, UInt32)) {
        let logKey = "\(methodName) [\(bundleId)] role=\(role ?? "nil")"
        DetectionCache.set(m, d, logKey: logKey)
        return (m, d)
    }
    
    // Selection method for autocomplete UI elements (ComboBox, SearchField)
    if role == "AXComboBox" { return cached(.selection, (0, 0, 0), "sel:combo") }
    if role == "AXSearchField" { return cached(.selection, (0, 0, 0), "sel:search") }
    
    // Spotlight - use axDirect with no delays
    if bundleId == "com.apple.Spotlight" || bundleId == "com.apple.systemuiserver" { 
        return cached(.axDirect, (0, 0, 0), "ax:spotlight")
    }
    
    // Arc/Dia browser - use selection method (same as Chrome which works)
    let theBrowserCompany = ["company.thebrowser.Browser", "company.thebrowser.Arc", "company.thebrowser.dia"]
    if theBrowserCompany.contains(bundleId) && (role == "AXTextField" || role == "AXTextArea") {
        return cached(.axDirect, (0, 0, 0), "ax:arc")
    }
    
    // Chromium-based browser address bars - use selection method
    if BundleConstants.chromiumBrowsers.contains(bundleId) && role == "AXTextField" {
        return cached(.selection, (0, 0, 0), "sel:chromium-addr")
    }
    
    // Firefox-based browsers (including Zen Browser) - selection for address bar, slow for content
    if BundleConstants.firefoxBrowsers.contains(bundleId) {
        if role == "AXTextField" || role == "AXWindow" || role == "AXTextArea" {
            if bundleId == "app.zen-browser.zen" {
                // Use axDirect method with fallback to optimized autocomplete
                // 1ms selection, 2ms wait, 1ms type - balanced for speed and correctness
                return cached(.axDirect, (0, 0, 0), "ax:zen")
            }
            return cached(.selection, (0, 0, 0), "sel:firefox-addr")
        } else {
            return cached(.slow, (3000, 8000, 3000), "slow:firefox")
        }
    }
    
    // Safari - use selection method
    if BundleConstants.safariBrowsers.contains(bundleId) && role == "AXTextField" {
        return cached(.selection, (0, 0, 0), "sel:safari")
    }
    if role == "AXTextField" && bundleId.hasPrefix("com.jetbrains") { 
        return cached(.selection, (0, 0, 0), "sel:jb") 
    }
    
    // Microsoft Office apps - use backspace method
    if bundleId == "com.microsoft.Excel" { return cached(.slow, (3000, 8000, 3000), "slow:excel") }
    if bundleId == "com.microsoft.Word" { return cached(.slow, (3000, 8000, 3000), "slow:word") }
    
    // Electron apps - higher delays
    if bundleId == "com.todesktop.230313mzl4w4u92" { return cached(.slow, (8000, 15000, 8000), "slow:claude") }
    if bundleId == "notion.id" { return cached(.slow, (8000, 15000, 8000), "slow:notion") }
    
    // Modern editors - instant method for speed
    if BundleConstants.modernEditors.contains(bundleId) { 
        return cached(.instant, (0, 0, 0), "instant:editor") 
    }
    
    // Terminals - high delays for stable rendering
    if BundleConstants.terminals.contains(bundleId) { 
        return cached(.slow, (8000, 25000, 8000), "slow:term") 
    }
    
    // JetBrains IDEs - moderate delays
    if bundleId.hasPrefix("com.jetbrains") { return cached(.slow, (3000, 8000, 3000), "slow:jb") }
    
    // Default: conservative delays
    return cached(.instant, (0, 0, 0), "default")
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
