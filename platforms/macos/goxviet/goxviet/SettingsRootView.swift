//
//  SettingsRootView.swift
//  GoxViet
//
//  Settings window rebuilt with WindowGroup-compatible glass root,
//  NavigationSplitView sidebar, and animated detail transitions.
//

import SwiftUI
import AppKit

struct EngineMetrics {
    let totalKeystrokes: UInt64
    let backspaceCount: UInt64
    let avgBufferLength: Double
}

struct SettingsRootView: View {
    // MARK: - Stored Settings
    @AppStorage("inputMethod") private var inputMethod = 0
    @AppStorage("modernToneStyle") private var modernToneStyle = false
    @AppStorage("escRestoreEnabled") private var escRestoreEnabled = true
    @AppStorage("freeToneEnabled") private var freeToneEnabled = false
    @AppStorage("instantRestoreEnabled") private var instantRestoreEnabled = true

    @AppStorage("smartModeEnabled") private var smartModeEnabled = true
    @AppStorage("com.goxviet.ime.autoDisableNonLatin") private var autoDisableForNonLatin = true
    
    // Use AppState for hideFromDock instead of @AppStorage
    @ObservedObject private var appState = AppState.shared

    // MARK: - View State
    @State private var perAppModes: [String: Bool] = [:]
    @State private var showClearConfirmation = false

    // MARK: - Body
    var body: some View {
        TabView {
            GeneralSettingsView(
                inputMethod: $inputMethod,
                modernToneStyle: $modernToneStyle,
                escRestoreEnabled: $escRestoreEnabled,
                freeToneEnabled: $freeToneEnabled,
                instantRestoreEnabled: $instantRestoreEnabled,
                autoDisableForNonLatin: $autoDisableForNonLatin
            )
            .tabItem {
                Label("General", systemImage: "gearshape")
            }

            PerAppSettingsView(
                smartModeEnabled: $smartModeEnabled,
                perAppModes: $perAppModes,
                showClearConfirmation: $showClearConfirmation,
                reloadAction: loadPerAppModes
            )
            .tabItem {
                Label("Per-App", systemImage: "app.badge")
            }

            AdvancedSettingsView(
                metrics: getEngineMetrics(),
                resetAction: resetEngineMetrics,
                openLogAction: openLogFile
            )
            .tabItem {
                Label("Advanced", systemImage: "slider.horizontal.3")
            }

            AboutSettingsView()
                .environmentObject(UpdateManager.shared)
            .tabItem {
                Label("About", systemImage: "info.circle")
            }
        }
        .frame(minWidth: 900, minHeight: 540)
        .onAppear {
            loadPerAppModes()
            syncToAppState()
        }
        .onReceive(NotificationCenter.default.publisher(for: .smartModeChanged)) { notification in
            if let newState = notification.object as? Bool {
                smartModeEnabled = newState
                loadPerAppModes()  // Refresh list
                Log.info("Settings smart mode updated: \(newState)")
            }
        }
        .onReceive(NotificationCenter.default.publisher(for: .inputMethodChanged)) { notification in
            if let method = notification.object as? Int {
                inputMethod = method
                Log.info("Settings input method updated: \(method == 0 ? "Telex" : "VNI")")
            }
        }
        .onReceive(NotificationCenter.default.publisher(for: .toneStyleChanged)) { notification in
            if let modern = notification.object as? Bool {
                modernToneStyle = modern
                Log.info("Settings tone style updated: \(modern ? "Modern" : "Traditional")")
            }
        }
    }



    // MARK: - Helpers
    private func loadPerAppModes() {
        // Show *known* applications with their effective Vietnamese typing state.
        // This reflects which apps are enabled/disabled, instead of only listing disabled overrides.
        perAppModes = AppState.shared.getKnownAppsWithStates()
    }

    private func syncToAppState() {
        AppState.shared.inputMethod = inputMethod
        AppState.shared.modernToneStyle = modernToneStyle
        AppState.shared.escRestoreEnabled = escRestoreEnabled
        AppState.shared.freeToneEnabled = freeToneEnabled
        AppState.shared.isSmartModeEnabled = smartModeEnabled
        AppState.shared.autoDisableForNonLatinEnabled = autoDisableForNonLatin
    }

    private func openLogFile() {
        if FileManager.default.fileExists(atPath: Log.logPath.path) {
            NSWorkspace.shared.open(Log.logPath)
        } else {
            Log.error("Log file not found at: \(Log.logPath.path)")
        }
    }

    private func getEngineMetrics() -> EngineMetrics {
        // TODO: Replace with actual Rust core metrics once FFI is ready.
        EngineMetrics(totalKeystrokes: 0, backspaceCount: 0, avgBufferLength: 0.0)
    }

