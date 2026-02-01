//
//  ShortcutEditorSheet.swift
//  goxviet
//
//  Created on 2026-02-01.
//  Part of Phase 2.9.2: Text Expansion UI Implementation
//

import SwiftUI

/// Modal sheet for adding or editing text expansion shortcuts
/// Provides validation and real-time preview
struct ShortcutEditorSheet: View {
    // MARK: - Properties
    
    @Binding var isPresented: Bool
    let existingShortcuts: [String]
    let editingTrigger: String?
    let editingReplacement: String?
    let onSave: (String, String) -> Void
    
    @State private var trigger: String
    @State private var replacement: String
    @State private var triggerError: String?
    @State private var replacementError: String?
    
    @FocusState private var focusedField: Field?
    
    // MARK: - Computed Properties
    
    private var isEditing: Bool {
        editingTrigger != nil
    }
    
    private var title: String {
        isEditing ? "Sửa gõ tắt" : "Thêm gõ tắt mới"
    }
    
    private var canSave: Bool {
        !trigger.isEmpty &&
        !replacement.isEmpty &&
        triggerError == nil &&
        replacementError == nil
    }
    
    // MARK: - Initialization
    
    init(
        isPresented: Binding<Bool>,
        existingShortcuts: [String],
        editingTrigger: String? = nil,
        editingReplacement: String? = nil,
        onSave: @escaping (String, String) -> Void
    ) {
        self._isPresented = isPresented
        self.existingShortcuts = existingShortcuts
        self.editingTrigger = editingTrigger
        self.editingReplacement = editingReplacement
        self.onSave = onSave
        
        // Initialize state
        _trigger = State(initialValue: editingTrigger ?? "")
        _replacement = State(initialValue: editingReplacement ?? "")
    }
    
    // MARK: - Body
    
    var body: some View {
        VStack(spacing: 0) {
            // Header
            headerView
            
            Divider()
            
            // Content
            ScrollView {
                VStack(alignment: .leading, spacing: 24) {
                    // Trigger field
                    triggerField
                    
                    // Replacement field
                    replacementField
                    
                    // Preview
                    previewSection
                    
                    // Guidelines
                    guidelinesSection
                }
                .padding(24)
            }
            
            Divider()
            
            // Footer buttons
            footerView
        }
        .frame(width: 500, height: 550)
        .onAppear {
            focusedField = .trigger
            validateTrigger()
            validateReplacement()
        }
    }
    
    // MARK: - View Components
    
    private var headerView: some View {
        HStack {
            Text(title)
                .font(.headline)
            
            Spacer()
            
            Button {
                isPresented = false
            } label: {
                Image(systemName: "xmark.circle.fill")
                    .foregroundColor(.secondary)
            }
            .buttonStyle(.plain)
            .keyboardShortcut(.escape, modifiers: [])
        }
        .padding()
    }
    
