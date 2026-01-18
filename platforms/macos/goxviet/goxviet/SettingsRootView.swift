//
//  SettingsRootView.swift
//  GoxViet
//
//  Settings window rebuilt with WindowGroup-compatible glass root,
//  NavigationSplitView sidebar, and animated detail transitions.
//

import SwiftUI
import AppKit

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
    @State private var selection: SettingsSection? = .general
    @State private var perAppModes: [String: Bool] = [:]
    @State private var showClearConfirmation = false

    // MARK: - Body
    var body: some View {
        VStack(spacing: 0) {
            // Tab Bar
            HStack(spacing: 0) {
                ForEach(SettingsSection.allCases, id: \.self) { section in
                    TabButton(
                        section: section,
                        isSelected: selection == section,
                        action: {
                            withAnimation(.easeInOut(duration: 0.2)) {
                                selection = section
                            }
                        }
                    )
                }
            }
            .padding(.horizontal, 20)
            .padding(.top, 12)
            .padding(.bottom, 12)
            
            Divider()
            
            // Content Area
            ScrollView {
                detailContent
                    .padding(32)
                    .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
            }
        }
        .frame(minWidth: 700, minHeight: 600)
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



    private var detailContent: some View {
        let currentSelection = selection ?? .general
        return Group {
            switch currentSelection {
            case .general:
                GeneralSettingsView(
                    inputMethod: $inputMethod,
                    modernToneStyle: $modernToneStyle,
                    escRestoreEnabled: $escRestoreEnabled,
                    freeToneEnabled: $freeToneEnabled,
                    instantRestoreEnabled: $instantRestoreEnabled,
                    autoDisableForNonLatin: $autoDisableForNonLatin
                )
            case .perApp:
                PerAppSettingsView(
                    smartModeEnabled: $smartModeEnabled,
                    perAppModes: $perAppModes,
                    showClearConfirmation: $showClearConfirmation,
                    reloadAction: loadPerAppModes
                )
            case .advanced:
                AdvancedSettingsView(
                    metrics: getEngineMetrics(),
                    resetAction: resetEngineMetrics,
                    openLogAction: openLogFile
                )
            case .about:
                AboutSettingsView()
            }
        }
        .id(currentSelection)
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
                Text("Settings")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
    }
}

// MARK: - Tab Button

private struct TabButton: View {
    let section: SettingsSection
    let isSelected: Bool
    let action: () -> Void
    
