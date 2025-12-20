//
//  RustBridge.swift
//  VietnameseIMEFast
//
//  Comprehensive Rust FFI Bridge with GoNhanh mechanisms
//

import Cocoa
import ApplicationServices

// MARK: - Logging System

enum Log {
    static let logPath = URL(fileURLWithPath: NSTemporaryDirectory()).appendingPathComponent("vietnameseime.log")
    static var isEnabled: Bool = false
    
    static func write(_ msg: String) {
        guard isEnabled else { return }
        let timestamp = DateFormatter.localizedString(from: Date(), dateStyle: .none, timeStyle: .medium)
        let line = "[\(timestamp)] \(msg)\n"
        
        if let data = line.data(using: .utf8) {
            if FileManager.default.fileExists(atPath: logPath.path) {
                if let handle = try? FileHandle(forWritingTo: logPath) {
                    handle.seekToEndOfFile()
                    handle.write(data)
                    handle.closeFile()
                }
            } else {
                try? data.write(to: logPath)
            }
        }
    }
    
    static func key(_ code: UInt16, _ result: String) { write("KEY[\(code)] → \(result)") }
    static func transform(_ bs: Int, _ chars: String) { write("TRANSFORM bs=\(bs) chars=\(chars)") }
    static func send(_ method: String, _ bs: Int, _ chars: String) { write("SEND[\(method)] bs=\(bs) chars=\(chars)") }
    static func method(_ name: String) { write("METHOD: \(name)") }
    static func info(_ msg: String) { write("INFO: \(msg)") }
    static func skip() { write("SKIP") }
    static func queue(_ msg: String) { write("QUEUE: \(msg)") }
}

// MARK: - Key Code Constants

enum KeyCode {
    static let backspace: CGKeyCode = 51
    static let forwardDelete: CGKeyCode = 117
    static let leftArrow: CGKeyCode = 123
    static let escape: CGKeyCode = 53
}

// MARK: - Event Marker

private let kEventMarker: Int64 = 0x564E5F494D45 // "VN_IME"

// MARK: - Injection Method

enum InjectionMethod: String {
    case instant        // Modern editors: zero delays for maximum speed
    case fast           // Default: backspace + text with minimal delays
    case slow           // Terminals/Electron: backspace + text with higher delays
    case selection      // Browser address bars: Shift+Left select + type replacement
    case autocomplete   // Spotlight: Forward Delete + backspace + text via proxy
}

// MARK: - Text Injector

class TextInjector {
    static let shared = TextInjector()
    
    private let semaphore = DispatchSemaphore(value: 1)
    
    private init() {}
    
    func injectSync(bs: Int, text: String, method: InjectionMethod, delays: (UInt32, UInt32, UInt32), proxy: CGEventTapProxy) {
        semaphore.wait()
        defer { semaphore.signal() }
        
        Log.send(method.rawValue, bs, text)
        
        switch method {
        case .instant:
            injectViaInstant(bs: bs, text: text, proxy: proxy)
        case .selection:
            injectViaSelection(bs: bs, text: text, delays: delays, proxy: proxy)
        case .autocomplete:
            injectViaAutocomplete(bs: bs, text: text, proxy: proxy)
        case .slow, .fast:
            injectViaBackspace(bs: bs, text: text, delays: delays, proxy: proxy)
        }
        
        // Settle time: 2ms for instant, 5ms for others
        usleep(method == .instant ? 2000 : 5000)
    }
    
    // MARK: - Injection Strategies
    
    /// Instant injection for modern editors with fast text buffers
    /// Zero delays between events for maximum speed
    private func injectViaInstant(bs: Int, text: String, proxy: CGEventTapProxy) {
        guard let src = CGEventSource(stateID: .privateState) else { return }
        
        // Batch backspace events - zero delays for maximum speed
        // Modern editors have fast text buffers and can handle rapid events
        postBackspaces(bs, source: src, proxy: proxy)
        
        // Type replacement text immediately - zero delay
        postText(text, source: src, delay: 0, proxy: proxy)
        Log.send("instant", bs, text)
    }
    
