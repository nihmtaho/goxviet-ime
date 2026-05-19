//
//  SettingsKeys.swift
//  GoxViet
//
//  Centralized UserDefaults key constants for GoxViet.
//  All keys use the com.goxviet.ime.* namespace.
//
//  nonisolated(unsafe) is required so that these constants are accessible from
//  nonisolated contexts under the project's default-MainActor isolation mode.
//

import Foundation

enum SettingsKey {
    // MARK: - Core Input Settings
    static let isEnabled              = "com.goxviet.ime.isEnabled"
    static let inputMethod            = "com.goxviet.ime.inputMethod"
    static let modernToneStyle        = "com.goxviet.ime.modernToneStyle"
    static let restoreShortcutEnabled = "com.goxviet.ime.restoreShortcutEnabled"
    static let freeToneEnabled        = "com.goxviet.ime.freeToneEnabled"
    static let instantRestoreEnabled  = "com.goxviet.ime.instantRestoreEnabled"
    static let smartModeEnabled       = "com.goxviet.ime.smartModeEnabled"

    // MARK: - IME Features
    static let autoDisableForNonLatin = "com.goxviet.ime.autoDisableNonLatin"
    static let hideFromDock           = "com.goxviet.ime.hideFromDock"
    static let outputEncoding         = "com.goxviet.ime.outputEncoding"
    static let shiftBackspaceEnabled  = "com.goxviet.ime.shiftBackspaceEnabled"
    static let textExpansionEnabled   = "com.goxviet.ime.textExpansionEnabled"
    static let shortcuts              = "com.goxviet.ime.shortcuts"

    // MARK: - Per-App Mode
    static let perAppModes            = "com.goxviet.ime.perAppModes"
    static let knownApps              = "com.goxviet.ime.knownApps"

    // MARK: - Keyboard Shortcuts
    static let toggleShortcut         = "com.goxviet.ime.toggleShortcut"
    static let restoreShortcut        = "com.goxviet.ime.restoreShortcut"

    // MARK: - Update Manager
    static let lastUpdateCheck        = "com.goxviet.ime.lastUpdateCheck"
    static let skipVersion            = "com.goxviet.ime.skipVersion"

    // MARK: - App Lifecycle
    static let lastKnownVersion       = "com.goxviet.ime.lastKnownVersion"
    static let permissionGranted      = "com.goxviet.ime.permissionGranted"
    static let hasLaunchedBefore      = "com.goxviet.ime.hasLaunchedBefore"

    // MARK: - Services
    static let loggingEnabled         = "com.goxviet.ime.loggingEnabled"

    // MARK: - Feature Gap US1–US5
    static let escRestoreEnabled        = "com.goxviet.ime.escRestoreEnabled"
    static let bracketShortcutsEnabled  = "com.goxviet.ime.bracketShortcutsEnabled"
    static let foreignConsonantsEnabled = "com.goxviet.ime.foreignConsonantsEnabled"
    static let autoCapitaliseEnabled    = "com.goxviet.ime.autoCapitaliseEnabled"
    static let wordHistoryEnabled       = "com.goxviet.ime.wordHistoryEnabled"

    // MARK: - Feedback & Onboarding
    static let soundEnabled             = "com.goxviet.ime.soundEnabled"
    static let remoteDesktopMode        = "com.goxviet.ime.remoteDesktopMode"
    static let hasCompletedOnboarding   = "com.goxviet.ime.hasCompletedOnboarding"
    static let debugLogEnabled          = "com.goxviet.ime.debugLogEnabled"
    static let disablePanelDetection    = "com.goxviet.ime.disablePanelDetection"
    static let restartOnClose           = "com.goxviet.ime.restartOnClose"

}
