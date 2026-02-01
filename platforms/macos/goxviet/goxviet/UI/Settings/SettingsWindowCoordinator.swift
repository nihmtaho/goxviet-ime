//
//  SettingsWindowCoordinator.swift
//  GoxViet
//
//  Phase 2 modular settings window coordinator
//  Simplifies TabView structure to avoid Swift 6 compiler issues
//

import SwiftUI

/// Lightweight coordinator for settings tabs
/// Delegates to individual enhanced views for actual UI
struct SettingsWindowCoordinator: View {
    @ObservedObject private var settingsManager = SettingsManager.shared
    @State private var perAppModes: [String: Bool] = [:]
    @State private var showClearConfirmation = false
    
    var body: some View {
        TabView {
            GeneralSettingsTab()
            PerAppSettingsTab(perAppModes: $perAppModes, showClearConfirmation: $showClearConfirmation)
            AdvancedSettingsTab()
            AboutSettingsTab()
        }
        .frame(minWidth: 900, minHeight: 540)
        .onAppear {
            loadPerAppModes()
            syncSettingsToLegacy()
        }
    }
    
    // MARK: - Helpers
    
    private func loadPerAppModes() {
        perAppModes = PerAppModeManagerEnhanced.shared.getKnownAppsWithStates()
    }
    
    private func syncSettingsToLegacy() {
        // SettingsManager already syncs to AppState internally via syncToAppState()
        // This is just a trigger to ensure initial sync on app launch
        SettingsManager.shared.setInputMethod(SettingsManager.shared.inputMethod)
    }
}

// MARK: - Individual Tab Wrappers

struct GeneralSettingsTab: View {
    @ObservedObject private var settingsManager = SettingsManager.shared
    
    var body: some View {
        GeneralSettingsView(
            inputMethod: $settingsManager.inputMethod,
            modernToneStyle: $settingsManager.modernToneStyle,
            escRestoreEnabled: $settingsManager.escRestoreEnabled,
            freeToneEnabled: $settingsManager.freeToneEnabled,
            instantRestoreEnabled: $settingsManager.instantRestoreEnabled,
            autoDisableForNonLatin: $settingsManager.autoDisableForNonLatin,
            shiftBackspaceEnabled: $settingsManager.shiftBackspaceEnabled
        )
        .tabItem {
            Label("General", systemImage: "gearshape")
        }
    }
}

struct PerAppSettingsTab: View {
    @ObservedObject private var settingsManager = SettingsManager.shared
    @Binding var perAppModes: [String: Bool]
    @Binding var showClearConfirmation: Bool
    
    var body: some View {
        PerAppSettingsView(
            smartModeEnabled: $settingsManager.smartModeEnabled,
            perAppModes: $perAppModes,
            showClearConfirmation: $showClearConfirmation,
            reloadAction: reloadModes
        )
        .tabItem {
            Label("Per-App", systemImage: "app.badge")
        }
    }
    
    private func reloadModes() {
        perAppModes = PerAppModeManagerEnhanced.shared.getKnownAppsWithStates()
    }
}

struct AdvancedSettingsTab: View {
    var body: some View {
        AdvancedSettingsView(
            metrics: getMetrics(),
            resetAction: resetMetrics,
            openLogAction: openLog
        )
        .environmentObject(SettingsManager.shared)
        .tabItem {
            Label("Advanced", systemImage: "slider.horizontal.3")
        }
    }
    
    private func getMetrics() -> EngineMetrics {
        EngineMetrics(totalKeystrokes: 0, backspaceCount: 0, avgBufferLength: 0.0)
    }
    
    private func resetMetrics() {
        Log.info("Metrics reset requested (not yet wired to Rust core)")
    }
    
    private func openLog() {
        if FileManager.default.fileExists(atPath: Log.logPath.path) {
            NSWorkspace.shared.open(Log.logPath)
        } else {
            Log.error("Log file not found at: \(Log.logPath.path)")
        }
    }
}

struct AboutSettingsTab: View {
    var body: some View {
        AboutSettingsView()
            .environmentObject(UpdateManager.shared)
            .tabItem {
                Label("About", systemImage: "info.circle")
            }
    }
}
