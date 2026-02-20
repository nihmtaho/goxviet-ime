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
    case charByChar     // Safari Google Docs: backspace + text character-by-character
    case selection      // Browser address bars: Shift+Left select + type replacement
    case emptyCharPrefix // Browser address bars: empty char to break autocomplete highlight (U+202F) + extra backspace
    case axDirect       // Spotlight primary: AX API direct text manipulation (macOS 13+)
    case syncProxy      // Games: synchronous injection via CGEventTapPostEvent(proxy)
    case passthrough    // iPhone Mirroring: pass through all keys (remote device handles input)
}

// MARK: - Text Injector

/// Handles text injection with proper sequencing to prevent race conditions
/// Following Single Responsibility Principle - only handles text injection
public final class TextInjector {
    static let shared = TextInjector()

    // Semaphore to block keyboard callback until injection completes
    private let semaphore = DispatchSemaphore(value: 1)

    private init() {}

    /// Post break key (Enter, punctuation) synthetically after text injection
    /// Used for auto-restore to ensure correct event ordering
    func postBreakKey(keyCode: CGKeyCode, shift: Bool) {
        guard let src = CGEventSource(stateID: .privateState) else { return }
        let flags: CGEventFlags = shift ? .maskShift : []
        postKey(keyCode, source: src, flags: flags)
    }

    /// Clear any session buffers or state
    /// Called when focus changes or mouse clicks occur
    /// Also clears detection cache since app/element context has changed
    func clearSessionBuffer() {
        // Clear detection cache - focus/app changed so cached method may be invalid
        DetectionCache.clear()
        Log.info("TextInjector: Session buffer and detection cache cleared")
    }

    /// Post a single key event through the event tap proxy
    /// Used by InputManager for special key injection
    func postKey(_ keyCode: CGKeyCode, source: CGEventSource, flags: CGEventFlags = [], proxy: CGEventTapProxy? = nil) {
        guard let keyDown = CGEvent(keyboardEventSource: source, virtualKey: keyCode, keyDown: true),
              let keyUp = CGEvent(keyboardEventSource: source, virtualKey: keyCode, keyDown: false) else {
            return
        }

        // Mark as injected to prevent reprocessing
        keyDown.setIntegerValueField(.eventSourceUserData, value: kEventMarker)
        keyUp.setIntegerValueField(.eventSourceUserData, value: kEventMarker)

        // Apply modifier flags if any
        if flags != [] {
            keyDown.flags = flags
            keyUp.flags = flags
        }

        // Post via proxy (synchronous) or session event tap (async)
        if let proxy = proxy {
            keyDown.tapPostEvent(proxy)
            keyUp.tapPostEvent(proxy)
        } else {
            keyDown.post(tap: .cgSessionEventTap)
            keyUp.post(tap: .cgSessionEventTap)
        }
    }

    /// Inject text replacement synchronously (blocks until complete)
    func injectSync(bs: Int, text: String, method: InjectionMethod, delays: (UInt32, UInt32, UInt32), proxy: CGEventTapProxy) {
        semaphore.wait()
        defer { semaphore.signal() }

        switch method {
        case .selection:
            injectViaSelection(bs: bs, text: text, delays: delays)
        case .axDirect:
            injectViaAXWithFallback(bs: bs, text: text, proxy: proxy)
        case .emptyCharPrefix:
            injectViaBackspace(bs: bs, text: text, delays: delays, emptyCharPrefix: true)
        case .charByChar:
            injectViaBackspace(bs: bs, text: text, delays: delays, charByChar: true)
        case .instant, .slow, .fast:
            injectViaBackspace(bs: bs, text: text, delays: delays)
        case .syncProxy:
            injectViaProxy(bs: bs, text: text, proxy: proxy)
        case .passthrough:
            // Should not reach here - passthrough is handled in keyboard callback
            break
        }

        // Settle time: 20ms for slow apps, 5ms for others
        usleep(method == .slow ? 20000 : 5000)
    }

    // MARK: - Injection Methods

