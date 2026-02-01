//
//  ShortcutRecorderSheet.swift
//  GoxViet
//
//  Sheet view for recording keyboard shortcuts
//

import SwiftUI

struct ShortcutRecorderSheet: View {
    @Binding var isRecording: Bool
    let onComplete: (KeyboardShortcut) -> Void
    let onCancel: () -> Void
    
    @State private var showInstructions = true
    
    var body: some View {
        VStack(spacing: 24) {
            // Icon
            Image(systemName: "keyboard.badge.ellipsis")
                .font(.system(size: 48))
                .foregroundColor(.accentColor)
            
            // Title
            VStack(spacing: 8) {
                Text("Press Your Shortcut")
                    .font(.system(size: 20, weight: .semibold))
                
                Text("Press any key combination to set as your toggle shortcut")
                    .font(.system(size: 13))
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
            }
            
            // Instructions
            if showInstructions {
                VStack(alignment: .leading, spacing: 8) {
                    InstructionRow(icon: "command.circle", text: "Use modifiers: ⌘ ⌥ ⌃ ⇧")
                    InstructionRow(icon: "keyboard", text: "Combine with any key or function key")
                    InstructionRow(icon: "exclamationmark.triangle", text: "Some shortcuts may conflict with system")
                    InstructionRow(icon: "escape", text: "Press ESC to cancel")
                }
                .padding(16)
                .background(
                    RoundedRectangle(cornerRadius: 8)
                        .fill(Color(nsColor: .textBackgroundColor))
                )
            }
            
            // Recorder
            ShortcutRecorder(
                isRecording: $isRecording,
                onComplete: { shortcut in
                    onComplete(shortcut)
                },
                onCancel: {
                    onCancel()
                }
            )
            .frame(height: 1) // Hidden, just for capturing
            
            // Cancel button
            Button {
                onCancel()
            } label: {
                Text("Cancel")
                    .frame(minWidth: 80)
            }
            .buttonStyle(.bordered)
            .keyboardShortcut(.cancelAction)
        }
        .padding(32)
        .frame(width: 480, height: 440)
        .onAppear {
            // Auto-hide instructions after 3 seconds
            DispatchQueue.main.asyncAfter(deadline: .now() + 3) {
                withAnimation {
                    showInstructions = false
                }
            }
        }
    }
}

// Helper view for instruction rows
private struct InstructionRow: View {
    let icon: String
    let text: String
    
    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .font(.system(size: 14))
                .foregroundColor(.accentColor)
                .frame(width: 20)
            
            Text(text)
                .font(.system(size: 12))
                .foregroundColor(.secondary)
            
            Spacer()
        }
    }
}

#Preview {
    ShortcutRecorderSheet(
        isRecording: .constant(true),
        onComplete: { _ in },
        onCancel: { }
    )
}
