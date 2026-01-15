//
//  InputSourceMonitor.swift
//  GoxViet
//
//  Monitors keyboard input source changes to automatically disable Vietnamese typing
//  when non-Latin input sources (Japanese, Korean, Chinese, etc.) are active.
//
//  This prevents unintended Vietnamese transformations when typing in other languages.
//

import Foundation
import Carbon
import Cocoa

/// Monitors keyboard input source changes and automatically disables Vietnamese typing
/// when non-Latin input methods are active (e.g., Japanese, Korean, Chinese)
class InputSourceMonitor {
    static let shared = InputSourceMonitor()
    
    // MARK: - Properties
    
    /// Whether the monitor is currently running
    private(set) var isRunning: Bool = false
    
    /// Whether Vietnamese input is temporarily disabled due to non-Latin input source
    private(set) var isTemporarilyDisabled: Bool = false
    
    /// The state before temporary disable (to restore when switching back to Latin)
    private var stateBeforeDisable: Bool = true
    
    /// Current input source ID
    private(set) var currentInputSourceId: String?
    
    /// Distributed notification observer
    private var observer: NSObjectProtocol?
    
    // MARK: - Input Source Categories
    
    /// Input source IDs that should trigger auto-disable
    /// These are non-Latin IME systems where Vietnamese typing would interfere
    private static let nonLatinInputSourcePrefixes: [String] = [
        // Japanese
        "com.apple.inputmethod.Kotoeri",
        "com.apple.inputmethod.Japanese",
        "jp.sourceforge.inputmethod.aquaskk",
        "com.google.inputmethod.Japanese",
        
        // Korean
        "com.apple.inputmethod.Korean",
        "com.apple.keylayout.2SetKorean",
        "com.apple.keylayout.3SetKorean",
        "com.apple.keylayout.390Sebulshik",
        "com.apple.keylayout.GongjinCheongRomaja",
        
        // Chinese (Simplified)
        "com.apple.inputmethod.SCIM",
        "com.apple.inputmethod.Pinyin",
        "com.apple.inputmethod.ChinaHandwriting",
        "com.sogou.inputmethod",
        "com.baidu.inputmethod",
        "com.iflytek.inputmethod",
        "com.tencent.inputmethod.QQInput",
        
        // Chinese (Traditional)
        "com.apple.inputmethod.TCIM",
        "com.apple.inputmethod.Zhuyin",
        "com.apple.inputmethod.Cangjie",
        "com.apple.inputmethod.TraditionalChinese",
        
        // Thai
        "com.apple.keylayout.Thai",
        "com.apple.inputmethod.Thai",
        
        // Arabic
        "com.apple.keylayout.Arabic",
        "com.apple.inputmethod.Arabic",
        
        // Hebrew
        "com.apple.keylayout.Hebrew",
        
        // Russian and Cyrillic
        "com.apple.keylayout.Russian",
        "com.apple.keylayout.Ukrainian",
        "com.apple.keylayout.Bulgarian",
        
        // Greek
        "com.apple.keylayout.Greek",
        
        // Hindi and Indic
        "com.apple.keylayout.Devanagari",
        "com.apple.inputmethod.Hindi",
        "com.apple.inputmethod.Tamil",
        "com.apple.inputmethod.Telugu",
        "com.apple.inputmethod.Kannada",
        "com.apple.inputmethod.Malayalam",
        "com.apple.inputmethod.Gujarati",
        "com.apple.inputmethod.Punjabi",
        "com.apple.inputmethod.Bengali",
        
        // Other non-Latin
        "com.apple.inputmethod.Georgian",
        "com.apple.inputmethod.Armenian"
    ]
    
