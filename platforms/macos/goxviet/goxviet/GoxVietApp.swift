//
//  GoxVietApp.swift
//  GoxViet
//
//  SwiftUI App lifecycle for menu bar application
//

import SwiftUI

@main
struct GoxVietApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    
    var body: some Scene {
        // Setting scene - app is menu bar only, Settings handled via WindowManager
        Settings {
            SettingsRootView()
        }
    }
}

// Empty scene to satisfy SwiftUI Scene requirement
struct EmptyScene: Scene {
    var body: some Scene {
        WindowGroup("Dummy", id: "dummy") {
            EmptyView()
        }
    }
}


