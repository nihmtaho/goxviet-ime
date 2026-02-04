//
//  AboutSettingsView.swift
//  GoxViet
//
//  Enhanced About page with modern design and credits
//

import SwiftUI

struct AboutSettingsView: View {
    @EnvironmentObject var updateManager: UpdateManager
    @State private var showChangelog = false

    
    var body: some View {
        ScrollView {
            VStack(spacing: 24) {
                // App Icon and Name
                VStack(spacing: 12) {
                    if let appIcon = NSImage(named: "AppIcon") {
                        Image(nsImage: appIcon)
                            .resizable()
                            .frame(width: 96, height: 96)
                            .clipShape(RoundedRectangle(cornerRadius: 20))
                            .shadow(color: .black.opacity(0.2), radius: 10, x: 0, y: 5)
                    }
                    
                    VStack(spacing: 4) {
                        Text("Gõ Việt")
                            .font(.system(size: 28, weight: .bold))
                        
                        Text("GoxViet")
                            .font(.system(size: 16, weight: .medium))
                            .foregroundColor(.secondary)
                        
                        if let version = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String {
                            Text("Version \(version)")
                                .font(.system(size: 13))
                                .foregroundColor(.secondary)
                        }
                    }
                }
                .padding(.top, 20)
                
                Divider()
                
                // Description
                VStack(spacing: 8) {
                    Text("Modern Vietnamese Input Method")
                        .font(.system(size: 15, weight: .semibold))
                    
                    Text("A high-performance, cross-platform Vietnamese IME built with Rust and SwiftUI")
                        .font(.system(size: 12))
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                        .fixedSize(horizontal: false, vertical: true)
                }
                .frame(maxWidth: 400)
                
                // Quick Links
                GroupBox {
                    VStack(spacing: 12) {
                        LinkButton(
                            title: "GitHub Repository",
                            icon: "chevron.left.forwardslash.chevron.right",
                            url: "https://github.com/nihmtaho/goxviet-ime"
                        )
                        
                        Divider()
                        
                        LinkButton(
                            title: "Documentation",
                            icon: "book",
                            url: "https://github.com/nihmtaho/goxviet-ime/docs"
                        )
                        
                        Divider()
                        
                        LinkButton(
                            title: "Report an Issue",
                            icon: "exclamationmark.bubble",
                            url: "https://github.com/nihmtaho/goxviet-ime/issues"
                        )
                        
                        Divider()
                        
                        Button {
                            showChangelog = true
                        } label: {
                            HStack {
                                Image(systemName: "list.bullet.clipboard")
                                    .frame(width: 20)
                                Text("View Changelog")
                                    .font(.system(size: 13))
                                Spacer()
                                Image(systemName: "chevron.right")
                                    .font(.system(size: 11))
                                    .foregroundColor(.secondary)
                            }
                        }
                        .buttonStyle(.plain)
                        .contentShape(Rectangle())
                        .sheet(isPresented: $showChangelog) {
                            ChangelogView()
                        }
                    }
                    .padding(8)
                } label: {
                    Label("Quick Links", systemImage: "link")
                        .font(.system(size: 14, weight: .semibold))
                }
                
                // Credits
                GroupBox {
                    VStack(alignment: .leading, spacing: 12) {
                        VStack(alignment: .leading, spacing: 4) {
                            Text("Development")
                                .font(.system(size: 12, weight: .semibold))
                            Text("Built with ❤️ by the GoxViet Team")
                                .font(.system(size: 11))
                                .foregroundColor(.secondary)
                        }
                        
                        Divider()
                        
                        VStack(alignment: .leading, spacing: 4) {
                            Text("Core Engine")
                                .font(.system(size: 12, weight: .semibold))
                            Text("Powered by Rust for high performance and memory safety")
                                .font(.system(size: 11))
                                .foregroundColor(.secondary)
                        }
                        
                        Divider()
                        
                        VStack(alignment: .leading, spacing: 4) {
                            Text("UI Framework")
                                .font(.system(size: 12, weight: .semibold))
                            Text("Native SwiftUI for macOS")
                                .font(.system(size: 11))
                                .foregroundColor(.secondary)
                        }
                        
                        Divider()
                        
                        VStack(alignment: .leading, spacing: 4) {
                            Text("Acknowledgments")
                                .font(.system(size: 12, weight: .semibold))
                            Text("Thanks to the Vietnamese IME community and open source contributors")
                                .font(.system(size: 11))
                                .foregroundColor(.secondary)
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding(8)
                } label: {
                    Label("Credits", systemImage: "person.3")
                        .font(.system(size: 14, weight: .semibold))
                }
                
                // Legal
                GroupBox {
                    VStack(alignment: .leading, spacing: 8) {
                        HStack {
                            Text("License")
                                .font(.system(size: 12, weight: .semibold))
                            Spacer()
                            Text("MIT")
                                .font(.system(size: 11))
                                .foregroundColor(.secondary)
                        }
                        
                        Divider()
                        
                        Text("Copyright © 2024-2026 GoxViet Contributors")
                            .font(.system(size: 11))
                            .foregroundColor(.secondary)
                    }
                    .padding(8)
                } label: {
                    Label("Legal", systemImage: "doc.text")
                        .font(.system(size: 14, weight: .semibold))
                }
                
                // Software Update Section
                GroupBox {
                    VStack(spacing: 12) {
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                switch updateManager.state {
                                case .checking:
                                    HStack(spacing: 8) {
                                        ProgressView()
                                            .controlSize(.small)
                                            .scaleEffect(0.8)
                                        Text("Checking for updates...")
                                            .font(.system(size: 13, weight: .semibold))
                                    }
                                    Text("Please wait...")
                                        .font(.system(size: 11))
                                        .foregroundColor(.secondary)
                                        
                                case .available(let info):
                                    HStack(spacing: 6) {
                                        Image(systemName: "checkmark.circle.fill")
                                            .foregroundColor(.green)
                                        Text("Update Available")
                                            .font(.system(size: 13, weight: .semibold))
                                    }
                                    Text("Version \(info.version) is ready to install")
                                        .font(.system(size: 11))
                                        .foregroundColor(.secondary)
                                        
                                case .upToDate:
                                    HStack(spacing: 6) {
                                        Image(systemName: "checkmark.circle.fill")
                                            .foregroundColor(.green)
                                        Text("You're up to date")
                                            .font(.system(size: 13, weight: .semibold))
                                    }
                                    if let version = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String {
                                        Text("Version \(version) is the latest version")
                                            .font(.system(size: 11))
                                            .foregroundColor(.secondary)
                                    }
                                    
                                case .error(let message):
                                    HStack(spacing: 6) {
                                        Image(systemName: "exclamationmark.triangle.fill")
                                            .foregroundColor(.red)
                                        Text("Check Failed")
                                            .font(.system(size: 13, weight: .semibold))
                                    }
                                    Text(message)
                                        .font(.system(size: 11))
                                        .foregroundColor(.secondary)
                                    
                                case .downloading:
                                    HStack(spacing: 8) {
                                        ProgressView()
                                            .controlSize(.small)
                                            .scaleEffect(0.8)
                                        Text("Downloading update...")
                                            .font(.system(size: 13, weight: .semibold))
                                    }
                                    
                                case .readyToInstall:
                                    HStack(spacing: 6) {
                                        Image(systemName: "arrow.down.circle.fill")
                                            .foregroundColor(.green)
                                        Text("Ready to Install")
                                            .font(.system(size: 13, weight: .semibold))
                                    }
                                    
                                case .installing:
                                    HStack(spacing: 8) {
                                        ProgressView()
                                            .controlSize(.small)
                                            .scaleEffect(0.8)
                                        Text("Installing...")
                                            .font(.system(size: 13, weight: .semibold))
                                    }
                                    Text("Application will restart automatically")
                                        .font(.system(size: 11))
                                        .foregroundColor(.secondary)

                                case .idle:
                                    HStack(spacing: 6) {
                                        Image(systemName: "arrow.triangle.2.circlepath")
                                            .foregroundColor(.secondary)
                                        Text("Software Update")
                                            .font(.system(size: 13, weight: .semibold))
                                    }
                                    Text("Check for the latest version")
                                        .font(.system(size: 11))
                                        .foregroundColor(.secondary)
                                }
                            }
                            
                            Spacer()
                            
                            if case .available = updateManager.state {
                                Button {
                                    updateManager.downloadUpdate()
                                } label: {
                                    Label("Update Now", systemImage: "arrow.down.circle.fill")
                                }
                                .buttonStyle(.borderedProminent)
                            } else {
                                Button {
                                    updateManager.checkForUpdatesManually()
                                } label: {
                                    Label(updateManager.isChecking ? "Checking..." : "Check for Updates", 
                                          systemImage: "arrow.clockwise")
                                }
                                .buttonStyle(.bordered)
                                .disabled(updateManager.isChecking)
                            }
                        }
                        
                        // Last checked info
                        if let lastChecked = updateManager.lastChecked, !updateManager.isChecking {
                            HStack(spacing: 6) {
                                Image(systemName: "clock")
                                    .font(.system(size: 10))
                                    .foregroundColor(.secondary)
                                Text("Last checked: \(formatDate(lastChecked))")
                                    .font(.system(size: 10))
                                    .foregroundColor(.secondary)
                                Spacer()
                            }
                            .padding(.top, 4)
                        }
                        
                        // Download progress (if downloading)
                        if case .downloading(let progress) = updateManager.state {
                            VStack(spacing: 8) {
                                ProgressView(value: progress)
                                    .progressViewStyle(.linear)
                                HStack {
                                    Text("\(Int(progress * 100))% downloaded")
                                        .font(.system(size: 11))
                                        .foregroundColor(.secondary)
                                    Spacer()
                                    Button("Cancel") {
                                        updateManager.cancelDownload()
                                    }
                                    .buttonStyle(.plain)
                                    .font(.system(size: 11))
                                    .foregroundColor(.red)
                                }
                            }
                            .padding(.top, 8)
                        }
                    }
                    .padding(8)
                } label: {
                    Label("Software Update", systemImage: "arrow.down.circle")
                        .font(.system(size: 14, weight: .semibold))
                }
                
                Spacer()
            }
            .padding(24)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
    
    private func formatDate(_ date: Date) -> String {
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .full
        return formatter.localizedString(for: date, relativeTo: Date())
    }
}

// Link Button Component
struct LinkButton: View {
    let title: String
    let icon: String
    let url: String
    
    var body: some View {
        Button {
            if let url = URL(string: url) {
                NSWorkspace.shared.open(url)
            }
        } label: {
            HStack {
                Image(systemName: icon)
                    .frame(width: 20)
                Text(title)
                    .font(.system(size: 13))
                Spacer()
                Image(systemName: "arrow.up.forward")
                    .font(.system(size: 11))
                    .foregroundColor(.secondary)
            }
        }
        .buttonStyle(.plain)
        .contentShape(Rectangle())
    }
}

// Changelog View
struct ChangelogView: View {
    @Environment(\.dismiss) var dismiss
    @State private var changelogContent = "Loading changelog..."
    
    var body: some View {
        VStack(spacing: 0) {
            // Header
            HStack {
                Text("Changelog")
                    .font(.title2)
                Spacer()
                Button {
                    dismiss()
                } label: {
                    Image(systemName: "xmark.circle.fill")
                        .foregroundColor(.secondary)
                        .font(.title2)
                }
                .buttonStyle(.plain)
            }
            .padding()
            
            Divider()
            
            // Content
            ScrollView {
                Text(changelogContent)
                    .font(.system(size: 12, design: .monospaced))
                    .textSelection(.enabled)
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding()
            }
        }
        .frame(width: 600, height: 500)
        .onAppear {
            loadChangelog()
        }
    }
    
    private func loadChangelog() {
        // Try to load CHANGELOG.md from bundle
        if let changelogURL = Bundle.main.url(forResource: "CHANGELOG", withExtension: "md"),
           let content = try? String(contentsOf: changelogURL) {
            changelogContent = content
        } else {
            changelogContent = """
            # Changelog
            
            No changelog available.
            
            Visit the GitHub repository for release notes:
            https://github.com/nihmtaho/goxviet-ime/releases
            """
        }
    }
}

#Preview {
    AboutSettingsView()
        .environmentObject(UpdateManager.shared)
        .frame(width: 700, height: 700)
}
