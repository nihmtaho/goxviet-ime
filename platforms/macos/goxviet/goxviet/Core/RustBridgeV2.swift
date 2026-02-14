//
//  RustBridgeV2.swift
//  GoxViet
//
//  v2 FFI Bridge - Clean Architecture API
//  Uses out parameter pattern for Swift ABI safety
//

import Cocoa
import Foundation

// MARK: - FFI v2 Types (matching Rust)

typealias FfiEnginePtr = UnsafeMutableRawPointer

enum FfiStatusCode: Int32 {
    case success = 0               // Success
    case errorNullEngine = -1      // Engine pointer is null
    case errorNullOutput = -2      // Output pointer is null
    case errorNullConfig = -3      // Config pointer is null
    case errorInvalidKey = -4      // Invalid key character
    case errorInvalidArgument = -5 // Invalid argument (generic)
    case errorProcessingFailed = -10 // Processing failed
    case errorInvalidUtf8 = -11    // Invalid UTF-8 encoding
    case errorOutOfMemory = -20    // Out of memory
    case errorAlreadyExists = -30  // Shortcut already exists
    case errorNotFound = -31       // Shortcut not found
    case errorUnknown = -98        // Unknown/generic error
    case errorPanic = -99          // Rust panic caught
}

// IMPORTANT: Must use struct (not enum) for FFI types!
// Swift enum stores a 1-byte discriminator, NOT the Int32 raw value.
// This would make FfiConfig_v2 = 3 bytes instead of the 12 bytes Rust expects.
struct FfiInputMethod: Equatable {
    let rawValue: Int32
    init(rawValue: Int32) { self.rawValue = rawValue }
    static let telex = FfiInputMethod(rawValue: 0)
    static let vni = FfiInputMethod(rawValue: 1)
}

struct FfiToneStyle: Equatable {
    let rawValue: Int32
    init(rawValue: Int32) { self.rawValue = rawValue }
    static let traditional = FfiToneStyle(rawValue: 0)  // Old style: hòa
    static let modern = FfiToneStyle(rawValue: 1)        // New style: hoà
}

struct FfiConfig_v2 {
    var input_method: FfiInputMethod
    var tone_style: FfiToneStyle
    var smart_mode: Bool
    var instant_restore_enabled: Bool
    var esc_restore_enabled: Bool
    var enable_shortcuts: Bool
}

struct FfiProcessResult_v2 {
    var text: UnsafeMutablePointer<CChar>?
    var backspace_count: UInt8
    var consumed: Bool
}

struct FfiVersionInfo {
    var major: UInt32
    var minor: UInt32
    var patch: UInt32
    var api_version: UInt32
}

// MARK: - FFI v2 Functions

@_silgen_name("ime_create_engine_v2")
func ime_create_engine_v2(_ config: UnsafePointer<FfiConfig_v2>?) -> FfiEnginePtr?

@_silgen_name("ime_destroy_engine_v2")
func ime_destroy_engine_v2(_ engine: FfiEnginePtr?)

@_silgen_name("ime_process_key_v2")
func ime_process_key_v2(_ engine: FfiEnginePtr?, _ key: CChar, _ out: UnsafeMutablePointer<FfiProcessResult_v2>) -> Int32

@_silgen_name("ime_process_key_ext_v2")
func ime_process_key_ext_v2(_ engine: FfiEnginePtr?, _ key: CChar, _ caps: Bool, _ shift: Bool, _ ctrl: Bool, _ out: UnsafeMutablePointer<FfiProcessResult_v2>) -> Int32

@_silgen_name("ime_get_config_v2")
func ime_get_config_v2(_ engine: FfiEnginePtr?, _ out: UnsafeMutablePointer<FfiConfig_v2>) -> Int32

@_silgen_name("ime_set_config_v2")
func ime_set_config_v2(_ engine: FfiEnginePtr?, _ config: UnsafePointer<FfiConfig_v2>) -> Int32

@_silgen_name("ime_get_version_v2")
func ime_get_version_v2(_ out: UnsafeMutablePointer<FfiVersionInfo>) -> Int32

@_silgen_name("ime_free_string_v2")
func ime_free_string_v2(_ s: UnsafeMutablePointer<CChar>?)

// MARK: - Shortcut Management FFI Functions

@_silgen_name("ime_add_shortcut_v2")
func ime_add_shortcut_v2(_ engine: FfiEnginePtr?, _ trigger: UnsafePointer<CChar>?, _ expansion: UnsafePointer<CChar>?) -> Int32

