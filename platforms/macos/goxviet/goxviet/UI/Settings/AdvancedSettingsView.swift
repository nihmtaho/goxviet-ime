//
//  AdvancedSettingsView.swift
//  GoxViet
//
//  Enhanced Advanced Settings with logs and diagnostics
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
    @State private var showAppPicker = false
    @State private var injectionProfiles: [PerAppInjectionProfile] = []

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
                    Text("Diagnostics and advanced configuration")
                        .font(.system(size: 13))
                        .foregroundColor(.secondary)
                }
                .padding(.bottom, 8)
                
                // Performance Section
                GroupBox {
                    VStack(spacing: 0) {
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Tắt phát hiện Spotlight/Raycast")
                                    .font(.system(size: 13, weight: .medium))
                                Text("Bỏ qua panel app, giảm CPU/RAM sử dụng")
                                    .font(.system(size: 11))
                                    .foregroundColor(.secondary)
                            }
                            Spacer()
                            Toggle("", isOn: Binding(
                                get: { settingsManager.disablePanelDetection },
                                set: { settingsManager.disablePanelDetection = $0 }
                            ))
                            .toggleStyle(.switch)
                            .labelsHidden()
                        }
                        .padding(12)

                        Divider()

                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Khởi động lại khi đóng cài đặt")
                                    .font(.system(size: 13, weight: .medium))
                                Text("Tự động giải phóng RAM của cài đặt khi đóng")
                                    .font(.system(size: 11))
                                    .foregroundColor(.secondary)
                            }
                            Spacer()
                            Toggle("", isOn: Binding(
                                get: { settingsManager.restartOnClose },
                                set: { settingsManager.restartOnClose = $0 }
                            ))
                            .toggleStyle(.switch)
                            .labelsHidden()
                        }
                        .padding(12)
                    }
                } label: {
                    Label("Hiệu suất", systemImage: "gauge.with.dots.needle.bottom.50percent")
                        .font(.system(size: 14, weight: .semibold))
                }

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
                                .fill(Color(NSColor.textBackgroundColor))
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

                        if loggingEnabled {
                            Divider()
                            InlineDebugLogView()
                                .padding(.horizontal, 12)
                                .padding(.bottom, 12)
                        }
                    }
                    .padding(8)
                } label: {
                    Label("Logging", systemImage: "doc.text")
                        .font(.system(size: 14, weight: .semibold))
                }
                
                // Remote Desktop Mode Section
                GroupBox {
                    VStack(spacing: 12) {
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Remote Desktop Mode (SessionTap)")
                                    .font(.system(size: 13, weight: .medium))
                                Text("Dùng cho RustDesk, AnyDesk, TeamViewer.")
                                    .font(.system(size: 11))
                                    .foregroundColor(.secondary)
                            }
                            Spacer()
                            Toggle("", isOn: Binding(
                                get: { settingsManager.remoteDesktopMode },
                                set: {
                                    settingsManager.remoteDesktopMode = $0
                                    InputManager.shared.useSessionTap = $0
                                }
                            ))
                            .toggleStyle(.switch)
                            .labelsHidden()
                            .controlSize(.small)
                        }
                    }
                    .padding(12)
                } label: {
                    Label("Chế độ kết nối từ xa", systemImage: "display.2")
                        .font(.system(size: 14, weight: .semibold))
                }

                // Debug Log Section
                GroupBox {
                    VStack(spacing: 12) {
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Bật debug log")
                                    .font(.system(size: 13, weight: .medium))
                                Text("Ghi log chi tiết cho mục đích gỡ lỗi")
                                    .font(.system(size: 11))
                                    .foregroundColor(.secondary)
                            }
                            Spacer()
                            Toggle("", isOn: Binding(
                                get: { settingsManager.debugLogEnabled },
                                set: { settingsManager.debugLogEnabled = $0 }
                            ))
                            .toggleStyle(.switch)
                            .labelsHidden()
                            .controlSize(.small)
                        }

                        if settingsManager.debugLogEnabled {
                            Divider()
                            DebugLogView()
                        }
                    }
                    .padding(12)
                } label: {
                    Label("Debug Log", systemImage: "ladybug")
                        .font(.system(size: 14, weight: .semibold))
                }

                // Per-App Injection Section
                GroupBox {
                    VStack(spacing: 12) {
                        if injectionProfiles.isEmpty {
                            HStack {
                                Text("Chưa có cấu hình nào")
                                    .font(.system(size: 12))
                                    .foregroundColor(.secondary)
                                Spacer()
                            }
                        } else {
                            ForEach(injectionProfiles, id: \.bundleId) { profile in
                                PerAppProfileRow(
                                    profile: profile,
                                    onChange: { updated in
                                        PerAppInjectionManager.shared.setProfile(updated)
                                        injectionProfiles = PerAppInjectionManager.shared.allProfiles
                                    },
                                    onDelete: {
                                        PerAppInjectionManager.shared.removeProfile(for: profile.bundleId)
                                        injectionProfiles = PerAppInjectionManager.shared.allProfiles
                                    }
                                )
                                Divider()
                            }
                        }

                        Button("Thêm app") { showAppPicker = true }
                            .buttonStyle(.bordered)
                    }
                    .padding(12)
                    .sheet(isPresented: $showAppPicker) {
                        AppPickerSheet(isPresented: $showAppPicker) { bundleId, _ in
                            let p = PerAppInjectionProfile(bundleId: bundleId)
                            PerAppInjectionManager.shared.setProfile(p)
                            injectionProfiles = PerAppInjectionManager.shared.allProfiles
                        }
                    }
                } label: {
                    Label("Cấu hình inject theo app", systemImage: "app.badge.checkmark")
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
            injectionProfiles = PerAppInjectionManager.shared.allProfiles
        }
        .onReceive(NotificationCenter.default.publisher(for: .loggingStateChanged)) { notification in
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

private struct PerAppProfileRow: View {
    let profile: PerAppInjectionProfile
    let onChange: (PerAppInjectionProfile) -> Void
    let onDelete: () -> Void

    @State private var resetHovered = false
    @State private var deleteHovered = false

    private var delayBinding: Binding<Double> {
        Binding(
            get: { Double(profile.delayPreset.rawValue) },
            set: { val in
                var p = profile
                p.delayPreset = DelayPreset(rawValue: Int(val.rounded())) ?? .none
                onChange(p)
            }
        )
    }

    private var appName: String {
        if let url = NSWorkspace.shared.urlForApplication(withBundleIdentifier: profile.bundleId) {
            return FileManager.default.displayName(atPath: url.path)
                .replacingOccurrences(of: ".app", with: "")
        }
        return profile.bundleId.components(separatedBy: ".").last ?? profile.bundleId
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            // Header row: app name + reset + delete
            HStack(spacing: 8) {
                VStack(alignment: .leading, spacing: 2) {
                    Text(appName)
                        .font(.system(size: 12, weight: .medium))
                    Text(profile.bundleId)
                        .font(.system(size: 9, design: .monospaced))
                        .foregroundColor(.secondary)
                        .lineLimit(1)
                        .truncationMode(.middle)
                }
                Spacer()
                Button {
                    PerAppInjectionManager.shared.reset(bundleId: profile.bundleId)
                    onChange(PerAppInjectionProfile(bundleId: profile.bundleId))
                } label: {
                    Image(systemName: "arrow.counterclockwise.circle.fill")
                        .font(.system(size: 14))
                        .foregroundColor(resetHovered ? .accentColor : Color(NSColor.quaternaryLabelColor))
                }
                .buttonStyle(.plain)
                .onHover { resetHovered = $0 }
                .help("Reset về mặc định")

                Button(action: onDelete) {
                    Image(systemName: "xmark.circle.fill")
                        .font(.system(size: 14))
                        .foregroundColor(deleteHovered ? .red : Color(NSColor.quaternaryLabelColor))
                }
                .buttonStyle(.plain)
                .onHover { deleteHovered = $0 }
            }

            // Delay slider
            VStack(alignment: .leading, spacing: 2) {
                HStack(spacing: 6) {
                    Text("Delay")
                        .font(.system(size: 10))
                        .foregroundColor(.secondary)
                        .frame(width: 40, alignment: .leading)
                    Slider(value: delayBinding, in: 0...4, step: 1)
                    Text(profile.delayPreset.displayName)
                        .font(.system(size: 10, weight: .medium))
                        .foregroundColor(profile.delayPreset.color)
                        .frame(width: 48, alignment: .trailing)
                }
                Text("Tăng nếu bị nuốt chữ · Giảm nếu app phản hồi nhanh")
                    .font(.system(size: 10))
                    .foregroundColor(Color(NSColor.tertiaryLabelColor))
                    .padding(.leading, 46)
            }

            // Injection method picker
            HStack(spacing: 4) {
                Text("Kiểu inject")
                    .font(.system(size: 10))
                    .foregroundColor(.secondary)
                Picker("", selection: Binding(
                    get: { profile.injectionMethod },
                    set: { method in
                        var p = profile
                        p.injectionMethod = method
                        onChange(p)
                    }
                )) {
                    ForEach(InjectionOverride.allCases, id: \.self) { method in
                        Text(method.displayName).tag(method)
                    }
                }
                .labelsHidden()
                .frame(width: 130)
                Spacer()
            }
        }
        .padding(.vertical, 8)
    }
}

