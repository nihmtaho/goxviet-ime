//
//  RustBridgeSafe.swift
//  GoxViet
//
//  Memory-safe Swift bridge to Rust core IME engine
//  Enhanced with proper error handling, memory management, and thread safety
//

import Cocoa
import ApplicationServices

// MARK: - Safe Rust Bridge Class

/// Thread-safe wrapper around Rust FFI with comprehensive error handling
final class RustBridgeSafe {
    // Singleton for thread-safe access
    static let shared = RustBridgeSafe()
    
    // MARK: - Properties
    
    private var isInitialized = false
    private let initLock = NSLock()
    private let ffiLock = NSRecursiveLock()
    
    // Track initialization status
    private(set) var initializationError: RustBridgeError?
    
    // MARK: - Lifecycle
    
    private init() {
        // Private initializer for singleton
    }
    
    deinit {
        Log.info("RustBridgeSafe deinit - cleaning up resources")
        // No explicit cleanup needed as Rust owns the global state
        // But we log for debugging
    }
    
    // MARK: - Initialization
    
    /// Initialize the Rust IME engine
    /// - Returns: Result indicating success or failure
    @discardableResult
    func initialize() -> RustBridgeResult<Void> {
        initLock.lock()
        defer { initLock.unlock() }
        
        guard !isInitialized else {
            Log.info("RustBridge already initialized")
            return .success(())
        }
        
        do {
            // Call FFI init
            ime_init()
            
            // Set default configuration with error handling
            try setMethodUnsafe(0)  // Telex
            try setEnabledUnsafe(true)
            try setModernToneUnsafe(false)
            try setEscRestoreUnsafe(true)
            try setInstantRestoreUnsafe(true)
            
            isInitialized = true
            Log.info("RustBridge initialized successfully with Telex mode")
            return .success(())
            
        } catch let error as RustBridgeError {
            initializationError = error
            Log.error("RustBridge initialization failed: \(error.localizedDescription)")
            return .failure(error)
        } catch {
            let bridgeError = RustBridgeError.ffiCallFailed("Unknown error during initialization")
            initializationError = bridgeError
            Log.error("RustBridge initialization failed: \(error)")
            return .failure(bridgeError)
        }
    }
    
    // MARK: - Configuration (Public Safe API)
    
    func setMethod(_ method: Int) -> RustBridgeResult<Void> {
        return performFFICall("setMethod") {
            try setMethodUnsafe(method)
        }
    }
    
    func setEnabled(_ enabled: Bool) -> RustBridgeResult<Void> {
        return performFFICall("setEnabled") {
            try setEnabledUnsafe(enabled)
        }
    }
    
    func setSkipWShortcut(_ skip: Bool) -> RustBridgeResult<Void> {
        return performFFICall("setSkipWShortcut") {
            ime_skip_w_shortcut(skip)
            Log.info("Skip W in shortcuts: \(skip)")
        }
    }
    
    func setEscRestore(_ enabled: Bool) -> RustBridgeResult<Void> {
        return performFFICall("setEscRestore") {
            try setEscRestoreUnsafe(enabled)
        }
    }
    
    func setFreeTone(_ enabled: Bool) -> RustBridgeResult<Void> {
        return performFFICall("setFreeTone") {
            ime_free_tone(enabled)
            Log.info("Free tone: \(enabled)")
        }
    }
    
    func setModernTone(_ modern: Bool) -> RustBridgeResult<Void> {
        return performFFICall("setModernTone") {
            try setModernToneUnsafe(modern)
        }
    }
    
    func setInstantRestore(_ enabled: Bool) -> RustBridgeResult<Void> {
        return performFFICall("setInstantRestore") {
            try setInstantRestoreUnsafe(enabled)
        }
    }

    func setShortcutsEnabled(_ enabled: Bool) -> RustBridgeResult<Void> {
        return performFFICall("setShortcutsEnabled") {
            ime_set_shortcuts_enabled(enabled)
            Log.info("Text expansion \(enabled ? "enabled" : "disabled")")
        }
    }

    func clearBuffer() -> RustBridgeResult<Void> {
        return performFFICall("clearBuffer") {
            ime_clear()
        }
    }
    
    func clearAll() -> RustBridgeResult<Void> {
        return performFFICall("clearAll") {
            ime_clear_all()
        }
    }
    
    func restoreWord(_ word: String) -> RustBridgeResult<Void> {
        return performFFICall("restoreWord") {
            guard !word.isEmpty else {
                throw RustBridgeError.invalidParameter("word cannot be empty")
            }
            
            guard let cString = word.cString(using: .utf8) else {
                throw RustBridgeError.stringEncodingFailed
            }
            
            cString.withUnsafeBufferPointer { ptr in
                guard let baseAddress = ptr.baseAddress else { return }
                ime_restore_word(baseAddress)
            }
            
            Log.info("Restored word: \(word)")
        }
    }
    
    // MARK: - Shortcut Management (Public Safe API)
    
    func addShortcut(trigger: String, replacement: String) -> RustBridgeResult<Void> {
        return performFFICall("addShortcut") {
            guard !trigger.isEmpty else {
                throw RustBridgeError.invalidParameter("trigger cannot be empty")
            }
            
            guard !replacement.isEmpty else {
                throw RustBridgeError.invalidParameter("replacement cannot be empty")
            }
            
            guard let triggerC = trigger.cString(using: .utf8),
                  let replacementC = replacement.cString(using: .utf8) else {
                throw RustBridgeError.stringEncodingFailed
            }
            
            triggerC.withUnsafeBufferPointer { triggerPtr in
                replacementC.withUnsafeBufferPointer { replacementPtr in
                    guard let triggerAddr = triggerPtr.baseAddress,
                          let replacementAddr = replacementPtr.baseAddress else { return }
                    ime_add_shortcut(triggerAddr, replacementAddr)
                }
            }
            
            Log.info("Added shortcut: \(trigger) → \(replacement)")
        }
    }
    
