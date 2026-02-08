//
//  TextExpansionSettingsView.swift
//  goxviet
//
//  Created on 2026-02-01.
//  Part of Phase 2.9.2: Text Expansion UI Implementation
//

import SwiftUI
import UniformTypeIdentifiers

/// Main settings view for Text Expansion (gõ tắt) feature
/// Provides CRUD operations, import/export, and search functionality
struct TextExpansionSettingsView: View {
    // MARK: - Properties
    
    @ObservedObject private var settingsManager = SettingsManager.shared
    @State private var searchText = ""
    @State private var showingAddSheet = false
    @State private var editingShortcut: TextShortcutItem? = nil
    
    // Computed property to track edit sheet state
    private var isEditSheetPresented: Bool {
        editingShortcut != nil
    }
    
    @State private var showingDeleteConfirmation = false
    @State private var shortcutToDelete: TextShortcutItem? = nil
    @State private var showingImportError = false
    @State private var importErrorMessage = ""
    @State private var showingExportSuccess = false

    // MARK: - Computed Properties
    
    private var filteredShortcuts: [TextShortcutItem] {
        if searchText.isEmpty {
            return settingsManager.shortcuts.sorted { $0.key < $1.key }
        }
        return settingsManager.shortcuts.filter { shortcut in
            shortcut.key.localizedCaseInsensitiveContains(searchText) ||
            shortcut.value.localizedCaseInsensitiveContains(searchText)
        }.sorted { $0.key < $1.key }
    }
    
    private var shortcutCount: Int {
        settingsManager.shortcuts.filter { $0.isEnabled }.count
    }
    
    private var shortcutCapacity: Int {
        100 // Default capacity
    }
    
    // MARK: - Body
    
    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                // Header
                headerView
                
                // Enable/Disable Toggle
                enableToggleSection
                
                // Toolbar
                toolbarSection
                
                // Shortcuts List
                shortcutsListSection
                