    private func resetEngineMetrics() {
        // TODO: Wire up to Rust core reset call.
        Log.info("Metrics reset requested")
    }
}

// MARK: - Sidebar Header

private struct SidebarHeader: View {
    var body: some View {
        VStack(spacing: 12) {
            if let appIcon = NSImage(named: "AppIcon") {
                Image(nsImage: appIcon)
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .frame(width: 64, height: 64)
                    .clipShape(RoundedRectangle(cornerRadius: 18, style: .continuous))
                    .overlay(
                        RoundedRectangle(cornerRadius: 18, style: .continuous)
                            .stroke(.white.opacity(0.25), lineWidth: 0.8)
                    )
                    .shadow(color: .black.opacity(0.25), radius: 12, x: 0, y: 8)
            }

            VStack(spacing: 2) {
                Text("GoxViet")
                    .font(.headline)
                    .foregroundStyle(.primary)
                Text("Settings")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
    }
}



// MARK: - Sections

enum SettingsSection: Hashable, CaseIterable {
    case general, perApp, advanced, about

    var title: String {
        switch self {
        case .general: return "General"
        case .perApp: return "Per-App"
        case .advanced: return "Advanced"
        case .about: return "About"
        }
    }

    var icon: String {
        switch self {
        case .general: return "gearshape"
        case .perApp: return "app.badge"
        case .advanced: return "slider.horizontal.3"
        case .about: return "info.circle"
        }
    }
}

// MARK: - General Settings

private struct GeneralSettingsView: View {
    @Binding var inputMethod: Int
    @Binding var modernToneStyle: Bool
    @Binding var escRestoreEnabled: Bool
    @Binding var freeToneEnabled: Bool
    @Binding var instantRestoreEnabled: Bool
    @Binding var autoDisableForNonLatin: Bool
    
    // Use AppState for hideFromDock
    @ObservedObject private var appState = AppState.shared

    var body: some View {
        ScrollView {
            LazyVGrid(columns: [GridItem(.adaptive(minimum: 360), spacing: 20, alignment: .top)], spacing: 20) {
                // Input Method & Tone Style Group
                GroupBox(label: Label("Input Configuration", systemImage: "keyboard")) {
                    VStack(alignment: .leading, spacing: 20) {
                        // Input Method
                        VStack(alignment: .leading, spacing: 10) {
                            Text("Input Method")
                                .font(.headline)
                            
                            Picker("Input Method", selection: $inputMethod) {
                                Text("Telex").tag(0)
                                Text("VNI").tag(1)
                            }
                            .pickerStyle(.segmented)
                            .labelsHidden()
                            .onChange(of: inputMethod) { oldValue, newValue in
                                AppState.shared.inputMethod = newValue
                                InputManager.shared.setInputMethod(newValue)
                                Log.info("Input method: (newValue == 0 ? 'Telex' : 'VNI')")
                            }
                            
                            Text(inputMethodDescription)
                                .font(.caption)
                                .foregroundStyle(.secondary)
                                .fixedSize(horizontal: false, vertical: true)
                        }
                        
                        Divider()
                        
                        // Tone Placement
                        VStack(alignment: .leading, spacing: 10) {
                            Text("Tone Placement")
                                .font(.headline)
                            
                            HStack(spacing: 24) {
                                HStack {
                                    RadioButton(isSelected: !modernToneStyle)
                                    VStack(alignment: .leading, spacing: 2) {
                                        Text("Traditional")
                                            .font(.callout)
                                        Text("hòa, thủy")
                                            .font(.caption2)
                                            .foregroundStyle(.secondary)
                                    }
                                }
                                .contentShape(Rectangle())
                                .onTapGesture {
                                    modernToneStyle = false
                                    AppState.shared.modernToneStyle = false
                                    InputManager.shared.setModernToneStyle(false)
                                }
                                
                                HStack {
                                    RadioButton(isSelected: modernToneStyle)
                                    VStack(alignment: .leading, spacing: 2) {
                                        Text("Modern")
                                            .font(.callout)
                                        Text("hoà, thuỷ")
                                            .font(.caption2)
                                            .foregroundStyle(.secondary)
                                    }
                                }
                                .contentShape(Rectangle())
                                .onTapGesture {
                                    modernToneStyle = true
                                    AppState.shared.modernToneStyle = true
                                    InputManager.shared.setModernToneStyle(true)
                                }
                            }
                        }
                    }
                    .padding(12)
                    .frame(maxWidth: .infinity, alignment: .leading)
                }
                
                // Smart Features Group
                GroupBox(label: Label("Smart Features", systemImage: "sparkles")) {
                    VStack(alignment: .leading, spacing: 16) {
                        ToggleRow(
                            title: "Free tone placement",
                            description: "Allow tone marks before completing the vowel.",
                            isOn: $freeToneEnabled
                        ) { newValue in
                            AppState.shared.freeToneEnabled = newValue
                            InputManager.shared.setFreeTone(newValue)
                            Log.info("Free tone: \(newValue)")
                        }
                        
                        Divider()
                        
                        ToggleRow(
                            title: "Auto-restore English words",
                            description: "Automatically restore English words without waiting for space.",
                            isOn: $instantRestoreEnabled
                        ) { newValue in
                            AppState.shared.instantRestoreEnabled = newValue
                            InputManager.shared.setInstantRestore(newValue)
                            Log.info("Instant auto-restore: \(newValue)")
                        }
                        
                        Divider()
                        
                        VStack(alignment: .leading, spacing: 8) {
                            Toggle("Auto-disable for non-Latin keyboards", isOn: $autoDisableForNonLatin)
                                .onChange(of: autoDisableForNonLatin) { _, newValue in
                                    AppState.shared.autoDisableForNonLatinEnabled = newValue
                                    if newValue {
                                        InputSourceMonitor.shared.refresh()
                                    }
                                    Log.info("Auto-disable for non-Latin: \(newValue)")
                                }
                            
                            Text("Disable Vietnamese typing when using Japanese, Korean, or Chinese keyboards.")
                                .font(.caption)
                                .foregroundStyle(.secondary)
                                .fixedSize(horizontal: false, vertical: true)
                            
                            if InputSourceMonitor.shared.isTemporarilyDisabled,
                               let inputSourceName = InputSourceMonitor.shared.getCurrentInputSourceDisplayName() {
                                HStack(spacing: 6) {
                                    Image(systemName: "keyboard.badge.ellipsis")
                                        .foregroundStyle(.orange)
                                    Text("Temporarily disabled: \(inputSourceName)")
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                }
                                .padding(.top, 4)
                            }
                        }
                        
                        Divider()
                        
                        VStack(alignment: .leading, spacing: 8) {
                            Toggle("Hide from Dock", isOn: $appState.hideFromDock)
                            Text("GoxViet appears only in the menu bar and Settings.")
                                .font(.caption)
                                .foregroundStyle(.secondary)
                        }
                    }
                    .padding(12)
                    .frame(maxWidth: .infinity, alignment: .leading)
                }
            }
            .padding(24)
            .frame(maxWidth: 1000)
        }
        .frame(maxWidth: .infinity, alignment: .top)
    }

    // Helper for consistency
    private struct ToggleRow: View {
        let title: String
        let description: String
        @Binding var isOn: Bool
        let onChange: (Bool) -> Void
        
        var body: some View {
            VStack(alignment: .leading, spacing: 8) {
                Toggle(title, isOn: $isOn)
                    .onChange(of: isOn) { _, newValue in
                        onChange(newValue)
                    }
                Text(description)
                    .font(.caption)
                    .foregroundStyle(.secondary)
                    .fixedSize(horizontal: false, vertical: true)
            }
        }
    }
    

    private var inputMethodDescription: String {
        switch inputMethod {
        case 0:
            return "s, f, r, x, j for tone marks. aw→ă, aa→â, ee→ê, dd→đ"
        case 1:
            return "1-5 for tone marks, 6→â/ê/ô, 7→ư/ơ, 8→ă, 9→đ"
        default:
            return ""
        }
    }
}

private struct RadioButton: View {
    let isSelected: Bool
    
    var body: some View {
        Circle()
            .stroke(Color.accentColor, lineWidth: 2)
            .frame(width: 16, height: 16)
            .overlay(
                isSelected ? Circle()
                    .fill(Color.accentColor)
                    .frame(width: 6, height: 6)
                : nil
            )
    }
}

// MARK: - Per-App Settings

private struct PerAppSettingsView: View {
    @Binding var smartModeEnabled: Bool
    @Binding var perAppModes: [String: Bool]
    @Binding var showClearConfirmation: Bool

    let reloadAction: () -> Void

    var body: some View {
        ScrollView {
            LazyVGrid(columns: [GridItem(.adaptive(minimum: 360), spacing: 20, alignment: .top)], spacing: 20) {
                // Smart Mode Configuration
                GroupBox(label: Label("Smart Mode Configuration", systemImage: "brain")) {
                    VStack(alignment: .leading, spacing: 16) {
                        HStack {
                            Toggle("Enable Smart Per-App Mode", isOn: $smartModeEnabled)
                                .toggleStyle(.switch)
                                .onChange(of: smartModeEnabled) { _, newValue in
                                    AppState.shared.isSmartModeEnabled = newValue
                                    if newValue {
                                        PerAppModeManager.shared.refresh()
                                    }
                                    Log.info("Smart mode: \(newValue)")
                                }
                            Spacer()
                        }
                        
                        Text("When enabled, GoxViet automatically remembers your typing preference (On/Off) for each application individually.")
                            .font(.subheadline)
                            .foregroundStyle(.secondary)
                            .fixedSize(horizontal: false, vertical: true)
                        
                        HStack(spacing: 12) {
                            Text("Example:")
                                .fontWeight(.semibold)
                                .font(.caption)
                            Text("Vietnamese ON in Mail")
                                .font(.caption)
                                .padding(.horizontal, 6)
                                .padding(.vertical, 2)
                                .background(Color.green.opacity(0.1))
                                .foregroundStyle(.green)
                                .cornerRadius(4)
                            Text("OFF in Terminal")
                                .font(.caption)
                                .padding(.horizontal, 6)
                                .padding(.vertical, 2)
                                .background(Color.secondary.opacity(0.1))
                                .foregroundStyle(.secondary)
                                .cornerRadius(4)
                        }
                    }
                    .padding(12)
                    .frame(maxWidth: .infinity, alignment: .leading)
                }
                
                // Saved Applications
                GroupBox(label: HStack {
                    Label("Saved Applications", systemImage: "list.bullet")
                    Spacer()
                    Text("\(perAppModes.count) / \(AppState.shared.getPerAppModesCapacity())")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }) {
                    VStack(alignment: .leading, spacing: 0) {
                        if perAppModes.isEmpty {
                            VStack(spacing: 16) {
                                Image(systemName: "app.dashed")
                                    .font(.system(size: 32))
                                    .foregroundStyle(.tertiary)
                                Text("No saved applications")
                                    .font(.callout)
                                    .foregroundStyle(.secondary)
                                Text("Toggle Vietnamese input in any app to save its state here.")
                                    .font(.caption)
                                    .foregroundStyle(.tertiary)
                                    .multilineTextAlignment(.center)
                            }
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 40)
                        } else {
                            ScrollView {
                                LazyVStack(spacing: 0) {
                                    ForEach(Array(perAppModes.keys.sorted()), id: \.self) { bundleId in
                                        VStack(spacing: 0) {
                                            PerAppModeRow(bundleId: bundleId, isEnabled: perAppModes[bundleId] ?? false, onRemove: {
                                                AppState.shared.clearPerAppMode(bundleId: bundleId)
                                                reloadAction()
                                            })
                                            .padding(.horizontal, 12)
                                            .padding(.vertical, 8)
                                            
                                            if bundleId != perAppModes.keys.sorted().last {
                                                Divider()
                                                    .padding(.leading, 44)
                                            }
                                        }
                                    }
                                }
                                .background(Color(NSColor.controlBackgroundColor).opacity(0.5))
                            }
                            .frame(maxHeight: 300)
                            .cornerRadius(6)
                            
                            HStack {
                                Spacer()
                                Button("Clear All Settings", role: .destructive) {
                                    showClearConfirmation = true
                                }
                                .font(.caption)
                                .padding(.top, 12)
                            }
                        }
                    }
                    .padding(12)
                    .frame(maxWidth: .infinity, alignment: .leading)
                }
            }
            .padding(24)
            .frame(maxWidth: 1000)
        }
        .frame(maxWidth: .infinity, alignment: .top)
        .alert("Clear All Settings?", isPresented: $showClearConfirmation) {
            Button("Cancel", role: .cancel) {}
            Button("Clear All", role: .destructive) {
                AppState.shared.clearAllPerAppModes()
                reloadAction()
            }
        } message: {
            Text("This will remove all saved per-app Vietnamese input states.")
        }
        .onAppear {
            reloadAction()
        }
    }
}

private struct PerAppModeRow: View {
    let bundleId: String
    let isEnabled: Bool
    let onRemove: () -> Void
    
