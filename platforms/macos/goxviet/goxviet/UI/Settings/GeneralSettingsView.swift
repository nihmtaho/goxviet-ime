//
//  GeneralSettingsView.swift
//  GoxViet
//
//  Enhanced General Settings with modern UI components
//

import SwiftUI

struct GeneralSettingsView: View {
    @Binding var inputMethod: Int
    @Binding var modernToneStyle: Bool
    @Binding var restoreShortcutEnabled: Bool
    @Binding var freeToneEnabled: Bool
    @Binding var instantRestoreEnabled: Bool
    @Binding var autoDisableForNonLatin: Bool
    @Binding var shiftBackspaceEnabled: Bool
    @Binding var escRestoreEnabled: Bool
    @Binding var bracketShortcutsEnabled: Bool
    @Binding var foreignConsonantsEnabled: Bool
    @Binding var autoCapitaliseEnabled: Bool
    @Binding var wordHistoryEnabled: Bool
    
    @State private var showResetConfirmation = false
    @State private var showImportExport = false
    
    // Shortcut settings
    @State private var currentShortcut: KeyboardShortcut = KeyboardShortcut.load()
    @State private var isRecordingShortcut = false
    
    // Restore shortcut
    @ObservedObject private var settingsManager = SettingsManager.shared
    @State private var isRecordingRestoreShortcut = false
    