    /// Post multiple backspace events in batch - ZERO delays between events
    /// Based on reference implementation for modern editors (VSCode, Zed, Sublime)
    /// These apps have fast text buffers and can handle rapid consecutive events
    private func postBackspaces(_ count: Int, source: CGEventSource, proxy: CGEventTapProxy) {
        guard count > 0 else { return }
        
        // Send all backspace events consecutively without any delays
        // This reduces event loop overhead and achieves < 16ms latency
        for _ in 0..<count {
            guard let dn = CGEvent(keyboardEventSource: source, virtualKey: KeyCode.backspace, keyDown: true),
                  let up = CGEvent(keyboardEventSource: source, virtualKey: KeyCode.backspace, keyDown: false) 
            else { continue }
            
            dn.setIntegerValueField(.eventSourceUserData, value: Int64(kEventMarker))
            up.setIntegerValueField(.eventSourceUserData, value: Int64(kEventMarker))
            
            // Post immediately - no usleep() calls
            dn.tapPostEvent(proxy)
            up.tapPostEvent(proxy)
        }
    }
    
    private func injectViaBackspace(bs: Int, text: String, delays: (UInt32, UInt32, UInt32), proxy: CGEventTapProxy) {
        guard let src = CGEventSource(stateID: .privateState) else { return }
        
        // Optimize: use batch backspace when no delay needed between keystrokes
        if delays.0 == 0 {
            postBackspaces(bs, source: src, proxy: proxy)
        } else {
            // Send backspaces with delays (for slower apps like terminals)
            for _ in 0..<bs {
                postKey(KeyCode.backspace, source: src, proxy: proxy)
                usleep(delays.0)
            }
        }
        
        // Wait between backspace and text if needed
        if bs > 0 { usleep(delays.1) }
        
        // Type replacement text
        postText(text, source: src, delay: delays.2, proxy: proxy)
        Log.send("bs", bs, text)
    }
    
    private func injectViaSelection(bs: Int, text: String, delays: (UInt32, UInt32, UInt32), proxy: CGEventTapProxy) {
        guard let src = CGEventSource(stateID: .privateState) else { return }
        
        let selDelay = delays.0 > 0 ? delays.0 : 1000
        let waitDelay = delays.1 > 0 ? delays.1 : 3000
        let textDelay = delays.2 > 0 ? delays.2 : 2000
        
        // 1. Select text using Shift + Left Arrow
        for _ in 0..<bs {
            postKey(KeyCode.leftArrow, source: src, flags: .maskShift, proxy: proxy)
            usleep(selDelay)
        }
        
        // 2. Wait for selection to stabilize
        if bs > 0 { usleep(waitDelay) }
        
        // 3. Type replacement (replaces selection)
        postText(text, source: src, delay: textDelay, proxy: proxy)
    }
    
    private func injectViaAutocomplete(bs: Int, text: String, proxy: CGEventTapProxy) {
        guard let src = CGEventSource(stateID: .privateState) else { return }
        
        // 1. Forward Delete to clear auto-selected suggestion
        postKey(KeyCode.forwardDelete, source: src, proxy: proxy)
        usleep(3000)
        
        // 2. Send backspaces
        for _ in 0..<bs {
            postKey(KeyCode.backspace, source: src, proxy: proxy)
            usleep(1000)
        }
        if bs > 0 { usleep(5000) }
        
        // 3. Type replacement text
        postText(text, source: src, proxy: proxy)
    }
    
    // MARK: - Helper Methods
    
    private func postKey(_ keyCode: CGKeyCode, source: CGEventSource, flags: CGEventFlags = [], proxy: CGEventTapProxy? = nil) {
        guard let dn = CGEvent(keyboardEventSource: source, virtualKey: keyCode, keyDown: true),
              let up = CGEvent(keyboardEventSource: source, virtualKey: keyCode, keyDown: false) else { return }
        
        // Mark as injected to avoid re-processing
        dn.setIntegerValueField(.eventSourceUserData, value: kEventMarker)
        up.setIntegerValueField(.eventSourceUserData, value: kEventMarker)
        
        if !flags.isEmpty {
            dn.flags = flags
            up.flags = flags
        }
        
        if let proxy = proxy {
            dn.tapPostEvent(proxy)
            up.tapPostEvent(proxy)
        } else {
            dn.post(tap: .cghidEventTap)
            up.post(tap: .cghidEventTap)
        }
    }
    
