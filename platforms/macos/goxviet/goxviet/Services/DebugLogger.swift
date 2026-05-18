//
//  DebugLogger.swift
//  GoxViet
//
//  Writes timestamped log lines to ~/Library/Logs/GoxViet/debug.log
//  when debugLogEnabled is true.
//

import Foundation

final class DebugLogger {
    nonisolated(unsafe) static var shared: DebugLogger!

    var isEnabled: Bool {
        get { SettingsManager.shared.debugLogEnabled }
        set {
            SettingsManager.shared.debugLogEnabled = newValue
            if !newValue { clear() }
        }
    }

    lazy var logFileURL: URL = {
        let logsDir = FileManager.default.urls(for: .libraryDirectory, in: .userDomainMask)[0]
            .appendingPathComponent("Logs/GoxViet", isDirectory: true)
        try? FileManager.default.createDirectory(at: logsDir, withIntermediateDirectories: true)
        return logsDir.appendingPathComponent("debug.log")
    }()

    nonisolated init() {}

    func log(_ message: String, level: String = "INFO") {
        guard isEnabled else { return }
        let timestamp = ISO8601DateFormatter().string(from: Date())
        let line = "[\(timestamp)] [\(level)] \(message)\n"
        guard let data = line.data(using: .utf8) else { return }

        if FileManager.default.fileExists(atPath: logFileURL.path) {
            if let handle = try? FileHandle(forWritingTo: logFileURL) {
                defer { try? handle.close() }
                handle.seekToEndOfFile()
                handle.write(data)
            }
        } else {
            try? data.write(to: logFileURL, options: .atomic)
        }
    }

    func clear() {
        try? FileManager.default.removeItem(at: logFileURL)
    }

    func readLog() -> String {
        (try? String(contentsOf: logFileURL, encoding: .utf8)) ?? ""
    }
}
