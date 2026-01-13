//
//  GoxVietApp.swift
//  GoxViet
//
//  SwiftUI App lifecycle - WindowGroup manages Settings UI
//

import SwiftUI

// Notification for opening Settings window
extension Notification.Name {
    static let openSettingsWindow = Notification.Name("openSettingsWindow")
}

@main
struct GoxVietApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    
    var body: some Scene {
        // Settings window - managed via WindowGroup
        WindowGroup(id: "settings") {
            SettingsWindowManager()
        }
        .windowStyle(.hiddenTitleBar)
        .defaultSize(width: 820, height: 520)
        .windowResizability(.contentSize)
        .defaultPosition(.center)
        .commands {
            // Remove "New" menu item
            // CommandGroup(replacing: .newItem) {}
            
            // Standard Edit menu
            // SwiftUI provides this automatically
            
            // Standard Window menu with Cmd+W support
            CommandGroup(replacing: .windowArrangement) {
                // SwiftUI provides standard window management including Cmd+W
            }
        }
    }
}

// Wrapper view that manages Settings window opening via Environment
struct SettingsWindowManager: View {
    @Environment(\.openWindow) private var openWindow
    @State private var hasSetupObserver = false
    
    var body: some View {
        SettingsRootView()
            .frame(minWidth: 820, minHeight: 520)
            .onAppear {
                // When Settings opens, temporarily show in dock
                NSApp.setActivationPolicy(.regular)
                Log.info("Settings opened - showing in dock")
                
                // Setup notification observer only once
                if !hasSetupObserver {
                    hasSetupObserver = true
                    NotificationCenter.default.addObserver(
                        forName: .openSettingsWindow,
                        object: nil,
                        queue: .main
                    ) { [self] _ in
                        handleOpenSettingsRequest()
                    }
                }
            }
            .onDisappear {
                // When Settings closes, restore hideFromDock state
                handleSettingsClosed()
            }
    }
    
    private func handleOpenSettingsRequest() {
        // Check if Settings window already exists
        var settingsWindowExists = false
        for window in NSApp.windows {
            if let identifier = window.identifier?.rawValue,
               identifier.contains("settings") || identifier.contains("AppWindow-settings") {
                // Window exists, just bring it to front
                NSApp.setActivationPolicy(.regular)
                window.makeKeyAndOrderFront(nil)
                NSApp.activate(ignoringOtherApps: true)
                settingsWindowExists = true
                Log.info("Settings window already exists - bringing to front")
                break
            }
        }
        
        // Only create new window if it doesn't exist
        if !settingsWindowExists {
            openWindow(id: "settings")
            NSApp.activate(ignoringOtherApps: true)
            Log.info("Created new Settings window")
        }
    }
    
    private func handleSettingsClosed() {
        Log.info("Settings closed - restoring hideFromDock state")
        
        // Restore activation policy based on hideFromDock flag
        let hideFromDock = AppState.shared.hideFromDock
        let policy: NSApplication.ActivationPolicy = hideFromDock ? .accessory : .regular
        NSApp.setActivationPolicy(policy)
        
        Log.info("Activation policy set to: \(hideFromDock ? ".accessory (hidden)" : ".regular (visible)")")
    }
}