@_silgen_name("ime_remove_shortcut_v2")
func ime_remove_shortcut_v2(_ engine: FfiEnginePtr?, _ trigger: UnsafePointer<CChar>?) -> Int32

@_silgen_name("ime_clear_shortcuts_v2")
func ime_clear_shortcuts_v2(_ engine: FfiEnginePtr?) -> Int32

@_silgen_name("ime_shortcuts_count_v2")
func ime_shortcuts_count_v2(_ engine: FfiEnginePtr?) -> Int32

@_silgen_name("ime_set_shortcuts_enabled_v2")
func ime_set_shortcuts_enabled_v2(_ engine: FfiEnginePtr?, _ enabled: Bool) -> Int32

@_silgen_name("ime_restore_to_raw_v2")
func ime_restore_to_raw_v2(_ engine: FfiEnginePtr?, _ out: UnsafeMutablePointer<FfiProcessResult_v2>) -> Int32

@_silgen_name("ime_reset_buffer_v2")
func ime_reset_buffer_v2(_ engine: FfiEnginePtr?) -> Int32

@_silgen_name("ime_reset_all_v2")
func ime_reset_all_v2(_ engine: FfiEnginePtr?) -> Int32

// MARK: - Swift Bridge Error

enum RustBridgeV2Error: Error, LocalizedError {
    case engineNotCreated
    case nullPointer
    case invalidEngine
    case processingError
    case configError
    case ffiCallFailed(String)
    
    var errorDescription: String? {
        switch self {
        case .engineNotCreated: return "Failed to create engine"
        case .nullPointer: return "NULL pointer passed to FFI"
        case .invalidEngine: return "Invalid engine pointer"
        case .processingError: return "Key processing failed"
        case .configError: return "Configuration error"
        case .ffiCallFailed(let msg): return "FFI call failed: \(msg)"
        }
    }
}

// MARK: - Process Result

struct ProcessResult {
    let text: String
    let backspaceCount: Int
    let consumed: Bool
}

// MARK: - RustBridgeV2 Class

/// Thread-safe v2 FFI bridge using out parameter pattern
final class RustBridgeV2 {
    static let shared = RustBridgeV2()
    
    // MARK: - Properties
    
    private var enginePtr: FfiEnginePtr?
    private let engineLock = NSLock()
    private var currentConfig: FfiConfig_v2
    
    // MARK: - Lifecycle
    
    private init() {
        // Default configuration
        currentConfig = FfiConfig_v2(
            input_method: .telex,
            tone_style: .modern,
            smart_mode: true,
            instant_restore_enabled: true,
            esc_restore_enabled: false,
            enable_shortcuts: true
        )
    }
    
    deinit {
        destroyEngine()
    }
    
    // MARK: - Engine Lifecycle
    
    /// Initialize engine with config
    func initialize(config: FfiConfig_v2? = nil) throws {
        engineLock.lock()
        defer { engineLock.unlock() }
        
        // Destroy existing engine
        if let existingPtr = enginePtr {
            ime_destroy_engine_v2(existingPtr)
            enginePtr = nil
        }
        
        // Create new engine
        if let config = config {
            currentConfig = config
            var cfg = config
            guard let ptr = ime_create_engine_v2(&cfg) else {
                throw RustBridgeV2Error.engineNotCreated
            }
            enginePtr = ptr
        } else {
            var cfg = currentConfig
            guard let ptr = ime_create_engine_v2(&cfg) else {
                throw RustBridgeV2Error.engineNotCreated
            }
            enginePtr = ptr
        }
        
        Log.info("RustBridgeV2 initialized with \(currentConfig.input_method == .telex ? "Telex" : "VNI")")
    }
    
    /// Destroy engine (called automatically on deinit)
    func destroyEngine() {
        engineLock.lock()
        defer { engineLock.unlock() }
        
        if let ptr = enginePtr {
            ime_destroy_engine_v2(ptr)
            enginePtr = nil
            Log.info("RustBridgeV2 engine destroyed")
        }
    }
    
    // MARK: - Key Processing
    