    // Advanced (merged)
    @State private var loggingEnabled: Bool = Log.isEnabled
    @State private var showLegacyEncodingWarning = false
    @State private var pendingEncoding: OutputEncoding?

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 14) {
                // Keyboard Shortcut Section (first — primary action)
                GroupBox {
                    VStack(spacing: 6) {
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Toggle Vietnamese Input")
                                    .font(.system(size: 13, weight: .medium))
                                Text("Shortcut to switch between Vietnamese and English")
                                    .font(.system(size: 11))
                                    .foregroundColor(.secondary)
                            }

                            Spacer()

                            // Display current shortcut
                            HStack(spacing: 4) {
                                ForEach(currentShortcut.displayParts, id: \.self) { part in
                                    Text(part)
                                        .font(.system(size: 11, weight: .medium))
                                        .padding(.horizontal, 6)
                                        .padding(.vertical, 3)
                                        .background(Color(NSColor.controlBackgroundColor))
                                        .cornerRadius(4)
                                        .overlay(
                                            RoundedRectangle(cornerRadius: 4)
                                                .stroke(Color(NSColor.separatorColor), lineWidth: 0.5)
                                        )
                                }
                            }
                            .padding(.trailing, 8)

                            Button(isRecordingShortcut ? "Recording..." : "Change") {
                                isRecordingShortcut = true
                            }
                            .adaptiveGlassButton()
                            .disabled(isRecordingShortcut)
                        }

                        // Conflict warning (if any)
                        if let conflict = currentShortcut.conflictInfo {
                            HStack(alignment: .top, spacing: 8) {
                                Image(systemName: "exclamationmark.triangle.fill")
                                    .foregroundColor(.orange)
                                    .font(.system(size: 12))
                                VStack(alignment: .leading, spacing: 2) {
                                    Text("Warning: Potential Conflict")
                                        .font(.system(size: 11, weight: .semibold))
                                        .foregroundColor(.orange)
                                    Text(conflict.message)
                                        .font(.system(size: 10))
                                        .foregroundColor(.secondary)
                                        .fixedSize(horizontal: false, vertical: true)
                                }
                                Spacer()
                            }
                            .padding(8)
                            .background(Color.orange.opacity(0.1))
                            .cornerRadius(6)
                        }

                        Divider()

                        // Preset shortcuts
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Quick Presets")
                                .font(.system(size: 11, weight: .medium))
                                .foregroundColor(.secondary)

                            HStack(spacing: 8) {
                                ForEach(Array(KeyboardShortcut.presets.enumerated()), id: \.offset) { idx, preset in
                                    Button {
                                        applyShortcut(preset)
                                    } label: {
                                        Text(preset.displayString)
                                            .font(.system(size: 10))
                                    }
                                    .buttonStyle(.plain)
                                    .padding(.horizontal, 10)
                                    .padding(.vertical, 6)
                                    .background(
                                        RoundedRectangle(cornerRadius: 6)
                                            .fill(preset == currentShortcut ? Color.accentColor.opacity(0.15) : Color(NSColor.controlBackgroundColor))
                                    )
                                    .overlay(
                                        RoundedRectangle(cornerRadius: 6)
                                            .stroke(preset == currentShortcut ? Color.accentColor : Color(NSColor.separatorColor), lineWidth: preset == currentShortcut ? 1.5 : 0.5)
                                    )
                                }
                            }
                        }
                    }
                    .padding(8)
                } label: {
                    Label("Keyboard Shortcut", systemImage: "command.circle")
                        .font(.system(size: 14, weight: .semibold))
                }
                .sheet(isPresented: $isRecordingShortcut) {
                    ShortcutRecorderSheet(
                        isRecording: $isRecordingShortcut,
                        onComplete: { newShortcut in
                            applyShortcut(newShortcut)
                            isRecordingShortcut = false
                        },
                        onCancel: {
                            isRecordingShortcut = false
                        }
                    )
                }

                // Input Method Section
                GroupBox {
                    VStack(spacing: 6) {
                        PickerRow(
                            title: "Input Method",
                            description: "Choose between Telex or VNI typing method",
                            systemImage: "keyboard",
                            selection: $inputMethod,
                            options: [(0, "Telex"), (1, "VNI")]
                        )

                        Divider()

                        // Input Method Preview
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Preview")
                                    .font(.system(size: 12, weight: .medium))
                                    .foregroundColor(.secondary)

                                if inputMethod == 0 {
                                    Text("viet = việt, hoa = hòa")
                                        .font(.system(size: 13, design: .monospaced))
                                } else {
                                    Text("vie65t = việt, hoa2 = hòa")
                                        .font(.system(size: 13, design: .monospaced))
                                }
                            }
                            Spacer()
                        }
                        .padding(.horizontal, 12)
                        .padding(.vertical, 8)
                        .background(
                            RoundedRectangle(cornerRadius: 6)
                                .fill(Color(NSColor.textBackgroundColor))
                        )
                    }
                    .padding(8)
                } label: {
                    Label("Input Method", systemImage: "keyboard.fill")
                        .font(.system(size: 14, weight: .semibold))
                }

                // Tone Settings Section
                GroupBox {
                    VStack(spacing: 6) {
                        ToggleRow(
                            title: "Modern Tone Placement",
                            description: "Use modern tone placement rules (hoà vs hòa)",
                            systemImage: "doc.text.magnifyingglass",
                            isOn: $modernToneStyle
                        )

                        Divider()

                        ToggleRow(
                            title: "Free Tone Marking",
                            description: "Allow tone marks on any character",
                            systemImage: "textformat",
                            isOn: $freeToneEnabled
                        )
                    }
                    .padding(8)
                } label: {
                    Label("Tone Settings", systemImage: "textformat.alt")
                        .font(.system(size: 14, weight: .semibold))
                }

                // Smart Features Section (includes Instant Auto-Restore)
                GroupBox {
                    VStack(spacing: 6) {
                        ToggleRow(
                            title: "Instant Auto-Restore",
                            description: "Automatically restore English words",
                            systemImage: "arrow.clockwise",
                            isOn: $instantRestoreEnabled
                        )

                        Divider()

                        ToggleRow(
                            title: "ESC Key Restore",
                            description: "Press ESC to revert Vietnamese text to the original keystrokes",
                            systemImage: "escape",
                            isOn: $escRestoreEnabled
                        )

                        Divider()

                        ToggleRow(
                            title: "Auto-Disable for Non-Latin Apps",
                            description: "Automatically disable IME for apps using non-Latin scripts",
                            systemImage: "globe",
                            isOn: $autoDisableForNonLatin
                        )

                        Divider()

                        // Restore Shortcut
                        VStack(spacing: 8) {
                            HStack {
                                HStack(spacing: 8) {
                                    Image(systemName: "arrow.uturn.backward")
                                        .font(.system(size: 14))
                                        .foregroundColor(.accentColor)
                                        .frame(width: 20)

                                    VStack(alignment: .leading, spacing: 2) {
                                        Text("Restore to Raw Input")
                                            .font(.system(size: 13, weight: .medium))
                                        Text("Shortcut to restore Vietnamese text back to raw keystrokes")
                                            .font(.system(size: 11))
                                            .foregroundColor(.secondary)
                                    }
                                }

                                Spacer()

                                Toggle("", isOn: $restoreShortcutEnabled)
                                    .toggleStyle(.switch)
                                    .labelsHidden()
                                    .controlSize(.small)
                            }

                            if restoreShortcutEnabled {
                                RestoreShortcutRecorderRow(
                                    shortcut: $settingsManager.restoreShortcut,
                                    isRecording: $isRecordingRestoreShortcut
                                )
                                .padding(.leading, 28)
                            }
                        }
                    }
                    .padding(8)
                } label: {
                    Label("Smart Features", systemImage: "sparkles")
                        .font(.system(size: 14, weight: .semibold))
                }

                // Editing Section (keyboard editing shortcuts)
                GroupBox {
                    VStack(spacing: 6) {
                        ToggleRowCustomTitle(
                            title: {
                                HStack(spacing: 4) {
                                    Text("Shift+Backspace Delete Word")
                                    Text("(Beta)")
                                        .font(.system(size: 10, weight: .semibold))
                                        .foregroundColor(.orange)
                                        .padding(.horizontal, 6)
                                        .padding(.vertical, 2)
                                        .background(Capsule().fill(Color.orange.opacity(0.2)))
                                }
                            },
                            description: "Quickly delete entire word with Shift+Backspace",
                            systemImage: "delete.left.fill",
                            isOn: $shiftBackspaceEnabled
                        )
                    }
                    .padding(8)
                } label: {
                    Label("Editing", systemImage: "pencil.and.outline")
                        .font(.system(size: 14, weight: .semibold))
                }

                // Vietnamese Extensions Section (US2–US5)
                GroupBox {
                    VStack(spacing: 6) {
                        ToggleRow(
                            title: "Bracket Shortcuts ([ / ])",
                            description: "In Telex mode, [ inserts ơ and ] inserts ư",
                            systemImage: "square.and.pencil",
                            isOn: $bracketShortcutsEnabled
                        )

                        Divider()

                        ToggleRow(
                            title: "Allow Foreign Consonants (z, w, j, f)",
                            description: "Type z, w, j, f at word start as literal consonants",
                            systemImage: "character.phonetic",
                            isOn: $foreignConsonantsEnabled
                        )

                        Divider()

                        ToggleRow(
                            title: "Auto-Capitalise After Sentence End",
                            description: "Automatically capitalise the first letter after . ! ?",
                            systemImage: "textformat.abc",
                            isOn: $autoCapitaliseEnabled
                        )

                        Divider()

                        ToggleRow(
                            title: "Backspace-After-Space Restore",
                            description: "Press Backspace after Space to restore the last typed word",
                            systemImage: "delete.backward",
                            isOn: $wordHistoryEnabled
                        )
                    }
                    .padding(8)
                } label: {
                    Label("Vietnamese Extensions", systemImage: "flag")
                        .font(.system(size: 14, weight: .semibold))
                }

                // Output Encoding Section
                GroupBox {
                    VStack(alignment: .leading, spacing: 12) {
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

                            Text("(Beta)")
                                .font(.system(size: 10, weight: .semibold))
                                .foregroundColor(.orange)
                                .padding(.horizontal, 6)
                                .padding(.vertical, 2)
                                .background(Capsule().fill(Color.orange.opacity(0.2)))
                        }
                        .onChange(of: settingsManager.outputEncoding) { _, newValue in
                            if newValue.isLegacy {
                                pendingEncoding = newValue
                                showLegacyEncodingWarning = true
                            }
                        }

                        if settingsManager.outputEncoding.isLegacy {
                            HStack(spacing: 8) {
                                Image(systemName: "exclamationmark.triangle.fill")
                                    .foregroundColor(.orange)
                                    .font(.system(size: 12))
                                Text("Legacy encoding selected — use Unicode for modern apps.")
                                    .font(.system(size: 10))
                                    .foregroundColor(.secondary)
                            }
                            .padding(8)
                            .background(Color.orange.opacity(0.1))
                            .cornerRadius(6)
                        }
                    }
                    .padding(8)
                } label: {
                    Label("Output Encoding", systemImage: "doc.plaintext")
                        .font(.system(size: 14, weight: .semibold))
                }

                // Logging Section
                GroupBox {
                    VStack(spacing: 6) {
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Application Logs")
                                    .font(.system(size: 13, weight: .medium))
                                Text("Enable logging for debugging")
                                    .font(.system(size: 11))
                                    .foregroundColor(.secondary)
                            }
                            Spacer()
                            Toggle("", isOn: $loggingEnabled)
                                .toggleStyle(.switch)
                                .labelsHidden()
                                .controlSize(.small)
                                .onChange(of: loggingEnabled) { _, newValue in
                                    if newValue {
                                        Log.enableLogging(reason: "User enabled in Settings")
                                    } else {
                                        Log.disableLogging(reason: "User disabled in Settings")
                                    }
                                }
                        }

                        Divider()

                        HStack {
                            Button {
                                if FileManager.default.fileExists(atPath: Log.logPath.path) {
                                    NSWorkspace.shared.open(Log.logPath)
                                }
                            } label: {
                                Label("Open Log File", systemImage: "doc.text.magnifyingglass")
                            }
                            .adaptiveGlassButton()

                            Spacer()

                            Button {
                                NSPasteboard.general.clearContents()
                                NSPasteboard.general.setString("~/Library/Logs/GoxViet/", forType: .string)
                            } label: {
                                Label("Copy Path", systemImage: "doc.on.doc")
                            }
                            .adaptiveGlassButton()

                            Button {
                                Log.clearLogs()
                            } label: {
                                Label("Clear Logs", systemImage: "trash")
                            }
                            .adaptiveGlassButton()
                            .foregroundColor(.red)
                        }

                        HStack {
                            Image(systemName: loggingEnabled ? "checkmark.circle.fill" : "xmark.circle.fill")
                                .foregroundColor(loggingEnabled ? .green : .secondary)
                            Text(loggingEnabled ? "Logging enabled" : "Logging disabled")
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

                // Quick Actions
                GroupBox {
                    HStack(spacing: 12) {
                        Button {
                            showResetConfirmation = true
                        } label: {
                            Label("Reset to Defaults", systemImage: "arrow.counterclockwise")
                        }
                        .adaptiveGlassButton()
                        .alert(isPresented: $showResetConfirmation) {
                            Alert(
                                title: Text("Reset Settings"),
                                message: Text("This will reset all general settings to their default values. This action cannot be undone."),
                                primaryButton: .destructive(Text("Reset")) { resetToDefaults() },
                                secondaryButton: .cancel(Text("Cancel"))
                            )
                        }

                        Spacer()

                        Button {
                            showImportExport = true
                        } label: {
                            Label("Import/Export", systemImage: "square.and.arrow.up")
                        }
                        .adaptiveGlassButton()
                        .sheet(isPresented: $showImportExport) {
                            ImportExportView()
                        }
                    }
                    .padding(8)
                } label: {
                    Label("Quick Actions", systemImage: "bolt")
                        .font(.system(size: 14, weight: .semibold))
                }

                Spacer()
            }
            .padding(20)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .onDisappear {
            // Cleanup to reduce memory footprint
            showResetConfirmation = false
            showImportExport = false
            isRecordingShortcut = false
            showLegacyEncodingWarning = false
        }
        .onReceive(NotificationCenter.default.publisher(for: .loggingStateChanged)) { notification in
            if let enabled = notification.object as? Bool {
                loggingEnabled = enabled
            }
        }
    }
    
    private func resetToDefaults() {
        inputMethod = 0  // Telex
        modernToneStyle = false
        restoreShortcutEnabled = true
        freeToneEnabled = false
        instantRestoreEnabled = true
        autoDisableForNonLatin = true
        settingsManager.restoreShortcut = .default
        escRestoreEnabled = false
        bracketShortcutsEnabled = false
        foreignConsonantsEnabled = false
        autoCapitaliseEnabled = false
        wordHistoryEnabled = false

        Log.info("General settings reset to defaults")
    }
    
    private func applyShortcut(_ shortcut: KeyboardShortcut) {
        guard shortcut.isValid else {
            Log.warning("Invalid shortcut attempted: \(shortcut)")
            return
        }
        
        currentShortcut = shortcut
        shortcut.save()
        
        // Notify InputManager to update shortcut
        NotificationCenter.default.post(
            name: NSNotification.Name("shortcutChanged"),
            object: shortcut
        )
        
        Log.info("Shortcut changed to: \(shortcut.displayString)")
    }
}

