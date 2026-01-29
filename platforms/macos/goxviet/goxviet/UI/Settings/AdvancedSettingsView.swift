//
//  AdvancedSettingsView.swift
//  GoxViet
//
//  Enhanced Advanced Settings with metrics, logs, and diagnostics
//

import SwiftUI
import AppKit

struct AdvancedSettingsView: View {
    let metrics: EngineMetrics
    let resetAction: () -> Void
    let openLogAction: () -> Void
    
    @State private var showResetConfirmation = false
    @State private var autoRefresh = false
    @State private var refreshTimer: Timer?
    @State private var currentMetrics: EngineMetrics
    
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
                
                // Shortcuts Management
                GroupBox {
                    VStack(spacing: 12) {
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Text Expansion Shortcuts")
                                    .font(.system(size: 13, weight: .medium))
                                Text("Manage your typing shortcuts")
                                    .font(.system(size: 11))
                                    .foregroundColor(.secondary)
                            }
                            
                            Spacer()
                            
                            Button {
                                openShortcutsManager()
                            } label: {
                                Label("Manage Shortcuts", systemImage: "arrow.right")
                            }
                            .buttonStyle(.bordered)
                        }
                    }
                    .padding(8)
                } label: {
                    Label("Shortcuts", systemImage: "text.badge.plus")
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
        }
        .onDisappear {
            stopAutoRefresh()
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