    private func postText(_ text: String, source: CGEventSource, delay: UInt32 = 0, proxy: CGEventTapProxy? = nil) {
        let utf16 = Array(text.utf16)
        var offset = 0
        
        // CGEvent has a limit on string length (typically 20 chars)
        while offset < utf16.count {
            let end = min(offset + 20, utf16.count)
            let chunk = Array(utf16[offset..<end])
            
            guard let dn = CGEvent(keyboardEventSource: source, virtualKey: 0, keyDown: true),
                  let up = CGEvent(keyboardEventSource: source, virtualKey: 0, keyDown: false) else { break }
            
            dn.setIntegerValueField(.eventSourceUserData, value: kEventMarker)
            up.setIntegerValueField(.eventSourceUserData, value: kEventMarker)
            
            dn.keyboardSetUnicodeString(stringLength: chunk.count, unicodeString: chunk)
            up.keyboardSetUnicodeString(stringLength: chunk.count, unicodeString: chunk)
            
            if let proxy = proxy {
                dn.tapPostEvent(proxy)
                up.tapPostEvent(proxy)
            } else {
                dn.post(tap: .cghidEventTap)
                up.post(tap: .cghidEventTap)
            }
            
            if delay > 0 { usleep(delay) }
            offset = end
        }
    }
}

// MARK: - Note on API
// The new Rust core uses a global singleton engine (initialized with ime_init)
// All FFI functions (including ImeResult struct) are declared in the bridging header

// MARK: - Rust Bridge

class RustBridge {
    private var isInitialized = false
    
    func initialize() {
        guard !isInitialized else { return }
        ime_init()
        
        // Set default configuration
        ime_method(0)  // 0 = Telex, 1 = VNI
        ime_enabled(true)  // Enable by default
        ime_modern(false)  // Use traditional tone placement
        ime_esc_restore(true)  // Enable ESC restore
        
        isInitialized = true
        Log.info("RustBridge initialized with Telex mode enabled")
    }
    
    func setMethod(_ method: Int) {
        Log.method("Setting input method to \(method)")
        ime_method(UInt8(method))
    }
    
    func setEnabled(_ enabled: Bool) {
        Log.info("IME \(enabled ? "enabled" : "disabled")")
        ime_enabled(enabled)
    }
    
    func setSkipWShortcut(_ skip: Bool) {
        Log.info("Skip W in shortcuts: \(skip)")
        ime_skip_w_shortcut(skip)
    }
    
    func setEscRestore(_ enabled: Bool) {
        Log.info("ESC restore: \(enabled)")
        ime_esc_restore(enabled)
    }
    
    func setFreeTone(_ enabled: Bool) {
        Log.info("Free tone: \(enabled)")
        ime_free_tone(enabled)
    }
    
    func setModernTone(_ modern: Bool) {
        Log.info("Modern tone style: \(modern)")
        ime_modern(modern)
    }
    
    func clearBuffer() {
        ime_clear()
    }
    
    func restoreWord(_ word: String) {
        guard let cString = word.cString(using: .utf8) else { return }
        Log.info("Restore word: \(word)")
        cString.withUnsafeBufferPointer { ptr in
            ime_restore_word(ptr.baseAddress!)
        }
    }
    
    // MARK: - Shortcut Management
    
    func addShortcut(trigger: String, replacement: String) {
        guard let triggerC = trigger.cString(using: .utf8),
              let replacementC = replacement.cString(using: .utf8) else { return }
        Log.info("Add shortcut: \(trigger) → \(replacement)")
        triggerC.withUnsafeBufferPointer { triggerPtr in
            replacementC.withUnsafeBufferPointer { replacementPtr in
                ime_add_shortcut(triggerPtr.baseAddress!, replacementPtr.baseAddress!)
            }
        }
    }
    