// Import/Export Sheet View
struct ImportExportView: View {
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack(spacing: 20) {
            Text("Import/Export Settings")
                .font(.title2)

            Text("Coming soon: Import and export your GoxViet settings")
                .foregroundColor(.secondary)

            if #available(macOS 12.0, *) {
                Button("Close") {
                    presentationMode.wrappedValue.dismiss()
                }
                .buttonStyle(.borderedProminent)
            } else {
                Button("Close") {
                    presentationMode.wrappedValue.dismiss()
                }
                .buttonStyle(.bordered)
            }
        }
        .frame(width: 400, height: 200)
        .padding()
    }
}

// MARK: - Restore Shortcut Inline Recorder

/// Inline recorder row for configuring the restore shortcut.
/// Supports repeated modifier taps (e.g., double Option, triple Command).
struct RestoreShortcutRecorderRow: View {
    @Binding var shortcut: RestoreShortcut
    @Binding var isRecording: Bool
    
    @State private var recordedKeys: [RestoreHotkey] = []
    @State private var lastTapTime: Date = .distantPast
    @State private var eventMonitor: Any?
    
    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack(spacing: 12) {
                // Current shortcut display
                if isRecording {
                    recordingView
                } else {
                    currentShortcutView
                }
                
                Spacer()
                
                if isRecording {
                    Button("Cancel") {
                        stopRecording(save: false)
                    }
                    .buttonStyle(.bordered)
                    .controlSize(.small)
                } else {
                    Button("Change") {
                        startRecording()
                    }
                    .buttonStyle(.bordered)
                    .controlSize(.small)
                }
            }
            