    var displayName: String {
        AppState.shared.getAppName(bundleId: bundleId)
    }
    
    var body: some View {
        HStack(spacing: 12) {
            Circle()
                .fill(isEnabled ? Color.green.opacity(0.3) : Color.gray.opacity(0.2))
                .frame(width: 8, height: 8)
            
            VStack(alignment: .leading, spacing: 2) {
                Text(displayName)
                    .font(.callout)
                Text(bundleId)
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }
            
            Spacer()
            
            HStack(spacing: 8) {
                Text(isEnabled ? "ON" : "OFF")
                    .font(.caption)
                    .fontWeight(.semibold)
                    .foregroundStyle(isEnabled ? .green : .gray)
                
                Button {
                    onRemove()
                } label: {
                    Image(systemName: "xmark.circle.fill")
                    .foregroundStyle(.secondary)
                    .font(.caption)
                }
                .buttonStyle(.plain)
                .help("Remove this app")
            }
        }
        .padding(.vertical, 8)
        .padding(.horizontal, 10)
        .background(.fill.opacity(0.3))
        .cornerRadius(6)
    }
}

// MARK: - Advanced Settings

private struct AdvancedSettingsView: View {
    let metrics: EngineMetrics
    let resetAction: () -> Void
    let openLogAction: () -> Void