private struct InlineDebugLogView: View {
    @State private var lines: [String] = []
    @State private var timer: Timer?

    private func logColor(_ line: String) -> Color {
        if line.contains("[KEY]")    { return .blue }
        if line.contains("[METHOD]") { return .orange }
        if line.contains("[QUEUE]")  { return .purple }
        if line.contains("[PERF]")   { return Color(NSColor.systemGreen) }
        return Color(NSColor.labelColor)
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            HStack {
                Text("Live Log")
                    .font(.system(size: 11, weight: .medium))
                    .foregroundColor(.secondary)
                Spacer()
                Button("Copy") {
                    NSPasteboard.general.clearContents()
                    NSPasteboard.general.setString(lines.joined(separator: "\n"), forType: .string)
                }
                .buttonStyle(.borderless)
                .font(.system(size: 11))

                Button("Clear") {
                    DebugLogger.shared.clear()
                    lines = []
                }
                .buttonStyle(.borderless)
                .font(.system(size: 11))
                .foregroundColor(.red)
            }

            ScrollViewReader { proxy in
                ScrollView {
                    VStack(alignment: .leading, spacing: 1) {
                        if lines.isEmpty {
                            Text("(no log entries)")
                                .font(.system(size: 10, design: .monospaced))
                                .foregroundColor(.secondary)
                                .padding(4)
                        } else {
                            ForEach(Array(lines.enumerated()), id: \.offset) { idx, line in
                                Text(line)
                                    .font(.system(size: 10, design: .monospaced))
                                    .foregroundColor(logColor(line))
                                    .frame(maxWidth: .infinity, alignment: .leading)
                                    .id(idx)
                            }
                        }
                    }
                    .padding(6)
                }
                .background(Color(NSColor.textBackgroundColor))
                .cornerRadius(4)
                .frame(minHeight: 120, maxHeight: 200)
                .onChange(of: lines.count) { _ in
                    if let last = lines.indices.last {
                        proxy.scrollTo(last, anchor: .bottom)
                    }
                }
            }
        }
        .onAppear {
            reload()
            timer = Timer.scheduledTimer(withTimeInterval: 1.0, repeats: true) { _ in reload() }
        }
        .onDisappear {
            timer?.invalidate()
            timer = nil
        }
    }

    private func reload() {
        let raw = DebugLogger.shared.readLog()
        lines = raw.components(separatedBy: "\n").filter { !$0.isEmpty }
    }
}