    /// Standard backspace injection: delete N chars, then type replacement
    /// - charByChar: character-by-character mode (slower but more reliable for Safari Google Docs)
    /// - emptyCharPrefix: send empty char (U+202F) first to break autocomplete highlight (for browser address bars)
    private func injectViaBackspace(bs: Int, text: String, delays: (UInt32, UInt32, UInt32), charByChar: Bool = false, emptyCharPrefix: Bool = false) {
        guard let src = CGEventSource(stateID: .privateState) else {
            Log.info("inject FAILED: no event source")
            return
        }

        let startTime = Log.isEnabled ? CFAbsoluteTimeGetCurrent() : 0
        var bs = bs

        // Empty char prefix: send U+202F to break autocomplete highlight, then +1 backspace
        if emptyCharPrefix {
            let emptyChar: [UniChar] = [0x202F]
            if let dn = CGEvent(keyboardEventSource: src, virtualKey: 0, keyDown: true),
               let up = CGEvent(keyboardEventSource: src, virtualKey: 0, keyDown: false) {
                dn.setIntegerValueField(.eventSourceUserData, value: kEventMarker)
                up.setIntegerValueField(.eventSourceUserData, value: kEventMarker)
                dn.keyboardSetUnicodeString(stringLength: 1, unicodeString: emptyChar)
                up.keyboardSetUnicodeString(stringLength: 1, unicodeString: emptyChar)
                dn.post(tap: .cgSessionEventTap)
                up.post(tap: .cgSessionEventTap)
            }
            usleep(delays.0 > 0 ? delays.0 : 1000)
            bs += 1  // +1 to also delete the empty char
        }

        for _ in 0..<bs {
                    postKey(KeyCodes.backspace, source: src)
            usleep(delays.0)
        }
        if bs > 0 { usleep(delays.1) }

        let chunks = postText(text, source: src, delay: delays.2, chunkSize: charByChar ? 1 : 20)

        if Log.isEnabled {
            let elapsed = (CFAbsoluteTimeGetCurrent() - startTime) * 1000
            let expected = (Double(bs) * Double(delays.0) + (bs > 0 ? Double(delays.1) : 0) + Double(chunks) * Double(delays.2)) / 1000
            Log.info("inject done: bs=\(bs) chunks=\(chunks) time=\(String(format: "%.1f", elapsed))ms expect=\(String(format: "%.1f", expected))ms")
        }
    }

    /// Synchronous proxy injection: uses CGEventTapPostEvent(proxy) for zero-delay delivery
    /// Events are injected directly into the event tap pipeline, guaranteeing correct ordering
    private func injectViaProxy(bs: Int, text: String, proxy: CGEventTapProxy) {
        guard let src = CGEventSource(stateID: .privateState) else { return }

        for _ in 0..<bs {
            postKey(KeyCodes.backspace, source: src, proxy: proxy)
        }

        postText(text, source: src, proxy: proxy)
    }

    /// Selection injection: Shift+Left to select, then type replacement (for browser address bars)
    /// For backspace-only (text empty): use backspace to properly delete spaces/punctuation
    /// For text replacement: use Shift+Left to select (normal behavior)
    private func injectViaSelection(bs: Int, text: String, delays: (UInt32, UInt32, UInt32)) {
        guard let src = CGEventSource(stateID: .privateState) else { return }

        let selDelay = delays.0 > 0 ? delays.0 : 1000
        let waitDelay = delays.1 > 0 ? delays.1 : 3000
        let textDelay = delays.2 > 0 ? delays.2 : 2000

        if bs > 0 {
            // If text is empty (backspace-only, no replacement), use backspace to properly delete spaces/punctuation
            // This fixes issue where Shift+Left selects space instead of deleting it
            if text.isEmpty {
                // Backspace-only: use backspace for all deletions
                for _ in 0..<bs {
            postKey(KeyCodes.backspace, source: src)
                    usleep(selDelay)
                }
            } else {
                // Text replacement: use Shift+Left to select (normal selection method)
                for _ in 0..<bs {
                    postKey(KeyCodes.leftArrow, source: src, flags: .maskShift)
                    usleep(selDelay)
                }
            }
            usleep(waitDelay)
        }

        postText(text, source: src, delay: textDelay)
    }