    @State private var currentShortcut = InputManager.shared.getCurrentShortcut()
    @State private var showRecordingSheet = false

    var body: some View {
        ScrollView {
            LazyVGrid(columns: [GridItem(.adaptive(minimum: 360), spacing: 20, alignment: .top)], spacing: 20) {
                // Keyboard Shortcuts Group
                GroupBox(label: Label("Keyboard Shortcuts", systemImage: "command")) {
                    VStack(alignment: .leading, spacing: 16) {
                        HStack {
                            Text("Toggle Input Source")
                                .font(.body)
                            Spacer()
                            Button(action: { showRecordingSheet = true }) {
                                HStack(spacing: 8) {
                                    Text(currentShortcut.displayString)
                                        .font(.system(.body, design: .monospaced))
                                        .fontWeight(.semibold)
                                    Image(systemName: "keyboard")
                                }
                                .padding(.horizontal, 10)
                                .padding(.vertical, 5)
                            }
                        }
                        
                        Text("Click the button above to record a new shortcut. Supports Control, Option, Shift, Command, and Fn modifiers.")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                            .fixedSize(horizontal: false, vertical: true)
                    }
                    .padding(12)
                    .frame(maxWidth: .infinity, alignment: .leading)
                }
                
                // Performance Metrics Group
                GroupBox(label: Label("Engine Performance", systemImage: "gauge.high")) {
                    VStack(alignment: .leading, spacing: 16) {
                        HStack(spacing: 40) {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Total Keystrokes")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                                Text(String(metrics.totalKeystrokes))
                                    .font(.title2.monospaced())
                            }
                            
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Backspace Count")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                                Text(String(metrics.backspaceCount))
                                    .font(.title2.monospaced())
                            }
                            
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Avg Buffer")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                                Text(String(format: "%.1f", metrics.avgBufferLength))
                                    .font(.title2.monospaced())
                            }
                        }
                        
                        Divider()
                        
                        Button("Reset Statistics") {
                            resetAction()
                        }
                        .font(.caption)
                    }
                    .padding(12)
                    .frame(maxWidth: .infinity, alignment: .leading)
                }
                