    /// Input source IDs that are Latin-based and should NOT trigger auto-disable
    /// When switching to these, Vietnamese will be re-enabled (restored to previous state)
    private static let latinInputSourcePrefixes: [String] = [
        "com.apple.keylayout.ABC",
        "com.apple.keylayout.US",
        "com.apple.keylayout.USExtended",
        "com.apple.keylayout.USInternational",
        "com.apple.keylayout.British",
        "com.apple.keylayout.Australian",
        "com.apple.keylayout.Canadian",
        "com.apple.keylayout.Irish",
        "com.apple.keylayout.French",
        "com.apple.keylayout.German",
        "com.apple.keylayout.Spanish",
        "com.apple.keylayout.Italian",
        "com.apple.keylayout.Portuguese",
        "com.apple.keylayout.Dutch",
        "com.apple.keylayout.Swedish",
        "com.apple.keylayout.Norwegian",
        "com.apple.keylayout.Danish",
        "com.apple.keylayout.Finnish",
        "com.apple.keylayout.Polish",
        "com.apple.keylayout.Czech",
        "com.apple.keylayout.Slovak",
        "com.apple.keylayout.Hungarian",
        "com.apple.keylayout.Romanian",
        "com.apple.keylayout.Croatian",
        "com.apple.keylayout.Slovenian",
        "com.apple.keylayout.Turkish",
        "com.apple.keylayout.Estonian",
        "com.apple.keylayout.Latvian",
        "com.apple.keylayout.Lithuanian",
        "com.apple.keylayout.Vietnamese",
        "com.apple.keylayout.VietnameseSimpleTelex",
        // Generic Latin
        "com.apple.keylayout"
    ]
    
    // MARK: - Initialization
    
    private init() {}
    
    // MARK: - Lifecycle
    
    /// Start monitoring input source changes
    func start() {
        guard !isRunning else {
            Log.info("InputSourceMonitor already running")
            return
        }
        
        // Get initial input source
        updateCurrentInputSource()
        
        // Register for input source change notifications
        // Using DistributedNotificationCenter to receive system-wide notifications
        observer = DistributedNotificationCenter.default().addObserver(
            forName: NSNotification.Name(kTISNotifySelectedKeyboardInputSourceChanged as String),
            object: nil,
            queue: .main
        ) { [weak self] _ in
            self?.handleInputSourceChanged()
        }
        
        isRunning = true
        Log.info("InputSourceMonitor started (current: \(currentInputSourceId ?? "unknown"))")
    }
    
    /// Stop monitoring input source changes
    func stop() {
        guard isRunning else {
            return
        }
        
        if let observer = observer {
            DistributedNotificationCenter.default().removeObserver(observer)
            self.observer = nil
        }
        
        // Restore state if temporarily disabled
        if isTemporarilyDisabled {
            restoreState()
        }
        
        isRunning = false
        currentInputSourceId = nil
        Log.info("InputSourceMonitor stopped")
    }
    
    // MARK: - Input Source Detection
    
    /// Update the current input source ID
    private func updateCurrentInputSource() {
        guard let inputSource = TISCopyCurrentKeyboardInputSource()?.takeRetainedValue() else {
            Log.warning("Failed to get current keyboard input source")
            return
        }
        
        if let inputSourceIdPtr = TISGetInputSourceProperty(inputSource, kTISPropertyInputSourceID) {
            let inputSourceId = Unmanaged<CFString>.fromOpaque(inputSourceIdPtr).takeUnretainedValue() as String
            currentInputSourceId = inputSourceId
        }
    }
    
    /// Handle input source change notification
    private func handleInputSourceChanged() {
        let previousInputSourceId = currentInputSourceId
        updateCurrentInputSource()
        
        guard let inputSourceId = currentInputSourceId else {
            return
        }
        
        // Only log if actually changed
        guard inputSourceId != previousInputSourceId else {
            return
        }
        
        Log.info("Input source changed: \(inputSourceId)")
        
        // Check if auto-disable is enabled
        guard AppState.shared.autoDisableForNonLatinEnabled else {
            return
        }
        
        // Check if this is a non-Latin input source
        if isNonLatinInputSource(inputSourceId) {
            // Non-Latin input source detected - disable Vietnamese temporarily
            if !isTemporarilyDisabled {
                temporarilyDisable()
            }
        } else if isLatinInputSource(inputSourceId) {
            // Latin input source detected - restore Vietnamese if was temporarily disabled
            if isTemporarilyDisabled {
                restoreState()
            }
        }
        // Note: If input source is neither explicitly non-Latin nor Latin, 
        // we keep the current state (conservative approach)
    }
    
