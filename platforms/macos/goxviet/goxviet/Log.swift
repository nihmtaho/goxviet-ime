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
    
    static var isEnabled: Bool = false
    
    static func write(_ msg: String) {
        guard isEnabled else { return }
        
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
    
    static func skip() {
        write("SKIP")
    }
    
    static func queue(_ msg: String) {
        write("QUEUE: \(msg)")
    }
}