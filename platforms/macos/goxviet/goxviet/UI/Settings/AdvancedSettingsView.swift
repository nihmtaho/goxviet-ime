//
//  AdvancedSettingsView.swift
//  GoxViet
//
//  Enhanced Advanced Settings with metrics, logs, and diagnostics
//

import SwiftUI
import AppKit
import UniformTypeIdentifiers

struct AdvancedSettingsView: View {
    let openLogAction: () -> Void
    
    @EnvironmentObject var settingsManager: SettingsManager
    
    @State private var showLegacyEncodingWarning = false
    @State private var pendingEncoding: OutputEncoding?
    
    init(openLogAction: @escaping () -> Void) {
        self.openLogAction = openLogAction
    }
    
    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                // Header
                VStack(alignment: .leading, spacing: 4) {
                    Text("Advanced Settings")
                        .font(.system(size: 20, weight: .semibold))
                    Text("Diagnostics, metrics, and advanced configuration")
                        .font(.system(size: 13))
                        .foregroundColor(.secondary)
                }
                .padding(.bottom, 8)
                
                // Output Encoding Section
                GroupBox {
                    VStack(alignment: .leading, spacing: 12) {
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Output Encoding")
                                    .font(.system(size: 14, weight: .semibold))
                                Text("(Beta)")
                                    .font(.system(size: 10, weight: .semibold))
                                    .foregroundColor(.orange)
                                    .padding(.horizontal, 6)
                                    .padding(.vertical, 2)
                                    .background(Capsule().fill(Color.orange.opacity(0.2)))
                                Text("Choose output text encoding format")
                                    .font(.system(size: 12))
                                    .foregroundColor(.secondary)
                            }
                            Spacer()
                        }
                        
                        Divider()
                        
                        HStack {
                            Text("Encoding:")
                                .font(.system(size: 12))
                                .foregroundColor(.secondary)
                                .frame(width: 80, alignment: .trailing)
                            
                            Picker("", selection: $settingsManager.outputEncoding) {
                                ForEach(OutputEncoding.allCases, id: \.self) { encoding in
                                    HStack {
                                        Text(encoding.displayName)
                                        if encoding.isLegacy {
                                            Text("(Legacy)")
                                                .font(.caption)
                                                .foregroundColor(.orange)
                                        }
                                    }
                                    .tag(encoding)
                                }
                            }
                            .pickerStyle(.menu)
                            .frame(width: 200)
                        }
                        .onChange(of: settingsManager.outputEncoding) { _, newValue in
                            if newValue.isLegacy {
                                pendingEncoding = newValue
                                showLegacyEncodingWarning = true
                            }
                        }
                        
                        // Description for selected encoding
                        HStack(spacing: 8) {
                            Image(systemName: "info.circle")
                                .foregroundColor(.blue)
                                .font(.system(size: 12))
                            Text(settingsManager.outputEncoding.description)
                                .font(.system(size: 11))
                                .foregroundColor(.secondary)
                                .fixedSize(horizontal: false, vertical: true)
                        }
                        .padding(8)
                        .background(Color.blue.opacity(0.05))
                        .cornerRadius(6)
                        
                        // Legacy encoding warning banner
                        if settingsManager.outputEncoding.isLegacy {
                            HStack(spacing: 8) {
                                Image(systemName: "exclamationmark.triangle.fill")
                                    .foregroundColor(.orange)
                                    .font(.system(size: 12))
                                VStack(alignment: .leading, spacing: 2) {
                                    Text("Legacy Encoding Selected")
                                        .font(.system(size: 11, weight: .semibold))
                                        .foregroundColor(.orange)
                                    Text("This encoding is for compatibility with older systems. Use Unicode for modern applications.")
                                        .font(.system(size: 10))
                                        .foregroundColor(.secondary)
                                        .fixedSize(horizontal: false, vertical: true)
                                }
                            }
                            .padding(10)
                            .background(Color.orange.opacity(0.1))
                            .cornerRadius(6)
                        }
                    }
                    .padding(12)
                }
                
                // Logging Section
                GroupBox {
                    VStack(spacing: 12) {
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Application Logs")
                                    .font(.system(size: 13, weight: .medium))
                                Text("View detailed logging information")
                                    .font(.system(size: 11))
                                    .foregroundColor(.secondary)
                            }
                            
                            Spacer()
                            
                            Button {
                                openLogAction()
                            } label: {
                                Label("Open Log File", systemImage: "doc.text.magnifyingglass")
                            }
                            .buttonStyle(.borderedProminent)
                        }
                        
                        Divider()
                        
                        HStack {
                            Image(systemName: "folder")
                                .foregroundColor(.secondary)
                            Text("~/Library/Logs/GoxViet/")
                                .font(.system(size: 11, design: .monospaced))
                                .foregroundColor(.secondary)
                            
                            Spacer()
                            
                            Button {
                                copyLogPath()
                            } label: {
                                Image(systemName: "doc.on.doc")
                            }
                            .buttonStyle(.plain)
                            .help("Copy log path")
                        }
                        .padding(8)
                        .background(
                            RoundedRectangle(cornerRadius: 6)
                                .fill(Color(nsColor: .textBackgroundColor))
                        )
                    }
                    .padding(8)
                } label: {
                    Label("Logging", systemImage: "doc.text")
                        .font(.system(size: 14, weight: .semibold))
                }
                
                Spacer()
            }
            .padding(24)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
    
    // MARK: - Actions
    
    private func copyLogPath() {
        let pasteboard = NSPasteboard.general
        pasteboard.clearContents()
        pasteboard.setString("~/Library/Logs/GoxViet/", forType: .string)
    }
}

#Preview {
    AdvancedSettingsView(openLogAction: { })
        .frame(width: 700, height: 500)
}