            // Presets row
            if !isRecording {
                HStack(spacing: 6) {
                    Text("Presets:")
                        .font(.system(size: 10))
                        .foregroundColor(.secondary)
                    
                    ForEach(Array(RestoreShortcut.presets.enumerated()), id: \.offset) { _, preset in
                        Button {
                            shortcut = preset
                        } label: {
                            Text(preset.displayString)
                                .font(.system(size: 10))
                        }
                        .buttonStyle(.plain)
                        .padding(.horizontal, 8)
                        .padding(.vertical, 3)
                        .background(
                            RoundedRectangle(cornerRadius: 4)
                                .fill(preset == shortcut ? Color.accentColor.opacity(0.15) : Color(NSColor.controlBackgroundColor))
                        )
                        .overlay(
                            RoundedRectangle(cornerRadius: 4)
                                .stroke(preset == shortcut ? Color.accentColor : Color(NSColor.separatorColor), lineWidth: preset == shortcut ? 1.5 : 0.5)
                        )
                    }
                }
            }
        }
    }
    
    // MARK: - Sub-views
    
    private var currentShortcutView: some View {
        HStack(spacing: 4) {
            ForEach(shortcut.displayParts, id: \.self) { part in
                Text(part)
                    .font(.system(size: 11, weight: .medium))
                    .padding(.horizontal, 6)
                    .padding(.vertical, 3)
                    .background(Color(NSColor.controlBackgroundColor))
                    .cornerRadius(4)
                    .overlay(
                        RoundedRectangle(cornerRadius: 4)
                            .stroke(Color(NSColor.separatorColor), lineWidth: 0.5)
                    )
            }
        }
    }
    
    private var recordingView: some View {
        HStack(spacing: 8) {
            // Pulsing indicator
            Circle()
                .fill(Color.red)
                .frame(width: 8, height: 8)
                .opacity(0.8)
            
            if recordedKeys.isEmpty {
                Text("Press modifier keys… (ESC to cancel)")
                    .font(.system(size: 11))
                    .foregroundColor(.secondary)
            } else {
                // Show recorded keys so far
                ForEach(Array(recordedKeys.enumerated()), id: \.offset) { _, key in
                    Text(key.displaySymbol)
                        .font(.system(size: 11, weight: .medium))
                        .padding(.horizontal, 6)
                        .padding(.vertical, 3)
                        .background(Color.accentColor.opacity(0.15))
                        .cornerRadius(4)
                }
                
                Text("(\(recordedKeys.count)/4)")
                    .font(.system(size: 10))
                    .foregroundColor(.secondary)
            }
        }
    }
    
    // MARK: - Recording Logic
    
    private func startRecording() {
        recordedKeys = []
        isRecording = true
        
        // Install local event monitor for flagsChanged and keyDown
        eventMonitor = NSEvent.addLocalMonitorForEvents(matching: [.flagsChanged, .keyDown]) { event in
            if event.type == .keyDown {
                // ESC cancels recording
                if event.keyCode == 53 {
                    stopRecording(save: false)
                    return nil
                }
                // Ignore other keys (modifier-only shortcut)
                return nil
            }
            
            if event.type == .flagsChanged {
                handleModifierEvent(event)
                return nil
            }
            
            return event
        }
    }
    
    private func handleModifierEvent(_ event: NSEvent) {
        let flags = CGEventFlags(rawValue: UInt64(event.modifierFlags.rawValue))
            .intersection(RestoreHotkey.allowedModifiers)
        
        // Only record on key-down of modifier (flags become non-empty)
        guard !flags.isEmpty else { return }
        
        let now = Date()
        let elapsed = now.timeIntervalSince(lastTapTime)
        lastTapTime = now
        
        // If too long since last tap, treat as fresh start
        if elapsed > shortcut.tapInterval && !recordedKeys.isEmpty {
            recordedKeys = []
        }
        
        let hotkey = RestoreHotkey(flags: flags.rawValue)
        recordedKeys.append(hotkey)
        
        // Auto-complete if max reached
        if recordedKeys.count >= 4 {
            stopRecording(save: true)
            return
        }
        
        // Schedule auto-complete after tapInterval
        DispatchQueue.main.asyncAfter(deadline: .now() + shortcut.tapInterval) { [self] in
            guard isRecording, !recordedKeys.isEmpty else { return }
            // If no new tap arrived, finalize
            if Date().timeIntervalSince(lastTapTime) >= shortcut.tapInterval * 0.9 {
                stopRecording(save: true)
            }
        }
    }
    
    private func stopRecording(save: Bool) {
        if let monitor = eventMonitor {
            NSEvent.removeMonitor(monitor)
            eventMonitor = nil
        }
        
        if save && !recordedKeys.isEmpty {
            let newShortcut = RestoreShortcut(keys: recordedKeys)
            if newShortcut.isValid {
                shortcut = newShortcut
                Log.info("Restore shortcut changed to: \(newShortcut.displayString)")
            }
        }
        
        recordedKeys = []
        isRecording = false
    }
}