    /// Autocomplete injection: Forward Delete to clear suggestion, then backspace + text via proxy
    /// Used for Spotlight where autocomplete auto-selects suggestion text after cursor
    private func injectViaAutocomplete(bs: Int, text: String, proxy: CGEventTapProxy) {
        guard let src = CGEventSource(stateID: .privateState) else { return }

        // Forward Delete clears auto-selected suggestion
        postKey(KeyCodes.forwardDelete, source: src, proxy: proxy)
        usleep(3000)

        // Backspaces remove typed characters
        for _ in 0..<bs {
            postKey(KeyCodes.backspace, source: src, proxy: proxy)
            usleep(1000)
        }
        if bs > 0 { usleep(5000) }

        // Type replacement text
        postText(text, source: src, proxy: proxy)
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

        // Work in UTF-16 units to align with AX ranges
        let nsText = fullText as NSString
        let length = nsText.length
        let cursor = min(max(range.location, 0), length)
        let selection = min(max(range.length, 0), length - cursor)

        // Handle autocomplete: when selection > 0, text after cursor is autocomplete suggestion
        let hasAutocompleteSuggestion = selection > 0

        // FIX: Convert bs (grapheme count from Rust) to UTF-16 offset for AX API
        // Rust engine returns bs as character count, but AX API uses UTF-16 offsets
        // We need to count back `bs` graphemes from cursor position
        let deleteStartUTF16: Int
        if bs > 0 {
            // Convert UTF-16 cursor position to String index
            let swiftText = fullText
            let utf16View = swiftText.utf16
            guard cursor <= utf16View.count else {
                Log.info("AX: cursor beyond text length")
                return false
            }
            let cursorIndex = utf16View.index(utf16View.startIndex, offsetBy: cursor)
            
            // Convert to String.Index for grapheme-aware operations
            let stringIndex = String.Index(cursorIndex, within: swiftText) ?? swiftText.endIndex
            
            // Count back `bs` graphemes
            var targetIndex = stringIndex
            var charsToDelete = bs
            while charsToDelete > 0 && targetIndex > swiftText.startIndex {
                targetIndex = swiftText.index(before: targetIndex)
                charsToDelete -= 1
            }
            
            // Convert back to UTF-16 offset
            deleteStartUTF16 = swiftText.utf16.distance(from: swiftText.utf16.startIndex, to: targetIndex)
        } else {
            deleteStartUTF16 = cursor
        }
        
        // SAFETY: Ensure deleteStart never goes below 0
        // This can happen if bs > number of characters before cursor
        let deleteStart = max(0, deleteStartUTF16)
        
        // DEBUG: Log the conversion for troubleshooting
        if bs > 0 {
            Log.info("AX: bs=\(bs) chars, cursor=\(cursor) UTF-16, deleteStart=\(deleteStart) UTF-16")
        }
        let prefix = nsText.substring(to: deleteStart)

        // CRITICAL FIX: When autocomplete suggestion exists, delete it completely
        let suffix = !hasAutocompleteSuggestion
            ? nsText.substring(from: cursor)
            : ""

        // NFC normalize to ensure Vietnamese diacritics are precomposed (matching reference impl)
        let newText = (prefix + text + suffix).precomposedStringWithCanonicalMapping

        // Write new value
        guard AXUIElementSetAttributeValue(axEl, kAXValueAttribute as CFString, newText as CFTypeRef) == .success else {
            Log.info("AX: write failed")
            return false
        }

        // Set cursor position to end of inserted text
        // Reference impl sets cursor for ALL apps including Spotlight
        let newCursorLocation = deleteStart + text.utf16.count
        var newCursor = CFRange(location: newCursorLocation, length: 0)
        if let newRange = AXValueCreate(.cfRange, &newCursor) {
            AXUIElementSetAttributeValue(axEl, kAXSelectedTextRangeAttribute as CFString, newRange)
        }

        return true
    }

    /// Try AX injection with retries, fallback to autocomplete method if all fail
    /// Spotlight can be busy searching, causing AX API to fail temporarily
    private func injectViaAXWithFallback(bs: Int, text: String, proxy: CGEventTapProxy) {
        // Try AX API up to 3 times (Spotlight might be busy)
        for attempt in 0..<3 {
            if attempt > 0 {
                usleep(5000)  // 5ms delay before retry
            }
            if injectViaAX(bs: bs, text: text) {
                return  // Success!
            }
        }

        // All AX attempts failed - fallback to autocomplete method
        Log.info("AX: fallback to autocomplete")
        injectViaAutocomplete(bs: bs, text: text, proxy: proxy)
    }
}

