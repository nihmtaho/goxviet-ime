//
//  SettingsWindowCoordinator.swift
//  GoxViet
//
//  Phase 2 modular settings window coordinator
//  Simplifies TabView structure to avoid Swift 6 compiler issues
//  Phase 3: Lazy loading tabs for memory optimization
//

import SwiftUI

/// Lightweight coordinator for settings tabs
/// Delegates to individual enhanced views for actual UI
/// Phase 3: Uses lazy loading to minimize memory footprint
struct SettingsWindowCoordinator: View {
    @StateObject private var settingsManager = SettingsManager.shared
    @State private var perAppModes: [String: Bool] = [:]
    @State private var showClearConfirmation = false
    
    // Track selected tab for lazy loading
    @State private var selectedTab = 0
    
    var body: some View {
        TabView(selection: $selectedTab) {
            // General Tab - Always load first
            GeneralSettingsTab()
                .tabItem {
                    Label("General", systemImage: "gearshape")
                }
                .tag(0)
            
            // Per-App Tab - Lazy load
            LazyView {
                PerAppSettingsTab(perAppModes: $perAppModes, showClearConfirmation: $showClearConfirmation)
            }
            .tabItem {
                Label("Per-App", systemImage: "app.badge")
            }
            .tag(1)
            
            // Text Expansion Tab - Lazy load
            LazyView {
                TextExpansionSettingsTab()
            }
            .tabItem {
                Label("Text Expansion", systemImage: "text.badge.plus")
            }
            .tag(2)
            
            // Advanced Tab - Lazy load
            LazyView {
                AdvancedSettingsTab()
            }
            .tabItem {
                Label("Advanced", systemImage: "slider.horizontal.3")
            }
            .tag(3)
            
            // About Tab - Lazy load
            LazyView {
                AboutSettingsTab()
            }
            .tabItem {
                Label("About", systemImage: "info.circle")
            }
            .tag(4)
        }
        .frame(minWidth: 900, minHeight: 540)
        .onAppear {
            loadPerAppModes()
            syncSettingsToLegacy()
        }
        .onDisappear {
            // Cleanup when settings window closes
            cleanupResources()
        }
        .onReceive(NotificationCenter.default.publisher(for: NSNotification.Name("settingsWindowDidClose"))) { _ in
            // Additional cleanup when window closes
            cleanupResources()
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
    
    private func cleanupResources() {
        // Use autoreleasepool for immediate deallocation
        autoreleasepool {
            // Clear all state data
            perAppModes.removeAll()
            showClearConfirmation = false
            selectedTab = 0
            
            // Post memory cleanup notification
            NotificationCenter.default.post(name: NSNotification.Name("settingsWindowCleanup"), object: nil)
        }
        
        // Force UI update to release views
        DispatchQueue.main.async {
            // Additional cleanup on next runloop
        }
    }
}

// MARK: - Notification Names

extension Notification.Name {
    static let settingsWindowCleanup = Notification.Name("com.goxviet.settingsWindowCleanup")
}

// MARK: - Lazy View Helper

/// A view that only renders its content when needed
/// This helps reduce initial memory footprint of Settings window
struct LazyView<Content: View>: View {
    let build: () -> Content
    
    init(@ViewBuilder _ build: @escaping () -> Content) {
        self.build = build
    }
    
    var body: Content {
        build()
    }
}

// MARK: - Individual Tab Wrappers

struct GeneralSettingsTab: View {
    @ObservedObject private var settingsManager = SettingsManager.shared
    
    var body: some View {
        GeneralSettingsView(
            inputMethod: $settingsManager.inputMethod,
            modernToneStyle: $settingsManager.modernToneStyle,
            restoreShortcutEnabled: $settingsManager.restoreShortcutEnabled,
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

struct TextExpansionSettingsTab: View {
    var body: some View {
        TextExpansionSettingsView()
            .tabItem {
                Label("Text Expansion", systemImage: "text.badge.plus")
            }
    }
}

struct AdvancedSettingsTab: View {
    var body: some View {
        AdvancedSettingsView(
            openLogAction: openLog
        )
        .environmentObject(SettingsManager.shared)
        .tabItem {
            Label("Advanced", systemImage: "slider.horizontal.3")
        }
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