// Notification names
extension Notification.Name {
    static let freeToneChanged = Notification.Name("com.goxviet.freeToneChanged")
    static let restoreShortcutChanged = Notification.Name("com.goxviet.restoreShortcutChanged")
    static let instantRestoreChanged = Notification.Name("com.goxviet.instantRestoreChanged")
    static let escRestoreChanged = Notification.Name("com.goxviet.escRestoreChanged")
    static let bracketShortcutsChanged = Notification.Name("com.goxviet.bracketShortcutsChanged")
    static let foreignConsonantsChanged = Notification.Name("com.goxviet.foreignConsonantsChanged")
    static let autoCapitaliseChanged = Notification.Name("com.goxviet.autoCapitaliseChanged")
    static let wordHistoryChanged = Notification.Name("com.goxviet.wordHistoryChanged")
}

#Preview {
    GeneralSettingsView(
        inputMethod: .constant(0),
        modernToneStyle: .constant(false),
        restoreShortcutEnabled: .constant(true),
        freeToneEnabled: .constant(false),
        instantRestoreEnabled: .constant(true),
        autoDisableForNonLatin: .constant(true),
        shiftBackspaceEnabled: .constant(true),
        escRestoreEnabled: .constant(false),
        bracketShortcutsEnabled: .constant(false),
        foreignConsonantsEnabled: .constant(false),
        autoCapitaliseEnabled: .constant(false),
        wordHistoryEnabled: .constant(false)
    )
    .frame(width: 700, height: 600)
}