// MARK: - App Detection

/// Static bundle ID constants for app detection
/// Using Set for O(1) lookup instead of Array O(n) contains()
/// Prevents reallocating these arrays on every keystroke
private struct BundleConstants {
    static let safariBrowsers: Set<String> = [
        "com.apple.Safari", "com.apple.SafariTechnologyPreview",
        "com.kagi.kagimacOS"
    ]
    
    static let terminals: Set<String> = [
        "com.apple.Terminal", "com.googlecode.iterm2", "io.alacritty",
        "com.github.wez.wezterm", "com.mitchellh.ghostty", "dev.warp.Warp-Stable",
        "net.kovidgoyal.kitty", "co.zeit.hyper", "org.tabby", "com.raphaelamorim.rio",
        "com.termius-dmg.mac"
        // Note: com.google.antigravity is a code editor (Cursor), handled separately
    ]

    // MARK: - App Categories for Method Detection

    /// Browsers and search panels that need emptyCharPrefix for address bars
    /// Note: Safari browsers (com.apple.Safari, com.kagi.kagimacOS) are handled separately
    static let addressBarBrowsers: Set<String> = [
        // The Browser Company
        "company.thebrowser.Browser", "company.thebrowser.Arc", "company.thebrowser.dia",
        // Firefox-based
        "org.mozilla.firefox", "org.mozilla.firefoxdeveloperedition", "org.mozilla.nightly",
        "org.waterfoxproject.waterfox", "io.gitlab.librewolf-community.librewolf",
        "one.ablaze.floorp", "org.torproject.torbrowser", "net.mullvad.mullvadbrowser",
        "app.zen-browser.zen",
        // Chromium-based
        "com.google.Chrome", "com.google.Chrome.canary", "com.google.Chrome.beta",
        "org.chromium.Chromium",
        "com.brave.Browser", "com.brave.Browser.beta", "com.brave.Browser.nightly",
        "com.microsoft.edgemac", "com.microsoft.edgemac.Beta",
        "com.microsoft.edgemac.Dev", "com.microsoft.edgemac.Canary",
        "com.vivaldi.Vivaldi", "com.vivaldi.Vivaldi.snapshot",
        "ru.yandex.desktop.yandex-browser",
        // Opera
        "com.opera.Opera", "com.operasoftware.Opera", "com.operasoftware.OperaGX",
        "com.operasoftware.OperaAir", "com.opera.OperaNext",
        // Others
        "com.sigmaos.sigmaos.macos", "com.pushplaylabs.sidekick",
        "com.firstversionist.polypane", "ai.perplexity.comet",
        "com.duckduckgo.macos.browser", "com.openai.atlas"
        // Note: WebKit-based browsers (com.kagi.kagimacOS) are handled in safariBrowsers
    ]

    /// AX roles that indicate an address bar
    static let addressBarRoles: Set<String> = ["AXTextField", "AXTextArea", "AXWindow"]

    /// Code editors that need slow injection (Electron/Monaco-based)
    /// Note: Terminals are handled separately in the 'terminals' set
    static let slowCodeEditors: Set<String> = [
        // VSCode-based IDEs
        "com.microsoft.VSCode", "com.google.antigravity", "com.todesktop.cursor",
        "com.visualstudio.code.oss", "com.vscodium",
        // Other code editors
        "dev.zed.Zed", "com.sublimetext.4", "com.sublimetext.3", "com.panic.Nova"
    ]

    /// Combined set of code editors and terminals for method detection
    static var codeEditorsAndTerminals: Set<String> {
        slowCodeEditors.union(terminals)
    }

    // MARK: - Individual App Bundle IDs

    static let screenContinuity = "com.apple.ScreenContinuity"
    static let spotlight = "com.apple.Spotlight"
    static let systemUIServer = "com.apple.systemuiserver"
    static let texstudio = "texstudio"
    static let caudex = "com.caudex.dev"
    static let claude = "com.todesktop.230313mzl4w4u92"
    static let notion = "notion.id"
    static let microsoftExcel = "com.microsoft.Excel"
    static let microsoftWord = "com.microsoft.Word"

    // MARK: - Bundle ID Prefixes

