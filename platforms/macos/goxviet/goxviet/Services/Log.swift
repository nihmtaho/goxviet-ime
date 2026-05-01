//
//  Log.swift
//  GoxViet
//
//  Logging utility for Gõ Việt IME debugging
//  Phase 3: Lazy evaluation and runtime toggle
//

import Foundation
import Combine

enum Log {
    static let logPath = FileManager.default.homeDirectoryForCurrentUser
        .appendingPathComponent("Library/Logs/GoxViet/keyboard.log")

    /// Touch /tmp/goxviet_debug.log to enable verbose logging without opening Preferences.
    /// Useful for production debugging: `touch /tmp/goxviet_debug.log` → enable,
    /// `rm /tmp/goxviet_debug.log` → disable.
    private static let debugTriggerPath = "/tmp/goxviet_debug.log"
    private(set) static var isFileDebugEnabled: Bool = false
    private static var debugTriggerTimer: Timer?

    /// Start checking for the debug trigger file every 5 seconds.
    /// Call once from AppDelegate after the main run loop is running.
    static func startDebugTriggerCheck() {
        guard debugTriggerTimer == nil else { return }
        checkDebugTrigger()
        let timer = Timer.scheduledTimer(withTimeInterval: 5.0, repeats: true) { _ in
            checkDebugTrigger()
        }
        RunLoop.main.add(timer, forMode: .common)
        debugTriggerTimer = timer
    }

    private static func checkDebugTrigger() {
        let enabled = FileManager.default.fileExists(atPath: debugTriggerPath)
        guard enabled != isFileDebugEnabled else { return }
        isFileDebugEnabled = enabled
        let msg = enabled
            ? "INFO: Debug logging ENABLED via \(debugTriggerPath)"
            : "INFO: Debug logging DISABLED (trigger file removed)"
        let timestamp = dateFormatter.string(from: Date())
        let line = "[\(timestamp)] \(msg)\n"
        writeRaw(line)
    }

    /// Cached formatter to avoid repeated allocation on every log write
    private static let dateFormatter: ISO8601DateFormatter = {
        let formatter = ISO8601DateFormatter()
        return formatter
    }()

    /// Whether logging is enabled (persisted in UserDefaults)
    static var isEnabled: Bool {
        get {
            UserDefaults.standard.object(forKey: SettingsKey.loggingEnabled) as? Bool ?? false
        }
        set {
            UserDefaults.standard.set(newValue, forKey: SettingsKey.loggingEnabled)
            // Post notification for UI updates
            NotificationCenter.default.post(
                name: .loggingStateChanged,
                object: newValue
            )
        }
    }

    static let maxLogSize: Int = 5 * 1024 * 1024  // 5MB limit

    /// Write log message with lazy evaluation
    /// Use @autoclosure to avoid string construction when logging is disabled
    static func write(_ msg: @autoclosure () -> String) {
        guard isEnabled || isFileDebugEnabled else { return }
        
        let message = msg()
        
        // Check log size and rotate if needed
        if let attrs = try? FileManager.default.attributesOfItem(atPath: logPath.path),
           let fileSize = attrs[.size] as? Int,
           fileSize > maxLogSize {
            rotateLog()
        }
        
        let timestamp = dateFormatter.string(from: Date())
        let line = "[\(timestamp)] \(message)\n"
        
        // Ensure directory exists
        let dir = logPath.deletingLastPathComponent()
        try? FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
        
        // Append to log file
        if let data = line.data(using: .utf8) {
            if FileManager.default.fileExists(atPath: logPath.path) {
                if let handle = try? FileHandle(forWritingTo: logPath) {
                    defer { handle.closeFile() }
                    handle.seekToEndOfFile()
                    handle.write(data)
                }
            } else {
                try? data.write(to: logPath)
            }
        }
    }
    
    private static func writeRaw(_ line: String) {
        let dir = logPath.deletingLastPathComponent()
        try? FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
        guard let data = line.data(using: .utf8) else { return }
        if FileManager.default.fileExists(atPath: logPath.path) {
            if let handle = try? FileHandle(forWritingTo: logPath) {
                defer { handle.closeFile() }
                handle.seekToEndOfFile()
                handle.write(data)
            }
        } else {
            try? data.write(to: logPath)
        }
    }

    private static func rotateLog() {
        let backupPath = logPath.deletingPathExtension().appendingPathExtension("old.log")
        try? FileManager.default.removeItem(at: backupPath)
        try? FileManager.default.moveItem(at: logPath, to: backupPath)
    }
    
    // MARK: - Log Level Methods with Lazy Evaluation
    
    /// Log key event - only evaluated if logging is enabled
    static func key(_ code: UInt16, _ result: @autoclosure () -> String) {
        write("KEY[\(code)] \(result())")
    }
    
    /// Log transformation event - only evaluated if logging is enabled
    static func transform(_ bs: Int, _ chars: @autoclosure () -> String) {
        write("TRANSFORM bs=\(bs) text='\(chars())'")
    }
    
    /// Log send event - only evaluated if logging is enabled
    static func send(_ method: @autoclosure () -> String, _ bs: Int, _ chars: @autoclosure () -> String) {
        write("SEND[\(method())] bs=\(bs) text='\(chars())'")
    }
    
    /// Log method change - only evaluated if logging is enabled
    static func method(_ name: @autoclosure () -> String) {
        write("METHOD: \(name())")
    }
    
    /// Log info message - only evaluated if logging is enabled
    static func info(_ msg: @autoclosure () -> String) {
        write("INFO: \(msg())")
    }
    
    /// Log warning message - only evaluated if logging is enabled
    static func warning(_ msg: @autoclosure () -> String) {
        write("WARNING: \(msg())")
    }
    
    /// Log error message - only evaluated if logging is enabled
    static func error(_ msg: @autoclosure () -> String) {
        write("ERROR: \(msg())")
    }
    
    /// Log skip event
    static func skip() {
        write("SKIP")
    }
    
    /// Log queue message - only evaluated if logging is enabled
    static func queue(_ msg: @autoclosure () -> String) {
        write("QUEUE: \(msg())")
    }
    
    // MARK: - Debug Helper
    
    /// Check if logging is enabled without writing
    static var isLoggingEnabled: Bool { isEnabled }
    
    /// Enable logging with optional reason
    static func enableLogging(reason: String? = nil) {
        isEnabled = true
        if let reason = reason {
            write("Logging enabled: \(reason)")
        }
    }
    
    /// Disable logging with optional reason
    static func disableLogging(reason: String? = nil) {
        if let reason = reason {
            write("Logging disabled: \(reason)")
        }
        isEnabled = false
    }
    
    /// Clear all log files
    static func clearLogs() {
        try? FileManager.default.removeItem(at: logPath)
        let backupPath = logPath.deletingPathExtension().appendingPathExtension("old.log")
        try? FileManager.default.removeItem(at: backupPath)
    }
}