    func removeShortcut(trigger: String) {
        guard let triggerC = trigger.cString(using: .utf8) else { return }
        Log.info("Remove shortcut: \(trigger)")
        triggerC.withUnsafeBufferPointer { ptr in
            ime_remove_shortcut(ptr.baseAddress!)
        }
    }
    
    func clearShortcuts() {
        Log.info("Clear all shortcuts")
        ime_clear_shortcuts()
    }
    
    func syncShortcuts(_ shortcuts: [(key: String, value: String, enabled: Bool)]) {
        clearShortcuts()
        for shortcut in shortcuts where shortcut.enabled {
            addShortcut(trigger: shortcut.key, replacement: shortcut.value)
        }
    }
}

// MARK: - Keyboard Hook Manager

class KeyboardHookManager {
    static let shared = KeyboardHookManager()
    
    private var eventTap: CFMachPort?
    private var runLoopSource: CFRunLoopSource?
    private var isRunning = false
    
    private init() {}
    
    func start() {
        guard !isRunning else { return }
        
        let options = [kAXTrustedCheckOptionPrompt.takeUnretainedValue() as String: true]
        let accessEnabled = AXIsProcessTrustedWithOptions(options as CFDictionary)
        
        if !accessEnabled {
            Log.info("Accessibility permission not granted")
            showAccessibilityAlert()
            return
        }
        
        let eventMask = (1 << CGEventType.keyDown.rawValue) |
                        (1 << CGEventType.keyUp.rawValue) |
                        (1 << CGEventType.flagsChanged.rawValue)
        
        guard let tap = CGEvent.tapCreate(
            tap: .cghidEventTap,
            place: .headInsertEventTap,
            options: .defaultTap,
            eventsOfInterest: CGEventMask(eventMask),
            callback: keyboardCallback,
            userInfo: nil
        ) else {
            Log.info("Failed to create event tap")
            return
        }
        
        self.eventTap = tap
        self.runLoopSource = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, tap, 0)
        CFRunLoopAddSource(CFRunLoopGetCurrent(), self.runLoopSource, .commonModes)
        CGEvent.tapEnable(tap: tap, enable: true)
        
        isRunning = true
        Log.info("KeyboardHookManager started")
    }
    
    func stop() {
        guard isRunning else { return }
        
        if let runLoopSource = self.runLoopSource {
            CFRunLoopRemoveSource(CFRunLoopGetCurrent(), runLoopSource, .commonModes)
            self.runLoopSource = nil
        }
        
        if let eventTap = self.eventTap {
            CGEvent.tapEnable(tap: eventTap, enable: false)
            self.eventTap = nil
        }
        
        isRunning = false
        Log.info("KeyboardHookManager stopped")
    }
    
    func getTap() -> CFMachPort? {
        return eventTap
    }
    
    func showAccessibilityAlert() {
        DispatchQueue.main.async {
            let alert = NSAlert()
            alert.messageText = "Accessibility Permission Required"
            alert.informativeText = "VietnameseIMEFast needs Accessibility permission to function.\n\nPlease grant permission in System Settings → Privacy & Security → Accessibility."
            alert.alertStyle = .warning
            alert.addButton(withTitle: "Open System Settings")
            alert.addButton(withTitle: "Cancel")
            
            if alert.runModal() == .alertFirstButtonReturn {
                NSWorkspace.shared.open(URL(string: "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")!)
            }
        }
    }
}

// MARK: - Word Restoration on Backspace