                #if DEBUG
                GroupBox(label: Label("Debug", systemImage: "ladybug")) {
                    HStack {
                        VStack(alignment: .leading, spacing: 4) {
                            Text("Logs")
                                .font(.headline)
                            Text(Log.logPath.path)
                                .font(.caption2.monospaced())
                                .foregroundStyle(.secondary)
                                .lineLimit(1)
                                .truncationMode(.middle)
                        }
                        Spacer()
                        Button("Open Log File", action: openLogAction)
                    }
                    .padding(12)
                }
                #endif
            }
            .padding(24)
            .frame(maxWidth: 1000)
        }
        .frame(maxWidth: .infinity, alignment: .top)
        .sheet(isPresented: $showRecordingSheet) {
            ShortcutRecordingSheet(
                isPresented: $showRecordingSheet,
                onComplete: { shortcut in
                    handleShortcutCaptured(shortcut)
                }
            )
        }
        .onReceive(NotificationCenter.default.publisher(for: .shortcutChanged)) { _ in
            currentShortcut = InputManager.shared.getCurrentShortcut()
        }
    }
}

private struct MetricRow: View {
    let label: String
    let value: String
    let icon: String
    
    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .font(.callout)
                .foregroundStyle(.blue)
                .frame(width: 20)
            
            Text(label)
                .font(.callout)
            
            Spacer()
            
            Text(value)
                .font(.system(.callout, design: .monospaced))
                .fontWeight(.semibold)
        }
        .padding(.vertical, 6)
    }
}

private extension AdvancedSettingsView {
    func handleShortcutCaptured(_ shortcut: KeyboardShortcut) {
        guard shortcut.isValid else {
            return
        }

        currentShortcut = shortcut
        InputManager.shared.setShortcut(shortcut)
        showRecordingSheet = false
    }
}

private struct ShortcutRecordingSheet: View {
    @Binding var isPresented: Bool
    let onComplete: (KeyboardShortcut) -> Void
    
    @State private var isRecording = true
    @State private var statusMessage = "Press the keyboard shortcut you want to use…"
    @State private var conflictWarning: String?
    @State private var capturedShortcut: KeyboardShortcut?
    
    private var capturedDisplayString: String {
        capturedShortcut?.displayString ?? ""
    }