    /// Check if the input source is a non-Latin IME
    private func isNonLatinInputSource(_ inputSourceId: String) -> Bool {
        return Self.nonLatinInputSourcePrefixes.contains { prefix in
            inputSourceId.hasPrefix(prefix)
        }
    }
    
    /// Check if the input source is Latin-based
    private func isLatinInputSource(_ inputSourceId: String) -> Bool {
        // Special case: generic keylayout without specific language is usually Latin
        if inputSourceId.hasPrefix("com.apple.keylayout.") {
            // If not in non-Latin list, assume Latin
            return !isNonLatinInputSource(inputSourceId)
        }
        
        return Self.latinInputSourcePrefixes.contains { prefix in
            inputSourceId.hasPrefix(prefix)
        }
    }
    
    // MARK: - State Management
    
    /// Temporarily disable Vietnamese input
    private func temporarilyDisable() {
        // Save current state
        stateBeforeDisable = AppState.shared.isEnabled
        
        // Only disable if currently enabled
        if stateBeforeDisable {
            isTemporarilyDisabled = true
            
            // Update Rust engine but don't change AppState.isEnabled
            // This way the menu bar icon still shows the "intended" state
            ime_enabled(false)
            
            // Clear buffer
            ime_clear()
            
            Log.info("Vietnamese input temporarily disabled (non-Latin input source active)")
            
            // Post notification for UI update - defer to avoid layout recursion
            DispatchQueue.main.async {
                NotificationCenter.default.post(
                    name: .inputSourceAutoDisabled,
                    object: self.currentInputSourceId
                )
            }
        }
    }
    
    /// Restore Vietnamese input state
    private func restoreState() {
        guard isTemporarilyDisabled else {
            return
        }
        
        isTemporarilyDisabled = false
        
        // Restore the state that was active before temporary disable
        ime_enabled(stateBeforeDisable)
        
        Log.info("Vietnamese input restored (Latin input source active)")
        
        // Post notification for UI update - defer to avoid layout recursion
        DispatchQueue.main.async {
            NotificationCenter.default.post(
                name: .inputSourceAutoRestored,
                object: nil
            )
        }
    }
    
    // MARK: - Public API
    
    /// Check if Vietnamese processing should be skipped
    /// This is called by InputManager to determine if key processing should happen
    func shouldSkipVietnameseProcessing() -> Bool {
        // Only skip if auto-disable is enabled AND currently temporarily disabled
        return AppState.shared.autoDisableForNonLatinEnabled && isTemporarilyDisabled
    }
    
    /// Get display name for current input source
    func getCurrentInputSourceDisplayName() -> String? {
        guard let inputSource = TISCopyCurrentKeyboardInputSource()?.takeRetainedValue() else {
            return nil
        }
        
        if let namePtr = TISGetInputSourceProperty(inputSource, kTISPropertyLocalizedName) {
            return Unmanaged<CFString>.fromOpaque(namePtr).takeUnretainedValue() as String
        }
        
        return currentInputSourceId
    }
    
    /// Force refresh the current state
    func refresh() {
        updateCurrentInputSource()
        handleInputSourceChanged()
    }
}

// MARK: - Notification Names

extension Notification.Name {
    /// Posted when Vietnamese input is automatically disabled due to non-Latin input source
    static let inputSourceAutoDisabled = Notification.Name("com.goxviet.ime.inputSourceAutoDisabled")
    
    /// Posted when Vietnamese input is automatically restored after switching back to Latin input source
    static let inputSourceAutoRestored = Notification.Name("com.goxviet.ime.inputSourceAutoRestored")
}