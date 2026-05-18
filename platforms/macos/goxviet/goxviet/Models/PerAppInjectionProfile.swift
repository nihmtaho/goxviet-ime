import SwiftUI

// MARK: - Per-App Injection Profile

struct PerAppInjectionProfile: Codable, Equatable {
    var bundleId: String
    var delayPreset: DelayPreset = .none
    var injectionMethod: InjectionOverride = .auto
    var isEnabled: Bool = true

    init(bundleId: String) {
        self.bundleId = bundleId
    }
}

// MARK: - Delay Preset

enum DelayPreset: Int, CaseIterable, Codable {
    case none = 0
    case low = 1
    case medium = 2
    case high = 3
    case veryHigh = 4

    var displayName: String {
        switch self {
        case .none:     return "Không"
        case .low:      return "Thấp"
        case .medium:   return "Vừa"
        case .high:     return "Cao"
        case .veryHigh: return "Rất cao"
        }
    }

    /// (backspaceµs, waitµs, textµs)
    var delays: (UInt32, UInt32, UInt32) {
        switch self {
        case .none:     return (200, 800, 500)
        case .low:      return (1000, 3000, 1500)
        case .medium:   return (3000, 8000, 3000)
        case .high:     return (8000, 25000, 8000)
        case .veryHigh: return (12000, 25000, 12000)
        }
    }

    static func closest(to delays: (UInt32, UInt32, UInt32)) -> DelayPreset {
        let wait = delays.1
        return allCases.min(by: {
            abs(Int($0.delays.1) - Int(wait)) < abs(Int($1.delays.1) - Int(wait))
        }) ?? .none
    }

    var color: Color {
        switch self {
        case .none:     return .blue
        case .low:      return .green
        case .medium:   return .orange
        case .high:     return Color(NSColor.systemRed)
        case .veryHigh: return .purple
        }
    }
}

// MARK: - Injection Override

enum InjectionOverride: Int, CaseIterable, Codable {
    case auto = -1
    case fast = 0
    case slow = 1
    case charByChar = 2
    case selection = 3
    case emptyCharPrefix = 4

    var displayName: String {
        switch self {
        case .auto:           return "Tự động"
        case .fast:           return "Fast"
        case .slow:           return "Slow"
        case .charByChar:     return "Char-by-char"
        case .selection:      return "Selection"
        case .emptyCharPrefix: return "Empty char"
        }
    }

    var subtitle: String {
        switch self {
        case .auto:           return "Để hệ thống chọn"
        case .fast:           return "Mặc định, backspace + text"
        case .slow:           return "Delay cao hơn cho Electron"
        case .charByChar:     return "Gõ từng ký tự, Safari/GDocs"
        case .selection:      return "Select + replace, combo box"
        case .emptyCharPrefix: return "Phá autocomplete trình duyệt"
        }
    }
}
