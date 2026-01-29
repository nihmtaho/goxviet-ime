//
//  SettingsRootView.swift
//  GoxViet
//
//  Settings window root - delegates to modular Phase 2 coordinator
//

import SwiftUI
import AppKit

/// Main settings window root view
/// Phase 2: Uses SettingsWindowCoordinator for modular architecture
struct SettingsRootView: View {
    var body: some View {
        SettingsWindowCoordinator()
    }
}