                // Footer
                footerView
            }
            .padding()
        }
        .frame(minWidth: 600, minHeight: 500)
        .onAppear {
            // Sync enabled state to engine when view appears
            RustBridge.shared.setShortcutsEnabled(settingsManager.textExpansionEnabled)
        }
        .sheet(isPresented: $showingAddSheet) {
            ShortcutEditorSheet(
                isPresented: $showingAddSheet,
                existingShortcuts: settingsManager.shortcuts.map { $0.key },
                onSave: { trigger, replacement in
                    addShortcut(trigger: trigger, replacement: replacement)
                }
            )
        }
        .sheet(item: $editingShortcut) { shortcut in
            ShortcutEditorSheet(
                isPresented: Binding(
                    get: { editingShortcut != nil },
                    set: { if !$0 { editingShortcut = nil } }
                ),
                existingShortcuts: settingsManager.shortcuts.map { $0.key }.filter { $0 != shortcut.key },
                editingTrigger: shortcut.key,
                editingReplacement: shortcut.value,
                onSave: { trigger, replacement in
                    updateShortcut(oldId: shortcut.id, newKey: trigger, newValue: replacement)
                }
            )
        }
        .alert("Xóa gõ tắt?", isPresented: $showingDeleteConfirmation) {
            Button("Hủy", role: .cancel) {
                shortcutToDelete = nil
            }
            Button("Xóa", role: .destructive) {
                if let shortcut = shortcutToDelete {
                    deleteShortcut(shortcut)
                }
                shortcutToDelete = nil
            }
        } message: {
            if let shortcut = shortcutToDelete {
                Text("Bạn có chắc muốn xóa gõ tắt '\(shortcut.key)'?")
            }
        }
        .alert("Lỗi nhập dữ liệu", isPresented: $showingImportError) {
            Button("OK", role: .cancel) {}
        } message: {
            Text(importErrorMessage)
        }
        .alert("Xuất thành công", isPresented: $showingExportSuccess) {
            Button("OK", role: .cancel) {}
        } message: {
            Text("Đã xuất \(shortcutCount) gõ tắt ra file JSON")
        }
    }
    
    // MARK: - View Components
    
    private var headerView: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack(spacing: 8) {
                Text("Gõ tắt (Text Expansion)")
                    .font(.title2)
                    .fontWeight(.semibold)
                Text("(Beta)")
                    .font(.system(size: 14, weight: .semibold))
                    .foregroundColor(.orange)
                    .padding(.horizontal, 8)
                    .padding(.vertical, 4)
                    .background(Capsule().fill(Color.orange.opacity(0.2)))
            }
            
            Text("Tự động thay thế từ viết tắt thành văn bản đầy đủ")
                .font(.subheadline)
                .foregroundColor(.secondary)
        }
    }
    
    private var enableToggleSection: some View {
        GroupBox {
            VStack(alignment: .leading, spacing: 12) {
                Toggle(isOn: $settingsManager.textExpansionEnabled) {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Bật gõ tắt")
                            .font(.headline)
                        Text("Cho phép tự động thay thế từ viết tắt")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }
                .toggleStyle(.switch)
                .onChange(of: settingsManager.textExpansionEnabled) { _, newValue in
                    RustBridge.shared.setShortcutsEnabled(newValue)
                    Log.info("Text expansion \(newValue ? "enabled" : "disabled")")
                }
                
                if !settingsManager.textExpansionEnabled {
                    HStack(spacing: 8) {
                        Image(systemName: "info.circle.fill")
                            .foregroundColor(.orange)
                        Text("Gõ tắt hiện đang tắt. Bật để sử dụng tính năng này.")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                    .padding(.top, 4)
                }
            }
            .padding()
        }
    }
    
    private var toolbarSection: some View {
        HStack(spacing: 12) {
            // Search field
            HStack {
                Image(systemName: "magnifyingglass")
                    .foregroundColor(.secondary)
                TextField("Tìm kiếm gõ tắt...", text: $searchText)
                    .textFieldStyle(.plain)
                if !searchText.isEmpty {
                    Button {
                        searchText = ""
                    } label: {
                        Image(systemName: "xmark.circle.fill")
                            .foregroundColor(.secondary)
                    }
                    .buttonStyle(.plain)
                }
            }
            .padding(8)
            .background(Color(NSColor.controlBackgroundColor))
            .cornerRadius(6)
            
            Spacer()
            
            // Action buttons
            Button {
                importShortcuts()
            } label: {
                Label("Nhập", systemImage: "square.and.arrow.down")
            }
            .help("Nhập danh sách gõ tắt từ file JSON")
            
            Button {
                exportShortcuts()
            } label: {
                Label("Xuất", systemImage: "square.and.arrow.up")
            }
            .help("Xuất danh sách gõ tắt ra file JSON")
            
            Button {
                showingAddSheet = true
            } label: {
                Label("Thêm", systemImage: "plus")
            }
            .keyboardShortcut("+", modifiers: .command)
            .help("Thêm gõ tắt mới (⌘+)")
        }
    }
    
    private var shortcutsListSection: some View {
        GroupBox {
            if settingsManager.shortcuts.isEmpty {
                emptyStateView
            } else if filteredShortcuts.isEmpty {
                noResultsView
            } else {
                shortcutsTable
            }
        }
    }
    
    private var emptyStateView: some View {
        VStack(spacing: 16) {
            Image(systemName: "text.badge.plus")
                .font(.system(size: 48))
                .foregroundColor(.secondary)
            
            Text("Chưa có gõ tắt nào")
                .font(.headline)
                .foregroundColor(.secondary)
            
            Text("Thêm gõ tắt để tự động thay thế từ viết tắt")
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
            
            Button {
                showingAddSheet = true
            } label: {
                Label("Thêm gõ tắt đầu tiên", systemImage: "plus.circle.fill")
            }
            .buttonStyle(.borderedProminent)
        }
        .frame(maxWidth: .infinity)
        .padding(40)
    }
    
    private var noResultsView: some View {
        VStack(spacing: 12) {
            Image(systemName: "magnifyingglass")
                .font(.system(size: 36))
                .foregroundColor(.secondary)
            
            Text("Không tìm thấy kết quả")
                .font(.headline)
                .foregroundColor(.secondary)
            
            Text("Thử tìm kiếm với từ khóa khác")
                .font(.subheadline)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity)
        .padding(30)
    }
    
    private var shortcutsTable: some View {
        VStack(spacing: 0) {
            // Table header
            HStack {
                Text("Viết tắt")
                    .font(.headline)
                    .frame(width: 150, alignment: .leading)
                
                Text("Thay thế")
                    .font(.headline)
                    .frame(maxWidth: .infinity, alignment: .leading)
                
                Text("Thao tác")
                    .font(.headline)
                    .frame(width: 120, alignment: .trailing)
            }
            .padding()
            .background(Color(NSColor.controlBackgroundColor))
            
            Divider()
            
            // Table rows
            ScrollView {
                LazyVStack(spacing: 0) {
                    ForEach(filteredShortcuts) { shortcut in
                        shortcutRow(shortcut)
                        Divider()
                    }
                }
            }
            .frame(minHeight: 200, maxHeight: 400)
        }
        .background(Color(NSColor.textBackgroundColor))
        .cornerRadius(8)
    }
    
    private func shortcutRow(_ shortcut: TextShortcutItem) -> some View {
        HStack(alignment: .top, spacing: 12) {
            // Trigger
            Text(shortcut.key)
                .font(.system(.body, design: .monospaced))
                .fontWeight(.medium)
                .frame(width: 150, alignment: .leading)
            
            // Replacement
            Text(shortcut.value)
                .font(.body)
                .frame(maxWidth: .infinity, alignment: .leading)
                .lineLimit(2)
            
            // Actions
            HStack(spacing: 8) {
                Button {
                    editingShortcut = shortcut
                } label: {
                    Image(systemName: "pencil")
                }
                .buttonStyle(.plain)
                .help("Sửa gõ tắt")
                
                Button {
                    shortcutToDelete = shortcut
                    showingDeleteConfirmation = true
                } label: {
                    Image(systemName: "trash")
                        .foregroundColor(.red)
                }
                .buttonStyle(.plain)
                .help("Xóa gõ tắt")
            }
            .frame(width: 120, alignment: .trailing)
        }
        .padding()
        .background(Color(NSColor.textBackgroundColor))
    }
    
    private var footerView: some View {
        HStack {
            VStack(alignment: .leading) {
                Text("\(shortcutCount) / \(shortcutCapacity) gõ tắt")
                    .font(.caption)
                    .foregroundColor(.secondary)
                Text("Auto-saved")
                    .font(.caption)
                    .foregroundColor(.green)
            }
            
            Spacer()
            
            if shortcutCount >= shortcutCapacity {
                HStack(spacing: 4) {
                    Image(systemName: "exclamationmark.triangle.fill")
                        .foregroundColor(.orange)
                    Text("Đã đạt giới hạn tối đa")
                        .font(.caption)
                        .foregroundColor(.orange)
                }
            }
        }
    }
    
    // MARK: - Actions
    
    /// Add new shortcut - modifies settingsManager.shortcuts directly
    /// Auto-saved via didSet and auto-synced to engine via Combine
    private func addShortcut(trigger: String, replacement: String) {
        guard !trigger.isEmpty else { return }
        
        // Check for duplicates
        if let index = settingsManager.shortcuts.firstIndex(where: { $0.key == trigger }) {
            // Update existing
            settingsManager.shortcuts[index].value = replacement
            settingsManager.shortcuts[index].isEnabled = true
            Log.info("Updated shortcut: \(trigger)")
        } else {
            // Add new
            let newShortcut = TextShortcutItem(key: trigger, value: replacement, isEnabled: true)
            settingsManager.shortcuts.append(newShortcut)
            Log.info("Added shortcut: \(trigger) → \(replacement)")
        }
    }
    
    /// Update existing shortcut by ID
    private func updateShortcut(oldId: UUID, newKey: String, newValue: String) {
        guard !newKey.isEmpty else { return }
        
        if let index = settingsManager.shortcuts.firstIndex(where: { $0.id == oldId }) {
            settingsManager.shortcuts[index].key = newKey
            settingsManager.shortcuts[index].value = newValue
            Log.info("Updated shortcut at index \(index)")
        }
    }
    
    /// Delete shortcut
    private func deleteShortcut(_ shortcut: TextShortcutItem) {
        settingsManager.shortcuts.removeAll { $0.id == shortcut.id }
        Log.info("Deleted shortcut: \(shortcut.key)")
    }
    
    /// Import shortcuts from custom text format
    private func importShortcuts() {
        let panel = NSOpenPanel()
        panel.allowedContentTypes = [.plainText]
        panel.allowsMultipleSelection = false
        panel.canChooseDirectories = false
        panel.message = "Chọn file chứa danh sách gõ tắt"
        
        panel.begin { [weak settingsManager] response in
            guard response == .OK, let url = panel.url else { return }
            
            do {
                let content = try String(contentsOf: url, encoding: .utf8)
                let imported = self.parseShortcuts(from: content)
                
                // Merge imported with existing
                for shortcut in imported {
                    if let index = settingsManager?.shortcuts.firstIndex(where: { $0.key == shortcut.key }) {
                        settingsManager?.shortcuts[index].value = shortcut.value
                        settingsManager?.shortcuts[index].isEnabled = true
                    } else {
                        settingsManager?.shortcuts.append(shortcut)
                    }
                }
                
                Log.info("Imported \(imported.count) shortcuts from \(url.lastPathComponent)")
            } catch {
                self.importErrorMessage = "Không thể đọc file: \(error.localizedDescription)"
                self.showingImportError = true
            }
        }
    }
    
    /// Parse shortcuts from custom text format
    private func parseShortcuts(from content: String) -> [TextShortcutItem] {
        var shortcuts: [TextShortcutItem] = []
        let lines = content.components(separatedBy: .newlines)
        
        for line in lines {
            let trimmed = line.trimmingCharacters(in: .whitespaces)
            guard !trimmed.isEmpty, !trimmed.hasPrefix(";"),
                  let colonIndex = trimmed.firstIndex(of: ":") else { continue }
            
            let key = String(trimmed[..<colonIndex]).trimmingCharacters(in: .whitespaces)
            let value = String(trimmed[trimmed.index(after: colonIndex)...]).trimmingCharacters(in: .whitespaces)
            
            guard !key.isEmpty else { continue }
            shortcuts.append(TextShortcutItem(key: key, value: value, isEnabled: true))
        }
        
        return shortcuts
    }
    
    /// Export shortcuts to custom text format
    private func exportShortcuts() {
        var lines = [";Gõ Việt - Bảng gõ tắt"]
        for shortcut in settingsManager.shortcuts where !shortcut.key.isEmpty {
            lines.append("\(shortcut.key):\(shortcut.value)")
        }
        let content = lines.joined(separator: "\n")
        
        let panel = NSSavePanel()
        panel.allowedContentTypes = [.plainText]
        panel.nameFieldStringValue = "goxviet-shortcuts.txt"
        panel.message = "Lưu danh sách gõ tắt"
        
        panel.begin { response in
            guard response == .OK, let url = panel.url else { return }
            
            do {
                try content.write(to: url, atomically: true, encoding: .utf8)
                self.showingExportSuccess = true
                Log.info("Exported \(self.settingsManager.shortcuts.count) shortcuts to \(url.lastPathComponent)")
            } catch {
                self.importErrorMessage = "Không thể lưu file: \(error.localizedDescription)"
                self.showingImportError = true
            }
        }
    }
}

// MARK: - Preview

#Preview {
    TextExpansionSettingsView()
        .frame(width: 700, height: 600)
}