    func removeShortcut(trigger: String) -> RustBridgeResult<Void> {
        return performFFICall("removeShortcut") {
            guard !trigger.isEmpty else {
                throw RustBridgeError.invalidParameter("trigger cannot be empty")
            }
            
            guard let triggerC = trigger.cString(using: .utf8) else {
                throw RustBridgeError.stringEncodingFailed
            }
            
            triggerC.withUnsafeBufferPointer { ptr in
                guard let baseAddress = ptr.baseAddress else { return }
                ime_remove_shortcut(baseAddress)
            }
            
            Log.info("Removed shortcut: \(trigger)")
        }
    }
    
    func clearShortcuts() -> RustBridgeResult<Void> {
        return performFFICall("clearShortcuts") {
            ime_clear_shortcuts()
            Log.info("Cleared all shortcuts")
        }
    }
    
    func syncShortcuts(_ shortcuts: [(key: String, value: String, enabled: Bool)]) -> RustBridgeResult<Void> {
        return performFFICall("syncShortcuts") {
            // Clear first
            ime_clear_shortcuts()
            
            // Add enabled shortcuts
            for shortcut in shortcuts where shortcut.enabled {
                guard let triggerC = shortcut.key.cString(using: .utf8),
                      let replacementC = shortcut.value.cString(using: .utf8) else {
                    Log.warning("Failed to encode shortcut: \(shortcut.key)")
                    continue
                }
                
                triggerC.withUnsafeBufferPointer { triggerPtr in
                    replacementC.withUnsafeBufferPointer { replacementPtr in
                        guard let triggerAddr = triggerPtr.baseAddress,
                              let replacementAddr = replacementPtr.baseAddress else { return }
                        ime_add_shortcut(triggerAddr, replacementAddr)
                    }
                }
            }
            
            Log.info("Synced \(shortcuts.filter { $0.enabled }.count) shortcuts")
        }
    }
    
    // MARK: - Key Processing (Public Safe API)
    
    func processKey(_ key: UInt16, caps: Bool, ctrl: Bool, shift: Bool) -> RustBridgeResult<ImeResult> {
        return performFFICall("processKey") {
            let resultPtr = ime_key_ext(key, caps, ctrl, shift)
            
            guard let result = resultPtr else {
                throw RustBridgeError.resultIsNull
            }
            
            // Validate result
            guard result.pointee.count <= result.pointee.capacity else {
                // Free invalid result
                ime_free(result)
                throw RustBridgeError.invalidResult
            }
            
            // Copy result to avoid memory issues
            let imeResult = result.pointee
            
            // CRITICAL: Free the Rust-allocated memory immediately
            ime_free(result)
            
            return imeResult
        }
    }
    
    // MARK: - Private Unsafe Helpers
    
    private func setMethodUnsafe(_ method: Int) throws {
        guard method == 0 || method == 1 else {
            throw RustBridgeError.invalidParameter("method must be 0 (Telex) or 1 (VNI)")
        }
        ime_method(UInt8(method))
        Log.method("Set input method to \(method == 0 ? "Telex" : "VNI")")
    }
    
    private func setEnabledUnsafe(_ enabled: Bool) throws {
        ime_enabled(enabled)
        Log.info("IME \(enabled ? "enabled" : "disabled")")
    }
    
    private func setEscRestoreUnsafe(_ enabled: Bool) throws {
        ime_esc_restore(enabled)
        Log.info("ESC restore: \(enabled)")
    }
    
    private func setModernToneUnsafe(_ modern: Bool) throws {
        ime_modern(modern)
        Log.info("Modern tone style: \(modern)")
    }
    
    private func setInstantRestoreUnsafe(_ enabled: Bool) throws {
        ime_instant_restore(enabled)
        Log.info("Instant auto-restore: \(enabled)")
    }
    
    // MARK: - Thread Safety Wrapper
    
    /// Execute FFI call with proper error handling and thread safety
    private func performFFICall<T>(_ functionName: String, _ block: () throws -> T) -> RustBridgeResult<T> {
        // Check initialization first (no lock needed for read)
        guard isInitialized else {
            let error = RustBridgeError.notInitialized
            Log.error("\(functionName) called before initialization")
            return .failure(error)
        }
        
        // Lock for FFI call
        ffiLock.lock()
        defer { ffiLock.unlock() }
        
        do {
            let result = try block()
            return .success(result)
        } catch let error as RustBridgeError {
            Log.error("\(functionName) failed: \(error.localizedDescription)")
            return .failure(error)
        } catch {
            let bridgeError = RustBridgeError.ffiCallFailed(functionName)
            Log.error("\(functionName) failed: \(error)")
            return .failure(bridgeError)
        }
    }
}

// MARK: - Convenience Extensions

extension RustBridgeSafe {
    /// Convenience method that logs errors but doesn't throw
    @discardableResult
    func setMethodLogged(_ method: Int) -> Bool {
        switch setMethod(method) {
        case .success:
            return true
        case .failure(let error):
            Log.error("setMethod failed: \(error.localizedDescription)")
            return false
        }
    }
    
    /// Check if bridge is ready to use
    var isReady: Bool {
        return isInitialized && initializationError == nil
    }
}
