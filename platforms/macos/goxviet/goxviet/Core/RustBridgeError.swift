//
//  RustBridgeError.swift
//  GoxViet
//
//  Error types for RustBridge FFI operations
//

import Foundation

/// Errors that can occur during RustBridge operations
enum RustBridgeError: Error, LocalizedError {
    case notInitialized
    case invalidParameter(String)
    case ffiCallFailed(String)
    case memoryAllocationFailed
    case stringEncodingFailed
    case resultIsNull
    case invalidResult
    
    var errorDescription: String? {
        switch self {
        case .notInitialized:
            return "RustBridge is not initialized. Call initialize() first."
        case .invalidParameter(let param):
            return "Invalid parameter: \(param)"
        case .ffiCallFailed(let function):
            return "FFI call failed: \(function)"
        case .memoryAllocationFailed:
            return "Memory allocation failed in FFI call"
        case .stringEncodingFailed:
            return "Failed to encode string to UTF-8"
        case .resultIsNull:
            return "FFI function returned NULL result"
        case .invalidResult:
            return "FFI function returned invalid result"
        }
    }
    
    var recoverySuggestion: String? {
        switch self {
        case .notInitialized:
            return "Ensure RustBridge.shared.initialize() is called during app startup."
        case .invalidParameter:
            return "Check the parameter values and try again."
        case .ffiCallFailed:
            return "Check the log files for more details."
        case .memoryAllocationFailed:
            return "Restart the application. If the problem persists, check available memory."
        case .stringEncodingFailed:
            return "Ensure the string contains valid Unicode characters."
        case .resultIsNull, .invalidResult:
            return "This may indicate a bug. Please report this issue."
        }
    }
}

/// Result type for FFI operations
typealias RustBridgeResult<T> = Result<T, RustBridgeError>