    /// Process a keystroke
    /// - Parameter key: ASCII key code (a-z, 0-9)
    /// - Returns: ProcessResult with text, backspace count, consumed flag
    func processKey(_ key: UInt8) throws -> ProcessResult {
        engineLock.lock()
        defer { engineLock.unlock() }
        
        guard let ptr = enginePtr else {
            throw RustBridgeV2Error.invalidEngine
        }
        
        var result = FfiProcessResult_v2(text: nil, backspace_count: 0, consumed: false)
        let statusCode = ime_process_key_v2(ptr, CChar(bitPattern: key), &result)
        let status = FfiStatusCode(rawValue: statusCode) ?? .errorUnknown
        
        // Handle errors
        switch status {
        case .success:
            break
        case .errorNullEngine:
            throw RustBridgeV2Error.invalidEngine
        case .errorNullOutput:
            throw RustBridgeV2Error.nullPointer
        case .errorInvalidKey:
            throw RustBridgeV2Error.processingError
        default:
            throw RustBridgeV2Error.processingError
        }
        
        // Extract text (must free!)
        let text: String
        if let cStr = result.text {
            defer { ime_free_string_v2(cStr) }
            text = String(cString: cStr)
        } else {
            text = ""
        }
        
        return ProcessResult(
            text: text,
            backspaceCount: Int(result.backspace_count),
            consumed: result.consumed
        )
    }
    
    /// Process a keystroke with extended modifiers
    /// - Parameters:
    ///   - key: ASCII key code (a-z, 0-9)
    ///   - caps: CapsLock state
    ///   - shift: Shift key pressed
    ///   - ctrl: Cmd/Ctrl/Alt pressed
    /// - Returns: ProcessResult with text, backspace count, consumed flag
    func processKeyExt(_ key: UInt8, caps: Bool, shift: Bool, ctrl: Bool) throws -> ProcessResult {
        engineLock.lock()
        defer { engineLock.unlock() }
        
        guard let ptr = enginePtr else {
            throw RustBridgeV2Error.invalidEngine
        }
        
        var result = FfiProcessResult_v2(text: nil, backspace_count: 0, consumed: false)
        let statusCode = ime_process_key_ext_v2(ptr, CChar(bitPattern: key), caps, shift, ctrl, &result)
        let status = FfiStatusCode(rawValue: statusCode) ?? .errorUnknown
        
        switch status {
        case .success:
            break
        case .errorNullEngine:
            throw RustBridgeV2Error.invalidEngine
        case .errorNullOutput:
            throw RustBridgeV2Error.nullPointer
        case .errorInvalidKey:
            throw RustBridgeV2Error.processingError
        default:
            throw RustBridgeV2Error.processingError
        }
        
        let text: String
        if let cStr = result.text {
            defer { ime_free_string_v2(cStr) }
            text = String(cString: cStr)
        } else {
            text = ""
        }
        
        return ProcessResult(
            text: text,
            backspaceCount: Int(result.backspace_count),
            consumed: result.consumed
        )
    }
    
    /// Restore current buffer to raw ASCII input (undo all Vietnamese transforms)
    /// Used for Double OPTION key restore
    func restoreToRaw() throws -> ProcessResult {
        engineLock.lock()
        defer { engineLock.unlock() }
        
        guard let ptr = enginePtr else {
            throw RustBridgeV2Error.invalidEngine
        }
        
        var result = FfiProcessResult_v2(text: nil, backspace_count: 0, consumed: false)
        let statusCode = ime_restore_to_raw_v2(ptr, &result)
        let status = FfiStatusCode(rawValue: statusCode) ?? .errorUnknown
        
        guard status == .success else {
            throw RustBridgeV2Error.processingError
        }
        
        let text: String
        if let cStr = result.text {
            defer { ime_free_string_v2(cStr) }
            text = String(cString: cStr)
        } else {
            text = ""
        }
        
        return ProcessResult(
            text: text,
            backspaceCount: Int(result.backspace_count),
            consumed: result.consumed
        )
    }
    
    /// Get current config
    func getConfig() throws -> FfiConfig_v2 {
        engineLock.lock()
        defer { engineLock.unlock() }
        
        guard let ptr = enginePtr else {
            throw RustBridgeV2Error.invalidEngine
        }
        
        var config = FfiConfig_v2(input_method: .telex, tone_style: .modern, smart_mode: true, instant_restore_enabled: true, esc_restore_enabled: false, enable_shortcuts: true)
        let status = ime_get_config_v2(ptr, &config)
        
        guard status == FfiStatusCode.success.rawValue else {
            throw RustBridgeV2Error.configError
        }
        
        return config
    }
    
    /// Set configuration
    func setConfig(_ config: FfiConfig_v2) throws {
        engineLock.lock()
        defer { engineLock.unlock() }
        
        guard let ptr = enginePtr else {
            throw RustBridgeV2Error.invalidEngine
        }
        
        var cfg = config
        let status = ime_set_config_v2(ptr, &cfg)

        guard status == FfiStatusCode.success.rawValue else {
            throw RustBridgeV2Error.configError
        }

        currentConfig = config
        Log.info("Config updated: \(config.input_method == .telex ? "Telex" : "VNI")")
    }
    
