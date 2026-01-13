//
//  UpdateWindowView.swift
//  GoxViet
//
//  Dedicated update window with circular progress indicator
//

import SwiftUI
import AppKit

struct UpdateWindowView: View {
    @ObservedObject private var updateManager = UpdateManager.shared
    @Environment(\.dismiss) private var dismiss
    
    var body: some View {
        ZStack {
            // Glass background
            Rectangle()
                .fill(.ultraThinMaterial)
                .overlay(
                    LinearGradient(
                        colors: [
                            Color.blue.opacity(0.15),
                            Color.purple.opacity(0.10),
                            Color.pink.opacity(0.08)
                        ],
                        startPoint: .topLeading,
                        endPoint: .bottomTrailing
                    )
                )
                .ignoresSafeArea()
            
            VStack(spacing: 24) {
                // Header
                headerView
                
                Spacer()
                
                // Main content based on state
                mainContentView
                
                Spacer()
                
                // Action buttons
                actionButtonsView
            }
            .padding(32)
        }
        .frame(width: 480, height: 520)
        .onAppear {
            if updateManager.updateState == .idle || updateManager.updateState == .error {
                updateManager.checkForUpdates(userInitiated: true)
            }
        }
    }
    
    // MARK: - Header
    
    private var headerView: some View {
        VStack(spacing: 16) {
            // App Icon
            if let appIcon = NSImage(named: "AppIcon") {
                Image(nsImage: appIcon)
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .frame(width: 80, height: 80)
                    .clipShape(RoundedRectangle(cornerRadius: 20, style: .continuous))
                    .overlay(
                        RoundedRectangle(cornerRadius: 20, style: .continuous)
                            .stroke(.white.opacity(0.25), lineWidth: 1)
                    )
                    .shadow(color: .black.opacity(0.2), radius: 12, x: 0, y: 8)
            }
            
            VStack(spacing: 4) {
                Text("GoxViet")
                    .font(.system(size: 28, weight: .bold))
                
                if let currentVersion = currentVersion() {
                    Text("Version \(currentVersion)")
                        .font(.callout)
                        .foregroundStyle(.secondary)
                }
            }
        }
    }
    
    // MARK: - Main Content
    
    @ViewBuilder
    private var mainContentView: some View {
        switch updateManager.updateState {
        case .idle:
            idleStateView
        case .checking:
            checkingStateView
        case .updateAvailable:
            updateAvailableView
        case .downloading:
            downloadingView
        case .readyToInstall:
            readyToInstallView
        case .upToDate:
            upToDateView
        case .error:
            errorStateView
        }
    }
    
    private var idleStateView: some View {
        VStack(spacing: 12) {
            Image(systemName: "arrow.down.circle")
                .font(.system(size: 48))
                .foregroundStyle(.blue)
            
            Text("Check for Updates")
                .font(.title3)
                .fontWeight(.semibold)
            
            Text("Keep GoxViet up to date with the latest features and improvements")
                .font(.callout)
                .foregroundStyle(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal, 24)
        }
    }
    
    private var checkingStateView: some View {
        VStack(spacing: 16) {
            ProgressView()
                .scaleEffect(1.5)
                .frame(height: 60)
            
            Text("Checking for updates...")
                .font(.title3)
                .fontWeight(.medium)
            
            Text("Please wait while we check for new versions")
                .font(.callout)
                .foregroundStyle(.secondary)
        }
    }
    
    private var updateAvailableView: some View {
        VStack(spacing: 16) {
            Image(systemName: "arrow.down.circle.fill")
                .font(.system(size: 64))
                .foregroundStyle(.green)
            
            Text("Update Available")
                .font(.title2)
                .fontWeight(.bold)
            
            if let latestVersion = updateManager.latestVersion {
                VStack(spacing: 8) {
                    Text("New Version: \(latestVersion)")
                        .font(.title3)
                        .fontWeight(.semibold)
                        .foregroundStyle(.green)
                    
                    if let currentVersion = currentVersion() {
                        Text("Current: \(currentVersion)")
                            .font(.callout)
                            .foregroundStyle(.secondary)
                    }
                }
                .padding(.vertical, 8)
            }
            
            Text("A new version is ready to download")
                .font(.callout)
                .foregroundStyle(.secondary)
        }
    }
    