    var body: some View {
        VStack(spacing: 20) {
            VStack(alignment: .leading, spacing: 12) {
                HStack {
                    Image(systemName: "keyboard.fill")
                        .font(.title2)
                        .foregroundStyle(.blue)
                    
                    VStack(alignment: .leading, spacing: 2) {
                        Text("Record New Shortcut")
                            .font(.headline)
                        Text("Supports Control, Option, Shift, Command, and Fn")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                }
                .padding(.bottom, 8)
                
                Divider()

                Text("Cách ghi nhanh:\n1) Giữ các phím Control/Option/Shift/Command/Fn,\n2) Nhấn phím chữ/số. Nếu muốn chỉ dùng phím bổ trợ (ví dụ Command+Shift), giữ nguyên ~0.35s, không nhấn thêm phím.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
                    .padding(.bottom, 4)

                Text("Cảnh báo: Shortcut có thể ghi đè phím tắt của ứng dụng khác, hãy chọn cẩn trọng.")
                    .font(.caption)
                    .foregroundStyle(.orange)
                    .fixedSize(horizontal: false, vertical: true)
                
                if capturedShortcut != nil {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Captured Shortcut:")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        
                        HStack {
                            Text(capturedDisplayString)
                                .font(.title2.monospaced())
                                .fontWeight(.semibold)
                                .padding(.horizontal, 16)
                                .padding(.vertical, 10)
                                .background(.blue.opacity(0.1))
                                .cornerRadius(8)
                            
                            Spacer()
                        }
                        
                        if let conflict = conflictWarning {
                            HStack(alignment: .top, spacing: 10) {
                                Image(systemName: "exclamationmark.triangle.fill")
                                    .foregroundStyle(.orange)
                                    .font(.body)
                                
                                Text(conflict)
                                    .font(.caption)
                                    .foregroundStyle(.primary)
                                    .fixedSize(horizontal: false, vertical: true)
                            }
                            .padding(12)
                            .frame(maxWidth: .infinity, alignment: .leading)
                            .background(Color.orange.opacity(0.1))
                            .overlay(
                                RoundedRectangle(cornerRadius: 8)
                                    .stroke(Color.orange.opacity(0.3), lineWidth: 1)
                            )
                            .cornerRadius(8)
                            .help(conflict)
                        }
                    }
                } else {
                    HStack(spacing: 12) {
                        ProgressView()
                        Text(statusMessage)
                            .font(.body)
                            .foregroundStyle(.secondary)
                    }
                    .padding(.vertical, 12)
                }
            }
            .padding(20)
            .background(.regularMaterial)
            .cornerRadius(12)
            
            HStack(spacing: 12) {
                Button("Cancel") {
                    isPresented = false
                }
                .buttonStyle(.bordered)
                
                Spacer()
                
                if capturedShortcut != nil {
                    Button("Record Again") {
                        capturedShortcut = nil
                        conflictWarning = nil
                        isRecording = true
                    }
                    .buttonStyle(.bordered)
                    
                    Button("Apply") {
                        if let shortcut = capturedShortcut {
                            onComplete(shortcut)
                        }
                    }
                    .buttonStyle(.borderedProminent)
                    .tint(.green)
                }
            }
        }
        .padding(24)
        .frame(minWidth: 420, idealWidth: 480)
        .overlay(
            ShortcutRecorder(
                isRecording: $isRecording,
                onComplete: { shortcut in
                    capturedShortcut = shortcut
                    conflictWarning = shortcut.conflictInfo?.message
                    isRecording = false
                },
                onCancel: {}
            )
            .allowsHitTesting(false)
        )
    }
}

// MARK: - About View

private struct AboutSettingsView: View {
    @EnvironmentObject private var updateManager: UpdateManager
    
