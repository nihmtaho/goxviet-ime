//
//  Log.swift
//  GoxViet
//
//  Logging utility for Gõ Việt IME debugging
//

import Foundation

enum Log {
    static let logPath = FileManager.default.homeDirectoryForCurrentUser
        .appendingPathComponent("Library/Logs/GoxViet/keyboard.log")
    
    #if DEBUG
    static var isEnabled: Bool = true
    #else
    static var isEnabled: Bool = false
    #endif
    
    static let maxLogSize: Int = 5 * 1024 * 1024  // 5MB limit
    
    static func write(_ msg: String) {
        guard isEnabled else { return }
        
        // Check log size and rotate if needed
        if let attrs = try? FileManager.default.attributesOfItem(atPath: logPath.path),
           let fileSize = attrs[.size] as? Int,
           fileSize > maxLogSize {
            rotateLog()
        }
        
        let timestamp = ISO8601DateFormatter().string(from: Date())
        let line = "[\(timestamp)] \(msg)\n"
        
        // Ensure directory exists
        let dir = logPath.deletingLastPathComponent()
        try? FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
        
        // Append to log file
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
    
    private static func rotateLog() {
        let backupPath = logPath.deletingPathExtension().appendingPathExtension("old.log")
        try? FileManager.default.removeItem(at: backupPath)
        try? FileManager.default.moveItem(at: logPath, to: backupPath)
    }
    
    static func key(_ code: UInt16, _ result: String) {
        write("KEY[\(code)] \(result)")
    }
    
    static func transform(_ bs: Int, _ chars: String) {
        write("TRANSFORM bs=\(bs) text='\(chars)'")
    }
    
    static func send(_ method: String, _ bs: Int, _ chars: String) {
        write("SEND[\(method)] bs=\(bs) text='\(chars)'")
    }
    
    static func method(_ name: String) {
        write("METHOD: \(name)")
    }
    
    static func info(_ msg: String) {
        write("INFO: \(msg)")
    }
    
    static func warning(_ msg: String) {
        write("WARNING: \(msg)")
    }
    
    static func error(_ msg: String) {
        write("ERROR: \(msg)")
    }
    
    static func skip() {
        write("SKIP")
    }
    
    static func queue(_ msg: String) {
        write("QUEUE: \(msg)")
    }
}