    static let riotGamesPrefix = "com.riotgames"
    static let blizzardPrefix = "com.blizzard"
    static let valvePrefix = "com.valvesoftware"
    static let jetBrainsPrefix = "com.jetbrains"
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

/// Helper function to resolve role from AX element
/// Tries multiple strategies to get a usable role string
func resolveRole(axEl: AXUIElement) -> String? {
    // Strategy 1: Try kAXRoleAttribute
    var roleVal: CFTypeRef?
    if AXUIElementCopyAttributeValue(axEl, kAXRoleAttribute as CFString, &roleVal) == .success,
       let role = roleVal as? String, !role.isEmpty {
        return role
    }
    
    // Strategy 2: Try kAXSubroleAttribute
    var subroleVal: CFTypeRef?
    if AXUIElementCopyAttributeValue(axEl, kAXSubroleAttribute as CFString, &subroleVal) == .success,
       let subrole = subroleVal as? String, !subrole.isEmpty {
        Log.info("resolveRole: using subrole '\(subrole)'")
        return subrole
    }
    
    // Strategy 3: Walk up parent chain looking for a role
    var parent: CFTypeRef?
    if AXUIElementCopyAttributeValue(axEl, kAXParentAttribute as CFString, &parent) == .success,
       let parentEl = parent {
        let parentElement = parentEl as! AXUIElement
        var parentRoleVal: CFTypeRef?
        if AXUIElementCopyAttributeValue(parentElement, kAXRoleAttribute as CFString, &parentRoleVal) == .success,
           let parentRole = parentRoleVal as? String, !parentRole.isEmpty {
            Log.info("resolveRole: using parent role '\(parentRole)'")
            return parentRole
        }
    }
    
    // Strategy 4: Check if it's a text input by testing for text attributes
    var valueVal: CFTypeRef?
    var rangeVal: CFTypeRef?
    let hasValue = AXUIElementCopyAttributeValue(axEl, kAXValueAttribute as CFString, &valueVal) == .success
    let hasRange = AXUIElementCopyAttributeValue(axEl, kAXSelectedTextRangeAttribute as CFString, &rangeVal) == .success
    
    if hasValue && hasRange {
        Log.info("resolveRole: inferred 'AXTextField' from value+range attributes")
        return "AXTextField"
    }
    
    Log.info("resolveRole: all strategies failed, returning nil")
    return nil
}

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
    
    // Try up to 3 times with small delay for transient failures
    var focusResult: AXError = .failure
    for attempt in 0..<3 {
        if attempt > 0 {
            usleep(1000) // 1ms delay between retries
        }
        focusResult = AXUIElementCopyAttributeValue(systemWide, kAXFocusedUIElementAttribute as CFString, &focused)
        if focusResult == .success {
            break
        }
    }
    
    if focusResult == .success, let el = focused {
        let axEl = el as! AXUIElement
        
        // Get role using resolveRole helper (handles role=nil cases)
        role = resolveRole(axEl: axEl)
        
        // Get owning app's bundle ID (works for Spotlight overlay)
        var pid: pid_t = 0
        if AXUIElementGetPid(axEl, &pid) == .success {
            if let app = NSRunningApplication(processIdentifier: pid) {
                bundleId = app.bundleIdentifier
            }
        }
    }
    
    // Fallback chain when AX fails
    if bundleId == nil {
        // 1. Try PerAppModeManagerEnhanced (tracks app switches including Spotlight)
        if let perAppBundleId = PerAppModeManagerEnhanced.shared.currentBundleId {
            bundleId = perAppBundleId
        }
        
        // 2. Try frontmost application
        if bundleId == nil {
            bundleId = NSWorkspace.shared.frontmostApplication?.bundleIdentifier
        }
        
        // 3. Use window list as last resort
        if bundleId == nil {
            if let windowList = CGWindowListCopyWindowInfo([.optionOnScreenOnly, .excludeDesktopElements], kCGNullWindowID) as? [[String: Any]] {
                for window in windowList {
                    if let ownerPID = window[kCGWindowOwnerPID as String] as? pid_t,
                       let windowLayer = window[kCGWindowLayer as String] as? Int,
                       windowLayer == 0,
                       let app = NSRunningApplication(processIdentifier: ownerPID),
                       let bid = app.bundleIdentifier {
                        bundleId = bid
                        break
                    }
                }
            }
        }
    }
    
