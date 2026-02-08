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
    @State private var shortcuts: [(trigger: String, replacement: String)] = []
    @State private var searchText = ""
    @State private var showingAddSheet = false
    @State private var editingShortcut: (trigger: String, replacement: String)? = nil
    
    // Computed property to track edit sheet state
    private var isEditSheetPresented: Bool {
        editingShortcut != nil
    }
    
    @State private var showingDeleteConfirmation = false
    @State private var shortcutToDelete: String? = nil
    @State private var showingImportError = false
    @State private var importErrorMessage = ""
    @State private var showingExportSuccess = false
    @State private var lastSaveTime: Date? = nil

    // MARK: - Computed Properties
    
    private var filteredShortcuts: [(trigger: String, replacement: String)] {
        if searchText.isEmpty {
            return shortcuts.sorted { $0.trigger < $1.trigger }
        }
        return shortcuts.filter { shortcut in
            shortcut.trigger.localizedCaseInsensitiveContains(searchText) ||
            shortcut.replacement.localizedCaseInsensitiveContains(searchText)
        }.sorted { $0.trigger < $1.trigger }
    }
    
    private var shortcutCount: Int {
        RustBridge.shared.getShortcutsCount()
    }
    
    private var shortcutCapacity: Int {
        RustBridge.shared.getShortcutsCapacity()
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
        .onReceive(NotificationCenter.default.publisher(for: .didSaveShortcuts)) { notification in
            if let date = notification.object as? Date {
                self.lastSaveTime = date
            }
        }
        .onChange(of: settingsManager.shortcutsLoaded) { _, loaded in
            if loaded {
                // Sync enabled state to engine
                RustBridge.shared.setShortcutsEnabled(settingsManager.textExpansionEnabled)
                loadShortcuts()
            }
        }
        .onChange(of: showingAddSheet) { _, newValue in
            // Refresh list when sheet dismisses
            if !newValue {
                // Add small delay to ensure sheet animation completes
                DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) {
                    loadShortcuts()
                }
            }
        }
        .sheet(isPresented: $showingAddSheet) {
            ShortcutEditorSheet(
                isPresented: $showingAddSheet,
                existingShortcuts: shortcuts.map { $0.trigger },
                onSave: { trigger, replacement in
                    addShortcut(trigger: trigger, replacement: replacement)
                }
            )
        }
        .onChange(of: isEditSheetPresented) { _, newValue in
            // Refresh list when edit sheet dismisses
            if !newValue {
                // Add small delay to ensure sheet animation completes
                DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) {
                    loadShortcuts()
                }
            }
        }
        .sheet(item: Binding(
            get: { editingShortcut.map { EditableShortcut(trigger: $0.trigger, replacement: $0.replacement) } },
            set: { editingShortcut = $0.map { ($0.trigger, $0.replacement) } }
        )) { shortcut in
            ShortcutEditorSheet(
                isPresented: Binding(
                    get: { editingShortcut != nil },
                    set: { if !$0 { editingShortcut = nil } }
                ),
                existingShortcuts: shortcuts.map { $0.trigger }.filter { $0 != shortcut.trigger },
                editingTrigger: shortcut.trigger,
                editingReplacement: shortcut.replacement,
                onSave: { trigger, replacement in
                    updateShortcut(oldTrigger: shortcut.trigger, newTrigger: trigger, replacement: replacement)
                }
            )
        }
        .alert("Xóa gõ tắt?", isPresented: $showingDeleteConfirmation) {
            Button("Hủy", role: .cancel) {
                shortcutToDelete = nil
            }
            Button("Xóa", role: .destructive) {
                if let trigger = shortcutToDelete {
                    deleteShortcut(trigger: trigger)
                }
                shortcutToDelete = nil
            }
        } message: {
            if let trigger = shortcutToDelete {
                Text("Bạn có chắc muốn xóa gõ tắt '\(trigger)'?")
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
            if shortcuts.isEmpty {
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
                    ForEach(filteredShortcuts, id: \.trigger) { shortcut in
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
    
    private func shortcutRow(_ shortcut: (trigger: String, replacement: String)) -> some View {
        HStack(alignment: .top, spacing: 12) {
            // Trigger
            Text(shortcut.trigger)
                .font(.system(.body, design: .monospaced))
                .fontWeight(.medium)
                .frame(width: 150, alignment: .leading)
            
            // Replacement
            Text(shortcut.replacement)
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
                    shortcutToDelete = shortcut.trigger
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
                if let date = lastSaveTime {
                    Text("Last Saved: \(date, formatter: itemFormatter)")
                        .font(.caption)
                        .foregroundColor(.green)
                } else {
                    Text("Last Saved: Never")
                        .font(.caption)
                        .foregroundColor(.gray)
                }
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
    
    private func loadShortcuts() {
        guard let json = RustBridge.shared.exportShortcutsJSON() else {
            Log.error("Failed to export shortcuts JSON from engine")
            self.shortcuts = []
            return
        }
        
        Log.info("Exported JSON from engine: \(json)")
        
        guard let data = json.data(using: .utf8) else {
            Log.error("Failed to convert JSON string to data")
            self.shortcuts = []
            return
        }
        
        do {
            let exportData = try JSONDecoder().decode(ShortcutsExport.self, from: data)
            Log.info("Loaded \(exportData.shortcuts.count) shortcuts from engine (version \(exportData.version))")
            DispatchQueue.main.async {
                self.shortcuts = exportData.shortcuts.map { (trigger: $0.trigger, replacement: $0.replacement) }
                Log.info("Updated shortcuts state: \(self.shortcuts.count) items")
            }
        } catch {
            Log.error("Failed to decode shortcuts JSON: \(error)")
            self.shortcuts = []
        }
    }
    
    private func addShortcut(trigger: String, replacement: String) {
        let success = RustBridge.shared.addShortcut(trigger: trigger, replacement: replacement)
        if success {
            Log.info("Added shortcut: \(trigger) → \(replacement)")
            SettingsManager.shared.saveShortcuts()
            // List will be refreshed by .onChange(of: showingAddSheet)
        } else {
            Log.error("Failed to add shortcut: \(trigger)")
        }
    }
    
    private func updateShortcut(oldTrigger: String, newTrigger: String, replacement: String) {
        // Remove old, add new
        RustBridge.shared.removeShortcut(trigger: oldTrigger)
        let success = RustBridge.shared.addShortcut(trigger: newTrigger, replacement: replacement)
        if success {
            Log.info("Updated shortcut: \(oldTrigger) → \(newTrigger): \(replacement)")
            SettingsManager.shared.saveShortcuts()
            // List will be refreshed by .onChange(of: isEditSheetPresented)
        } else {
            Log.error("Failed to update shortcut: \(oldTrigger) → \(newTrigger)")
        }
    }
    
    private func deleteShortcut(trigger: String) {
        RustBridge.shared.removeShortcut(trigger: trigger)
        Log.info("Deleted shortcut: \(trigger)")
        SettingsManager.shared.saveShortcuts()
        // Refresh immediately for delete since no sheet is involved
        DispatchQueue.main.async {
            loadShortcuts()
        }
    }
    
    private func importShortcuts() {
        let panel = NSOpenPanel()
        panel.allowedContentTypes = [.json]
        panel.allowsMultipleSelection = false
        panel.canChooseDirectories = false
        panel.message = "Chọn file JSON chứa danh sách gõ tắt"
        
        panel.begin { response in
            guard response == .OK, let url = panel.url else { return }
            
            do {
                let json = try String(contentsOf: url, encoding: .utf8)
                let count = RustBridge.shared.importShortcutsJSON(json)
                
                if count >= 0 {
                    loadShortcuts()
                    SettingsManager.shared.saveShortcuts()
                    Log.info("Imported \(count) shortcuts from \(url.lastPathComponent)")
                } else {
                    importErrorMessage = "File JSON không đúng định dạng"
                    showingImportError = true
                }
            } catch {
                importErrorMessage = "Không thể đọc file: \(error.localizedDescription)"
                showingImportError = true
            }
        }
    }
    
    private func exportShortcuts() {
        guard let json = RustBridge.shared.exportShortcutsJSON() else {
            importErrorMessage = "Không thể xuất dữ liệu"
            showingImportError = true
            return
        }
        
        let panel = NSSavePanel()
        panel.allowedContentTypes = [.json]
        panel.nameFieldStringValue = "goxviet-shortcuts.json"
        panel.message = "Lưu danh sách gõ tắt"
        
        panel.begin { response in
            guard response == .OK, let url = panel.url else { return }
            
            do {
                try json.write(to: url, atomically: true, encoding: .utf8)
                showingExportSuccess = true
                Log.info("Exported \(shortcutCount) shortcuts to \(url.lastPathComponent)")
            } catch {
                importErrorMessage = "Không thể lưu file: \(error.localizedDescription)"
                showingImportError = true
            }
        }
    }
}

private let itemFormatter: DateFormatter = {
    let formatter = DateFormatter()
    formatter.dateStyle = .medium
    formatter.timeStyle = .medium
    return formatter
}()

// MARK: - Supporting Types

private struct EditableShortcut: Identifiable {
    let id = UUID()
    let trigger: String
    let replacement: String
}

// MARK: - JSON Decoding Models

private struct ShortcutsExport: Codable {
    let version: Int
    let shortcuts: [ShortcutItem]
}

private struct ShortcutItem: Codable {
    let trigger: String
    let replacement: String
    let enabled: Bool
    let method: String
    let condition: String
}

// MARK: - Preview

#Preview {
    TextExpansionSettingsView()
        .frame(width: 700, height: 600)
}