    private var triggerField: some View {
        VStack(alignment: .leading, spacing: 8) {
            Label {
                Text("Từ viết tắt")
                    .font(.headline)
            } icon: {
                Image(systemName: "text.cursor")
            }
            
            TextField("Ví dụ: brb, omw, ty...", text: $trigger)
                .textFieldStyle(.roundedBorder)
                .font(.system(.body, design: .monospaced))
                .focused($focusedField, equals: .trigger)
                .onChange(of: trigger) { _, _ in
                    validateTrigger()
                }
                .onSubmit {
                    focusedField = .replacement
                }
            
            if let error = triggerError {
                HStack(spacing: 4) {
                    Image(systemName: "exclamationmark.circle.fill")
                        .foregroundColor(.red)
                    Text(error)
                        .font(.caption)
                        .foregroundColor(.red)
                }
            } else {
                Text("Tối đa 20 ký tự, chỉ chữ cái và số")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
    }
    
    private var replacementField: some View {
        VStack(alignment: .leading, spacing: 8) {
            Label {
                Text("Văn bản thay thế")
                    .font(.headline)
            } icon: {
                Image(systemName: "text.alignleft")
            }
            
            TextEditor(text: $replacement)
                .font(.body)
                .frame(minHeight: 100, maxHeight: 150)
                .lineLimit(5...10)
                .scrollContentBackground(.hidden)
                .background(Color(nsColor: .textBackgroundColor))
                .border(Color.secondary.opacity(0.3), width: 1)
                .focused($focusedField, equals: .replacement)
                .onChange(of: replacement) { _, _ in
                    validateReplacement()
                }
            
            if let error = replacementError {
                HStack(spacing: 4) {
                    Image(systemName: "exclamationmark.circle.fill")
                        .foregroundColor(.red)
                    Text(error)
                        .font(.caption)
                        .foregroundColor(.red)
                }
            } else {
                HStack {
                    Text("\(replacement.count) ký tự")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    
                    Spacer()
                    
                    Text("Tối đa 200 ký tự")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
        }
    }
    
    private var previewSection: some View {
        GroupBox {
            VStack(alignment: .leading, spacing: 12) {
                HStack {
                    Image(systemName: "eye")
                        .foregroundColor(.secondary)
                    Text("Xem trước")
                        .font(.headline)
                }
                
                if !trigger.isEmpty && !replacement.isEmpty {
                    HStack(alignment: .center, spacing: 12) {
                        Text(trigger)
                            .font(.system(.body, design: .monospaced))
                            .fontWeight(.medium)
                            .padding(.horizontal, 12)
                            .padding(.vertical, 6)
                            .background(Color.accentColor.opacity(0.1))
                            .cornerRadius(6)
                        
                        Image(systemName: "arrow.right")
                            .foregroundColor(.secondary)
                        
                        Text(replacement)
                            .font(.body)
                            .padding(.horizontal, 12)
                            .padding(.vertical, 6)
                            .frame(maxWidth: .infinity, alignment: .leading)
                            .background(Color.secondary.opacity(0.1))
                            .cornerRadius(6)
                    }
                } else {
                    Text("Nhập từ viết tắt và văn bản thay thế để xem trước")
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .italic()
                }
            }
            .padding()
        }
    }
    
    private var guidelinesSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Image(systemName: "info.circle")
                    .foregroundColor(.blue)
                Text("Hướng dẫn")
                    .font(.headline)
            }
            
            VStack(alignment: .leading, spacing: 8) {
                guidelineItem(
                    icon: "checkmark.circle.fill",
                    color: .green,
                    text: "Từ viết tắt nên ngắn gọn, dễ nhớ (2-5 ký tự)"
                )
                
                guidelineItem(
                    icon: "checkmark.circle.fill",
                    color: .green,
                    text: "Văn bản thay thế có thể chứa dấu câu, emoji"
                )
                
                guidelineItem(
                    icon: "xmark.circle.fill",
                    color: .red,
                    text: "Tránh dùng từ viết tắt trùng với từ thông dụng"
                )
                
                guidelineItem(
                    icon: "lightbulb.fill",
                    color: .orange,
                    text: "Ví dụ: 'brb' → 'be right back', 'ty' → 'thank you'"
                )
            }
        }
        .padding()
        .background(Color.secondary.opacity(0.05))
        .cornerRadius(8)
    }
    
    private func guidelineItem(icon: String, color: Color, text: String) -> some View {
        HStack(alignment: .top, spacing: 8) {
            Image(systemName: icon)
                .foregroundColor(color)
                .frame(width: 16)
            
            Text(text)
                .font(.caption)
                .foregroundColor(.secondary)
        }
    }
    
    private var footerView: some View {
        HStack {
            Button("Hủy") {
                isPresented = false
            }
            .keyboardShortcut(.escape, modifiers: [])
            
            Spacer()
            
            Button(isEditing ? "Cập nhật" : "Thêm") {
                save()
            }
            .keyboardShortcut(.return, modifiers: .command)
            .disabled(!canSave)
        }
        .padding()
    }
    
    // MARK: - Validation
    
    private func validateTrigger() {
        let trimmed = trigger.trimmingCharacters(in: .whitespaces)
        
        // Empty check
        if trimmed.isEmpty {
            triggerError = nil
            return
        }
        
        // Length check (max 20 characters per Rust core)
        if trimmed.count > 20 {
            triggerError = "Tối đa 20 ký tự"
            return
        }
        
        // Alphanumeric only
        let alphanumeric = CharacterSet.alphanumerics
        if trimmed.unicodeScalars.contains(where: { !alphanumeric.contains($0) }) {
            triggerError = "Chỉ được dùng chữ cái và số"
            return
        }
        
        // Duplicate check (exclude current trigger when editing)
        if existingShortcuts.contains(trimmed) {
            triggerError = "Từ viết tắt này đã tồn tại"
            return
        }
        
        // Common word warning (optional, can be removed)
        let commonWords = ["the", "and", "for", "you", "not", "but", "can", "all", "was", "are"]
        if commonWords.contains(trimmed.lowercased()) {
            triggerError = "Tránh dùng từ thông dụng"
            return
        }
        
        triggerError = nil
    }
    
    private func validateReplacement() {
        let trimmed = replacement.trimmingCharacters(in: .whitespacesAndNewlines)
        
        // Empty check (only check trimmed version for validation)
        if trimmed.isEmpty {
            replacementError = nil
            return
        }
        
        // Length check (max 200 characters - reasonable limit)
        if replacement.count > 200 {
            replacementError = "Tối đa 200 ký tự"
            return
        }
        
        replacementError = nil
    }
    
    // MARK: - Actions
    
    private func save() {
        // Trim whitespace only when saving
        trigger = trigger.trimmingCharacters(in: .whitespaces)
        replacement = replacement.trimmingCharacters(in: .whitespacesAndNewlines)
        
        validateTrigger()
        validateReplacement()
        
        guard canSave else { return }
        
        onSave(trigger, replacement)
        isPresented = false
    }
    
    // MARK: - Supporting Types
    
    private enum Field: Hashable {
        case trigger
        case replacement
    }
}

// MARK: - Preview

#Preview("Add New") {
    ShortcutEditorSheet(
        isPresented: .constant(true),
        existingShortcuts: ["brb", "omw"],
        onSave: { trigger, replacement in
            print("Save: \(trigger) → \(replacement)")
        }
    )
}

#Preview("Edit Existing") {
    ShortcutEditorSheet(
        isPresented: .constant(true),
        existingShortcuts: ["brb", "omw"],
        editingTrigger: "ty",
        editingReplacement: "thank you",
        onSave: { trigger, replacement in
            print("Update: \(trigger) → \(replacement)")
        }
    )
}
