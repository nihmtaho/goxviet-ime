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
    @Binding var escRestoreEnabled: Bool
    @Binding var freeToneEnabled: Bool
    @Binding var instantRestoreEnabled: Bool
    @Binding var autoDisableForNonLatin: Bool
    
    @State private var showResetConfirmation = false
    @State private var showImportExport = false
    
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
                        .onChange(of: inputMethod) { newValue in
                            NotificationCenter.default.post(
                                name: .inputMethodChanged,
                                object: newValue
                            )
                            Log.info("Input method changed to: \(newValue == 0 ? "Telex" : "VNI")")
                        }
                        
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
                        .onChange(of: modernToneStyle) { newValue in
                            NotificationCenter.default.post(
                                name: .toneStyleChanged,
                                object: newValue
                            )
                            Log.info("Tone style changed to: \(newValue ? "Modern" : "Traditional")")
                        }
                        
                        Divider()
                        
                        ToggleRow(
                            title: "Free Tone Marking",
                            description: "Allow tone marks on any character",
                            systemImage: "textformat",
                            isOn: $freeToneEnabled
                        )
                        .onChange(of: freeToneEnabled) { newValue in
                            NotificationCenter.default.post(
                                name: .freeToneChanged,
                                object: newValue
                            )
                        }
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
                            title: "ESC Key Restore",
                            description: "Press ESC to restore original text",
                            systemImage: "arrow.uturn.backward",
                            isOn: $escRestoreEnabled
                        )
                        .onChange(of: escRestoreEnabled) { newValue in
                            NotificationCenter.default.post(
                                name: .escRestoreChanged,
                                object: newValue
                            )
                        }
                        
                        Divider()
                        
                        ToggleRow(
                            title: "Instant Auto-Restore",
                            description: "Automatically restore English words",
                            systemImage: "arrow.clockwise",
                            isOn: $instantRestoreEnabled
                        )
                        .onChange(of: instantRestoreEnabled) { newValue in
                            NotificationCenter.default.post(
                                name: .instantRestoreChanged,
                                object: newValue
                            )
                        }
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
                        .onChange(of: autoDisableForNonLatin) { newValue in
                            AppState.shared.autoDisableForNonLatinEnabled = newValue
                        }
                    }
                    .padding(8)
                } label: {
                    Label("Smart Features", systemImage: "sparkles")
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
    }
    
    private func resetToDefaults() {
        inputMethod = 0  // Telex
        modernToneStyle = false
        escRestoreEnabled = true
        freeToneEnabled = false
        instantRestoreEnabled = true
        autoDisableForNonLatin = true
        
        Log.info("General settings reset to defaults")
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

// Notification names
extension Notification.Name {
    static let freeToneChanged = Notification.Name("com.goxviet.freeToneChanged")
    static let escRestoreChanged = Notification.Name("com.goxviet.escRestoreChanged")
    static let instantRestoreChanged = Notification.Name("com.goxviet.instantRestoreChanged")
}

#Preview {
    GeneralSettingsView(
        inputMethod: .constant(0),
        modernToneStyle: .constant(false),
        escRestoreEnabled: .constant(true),
        freeToneEnabled: .constant(false),
        instantRestoreEnabled: .constant(true),
        autoDisableForNonLatin: .constant(true)
    )
    .frame(width: 700, height: 600)
}
