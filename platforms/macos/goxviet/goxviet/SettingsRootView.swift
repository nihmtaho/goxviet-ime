//
//  SettingsRootView.swift
//  GoxViet
//
//  Settings window integrated with enhanced Phase 2 components.
//  Uses SettingsManager as single source of truth for all settings.
//

import SwiftUI
import AppKit

/// Main settings window root view
/// Displays TabView with 4 sections: General, Per-App, Advanced, About
struct SettingsRootView: View {
    // MARK: - Settings Manager (Single Source of Truth)
    @ObservedObject private var settingsManager = SettingsManager.shared
    
    // MARK: - View State
    @State private var perAppModes: [String: Bool] = [:]
    @State private var showClearConfirmation = false

    // MARK: - Body
    var body: some View {
        TabView {
            // General Settings - Input method, tone style, smart features
            GeneralSettingsView(
                inputMethod: $settingsManager.inputMethod,
                modernToneStyle: $settingsManager.modernToneStyle,
                escRestoreEnabled: $settingsManager.escRestoreEnabled,
                freeToneEnabled: $settingsManager.freeToneEnabled,
                instantRestoreEnabled: $settingsManager.instantRestoreEnabled,
                autoDisableForNonLatin: $settingsManager.autoDisableForNonLatin
            )
            .tabItem {
                Label("General", systemImage: "gearshape")
            }

            // Per-App Settings - Smart mode and app-specific overrides
            PerAppSettingsView(
                smartModeEnabled: $settingsManager.smartModeEnabled,
                perAppModes: $perAppModes,
                showClearConfirmation: $showClearConfirmation,
                reloadAction: loadPerAppModes
            )
            .tabItem {
                Label("Per-App", systemImage: "app.badge")
            }

            // Advanced Settings - Metrics, logs, diagnostics
            AdvancedSettingsView(
                metrics: getEngineMetrics(),
                resetAction: resetEngineMetrics,
                openLogAction: openLogFile
            )
            .tabItem {
                Label("Advanced", systemImage: "slider.horizontal.3")
            }

            // About - Version info, updates, credits
            AboutSettingsView()
                .environmentObject(UpdateManager.shared)
            .tabItem {
                Label("About", systemImage: "info.circle")
            }
        }
        .frame(minWidth: 900, minHeight: 540)
        .onAppear {
            loadPerAppModes()
            syncSettingsToLegacyComponents()
        }
        .onReceive(TypedNotificationCenter.publisher(for: .smartModeChanged)) { notification in
            settingsManager.smartModeEnabled = notification.payload.enabled
            loadPerAppModes()
            Log.info("Settings: Smart mode updated to \(notification.payload.enabled)")
        }
        .onReceive(TypedNotificationCenter.publisher(for: .inputMethodChanged)) { notification in
            settingsManager.inputMethod = notification.payload.method
            Log.info("Settings: Input method updated to \(notification.payload.method == 0 ? "Telex" : "VNI")")
        }
        .onReceive(TypedNotificationCenter.publisher(for: .modernToneStyleChanged)) { notification in
            settingsManager.modernToneStyle = notification.payload.enabled
            Log.info("Settings: Tone style updated to \(notification.payload.enabled ? "Modern" : "Traditional")")
        }
        .onReceive(TypedNotificationCenter.publisher(for: .perAppModesChanged)) { _ in
            loadPerAppModes()
        }
    }

    // MARK: - Helpers
    
    /// Load per-app modes from AppState (legacy compatibility)
    /// TODO: Migrate to PerAppModeManagerEnhanced
    private func loadPerAppModes() {
        perAppModes = AppState.shared.getKnownAppsWithStates()
    }

    /// Sync SettingsManager to legacy components (temporary during migration)
    /// TODO: Remove after full migration to SettingsManager
    private func syncSettingsToLegacyComponents() {
        AppState.shared.inputMethod = settingsManager.inputMethod
        AppState.shared.modernToneStyle = settingsManager.modernToneStyle
        AppState.shared.escRestoreEnabled = settingsManager.escRestoreEnabled
        AppState.shared.freeToneEnabled = settingsManager.freeToneEnabled
        AppState.shared.isSmartModeEnabled = settingsManager.smartModeEnabled
        AppState.shared.autoDisableForNonLatinEnabled = settingsManager.autoDisableForNonLatin
    }

    /// Open log file in default text editor
    private func openLogFile() {
        if FileManager.default.fileExists(atPath: Log.logPath.path) {
            NSWorkspace.shared.open(Log.logPath)
        } else {
            Log.error("Log file not found at: \(Log.logPath.path)")
        }
    }

    /// Get engine metrics from Rust core
    /// TODO: Wire up to actual FFI call via RustBridgeSafe
    private func getEngineMetrics() -> EngineMetrics {
        EngineMetrics(totalKeystrokes: 0, backspaceCount: 0, avgBufferLength: 0.0)
    }

    /// Reset engine metrics in Rust core
    /// TODO: Wire up to actual FFI call via RustBridgeSafe
    private func resetEngineMetrics() {
        Log.info("Metrics reset requested (not yet wired to Rust core)")
    }
}