    guard let bundleId = bundleId else { return (.fast, (200, 800, 500)) }
    
    // Helper to cache and return result (only logs when method+app changes)
    func cached(_ m: InjectionMethod, _ d: (UInt32, UInt32, UInt32), _ methodName: String) -> (InjectionMethod, (UInt32, UInt32, UInt32)) {
        let logKey = "\(methodName) [\(bundleId)] role=\(role ?? "nil")"
        DetectionCache.set(m, d, logKey: logKey)
        return (m, d)
    }
    
    // iPhone Mirroring (ScreenContinuity) - pass through all keys
    if bundleId == BundleConstants.screenContinuity {
        return cached(.passthrough, (0, 0, 0), "pass:iphone")
    }

    // Selection method for autocomplete UI elements (ComboBox, SearchField)
    if role == "AXComboBox" { return cached(.selection, (0, 0, 0), "sel:combo") }
    if role == "AXSearchField" { return cached(.selection, (0, 0, 0), "sel:search") }

    // Spotlight and systemUIServer - use autocomplete method
    // axDirect causes focus loss, emptyCharPrefix causes duplicate chars
    // Try autocomplete method which uses Forward Delete + backspace
    if bundleId == BundleConstants.spotlight || bundleId == BundleConstants.systemUIServer {
        return cached(.axDirect, (0, 0, 0), "ax:spotlight")
    }

    // Safari: address bar uses emptyCharPrefix, content areas (Google Docs) use charByChar
    if BundleConstants.safariBrowsers.contains(bundleId) {
        if role == "AXTextField" { return cached(.emptyCharPrefix, (0, 0, 0), "ax:safari") }
        return cached(.emptyCharPrefix, (0, 0, 0), "char:safari")
    }

    // Browser address bars: emptyCharPrefix to break autocomplete
    // FIX: Also apply when role=nil but bundleId is a known browser (AX focus might fail)
    if BundleConstants.addressBarBrowsers.contains(bundleId) {
        if let role = role {
            if BundleConstants.addressBarRoles.contains(role) {
                return cached(.emptyCharPrefix, (0, 0, 0), "emptyChar:browser")
            }
            // Role is known but not an address bar role, use charByChar for content areas
            return cached(.charByChar, (0, 0, 0), "char:browser")
        } else {
            // Role is nil (AX focus failed), assume address bar for known browsers
            Log.info("AX: role=nil for known browser \(bundleId), using emptyCharPrefix")
            return cached(.emptyCharPrefix, (0, 0, 0), "emptyChar:browser:unknown")
        }
    }

    // Games - synchronous proxy injection
    if bundleId.hasPrefix(BundleConstants.riotGamesPrefix) {
        return cached(.syncProxy, (0, 0, 0), "sync:game:riot")
    }
    if bundleId.hasPrefix(BundleConstants.blizzardPrefix) {
        return cached(.syncProxy, (0, 0, 0), "sync:game:blizzard")
    }
    if bundleId.hasPrefix(BundleConstants.valvePrefix) {
        return cached(.syncProxy, (0, 0, 0), "sync:game:valve")
    }

    if role == "AXTextField" && bundleId.hasPrefix(BundleConstants.jetBrainsPrefix) {
        return cached(.selection, (0, 0, 0), "sel:jb")
    }
    
    // Code editors & terminals - higher delays for Monaco/Electron-based apps
    if BundleConstants.codeEditorsAndTerminals.contains(bundleId) {
        return cached(.slow, (8000, 25000, 8000), "slow:code")
    }

    // LaTeX editors (Qt-based) - charByChar for reliable Unicode
    if bundleId == BundleConstants.texstudio {
        return cached(.charByChar, (3000, 8000, 3000), "char:texstudio")
    }
    if bundleId.hasPrefix(BundleConstants.jetBrainsPrefix) {
        return cached(.slow, (8000, 25000, 8000), "slow:jb")
    }

    // Caudex - char-by-char with higher delays
    if bundleId == BundleConstants.caudex {
        return cached(.charByChar, (5000, 15000, 5000), "char:caudex")
    }

    // Electron apps - higher delays for Monaco editor
    if bundleId == BundleConstants.claude {
        return cached(.slow, (8000, 15000, 8000), "slow:claude")
    }
    if bundleId == BundleConstants.notion {
        return cached(.slow, (12000, 25000, 12000), "slow:notion")
    }

