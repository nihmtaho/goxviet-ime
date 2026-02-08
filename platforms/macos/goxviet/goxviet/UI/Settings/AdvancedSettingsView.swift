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
    let metrics: EngineMetrics
    let resetAction: () -> Void
    let openLogAction: () -> Void
    
    @EnvironmentObject var settingsManager: SettingsManager
    
    @State private var showResetConfirmation = false
    @State private var showLegacyEncodingWarning = false
    @State private var pendingEncoding: OutputEncoding?
    @State private var autoRefresh = false
    @State private var refreshTimer: Timer?
    @State private var currentMetrics: EngineMetrics
    @State private var loggingEnabled: Bool = Log.isEnabled
    
    init(metrics: EngineMetrics, resetAction: @escaping () -> Void, openLogAction: @escaping () -> Void) {
        self.metrics = metrics
        self.resetAction = resetAction
        self.openLogAction = openLogAction
        self._currentMetrics = State(initialValue: metrics)
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
                
                // Engine Metrics Section
                GroupBox {
                    VStack(spacing: 16) {
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Engine Performance Metrics")
                                    .font(.system(size: 14, weight: .semibold))
                                Text("Real-time statistics from the Rust core engine")
                                    .font(.system(size: 12))
                                    .foregroundColor(.secondary)
                            }
                            
                            Spacer()
                            
                            Toggle("Auto-refresh", isOn: $autoRefresh)
                                .toggleStyle(.switch)
                                .onChange(of: autoRefresh) { newValue in
                                    if newValue {
                                        startAutoRefresh()
                                    } else {
                                        stopAutoRefresh()
                                    }
                                }
                        }
                        
                        MetricsChartView(metrics: currentMetrics)
                        
                        HStack {
                            Button {
                                refreshMetrics()
                            } label: {
                                Label("Refresh", systemImage: "arrow.clockwise")
                            }
                            .buttonStyle(.bordered)
                            
                            Button {
                                exportMetrics()
                            } label: {
                                Label("Export", systemImage: "square.and.arrow.up")
                            }
                            .buttonStyle(.bordered)
                            
                            Spacer()
                            
                            Button {
                                showResetConfirmation = true
                            } label: {
                                Label("Reset", systemImage: "trash")
                            }
                            .buttonStyle(.bordered)
                            .alert("Reset Engine Metrics", isPresented: $showResetConfirmation) {
                                Button("Cancel", role: .cancel) { }
                                Button("Reset", role: .destructive) {
                                    resetAction()
                                    refreshMetrics()
                                }
                            } message: {
                                Text("This will reset all engine metrics. This action cannot be undone.")
                            }
                            .alert("Legacy Encoding Warning", isPresented: $showLegacyEncodingWarning) {
                                Button("Cancel", role: .cancel) {
                                    // Revert to Unicode
                                    settingsManager.outputEncoding = .unicode
                                    pendingEncoding = nil
                                }
                                Button("Use Legacy Encoding", role: .destructive) {
                                    // Keep the selection
                                    pendingEncoding = nil
                                }
                            } message: {
                                if let encoding = pendingEncoding {
                                    Text("You selected \(encoding.displayName), which is a legacy encoding. Modern applications should use Unicode (UTF-8) for best compatibility. Continue with legacy encoding?")
                                } else {
                                    Text("Legacy encodings may cause compatibility issues.")
                                }
                            }
                        }
                    }
                    .padding(8)
                } label: {
                    Label("Engine Metrics", systemImage: "gauge.badge.plus")
                        .font(.system(size: 14, weight: .semibold))
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
                
                // Memory Profiling Section
                GroupBox {
                    VStack(spacing: 12) {
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Memory Profiling")
                                    .font(.system(size: 13, weight: .medium))
                                Text("Real-time memory usage monitoring and analysis")
                                    .font(.system(size: 11))
                                    .foregroundColor(.secondary)
                            }
                            
                            Spacer()
                            
                            NavigationLink {
                                MemoryProfilingView()
                            } label: {
                                Label("Open Profiler", systemImage: "arrow.right")
                            }
                            .buttonStyle(.borderedProminent)
                        }
                        
                        Divider()
                        
                        // Quick stats
                        HStack(spacing: 16) {
                            QuickStatView(
                                label: "Current Usage",
                                value: MemoryProfiler.shared.currentStats.formattedUsedMemory,
                                icon: "memorychip"
                            )
                            
                            QuickStatView(
                                label: "Peak",
                                value: MemoryProfiler.shared.currentStats.formattedPeakMemory,
                                icon: "arrow.up.circle"
                            )
                            
                            QuickStatView(
                                label: "Available",
                                value: MemoryProfiler.shared.currentStats.formattedAvailableMemory,
                                icon: "externaldrive"
                            )
                        }
                        .padding(.vertical, 4)
                    }
                    .padding(8)
                } label: {
                    Label("Memory Profiling", systemImage: "cpu")
                        .font(.system(size: 14, weight: .semibold))
                }
                

                
                // System Information
                GroupBox {
                    VStack(spacing: 8) {
                        InfoRow(label: "macOS Version", value: ProcessInfo.processInfo.operatingSystemVersionString)
                        Divider()
                        InfoRow(label: "Architecture", value: getArchitecture())
                        Divider()
                        InfoRow(label: "Memory Usage", value: getMemoryUsage())
                        Divider()
                        InfoRow(label: "Bundle Version", value: getBundleVersion())
                    }
                    .padding(8)
                } label: {
                    Label("System Info", systemImage: "info.circle")
                        .font(.system(size: 14, weight: .semibold))
                }
                
                Spacer()
            }
            .padding(24)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .onAppear {
            refreshMetrics()
            // Sync logging state on appear
            loggingEnabled = Log.isEnabled
        }
        .onDisappear {
            stopAutoRefresh()
        }
        .onReceive(NotificationCenter.default.publisher(for: NSNotification.Name("com.goxviet.loggingStateChanged"))) { notification in
            if let enabled = notification.object as? Bool {
                loggingEnabled = enabled
            }
        }
    }
    
    // MARK: - Actions
    
    private func refreshMetrics() {
        // In real implementation, fetch from RustBridge
        // For now, use passed metrics
        currentMetrics = metrics
    }
    
    private func startAutoRefresh() {
        refreshTimer = Timer.scheduledTimer(withTimeInterval: 2.0, repeats: true) { _ in
            refreshMetrics()
        }
    }
    
    private func stopAutoRefresh() {
        refreshTimer?.invalidate()
        refreshTimer = nil
    }
    
    private func exportMetrics() {
        let savePanel = NSSavePanel()
        savePanel.allowedContentTypes = [.json]
        savePanel.nameFieldStringValue = "goxviet_metrics.json"
        
        savePanel.begin { response in
            guard response == .OK, let url = savePanel.url else { return }
            
            let metricsDict: [String: Any] = [
                "totalKeystrokes": currentMetrics.totalKeystrokes,
                "backspaceCount": currentMetrics.backspaceCount,
                "avgBufferLength": currentMetrics.avgBufferLength,
                "exportedAt": ISO8601DateFormatter().string(from: Date())
            ]
            
            if let jsonData = try? JSONSerialization.data(withJSONObject: metricsDict, options: .prettyPrinted) {
                try? jsonData.write(to: url)
                Log.info("Metrics exported to: \(url.path)")
            }
        }
    }
    
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
    
    private func openShortcutsManager() {
        // TODO: Implement shortcuts manager
        Log.info("Opening shortcuts manager (not yet implemented)")
    }
    
    // MARK: - System Info Helpers
    
    private func getArchitecture() -> String {
        #if arch(arm64)
        return "Apple Silicon (ARM64)"
        #elseif arch(x86_64)
        return "Intel (x86_64)"
        #else
        return "Unknown"
        #endif
    }
    
    private func getMemoryUsage() -> String {
        var info = mach_task_basic_info()
        var count = mach_msg_type_number_t(MemoryLayout<mach_task_basic_info>.size) / 4
        
        let kerr: kern_return_t = withUnsafeMutablePointer(to: &info) {
            $0.withMemoryRebound(to: integer_t.self, capacity: 1) {
                task_info(mach_task_self_, task_flavor_t(MACH_TASK_BASIC_INFO), $0, &count)
            }
        }
        
        if kerr == KERN_SUCCESS {
            let usedMB = Double(info.resident_size) / 1024.0 / 1024.0
            return String(format: "%.1f MB", usedMB)
        }
        return "N/A"
    }
    
    private func getBundleVersion() -> String {
        if let version = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String,
           let build = Bundle.main.object(forInfoDictionaryKey: "CFBundleVersion") as? String {
            return "\(version) (\(build))"
        }
        return "Unknown"
    }
}

