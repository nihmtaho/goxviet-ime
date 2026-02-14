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
    
    @State private var showResetConfirmation = false
    @State private var showImportExport = false
    
    // Shortcut settings
    @State private var currentShortcut: KeyboardShortcut = KeyboardShortcut.load()
    @State private var isRecordingShortcut = false
    
    // Restore shortcut
    @ObservedObject private var settingsManager = SettingsManager.shared
    @State private var isRecordingRestoreShortcut = false
    
    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                // Header
                VStack(alignment: .leading, spacing: 4) {
                    Text("General Settings")
                        .font(.system(size: 20, weight: .semibold))
                    Text("Configure input method and typing behavior")
                        .font(.system(size: 13))
                        .foregroundColor(.secondary)
                }
                .padding(.bottom, 8)
                
                // Input Method Section
                GroupBox {
                    VStack(spacing: 12) {
                        PickerRow(
                            title: "Input Method",
                            description: "Choose between Telex or VNI typing method",
                            systemImage: "keyboard",
                            selection: $inputMethod,
                            options: [(0, "Telex"), (1, "VNI")]
                        )
                        // SettingsManager handles notification and sync
                        
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
                                .fill(Color(nsColor: .textBackgroundColor))
                        )
                    }
                    .padding(8)
                } label: {
                    Label("Input Method", systemImage: "keyboard.fill")
                        .font(.system(size: 14, weight: .semibold))
                }
                
                // Tone Settings Section
                GroupBox {
                    VStack(spacing: 12) {
                        ToggleRow(
                            title: "Modern Tone Placement",
                            description: "Use modern tone placement rules (hoà vs hòa)",
                            systemImage: "doc.text.magnifyingglass",
                            isOn: $modernToneStyle
                        )
                        // SettingsManager handles notification and sync
                        
                        Divider()
                        
                        ToggleRow(
                            title: "Free Tone Marking",
                            description: "Allow tone marks on any character",
                            systemImage: "textformat",
                            isOn: $freeToneEnabled
                        )
                        // SettingsManager handles notification and sync
                    }
                    .padding(8)
                } label: {
                    Label("Tone Settings", systemImage: "textformat.alt")
                        .font(.system(size: 14, weight: .semibold))
                }
                
                // Restore Settings Section
                GroupBox {
                    VStack(spacing: 12) {
                        ToggleRow(
                            title: "Instant Auto-Restore",
                            description: "Automatically restore English words",
                            systemImage: "arrow.clockwise",
                            isOn: $instantRestoreEnabled
                        )
                        
                        Divider()
                        
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
                    Label("Auto-Restore", systemImage: "arrow.counterclockwise")
                        .font(.system(size: 14, weight: .semibold))
                }
                
                // Smart Features Section
                GroupBox {
                    VStack(spacing: 12) {
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
                
                // Keyboard Shortcut Section
                GroupBox {
                    VStack(spacing: 12) {
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
                                        .background(Color(nsColor: .controlBackgroundColor))
                                        .cornerRadius(4)
                                        .overlay(
                                            RoundedRectangle(cornerRadius: 4)
                                                .stroke(Color(nsColor: .separatorColor), lineWidth: 0.5)
                                        )
                                }
                            }
                            .padding(.trailing, 8)
                            
                            Button(isRecordingShortcut ? "Recording..." : "Change") {
                                isRecordingShortcut = true
                            }
                            .buttonStyle(.bordered)
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
                                            .fill(preset == currentShortcut ? Color.accentColor.opacity(0.15) : Color(nsColor: .controlBackgroundColor))
                                    )
                                    .overlay(
                                        RoundedRectangle(cornerRadius: 6)
                                            .stroke(preset == currentShortcut ? Color.accentColor : Color(nsColor: .separatorColor), lineWidth: preset == currentShortcut ? 1.5 : 0.5)
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
                
                // Quick Actions
                GroupBox {
                    HStack(spacing: 12) {
                        Button {
                            showResetConfirmation = true
                        } label: {
                            Label("Reset to Defaults", systemImage: "arrow.counterclockwise")
                        }
                        .buttonStyle(.bordered)
                        .alert("Reset Settings", isPresented: $showResetConfirmation) {
                            Button("Cancel", role: .cancel) { }
                            Button("Reset", role: .destructive) {
                                resetToDefaults()
                            }
                        } message: {
                            Text("This will reset all general settings to their default values. This action cannot be undone.")
                        }
                        
                        Spacer()
                        
                        Button {
                            showImportExport = true
                        } label: {
                            Label("Import/Export", systemImage: "square.and.arrow.up")
                        }
                        .buttonStyle(.bordered)
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
            .padding(24)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .onDisappear {
            // Cleanup to reduce memory footprint
            showResetConfirmation = false
            showImportExport = false
            isRecordingShortcut = false
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
    @Environment(\.dismiss) var dismiss
    
    var body: some View {
        VStack(spacing: 20) {
            Text("Import/Export Settings")
                .font(.title2)
            
            Text("Coming soon: Import and export your GoxViet settings")
                .foregroundColor(.secondary)
            
            Button("Close") {
                dismiss()
            }
            .buttonStyle(.borderedProminent)
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
                                .fill(preset == shortcut ? Color.accentColor.opacity(0.15) : Color(nsColor: .controlBackgroundColor))
                        )
                        .overlay(
                            RoundedRectangle(cornerRadius: 4)
                                .stroke(preset == shortcut ? Color.accentColor : Color(nsColor: .separatorColor), lineWidth: preset == shortcut ? 1.5 : 0.5)
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
                    .background(Color(nsColor: .controlBackgroundColor))
                    .cornerRadius(4)
                    .overlay(
                        RoundedRectangle(cornerRadius: 4)
                            .stroke(Color(nsColor: .separatorColor), lineWidth: 0.5)
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
}

#Preview {
    GeneralSettingsView(
        inputMethod: .constant(0),
        modernToneStyle: .constant(false),
        restoreShortcutEnabled: .constant(true),
        freeToneEnabled: .constant(false),
        instantRestoreEnabled: .constant(true),
        autoDisableForNonLatin: .constant(true),
        shiftBackspaceEnabled: .constant(true)
    )
    .frame(width: 700, height: 600)
}