func getWordToRestoreOnBackspace() -> String? {
    // Get the currently focused UI element
    let systemWide = AXUIElementCreateSystemWide()
    var focusedElement: CFTypeRef?
    
    guard AXUIElementCopyAttributeValue(systemWide, kAXFocusedUIElementAttribute as CFString, &focusedElement) == .success,
          let element = focusedElement else {
        return nil
    }
    
    let axElement = element as! AXUIElement
    
    // Try to get selected text range
    var rangeValue: CFTypeRef?
    guard AXUIElementCopyAttributeValue(axElement, kAXSelectedTextRangeAttribute as CFString, &rangeValue) == .success,
          let range = rangeValue else {
        return nil
    }
    
    var rangeStruct = CFRange()
    AXValueGetValue(range as! AXValue, .cfRange, &rangeStruct)
    
    // If there's a selection, get the selected text
    if rangeStruct.length > 0 {
        var selectedText: CFTypeRef?
        AXUIElementCopyParameterizedAttributeValue(axElement, kAXStringForRangeParameterizedAttribute as CFString, range as! AXValue, &selectedText)
        return selectedText as? String
    }
    
    // Otherwise, try to get the word before the cursor
    if rangeStruct.location > 0 {
        // Get text in a range before cursor
        let lookbackLength = min(rangeStruct.location, 20)
        var lookbackRange = CFRangeMake(rangeStruct.location - lookbackLength, lookbackLength)
        let lookbackValue = AXValueCreate(.cfRange, &lookbackRange)!
        
        var textBefore: CFTypeRef?
        AXUIElementCopyParameterizedAttributeValue(axElement, kAXStringForRangeParameterizedAttribute as CFString, lookbackValue, &textBefore)
        
        if let text = textBefore as? String {
            // Find the last word (sequence of letters)
            let words = text.components(separatedBy: CharacterSet.letters.inverted)
            return words.last?.trimmingCharacters(in: .whitespaces)
        }
    }
    
    return nil
}

// MARK: - CGEventFlags Extension

extension CGEventFlags {
    var modifierCount: Int {
        var count = 0
        if contains(.maskCommand) { count += 1 }
        if contains(.maskControl) { count += 1 }
        if contains(.maskAlternate) { count += 1 }
        if contains(.maskShift) { count += 1 }
        return count
    }
}

// MARK: - Shortcut Recording

private var isRecordingShortcut = false

func startShortcutRecording() {
    isRecordingShortcut = true
    Log.info("Started shortcut recording")
}

func stopShortcutRecording() {
    isRecordingShortcut = false
    Log.info("Stopped shortcut recording")
}

func setupShortcutObserver() {
    // This would be called from the settings UI to enable shortcut recording
    // The recorded shortcut would be posted via NotificationCenter
}

func matchesToggleShortcut(keyCode: UInt16, flags: CGEventFlags) -> Bool {
    // Load current shortcut configuration (default: Control+Space)
    let currentShortcut = KeyboardShortcut.load()
    return currentShortcut.matches(keyCode: keyCode, flags: flags)
}

func matchesModifierOnlyShortcut(flags: CGEventFlags) -> Bool {
    // Check if current shortcut is modifier-only (e.g., double-tap Shift)
    let currentShortcut = KeyboardShortcut.load()
    guard currentShortcut.isModifierOnly else { return false }
    
    let savedFlags = CGEventFlags(rawValue: currentShortcut.modifiers)
    let kModifierMask: CGEventFlags = [.maskControl, .maskAlternate, .maskShift, .maskCommand]
    return flags.intersection(kModifierMask) == savedFlags.intersection(kModifierMask)
}

// MARK: - Keyboard Event Callback

func keyboardCallback(
    proxy: CGEventTapProxy,
    type: CGEventType,
    event: CGEvent,
    refcon: UnsafeMutableRawPointer?
) -> Unmanaged<CGEvent>? {
    // This callback should be connected to InputManager
    // For now, just pass through
    return Unmanaged.passUnretained(event)
}

