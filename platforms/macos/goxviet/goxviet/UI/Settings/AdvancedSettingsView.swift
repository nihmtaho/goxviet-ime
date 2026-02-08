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
    @State private var loggingEnabled: Bool = Log.isEnabled
    
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
                                Text("Enable logging for debugging purposes")
                                    .font(.system(size: 11))
                                    .foregroundColor(.secondary)
                            }
                            
                            Spacer()
                            
                            Toggle("", isOn: $loggingEnabled)
                                .toggleStyle(.switch)
                                .onChange(of: loggingEnabled) { _, newValue in
                                    if newValue {
                                        Log.enableLogging(reason: "User enabled in Advanced Settings")
                                    } else {
                                        Log.disableLogging(reason: "User disabled in Advanced Settings")
                                    }
                                }
                        }
                        
                        Divider()
                        
                        // Log file actions
                        HStack {
                            Button {
                                openLogAction()
                            } label: {
                                Label("Open Log File", systemImage: "doc.text.magnifyingglass")
                            }
                            .buttonStyle(.bordered)
                            .disabled(!loggingEnabled && !FileManager.default.fileExists(atPath: Log.logPath.path))
                            
                            Spacer()
                            
                            Button {
                                copyLogPath()
                            } label: {
                                Label("Copy Path", systemImage: "doc.on.doc")
                            }
                            .buttonStyle(.bordered)
                            
                            Button {
                                clearLogs()
                            } label: {
                                Label("Clear Logs", systemImage: "trash")
                            }
                            .buttonStyle(.bordered)
                            .foregroundColor(.red)
                        }
                        
                        // Log path display
                        HStack {
                            Image(systemName: "folder")
                                .foregroundColor(.secondary)
                            Text("~/Library/Logs/GoxViet/keyboard.log")
                                .font(.system(size: 11, design: .monospaced))
                                .foregroundColor(.secondary)
                            
                            Spacer()
                        }
                        .padding(8)
                        .background(
                            RoundedRectangle(cornerRadius: 6)
                                .fill(Color(nsColor: .textBackgroundColor))
                        )
                        
                        // Logging status
                        HStack {
                            Image(systemName: loggingEnabled ? "checkmark.circle.fill" : "xmark.circle.fill")
                                .foregroundColor(loggingEnabled ? .green : .secondary)
                            Text(loggingEnabled ? "Logging is enabled" : "Logging is disabled")
                                .font(.system(size: 12))
                                .foregroundColor(.secondary)
                            Spacer()
                            if loggingEnabled {
                                Text("May impact performance")
                                    .font(.system(size: 11))
                                    .foregroundColor(.orange)
                            }
                        }
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
        .onAppear {
            // Sync logging state on appear
            loggingEnabled = Log.isEnabled
        }
        .onReceive(NotificationCenter.default.publisher(for: NSNotification.Name("com.goxviet.loggingStateChanged"))) { notification in
            if let enabled = notification.object as? Bool {
                loggingEnabled = enabled
            }
        }
    }
    
    // MARK: - Actions
    
    private func copyLogPath() {
        let pasteboard = NSPasteboard.general
        pasteboard.clearContents()
        pasteboard.setString("~/Library/Logs/GoxViet/", forType: .string)
    }
    
    private func clearLogs() {
        Log.clearLogs()
        // Show confirmation
        let alert = NSAlert()
        alert.messageText = "Logs Cleared"
        alert.informativeText = "All log files have been removed."
        alert.alertStyle = .informational
        alert.addButton(withTitle: "OK")
        alert.runModal()
    }
}

#Preview {
    AdvancedSettingsView(
        openLogAction: { }
    )
    .frame(width: 700, height: 700)
}
