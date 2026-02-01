//
//  OutputEncoding.swift
//  GoxViet
//
//  Enum defining output encoding formats for Vietnamese text
//

import Foundation

/// Output encoding formats for Vietnamese text
/// Maps to core engine encoding values (0-3)
enum OutputEncoding: Int, CaseIterable, Identifiable, Codable {
    case unicode = 0
    case tcvn3 = 1
    case vni = 2
    case cp1258 = 3
    
    var id: Int { rawValue }
    
    /// Display name for UI
    var displayName: String {
        switch self {
        case .unicode: return "Unicode (Default)"
        case .tcvn3: return "TCVN3 (Legacy)"
        case .vni: return "VNI Windows (Legacy)"
        case .cp1258: return "CP1258 (Windows-1258)"
        }
    }
    
    /// Short name without annotation
    var shortName: String {
        switch self {
        case .unicode: return "Unicode"
        case .tcvn3: return "TCVN3"
        case .vni: return "VNI Windows"
        case .cp1258: return "CP1258"
        }
    }
    
    /// Detailed description for help text
    var description: String {
        switch self {
        case .unicode:
            return "Modern Unicode standard, compatible with all applications. Recommended for most users."
        case .tcvn3:
            return "Legacy TCVN3 encoding for older Vietnamese applications. May not display correctly in modern apps."
        case .vni:
            return "VNI Windows encoding for legacy software compatibility. Use only if required by specific applications."
        case .cp1258:
            return "Windows-1258 code page for Windows applications. Limited character support compared to Unicode."
        }
    }
    
    /// Whether this is a legacy encoding (requires warning)
    var isLegacy: Bool {
        self != .unicode
    }
    
    /// Icon name for UI representation
    var iconName: String {
        switch self {
        case .unicode: return "checkmark.circle.fill"
        case .tcvn3: return "exclamationmark.triangle.fill"
        case .vni: return "exclamationmark.triangle.fill"
        case .cp1258: return "exclamationmark.triangle.fill"
        }
    }
    
    /// Icon color for UI representation
    var iconColor: String {
        switch self {
        case .unicode: return "green"
        case .tcvn3, .vni, .cp1258: return "orange"
        }
    }
}