// MARK: - App Detection for Injection Method

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
    
    // Browser address bars (AXTextField with autocomplete)
    let browsers = [
        // Chromium-based
        "com.google.Chrome",             // Google Chrome
        "com.google.Chrome.canary",      // Chrome Canary
        "com.google.Chrome.beta",        // Chrome Beta
        "org.chromium.Chromium",         // Chromium
        "com.brave.Browser",             // Brave
        "com.brave.Browser.beta",        // Brave Beta
        "com.brave.Browser.nightly",     // Brave Nightly
        "com.microsoft.edgemac",         // Microsoft Edge
        "com.microsoft.edgemac.Beta",    // Edge Beta
        "com.microsoft.edgemac.Dev",     // Edge Dev
        "com.microsoft.edgemac.Canary",  // Edge Canary
        "com.vivaldi.Vivaldi",           // Vivaldi
        "com.vivaldi.Vivaldi.snapshot",  // Vivaldi Snapshot
        "ru.yandex.desktop.yandex-browser", // Yandex Browser
        // Opera
        "com.opera.Opera",               // Opera
        "com.operasoftware.Opera",       // Opera (alt)
        "com.operasoftware.OperaGX",     // Opera GX
        "com.operasoftware.OperaAir",    // Opera Air
        "com.opera.OperaNext",           // Opera Next
        // Firefox-based
        "org.mozilla.firefox",           // Firefox
        "org.mozilla.firefoxdeveloperedition", // Firefox Developer
        "org.mozilla.nightly",           // Firefox Nightly
        "org.waterfoxproject.waterfox",  // Waterfox
        "io.gitlab.librewolf-community.librewolf", // LibreWolf
        "one.ablaze.floorp",             // Floorp
        "org.torproject.torbrowser",     // Tor Browser
        "net.mullvad.mullvadbrowser",    // Mullvad Browser
        // Safari
        "com.apple.Safari",              // Safari
        "com.apple.SafariTechnologyPreview", // Safari Tech Preview
        // WebKit-based
        "com.kagi.kagimacOS",            // Orion (Kagi)
        // Arc & Others
        "company.thebrowser.Browser",    // The Browser Company
        "company.thebrowser.Arc",        // Arc
        "company.thebrowser.dia",        // Dia (The Browser Company)
        "app.zen-browser.zen",           // Zen Browser
        "com.sigmaos.sigmaos.macos",     // SigmaOS
        "com.pushplaylabs.sidekick",     // Sidekick
        "com.firstversionist.polypane",  // Polypane
        "ai.perplexity.comet",           // Comet (Perplexity AI)
        "com.duckduckgo.macos.browser"   // DuckDuckGo
    ]
    if browsers.contains(bundleId) && role == "AXTextField" { Log.method("sel:browser"); return (.selection, (0, 0, 0)) }
    if role == "AXTextField" && bundleId.hasPrefix("com.jetbrains") { Log.method("sel:jb"); return (.selection, (0, 0, 0)) }
    
    // Microsoft Office apps - use backspace method instead of selection
    // Selection method conflicts with Office's autocomplete/suggestion features
    // which can cause the first character to be lost
    if bundleId == "com.microsoft.Excel" { Log.method("slow:excel"); return (.slow, (3000, 8000, 3000)) }
    if bundleId == "com.microsoft.Word" { Log.method("slow:word"); return (.slow, (3000, 8000, 3000)) }
    
    // Electron apps - higher delays for reliable text replacement
    if bundleId == "com.todesktop.230313mzl4w4u92" { Log.method("slow:claude"); return (.slow, (8000, 15000, 8000)) }
    if bundleId == "notion.id" { Log.method("slow:notion"); return (.slow, (8000, 15000, 8000)) }
    
    // Modern editors - instant method with zero delays for maximum speed
    // These apps have fast text buffers and rendering, no need for delays
    let modernEditors = [
        // Code Editors (fast text buffer)
        "com.microsoft.VSCode",          // Visual Studio Code
        "dev.zed.Zed",                   // Zed
        "com.sublimetext.4",             // Sublime Text 4
        "com.sublimetext.3",             // Sublime Text 3
        "com.panic.Nova",                // Nova
        "com.github.atom",               // Atom (if still used)
        "com.github.GitHubClient",       // GitHub Desktop editor
        "com.coteditor.CotEditor",       // CotEditor
        "com.microsoft.VSCodeInsiders",  // VSCode Insiders
        "com.vscodium",                  // VSCodium
        "dev.zed.preview"                // Zed Preview
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

// MARK: - Send Replacement Helper

func sendReplacement(backspace bs: Int, chars: [Character], proxy: CGEventTapProxy) {
    let text = String(chars)
    let (method, delays) = detectMethod()
    TextInjector.shared.injectSync(bs: bs, text: text, method: method, delays: delays, proxy: proxy)
}

// MARK: - Per-App Mode Manager

// MARK: - Per-App Mode Manager and Notifications
// Moved to separate files: PerAppModeManager.swift and AppState.swift