    // Microsoft Office apps - use backspace method
    if bundleId == BundleConstants.microsoftExcel {
        return cached(.slow, (3000, 8000, 3000), "slow:excel")
    }
    if bundleId == BundleConstants.microsoftWord {
        return cached(.slow, (3000, 8000, 3000), "slow:word")
    }
    
    // Default: safe delays (changed from instant to fast)
    return cached(.fast, (1000, 3000, 1500), "default")
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
    // Use resolveRole helper to handle role=nil cases
    let role = resolveRole(axEl: axEl)
    
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

// MARK: - Event Injection Helpers

/// Marker value for identifying events injected by Gõ Việt
/// Used to prevent processing our own injected events
let kEventMarker: Int64 = 0x474F5856  // "GOXV" in hex

/// Post a single key event
/// - Parameters:
///   - keyCode: The virtual key code to post
///   - source: The CGEventSource to use
///   - flags: Optional event flags (e.g., .maskShift)
///   - proxy: Optional event tap proxy for synchronous injection
func postKey(_ keyCode: CGKeyCode, source: CGEventSource, flags: CGEventFlags = [], proxy: CGEventTapProxy? = nil) {
    guard let keyDown = CGEvent(keyboardEventSource: source, virtualKey: keyCode, keyDown: true),
          let keyUp = CGEvent(keyboardEventSource: source, virtualKey: keyCode, keyDown: false) else {
        return
    }
    
    // Mark as injected to prevent reprocessing
    keyDown.setIntegerValueField(.eventSourceUserData, value: kEventMarker)
    keyUp.setIntegerValueField(.eventSourceUserData, value: kEventMarker)
    
    // Apply modifier flags if any
    if flags != [] {
        keyDown.flags = flags
        keyUp.flags = flags
    }
    
    // Post via proxy (synchronous) or session event tap (async)
    if let proxy = proxy {
        keyDown.tapPostEvent(proxy)
        keyUp.tapPostEvent(proxy)
    } else {
        keyDown.post(tap: .cgSessionEventTap)
        keyUp.post(tap: .cgSessionEventTap)
    }
}

/// Post text as Unicode keyboard events
/// - Parameters:
///   - text: The text to post
///   - source: The CGEventSource to use
///   - delay: Microsecond delay between chunks (0 for no delay)
///   - chunkSize: Number of characters per chunk (for character-by-character mode)
///   - proxy: Optional event tap proxy for synchronous injection
/// - Returns: Number of chunks posted
@discardableResult
func postText(_ text: String, source: CGEventSource, delay: UInt32 = 0, chunkSize: Int = 20, proxy: CGEventTapProxy? = nil) -> Int {
    let utf16Chars = Array(text.utf16)
    var chunksPosted = 0
    
    // Process in chunks
    for i in stride(from: 0, to: utf16Chars.count, by: chunkSize) {
        let end = min(i + chunkSize, utf16Chars.count)
        let chunk = Array(utf16Chars[i..<end])
        
        // Create key down event with Unicode string
        guard let keyDown = CGEvent(keyboardEventSource: source, virtualKey: 0, keyDown: true),
              let keyUp = CGEvent(keyboardEventSource: source, virtualKey: 0, keyDown: false) else {
            continue
        }
        
        // Set Unicode string
        keyDown.keyboardSetUnicodeString(stringLength: chunk.count, unicodeString: chunk)
        keyUp.keyboardSetUnicodeString(stringLength: chunk.count, unicodeString: chunk)
        
        // Mark as injected
        keyDown.setIntegerValueField(.eventSourceUserData, value: kEventMarker)
        keyUp.setIntegerValueField(.eventSourceUserData, value: kEventMarker)
        
        // Post via proxy (synchronous) or session event tap (async)
        if let proxy = proxy {
            keyDown.tapPostEvent(proxy)
            keyUp.tapPostEvent(proxy)
        } else {
            keyDown.post(tap: .cgSessionEventTap)
            keyUp.post(tap: .cgSessionEventTap)
        }
        
        chunksPosted += 1
        
        // Apply delay between chunks if specified
        if delay > 0 && end < utf16Chars.count {
            usleep(delay)
        }
    }
    
    return chunksPosted
}