    private var downloadingView: some View {
        VStack(spacing: 24) {
            // Circular progress indicator
            ZStack {
                // Background circle
                Circle()
                    .stroke(Color.blue.opacity(0.2), lineWidth: 12)
                    .frame(width: 160, height: 160)
                
                // Progress circle
                Circle()
                    .trim(from: 0, to: updateManager.downloadProgress)
                    .stroke(
                        LinearGradient(
                            colors: [.blue, .purple],
                            startPoint: .topLeading,
                            endPoint: .bottomTrailing
                        ),
                        style: StrokeStyle(lineWidth: 12, lineCap: .round)
                    )
                    .frame(width: 160, height: 160)
                    .rotationEffect(.degrees(-90))
                    .animation(.easeInOut(duration: 0.3), value: updateManager.downloadProgress)
                
                // Percentage text
                VStack(spacing: 4) {
                    Text("\(Int(updateManager.downloadProgress * 100))%")
                        .font(.system(size: 36, weight: .bold))
                        .monospacedDigit()
                    
                    Text("Downloading")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }
            
            Text(updateManager.statusMessage)
                .font(.callout)
                .foregroundStyle(.secondary)
        }
    }
    
    private var readyToInstallView: some View {
        VStack(spacing: 16) {
            Image(systemName: "checkmark.circle.fill")
                .font(.system(size: 64))
                .foregroundStyle(.green)
            
            Text("Ready to Install")
                .font(.title2)
                .fontWeight(.bold)
            
            if let latestVersion = updateManager.latestVersion {
                Text("Version \(latestVersion)")
                    .font(.title3)
                    .foregroundStyle(.green)
            }
            
            Text("The update has been downloaded and is ready to install")
                .font(.callout)
                .foregroundStyle(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal, 24)
        }
    }
    
    private var upToDateView: some View {
        VStack(spacing: 16) {
            Image(systemName: "checkmark.circle.fill")
                .font(.system(size: 64))
                .foregroundStyle(.green)
            
            Text("You're Up to Date")
                .font(.title2)
                .fontWeight(.bold)
            
            if let currentVersion = currentVersion() {
                Text("Version \(currentVersion)")
                    .font(.title3)
                    .foregroundStyle(.secondary)
            }
            
            Text("You have the latest version of GoxViet")
                .font(.callout)
                .foregroundStyle(.secondary)
            
            if let lastChecked = updateManager.lastChecked {
                Text("Last checked: \(RelativeDateTimeFormatter().localizedString(for: lastChecked, relativeTo: Date()))")
                    .font(.caption)
                    .foregroundStyle(.tertiary)
                    .padding(.top, 4)
            }
        }
    }
    
    private var errorStateView: some View {
        VStack(spacing: 16) {
            Image(systemName: "exclamationmark.triangle.fill")
                .font(.system(size: 64))
                .foregroundStyle(.orange)
            
            Text("Update Check Failed")
                .font(.title2)
                .fontWeight(.bold)
            
            Text(updateManager.statusMessage)
                .font(.callout)
                .foregroundStyle(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal, 24)
        }
    }
    
    // MARK: - Action Buttons
    
    @ViewBuilder
    private var actionButtonsView: some View {
        HStack(spacing: 12) {
            // Cancel/Close button
            if updateManager.updateState == .downloading {
                Button("Cancel") {
                    updateManager.cancelDownload()
                }
                .buttonStyle(.bordered)
            }
            
            Spacer()
            
            // Primary action button
            switch updateManager.updateState {
            case .idle, .error:
                HStack(spacing: 12) {
                    #if DEBUG
                    Button {
                        UpdateSimulator.shared.simulateUpdateFlow()
                    } label: {
                        Label("Simulator", systemImage: "play.circle.fill")
                    }
                    .buttonStyle(.bordered)
                    #endif
                    
                    Button {
                        updateManager.checkForUpdates(userInitiated: true)
                    } label: {
                        Label("Check for Updates", systemImage: "arrow.triangle.2.circlepath")
                    }
                    .buttonStyle(.borderedProminent)
                    .tint(.blue)
                }
                
            case .updateAvailable:
                HStack(spacing: 12) {
                    Button {
                        openReleasePage()
                    } label: {
                        Label("Release Notes", systemImage: "doc.text")
                    }
                    .buttonStyle(.bordered)
                    
                    Button {
                        #if DEBUG
                        UpdateSimulator.shared.simulateDownload()
                        #else
                        updateManager.downloadUpdate()
                        #endif
                    } label: {
                        Label("Download", systemImage: "arrow.down.circle.fill")
                    }
                    .buttonStyle(.borderedProminent)
                    .tint(.green)
                }
                
            case .readyToInstall:
                Button {
                    #if DEBUG
                    UpdateSimulator.shared.reset()
                    #else
                    // Installation will be triggered by UpdateManager
                    // This is handled in the existing installDMG flow
                    #endif
                } label: {
                    Label("Install & Relaunch", systemImage: "arrow.clockwise.circle.fill")
                        .font(.headline)
                }
                .buttonStyle(.borderedProminent)
                .tint(.green)
                .controlSize(.large)
                
            case .upToDate:
                Button("Close") {
                    dismiss()
                }
                .buttonStyle(.borderedProminent)
                
            default:
                EmptyView()
            }
        }
    }
    
    // MARK: - Helpers
    
    private func currentVersion() -> String? {
        guard let version = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String else {
            return nil
        }
        return version
    }
    
    private func openReleasePage() {
        let version = updateManager.latestVersion ?? "latest"
        let urlString: String
        if version == "latest" {
            urlString = "https://github.com/nihmtaho/goxviet/releases/latest"
        } else {
            urlString = "https://github.com/nihmtaho/goxviet/releases/tag/v\(version)"
        }
        
        if let url = URL(string: urlString) {
            NSWorkspace.shared.open(url)
        }
    }
}

// MARK: - Preview

#Preview {
    UpdateWindowView()
        .frame(width: 480, height: 520)
}
