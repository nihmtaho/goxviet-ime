//
//  InputMethodDefinition.swift
//  GoxViet
//
//  Sprint D (T6.3) — Data-driven input method definitions.
//  Each definition is a JSON-encoded InputMethodConfig passed to
//  `ime_load_input_config_v2` on the Rust side.
//
//  Based on KieuGo.ini pattern.
//

import Foundation

// MARK: - InputMethodDefinition

/// Pre-built JSON configs for standard Vietnamese input methods.
///
/// These mirror the built-in `InputMethodConfig::telex()` / `InputMethodConfig::vni()`
/// in the Rust domain layer. Swift chooses which JSON to pass to Rust via
/// `RustBridgeV2.loadInputConfig(_:)` when the user changes input method.
enum InputMethodDefinition {

    // MARK: Telex

    /// JSON config for Telex input method.
    ///
    /// Tone keys: s f r x j → sắc huyền hỏi ngã nặng
    /// Remove tone: z
    /// Modifiers: aa ee oo → â ê ô | aw ow uw w → ă ơ ư
    /// Stroke: dd → đ
    static let telexJSON: String = """
    {
      "name": "telex",
      "mappings": {
        "s":   "tone_sac",
        "f":   "tone_huyen",
        "r":   "tone_hoi",
        "x":   "tone_nga",
        "j":   "tone_nang",
        "z":   "xoa_dau",
        "aa":  "mod_a",
        "ee":  "mod_e",
        "oo":  "mod_o",
        "aw":  "mod_a_w",
        "ow":  "mod_o_w",
        "uw":  "mod_u_w",
        "w":   "mod_u_w",
        "dd":  "stroke_d",
        "uow": "compound_u_o_a"
      }
    }
    """

    // MARK: VNI

    /// JSON config for VNI input method.
    ///
    /// Tone keys: 1 2 3 4 5 → sắc huyền hỏi ngã nặng
    /// Remove tone: 0
    /// Circumflex: 6 → â/ê/ô
    /// Horn: 7 → ơ/ư
    /// Breve: 8 → ă
    /// Stroke: 9 → đ
    static let vniJSON: String = """
    {
      "name": "vni",
      "mappings": {
        "1": "tone_sac",
        "2": "tone_huyen",
        "3": "tone_hoi",
        "4": "tone_nga",
        "5": "tone_nang",
        "0": "xoa_dau",
        "6": "mod_a",
        "7": "mod_o_w",
        "8": "mod_a_w",
        "9": "stroke_d"
      }
    }
    """

    // MARK: Helpers

    /// Returns the appropriate JSON string for the given FfiInputMethod.
    static func json(for method: FfiInputMethod) -> String {
        switch method {
        case .vni:   return vniJSON
        default:     return telexJSON
        }
    }
}