    var body: some View {
        Button(action: action) {
            VStack(spacing: 4) {
                Image(systemName: section.icon)
                    .font(.system(size: 16))
                    .foregroundStyle(isSelected ? .primary : .secondary)
                
                Text(section.title)
                    .font(.system(size: 11, weight: isSelected ? .semibold : .regular))
                    .foregroundStyle(isSelected ? .primary : .secondary)
            }
            .frame(maxWidth: .infinity)
            .padding(.vertical, 9)
            .contentShape(Rectangle())
            .background(
                RoundedRectangle(cornerRadius: 8, style: .continuous)
                    .fill(isSelected ? Color.accentColor.opacity(0.15) : Color.clear)
            )
            .overlay(
                RoundedRectangle(cornerRadius: 8, style: .continuous)
                    .strokeBorder(isSelected ? Color.accentColor.opacity(0.3) : Color.clear, lineWidth: 1)
            )
        }
        .buttonStyle(.plain)
        .focusable(false)
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
        Form {
            Section {
                Picker("Input Method", selection: $inputMethod) {
                    Text("Telex").tag(0)
                    Text("VNI").tag(1)
                }
                .pickerStyle(.segmented)
                .onChange(of: inputMethod) { oldValue, newValue in
                    AppState.shared.inputMethod = newValue
                    InputManager.shared.setInputMethod(newValue)
                    Log.info("Input method: (newValue == 0 ? 'Telex' : 'VNI')")
                }
                Text(inputMethodDescription)
                    .font(.caption)
            } header: {
                Label("Input Method", systemImage: "keyboard")
            }

            Section {
                Picker("Tone Style", selection: $modernToneStyle) {
                    Text("Traditional (hòa, thủy)").tag(false)
                    Text("Modern (hoà, thuỷ)").tag(true)
                }
                .pickerStyle(.radioGroup)
                .onChange(of: modernToneStyle) { _, newValue in
                    AppState.shared.modernToneStyle = newValue
                    InputManager.shared.setModernToneStyle(newValue)
                    Log.info("Tone style: \(newValue ? "Modern" : "Traditional")")
                }

                Text("Choose where tone marks are placed in compound vowels.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            } header: {
                Label("Tone Placement", systemImage: "textformat")
            }

            Section {
                Toggle("Free tone placement", isOn: $freeToneEnabled)
                    .onChange(of: freeToneEnabled) { _, newValue in
                        AppState.shared.freeToneEnabled = newValue
                        InputManager.shared.setFreeTone(newValue)
                        Log.info("Free tone: \(newValue)")
                    }

                Text("Allow tone marks before completing the vowel.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
                
                Toggle("Auto-restore English words", isOn: $instantRestoreEnabled)
                    .onChange(of: instantRestoreEnabled) { _, newValue in
                        AppState.shared.instantRestoreEnabled = newValue
                        InputManager.shared.setInstantRestore(newValue)
                        Log.info("Instant auto-restore: \(newValue)")
                    }
                
                Text("Automatically restore English words (like \"Windows\", \"GitHub\") without waiting for space.")
                    .font(.caption)
                    .foregroundStyle(.secondary)

                // --- Multi-Language Support ---
                Toggle("Auto-disable for non-Latin keyboards", isOn: $autoDisableForNonLatin)
                    .onChange(of: autoDisableForNonLatin) { _, newValue in
                        AppState.shared.autoDisableForNonLatinEnabled = newValue
                        if newValue {
                            InputSourceMonitor.shared.refresh()
                        }
                        Log.info("Auto-disable for non-Latin: \(newValue)")
                    }

                Text("Temporarily disable Vietnamese typing when using Japanese, Korean, Chinese, or other non-Latin input methods.")
                    .font(.caption)
                    .foregroundStyle(.secondary)

                if InputSourceMonitor.shared.isTemporarilyDisabled,
                   let inputSourceName = InputSourceMonitor.shared.getCurrentInputSourceDisplayName() {
                    HStack(spacing: 8) {
                        Image(systemName: "keyboard.badge.ellipsis")
                            .foregroundStyle(.orange)
                        Text("Vietnamese temporarily disabled")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        Spacer()
                        Text(inputSourceName)
                            .font(.caption)
                            .foregroundStyle(.orange)
                    }
                    .padding(.vertical, 4)
                }
            } header: {
                Label("Smart Features", systemImage: "sparkles")
            }
            
            Section {
                Toggle("Hide from Dock", isOn: $appState.hideFromDock)
                
                Text("When enabled, GoxViet will only appear in the menu bar.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            } header: {
                Label("Appearance", systemImage: "dock.rectangle")
            }
        }
        .formStyle(.grouped)
        .scrollContentBackground(.hidden)
    }

    private var inputMethodDescription: String {
        switch inputMethod {
        case 0:
            return "Telex: s, f, r, x, j for tone marks. aw → ă, aa → â, ee → ê, dd → đ."
        case 1:
            return "VNI: 1-5 for tone marks, 6 → â/ê/ô, 7 → ư/ơ, 8 → ă, 9 → đ."
        default:
            return ""
        }
    }
}

// MARK: - Per-App Settings

private struct PerAppSettingsView: View {
    @Binding var smartModeEnabled: Bool
    @Binding var perAppModes: [String: Bool]
    @Binding var showClearConfirmation: Bool

    let reloadAction: () -> Void

    var body: some View {
        Form {
            Section {
                Toggle("Enable Smart Per-App Mode", isOn: $smartModeEnabled)
                    .onChange(of: smartModeEnabled) { _, newValue in
                        AppState.shared.isSmartModeEnabled = newValue
                        if newValue {
                            PerAppModeManager.shared.refresh()
                        }
                        Log.info("Smart mode: \(newValue)")
                    }

                Text("Automatically remember Vietnamese input state for each application.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            } header: {
                Label("Smart Mode", systemImage: "brain")
            }

            Section {
                if perAppModes.isEmpty {
                    ContentUnavailableView(
                        "No Applications Detected",
                        systemImage: "app.dashed",
                        description: Text("Saved Applications will show which apps have Vietnamese typing enabled or disabled.")
                    )
                } else {
                    ForEach(Array(perAppModes.keys.sorted()), id: \.self) { bundleId in
                        LabeledContent {
                            HStack(spacing: 8) {
                                let isEnabled = perAppModes[bundleId] ?? true
                                Text(isEnabled ? "Enabled" : "Disabled")
                                    .font(.caption)
                                    .foregroundStyle(isEnabled ? .green : .red)

                                Button {
                                    // Remove saved visibility + any override for this app
                                    AppState.shared.clearPerAppMode(bundleId: bundleId)
                                    reloadAction()
                                } label: {
                                    Image(systemName: "xmark.circle.fill")
                                        .foregroundStyle(.secondary)
                                }
                                .buttonStyle(.plain)
                                .help("Remove this app")
                            }
                        } label: {
                            VStack(alignment: .leading, spacing: 2) {
                                Text(AppState.shared.getAppName(bundleId: bundleId))
                                Text(bundleId)
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                        }
                    }

                    Button("Clear All", role: .destructive) {
                        showClearConfirmation = true
                    }
                    .alert("Clear All Settings?", isPresented: $showClearConfirmation) {
                        Button("Cancel", role: .cancel) {}
                        Button("Clear All", role: .destructive) {
                            AppState.shared.clearAllPerAppModes()
                            reloadAction()
                        }
                    } message: {
                        Text("This will remove all saved per-app Vietnamese input states.")
                    }
                }
            } header: {
                HStack {
                    Label("Saved Applications", systemImage: "list.bullet")
                    Spacer()
                    Text("\(perAppModes.count) / \(AppState.shared.getPerAppModesCapacity())")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }
        }
        .formStyle(.grouped)
        .scrollContentBackground(.hidden)
        .onAppear {
            reloadAction()
        }
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
        Form {
            Section {
                LabeledContent("Toggle Input") {
                    Button(action: { showRecordingSheet = true }) {
                        HStack(spacing: 10) {
                            Text(currentShortcut.displayString)
                                .font(.body.monospaced())
                                .fontWeight(.semibold)
                            
                            Spacer()
                            
                            Image(systemName: "pencil.circle.fill")
                                .font(.body)
                                .foregroundStyle(.blue)
                        }
                        .padding(.horizontal, 12)
                        .padding(.vertical, 8)
                        .background(.quaternary)
                        .cornerRadius(8)
                    }
                    .buttonStyle(.plain)
                }
                
                Text("Click to record a new shortcut")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            } header: {
                Label("Keyboard Shortcuts", systemImage: "command")
            }

            Section {
                LabeledContent("Total Keystrokes") {
                    Text("\(metrics.totalKeystrokes)")
                        .font(.title3.monospacedDigit())
                        .fontWeight(.semibold)
                }

                LabeledContent("Backspace Count") {
                    Text("\(metrics.backspaceCount)")
                        .font(.title3.monospacedDigit())
                        .fontWeight(.semibold)
                }

                LabeledContent("Avg Buffer Length") {
                    Text(String(format: "%.1f", metrics.avgBufferLength))
                        .font(.title3.monospacedDigit())
                        .fontWeight(.semibold)
                }

                Button("Reset Statistics") {
                    resetAction()
                }
            } header: {
                Label("Performance", systemImage: "gauge.high")
            }

            #if DEBUG
            Section {
                Button("Open Log File") {
                    openLogAction()
                }

                Text("Location: ~/Library/Logs/GoxViet/keyboard.log")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            } header: {
                Label("Debug", systemImage: "ladybug")
            }
            #endif
        }
        .formStyle(.grouped)
        .scrollContentBackground(.hidden)
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
                            HStack(spacing: 8) {
                                Image(systemName: "exclamationmark.circle.fill")
                                    .foregroundStyle(.orange)
                                Text(conflict)
                                    .font(.caption)
                                    .foregroundStyle(.orange)
                            }
                            .padding(.top, 4)
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
    var body: some View {
        HStack(spacing: 40) {
            // Left side - App info
            VStack(spacing: 20) {
                // App Icon
                if let appIcon = NSImage(named: "AppIcon") {
                    Image(nsImage: appIcon)
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                        .frame(width: 100, height: 100)
                        .clipShape(RoundedRectangle(cornerRadius: 24, style: .continuous))
                        .overlay(
                            RoundedRectangle(cornerRadius: 24, style: .continuous)
                                .stroke(.white.opacity(0.22), lineWidth: 1)
                        )
                        .shadow(color: .black.opacity(0.18), radius: 12, x: 0, y: 8)
                }

                VStack(spacing: 4) {
                    Text("GoxViet")
                        .font(.system(size: 28, weight: .bold))
                    Text("Gõ Việt")
                        .font(.title3)
                        .foregroundStyle(.secondary)

                    if let version = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String,
                       let build = Bundle.main.object(forInfoDictionaryKey: "CFBundleVersion") as? String {
                        Text("Version \(version) (\(build))")
                            .font(.caption)
                            .foregroundStyle(.tertiary)
                            .padding(.top, 4)
                    }
                }
                
                Text("A modern Vietnamese input method editor for macOS.")
                    .font(.body)
                    .multilineTextAlignment(.center)
                    .foregroundStyle(.secondary)
                    .fixedSize(horizontal: false, vertical: true)
                
                // Actions
                VStack(spacing: 12) {
                    Button {
                        NotificationCenter.default.post(name: .openUpdateWindow, object: nil)
                    } label: {
                        HStack {
                            Image(systemName: "arrow.down.circle.fill")
                            Text("Check for Updates...")
                        }
                        .frame(maxWidth: .infinity)
                        .padding(.vertical, 8)
                    }
                    .buttonStyle(.borderedProminent)
                    .tint(.blue)
                    
                    HStack(spacing: 16) {
                        Link(destination: URL(string: "https://github.com/nihmtaho/goxviet-ime")!) {
                            Label("GitHub", systemImage: "link")
                        }
                        Link(destination: URL(string: "https://github.com/nihmtaho/goxviet-ime/issues")!) {
                            Label("Report Issue", systemImage: "exclamationmark.bubble")
                        }
                    }
                    .font(.callout)
                }
                
                Spacer()
                
                Text("© 2025 GoxViet. All rights reserved.")
                    .font(.caption)
                    .foregroundStyle(.tertiary)
            }
            .frame(maxWidth: 320)
            
            // Right side - Features
            VStack(alignment: .leading, spacing: 16) {
                Text("Features")
                    .font(.title2.bold())
                    .padding(.bottom, 8)
                
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
                            .font(.title2)
                            .foregroundStyle(feature.1)
                            .frame(width: 32)
                        
                        VStack(alignment: .leading, spacing: 4) {
                            Text(feature.2)
                                .font(.system(size: 14, weight: .semibold))
                            Text(feature.3)
                                .font(.caption)
                                .foregroundStyle(.secondary)
                                .fixedSize(horizontal: false, vertical: true)
                        }
                    }
                    .padding(.vertical, 4)
                }
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
        .padding(.horizontal, 40)
        .padding(.vertical, 32)
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .top)
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

struct EngineMetrics {
    let totalKeystrokes: UInt64
    let backspaceCount: UInt64
    let avgBufferLength: Double
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

