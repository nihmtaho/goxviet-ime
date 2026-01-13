//
//  GoxVietApp.swift
//  GoxViet
//
//  SwiftUI App lifecycle - WindowGroup manages Settings UI
//

import SwiftUI

@main
struct GoxVietApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    
    var body: some Scene {
        // We use manual window management via WindowManager/NSWindow
        // to ensure complete memory cleanup when windows are closed.
        // This Settings scene is just a placeholder to satisfy the App protocol
        // efficiently without creating default windows.
        Settings {
            EmptyView()
        }
    }
}