// Quick Stat Component for Memory Preview
struct QuickStatView: View {
    let label: String
    let value: String
    let icon: String
    
    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            HStack(spacing: 4) {
                Image(systemName: icon)
                    .font(.system(size: 10))
                    .foregroundColor(.accentColor)
                Text(label)
                    .font(.system(size: 10))
                    .foregroundColor(.secondary)
            }
            Text(value)
                .font(.system(size: 13, weight: .semibold, design: .monospaced))
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(8)
        .background(
            RoundedRectangle(cornerRadius: 6)
                .fill(Color(nsColor: .textBackgroundColor))
        )
    }
}

// Info Row Component
struct InfoRow: View {
    let label: String
    let value: String
    
    var body: some View {
        HStack {
            Text(label)
                .font(.system(size: 12))
                .foregroundColor(.secondary)
            
            Spacer()
            
            Text(value)
                .font(.system(size: 12, design: .monospaced))
                .foregroundColor(.primary)
        }
        .padding(.vertical, 4)
    }
}

#Preview {
    AdvancedSettingsView(
        metrics: EngineMetrics(
            totalKeystrokes: 12345,
            backspaceCount: 2345,
            avgBufferLength: 4.25
        ),
        resetAction: { },
        openLogAction: { }
    )
    .frame(width: 700, height: 700)
}