    /// Change input method (Telex/VNI)
    func setInputMethod(_ method: FfiInputMethod) throws {
        var config = try getConfig()
        config.input_method = method
        try setConfig(config)
    }
    
    /// Change tone style (old/new)
    func setToneStyle(_ style: FfiToneStyle) throws {
        var config = try getConfig()
        config.tone_style = style
        try setConfig(config)
    }
    
    /// Toggle smart mode
    func setSmartMode(_ enabled: Bool) throws {
        var config = try getConfig()
        config.smart_mode = enabled
        try setConfig(config)
    }
    
    // MARK: - Version Info
    
    /// Get engine version
    static func getVersion() -> (major: Int, minor: Int, patch: Int) {
        var info = FfiVersionInfo(major: 0, minor: 0, patch: 0, api_version: 0)
        _ = ime_get_version_v2(&info)
        return (Int(info.major), Int(info.minor), Int(info.patch))
    }
    
    // MARK: - Shortcut Management
    
    /// Add shortcut
    func addShortcut(trigger: String, expansion: String) throws {
        engineLock.lock()
        defer { engineLock.unlock() }
        
        guard let ptr = enginePtr else {
            throw RustBridgeV2Error.invalidEngine
        }
        
        let status = trigger.withCString { triggerPtr in
            expansion.withCString { expansionPtr in
                ime_add_shortcut_v2(ptr, triggerPtr, expansionPtr)
            }
        }
        
        guard status == FfiStatusCode.success.rawValue else {
            if status == FfiStatusCode.errorAlreadyExists.rawValue {
                throw RustBridgeV2Error.configError // Or add specific error
            }
            throw RustBridgeV2Error.configError
        }
    }
    
    /// Remove shortcut
    func removeShortcut(trigger: String) throws {
        engineLock.lock()
        defer { engineLock.unlock() }
        
        guard let ptr = enginePtr else {
            throw RustBridgeV2Error.invalidEngine
        }
        
        let status = trigger.withCString { triggerPtr in
            ime_remove_shortcut_v2(ptr, triggerPtr)
        }
        
        guard status == FfiStatusCode.success.rawValue else {
            throw RustBridgeV2Error.configError
        }
    }
    
    /// Clear all shortcuts
    func clearShortcuts() throws {
        engineLock.lock()
        defer { engineLock.unlock() }
        
        guard let ptr = enginePtr else {
            throw RustBridgeV2Error.invalidEngine
        }
        
        let status = ime_clear_shortcuts_v2(ptr)
        guard status == FfiStatusCode.success.rawValue else {
            throw RustBridgeV2Error.configError
        }
    }
    
    /// Get shortcut count
    func getShortcutsCount() -> Int {
        engineLock.lock()
        defer { engineLock.unlock() }
        
        guard let ptr = enginePtr else {
            return 0
        }
        
        return Int(ime_shortcuts_count_v2(ptr))
    }
    
    /// Set shortcuts enabled/disabled
    func setShortcutsEnabled(_ enabled: Bool) throws {
        engineLock.lock()
        defer { engineLock.unlock() }
        
        guard let ptr = enginePtr else {
            throw RustBridgeV2Error.invalidEngine
        }
        
        let status = ime_set_shortcuts_enabled_v2(ptr, enabled)
        guard status == FfiStatusCode.success.rawValue else {
            throw RustBridgeV2Error.configError
        }
    }
    
    // MARK: - Buffer Reset (preserves shortcuts and config)
    
    /// Reset buffer state without destroying engine
    func resetBuffer() throws {
        engineLock.lock()
        defer { engineLock.unlock() }
        
        guard let ptr = enginePtr else {
            throw RustBridgeV2Error.invalidEngine
        }
        
        let status = ime_reset_buffer_v2(ptr)
        guard status == FfiStatusCode.success.rawValue else {
            throw RustBridgeV2Error.ffiCallFailed("ime_reset_buffer_v2")
        }
    }
    
    /// Reset all state including word history (preserves shortcuts and config)
    func resetAll() throws {
        engineLock.lock()
        defer { engineLock.unlock() }
        
        guard let ptr = enginePtr else {
            throw RustBridgeV2Error.invalidEngine
        }
        
        let status = ime_reset_all_v2(ptr)
        guard status == FfiStatusCode.success.rawValue else {
            throw RustBridgeV2Error.ffiCallFailed("ime_reset_all_v2")
        }
    }
}
