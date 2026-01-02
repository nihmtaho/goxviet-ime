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
    @AppStorage("smartModeEnabled") private var smartModeEnabled = true
    @AppStorage("com.goxviet.ime.autoDisableNonLatin") private var autoDisableForNonLatin = true

    // MARK: - View State
    @State private var selection: SettingsSection? = .general
    @State private var perAppModes: [String: Bool] = [:]
    @State private var showClearConfirmation = false

    // MARK: - Body
    var body: some View {
        ZStack {
            SettingsGlassBackground()

            NavigationSplitView {
                sidebar
            } detail: {
                detailPanel
            }
            .navigationSplitViewStyle(.balanced)
        }
        .frame(minWidth: 760, idealWidth: 840, minHeight: 520, idealHeight: 580)
        .background(Color.clear)
        .onAppear {
            loadPerAppModes()
            syncToAppState()
        }
    }

    // MARK: - Sidebar
    private var sidebar: some View {
        VStack(spacing: 0) {
            SidebarHeader()
                .padding(.horizontal, 16)
                .padding(.top, 28)
                .padding(.bottom, 12)

            List(SettingsSection.allCases, id: \.self, selection: $selection) { section in
                HStack(spacing: 8) {
                    Image(systemName: section.icon)
                        .foregroundColor(.secondary)
                        .frame(width: 16, height: 16)
                    Text(section.title)
                        .font(.system(size: 14, weight: .medium))
                }
                .tag(section)
            }
            .scrollContentBackground(.hidden)
            .listStyle(.sidebar)
        }
        .background(.ultraThinMaterial)
        .navigationSplitViewColumnWidth(min: 200, ideal: 220, max: 240)
        .navigationTitle("Settings")
//        .toolbar {
//            ToolbarItem(placement: .navigation) {
//                Button(action: toggleSidebar) {
//                    Image(systemName: "sidebar.left")
//                }
//            }
//        }
    }

    // MARK: - Detail Panel
    private var detailPanel: some View {
        ZStack(alignment: .topLeading) {
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(.regularMaterial)
                .overlay(
                    RoundedRectangle(cornerRadius: 16, style: .continuous)
                        .stroke(.white.opacity(0.12), lineWidth: 1)
                )
                .shadow(color: .black.opacity(0.15), radius: 24, x: 0, y: 18)
                .padding(.vertical, 10)
                .padding(.trailing, 10)
                .padding(.leading, 10)

            detailContent
                .padding(20)
                .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
        }
        .navigationTitle(selection?.title ?? "Settings")
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
                    freeToneEnabled: $freeToneEnabled
                )
            case .perApp:
                PerAppSettingsView(
                    smartModeEnabled: $smartModeEnabled,
                    autoDisableForNonLatin: $autoDisableForNonLatin,
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

    private func toggleSidebar() {
        withAnimation(.easeInOut(duration: 0.25)) {
            _ = NSApp.keyWindow?.firstResponder?.tryToPerform(#selector(NSSplitViewController.toggleSidebar(_:)), with: nil)
        }
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

    var body: some View {
        Form {
            Section {
                Picker("Input Method", selection: $inputMethod) {
                    Text("Telex").tag(0)
                    Text("VNI").tag(1)
                }
                .pickerStyle(.segmented)
                .onChange(of: inputMethod) { _, newValue in
                    AppState.shared.inputMethod = newValue
                    InputManager.shared.setInputMethod(newValue)
                    Log.info("Input method: \(newValue == 0 ? "Telex" : "VNI")")
                }

                Text(inputMethodDescription)
                    .font(.caption)
                    .foregroundStyle(.secondary)
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
                Toggle("ESC key restores original word", isOn: $escRestoreEnabled)
                    .onChange(of: escRestoreEnabled) { _, newValue in
                        AppState.shared.escRestoreEnabled = newValue
                        InputManager.shared.setEscRestore(newValue)
                        Log.info("ESC restore: \(newValue)")
                    }

                Text("Press ESC to restore the original unaccented word.")
                    .font(.caption)
                    .foregroundStyle(.secondary)

                Toggle("Free tone placement", isOn: $freeToneEnabled)
                    .onChange(of: freeToneEnabled) { _, newValue in
                        AppState.shared.freeToneEnabled = newValue
                        InputManager.shared.setFreeTone(newValue)
                        Log.info("Free tone: \(newValue)")
                    }

                Text("Allow tone marks before completing the vowel.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            } header: {
                Label("Smart Features", systemImage: "sparkles")
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
    @Binding var autoDisableForNonLatin: Bool
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
                Label("Multi-Language Support", systemImage: "globe")
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

    var body: some View {
        Form {
            Section {
                LabeledContent("Toggle Input") {
                    Text(InputManager.shared.getCurrentShortcut().displayString)
                        .font(.body.monospaced())
                        .padding(.horizontal, 12)
                        .padding(.vertical, 6)
                        .background(.quaternary)
                        .cornerRadius(6)
                }

                Text("Change in System Settings → Keyboard → Keyboard Shortcuts → Input Sources.")
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
    }
}

// MARK: - About View

private struct AboutSettingsView: View {
    var body: some View {
        ScrollView {
            VStack(spacing: 32) {
                Spacer(minLength: 24)

                Image(systemName: "keyboard.fill")
                    .font(.system(size: 80))
                    .foregroundStyle(
                        .linearGradient(
                            colors: [.blue, .purple],
                            startPoint: .topLeading,
                            endPoint: .bottomTrailing
                        )
                    )
                    .symbolEffect(.pulse)

                VStack(spacing: 8) {
                    Text("GoxViet")
                        .font(.system(size: 40, weight: .bold))
                    Text("Gõ Việt")
                        .font(.title3)
                        .foregroundStyle(.secondary)

                    if let version = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String,
                       let build = Bundle.main.object(forInfoDictionaryKey: "CFBundleVersion") as? String {
                        Text("Version \(version) (\(build))")
                            .font(.caption)
                            .foregroundStyle(.tertiary)
                    }
                }

                Text("A modern Vietnamese input method editor for macOS.")
                    .font(.body)
                    .multilineTextAlignment(.center)
                    .foregroundStyle(.secondary)
                    .padding(.horizontal, 32)

                VStack(alignment: .leading, spacing: 12) {
                    FeatureRow(icon: "bolt.fill", color: .yellow, text: "High-performance Rust core (< 16ms latency)")
                    FeatureRow(icon: "brain.fill", color: .pink, text: "Smart per-app mode")
                    FeatureRow(icon: "keyboard.badge.ellipsis", color: .blue, text: "Telex & VNI input methods")
                    FeatureRow(icon: "textformat", color: .green, text: "Modern & Traditional tone styles")
                }
                .padding(24)
                .background(.ultraThinMaterial, in: RoundedRectangle(cornerRadius: 16, style: .continuous))

                HStack(spacing: 18) {
                    Link(destination: URL(string: "https://github.com/goxviet/goxviet")!) {
                        Label("GitHub", systemImage: "link")
                    }

                    Link(destination: URL(string: "https://github.com/goxviet/goxviet/issues")!) {
                        Label("Report Issue", systemImage: "exclamationmark.bubble")
                    }
                }
                .font(.callout)

                Text("© 2025 GoxViet. All rights reserved.")
                    .font(.caption)
                    .foregroundStyle(.tertiary)
                    .padding(.bottom, 12)
            }
            .frame(maxWidth: .infinity)
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
        .frame(width: 840, height: 580)
}