    var body: some View {
        ScrollView {
            LazyVGrid(columns: [GridItem(.adaptive(minimum: 360), spacing: 20, alignment: .top)], spacing: 20) {
                
                // App Information Group
                GroupBox() {
                    VStack(spacing: 20) {
                        // Icon & Title
                        VStack(spacing: 8) {
                            if let appIcon = NSImage(named: "AppIcon") {
                                Image(nsImage: appIcon)
                                    .resizable()
                                    .aspectRatio(contentMode: .fit)
                                    .frame(width: 80, height: 80)
                                    .clipShape(RoundedRectangle(cornerRadius: 20, style: .continuous))
                                    .overlay(
                                        RoundedRectangle(cornerRadius: 20, style: .continuous)
                                            .stroke(.white.opacity(0.1), lineWidth: 1)
                                    )
                                    .shadow(color: .black.opacity(0.2), radius: 10, x: 0, y: 5)
                            }
                            
                            VStack(spacing: 4) {
                                Text("Gõ Việt")
                                    .font(.title2.bold())
                                
                                if let version = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String,
                                   let build = Bundle.main.object(forInfoDictionaryKey: "CFBundleVersion") as? String {
                                    Text("Version \(version) (\(build))")
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                }
                            }
                        }
                        
                        Text("A modern Vietnamese input method editor for macOS, built with performance and simplicity in mind.")
                            .font(.callout)
                            .multilineTextAlignment(.center)
                            .foregroundStyle(.secondary)
                            .fixedSize(horizontal: false, vertical: true)
                            .padding(.horizontal, 8)
                        
                        Divider()
                        
                        // Update Status (dynamic based on state)
                        updateStatusView
                        
                        // External Links
                        HStack(spacing: 16) {
                            Link(destination: URL(string: "https://github.com/nihmtaho/goxviet-ime")!) {
                                Label("GitHub", systemImage: "link")
                            }
                            Link(destination: URL(string: "https://github.com/nihmtaho/goxviet-ime/issues")!) {
                                Label("Report Issue", systemImage: "exclamationmark.bubble")
                            }
                        }
                        .font(.subheadline)
                        .buttonStyle(.link)
                        
                        Text("© 2025 GoxViet. All rights reserved.")
                            .font(.caption2)
                            .foregroundStyle(.tertiary)
                            .padding(.top, 4)
                    }
                    .padding(20)
                    .frame(maxWidth: .infinity)
                }
                
                // Features Group
                GroupBox() {
                    VStack(alignment: .leading, spacing: 16) {
                        let features: [(String, Color, String, String)] = [
                            ("bolt.fill", .yellow, "High Performance", "Rust core delivers <16ms latency for instant typing"),
                            ("brain.fill", .pink, "Smart Per-App Mode", "Automatically remembers settings for each application"),
                            ("keyboard.badge.ellipsis", .blue, "Flexible Input", "Support both Telex and VNI typing methods"),
                            ("textformat", .green, "Tone Placement", "Choose between modern and traditional tone styles"),
                            ("sparkles", .orange, "Multi-Language", "Auto-disable for Japanese, Korean, Chinese keyboards")
                        ]
                        
                        ForEach(Array(features.enumerated()), id: \.offset) { index, feature in
                            HStack(alignment: .top, spacing: 12) {
                                Image(systemName: feature.0)
                                    .font(.title3)
                                    .foregroundStyle(feature.1)
                                    .frame(width: 24)
                                
                                VStack(alignment: .leading, spacing: 2) {
                                    Text(feature.2)
                                        .font(.subheadline.weight(.semibold))
                                    Text(feature.3)
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                        .fixedSize(horizontal: false, vertical: true)
                                }
                            }
                            .padding(.vertical, 4)
                            
                            if index < features.count - 1 {
                                Divider()
                                    .padding(.leading, 36)
                            }
                        }
                    }
                    .padding(16)
                    .frame(maxWidth: .infinity, alignment: .leading)
                }
            }
            .padding(24)
            .frame(maxWidth: 1000)
        }
        .frame(maxWidth: .infinity, alignment: .top)
        .onAppear {
            // Auto-check for updates when About tab appears if idle
            if updateManager.updateState == .idle {
                updateManager.checkForUpdates(userInitiated: false)
            }
        }
    }
    
    // MARK: - Update Status Views
    
    @ViewBuilder
    private var updateStatusView: some View {
        VStack(spacing: 16) {
            switch updateManager.updateState {
            case .idle:
                idleUpdateView
            case .checking:
                checkingUpdateView
            case .updateAvailable:
                updateAvailableView
            case .downloading:
                downloadingView
            case .readyToInstall:
                readyToInstallView
            case .upToDate:
                upToDateView
            case .error:
                errorView
            }
        }
    }
    
    private var idleUpdateView: some View {
        Button {
            updateManager.checkForUpdates(userInitiated: true)
        } label: {
            HStack {
                Image(systemName: "arrow.down.circle.fill")
                Text("Check for Updates")
            }
            .frame(maxWidth: .infinity)
            .padding(.vertical, 6)
        }
        .buttonStyle(.borderedProminent)
        .tint(.blue)
        .controlSize(.large)
    }
    
    private var checkingUpdateView: some View {
        VStack(spacing: 12) {
            ProgressView()
                .scaleEffect(1.2)
            Text("Checking for updates...")
                .font(.callout)
                .foregroundStyle(.secondary)
        }
        .frame(height: 60)
    }
    
    private var updateAvailableView: some View {
        VStack(spacing: 12) {
            HStack(spacing: 8) {
                Image(systemName: "arrow.down.circle.fill")
                    .foregroundStyle(.green)
                if let latestVersion = updateManager.latestVersion {
                    Text("Version \(latestVersion) Available")
                        .font(.subheadline.weight(.semibold))
                }
            }
            
            Button {
                updateManager.downloadUpdate()
            } label: {
                HStack {
                    Image(systemName: "arrow.down.circle")
                    Text("Download Update")
                }
                .frame(maxWidth: .infinity)
                .padding(.vertical, 6)
            }
            .buttonStyle(.borderedProminent)
            .tint(.green)
            .controlSize(.large)
        }
    }
    
    private var downloadingView: some View {
        VStack(spacing: 12) {
            // Compact circular progress
            ZStack {
                Circle()
                    .stroke(Color.blue.opacity(0.2), lineWidth: 8)
                    .frame(width: 80, height: 80)
                
                Circle()
                    .trim(from: 0, to: updateManager.downloadProgress)
                    .stroke(
                        LinearGradient(
                            colors: [.blue, .purple],
                            startPoint: .topLeading,
                            endPoint: .bottomTrailing
                        ),
                        style: StrokeStyle(lineWidth: 8, lineCap: .round)
                    )
                    .frame(width: 80, height: 80)
                    .rotationEffect(.degrees(-90))
                    .animation(.easeInOut(duration: 0.3), value: updateManager.downloadProgress)
                
                Text("\(Int(updateManager.downloadProgress * 100))%")
                    .font(.system(size: 18, weight: .bold))
                    .monospacedDigit()
            }
            
            Text(updateManager.statusMessage)
                .font(.caption)
                .foregroundStyle(.secondary)
            
            Button {
                updateManager.cancelDownload()
            } label: {
                Text("Cancel")
                    .foregroundStyle(.red)
            }
            .buttonStyle(.plain)
        }
    }
    
    private var readyToInstallView: some View {
        VStack(spacing: 12) {
            HStack(spacing: 8) {
                Image(systemName: "checkmark.circle.fill")
                    .foregroundStyle(.green)
                if let latestVersion = updateManager.latestVersion {
                    Text("Version \(latestVersion) Ready")
                        .font(.subheadline.weight(.semibold))
                }
            }
            
            Button {
                // Installation happens automatically when download completes
                // This button is just informational/disabled in production
                #if DEBUG
                // In debug, could reset for testing
                #endif
            } label: {
                HStack {
                    Image(systemName: "arrow.clockwise.circle")
                    Text("Install & Relaunch")
                }
                .frame(maxWidth: .infinity)
                .padding(.vertical, 6)
            }
            .buttonStyle(.borderedProminent)
            .tint(.green)
            .controlSize(.large)
            .disabled(true) // Installation is automatic
            
            Text("Installation will begin automatically")
                .font(.caption)
                .foregroundStyle(.secondary)
        }
    }
    
    private var upToDateView: some View {
        VStack(spacing: 8) {
            HStack(spacing: 8) {
                Image(systemName: "checkmark.circle.fill")
                    .foregroundStyle(.green)
                Text("You're up to date")
                    .font(.subheadline.weight(.semibold))
            }
            
            if let lastChecked = updateManager.lastChecked {
                Text("Last checked: \(lastChecked, style: .relative) ago")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
    }
    
    private var errorView: some View {
        VStack(spacing: 12) {
            HStack(spacing: 8) {
                Image(systemName: "exclamationmark.triangle.fill")
                    .foregroundStyle(.orange)
                Text("Update check failed")
                    .font(.subheadline.weight(.semibold))
            }
            
            Text(updateManager.statusMessage)
                .font(.caption)
                .foregroundStyle(.secondary)
                .multilineTextAlignment(.center)
            
            Button {
                updateManager.checkForUpdates(userInitiated: true)
            } label: {
                Text("Try Again")
            }
            .buttonStyle(.bordered)
        }
    }
    
    private func currentVersion() -> String? {
        guard let version = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String,
              let build = Bundle.main.object(forInfoDictionaryKey: "CFBundleVersion") as? String else {
            return nil
        }
        return "\(version) (\(build))"
    }

    private func openReleasePage() {
        if let url = URL(string: "https://github.com/nihmtaho/goxviet-ime/releases/latest") {
            NSWorkspace.shared.open(url)
        }
    }
}

// MARK: - Shared Components

private struct FeatureRow: View {
    let icon: String
    let color: Color
    let text: String

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .foregroundStyle(color)
                .frame(width: 24)
            Text(text)
                .font(.callout)
        }
    }
}

// MARK: - Background

private struct SettingsGlassBackground: View {
    var body: some View {
        Rectangle()
            .fill(.ultraThinMaterial)
            .overlay(
                LinearGradient(
                    colors: [
                        Color.white.opacity(0.24),
                        Color.blue.opacity(0.12),
                        Color.pink.opacity(0.08)
                    ],
                    startPoint: .topLeading,
                    endPoint: .bottomTrailing
                )
            )
            .overlay(
                RadialGradient(
                    colors: [Color.white.opacity(0.25), .clear],
                    center: .topLeading,
                    startRadius: 20,
                    endRadius: 500
                )
            )
            .ignoresSafeArea()
    }
}


// MARK: - Preview

#Preview {
    SettingsRootView()
        .frame(width: 800, height: 650)
}
