//
//  TextExpansionSettingsView.swift
//  goxviet
//
//  Created on 2026-02-01.
//  Refactored to use SettingsManager as source of truth
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
    @State private var showingDeleteConfirmation = false
    @State private var shortcutToDelete: TextShortcutItem? = nil
    @State private var showingImportError = false
    @State private var importErrorMessage = ""
    @State private var showingExportSuccess = false
    @State private var lastSaveTime: Date? = nil

    // MARK: - Computed Properties
    
    private var filteredShortcuts: [TextShortcutItem] {
        if searchText.isEmpty {
            return settingsManager.shortcuts.sorted { $0.trigger < $1.trigger }
        }
        return settingsManager.shortcuts.filter { shortcut in
            shortcut.trigger.localizedCaseInsensitiveContains(searchText) ||
            shortcut.replacement.localizedCaseInsensitiveContains(searchText)
        }.sorted { $0.trigger < $1.trigger }
    }
    
    private var shortcutCount: Int {
        settingsManager.shortcuts.count
    }
    
    private var enabledShortcutCount: Int {
        settingsManager.shortcuts.filter { $0.isEnabled }.count
    }
    
    // MARK: - Body
    
    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                headerView
                enableToggleSection
                toolbarSection
                shortcutsListSection
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
        .sheet(item: $editingShortcut) { shortcut in
            ShortcutEditorSheet(
                isPresented: Binding(
                    get: { editingShortcut != nil },
                    set: { if !$0 { editingShortcut = nil } }
                ),
                existingShortcuts: settingsManager.shortcuts.map { $0.trigger },
                editingTrigger: shortcut.trigger,
                editingReplacement: shortcut.replacement,
                onSave: { trigger, replacement in
                    settingsManager.updateShortcut(
                        oldTrigger: shortcut.trigger,
                        newTrigger: trigger,
                        replacement: replacement
                    )
                    editingShortcut = nil
                }
            )
        }
        .sheet(isPresented: $showingAddSheet) {
            ShortcutEditorSheet(
                isPresented: $showingAddSheet,
                existingShortcuts: settingsManager.shortcuts.map { $0.trigger },
                onSave: { trigger, replacement in
                    settingsManager.addShortcut(trigger: trigger, replacement: replacement)
                }
            )
        }
        .alert("Xóa gõ tắt?", isPresented: $showingDeleteConfirmation) {
            Button("Hủy", role: .cancel) {
                shortcutToDelete = nil
            }
            Button("Xóa", role: .destructive) {
                if let shortcut = shortcutToDelete {
                    settingsManager.removeShortcut(trigger: shortcut.trigger)
                }
                shortcutToDelete = nil
            }
        } message: {
            if let shortcut = shortcutToDelete {
                Text("Bạn có chắc muốn xóa gõ tắt '\(shortcut.trigger)'?")
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
            Text("Đã xuất \(shortcutCount) gõ tắt ra file")
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
            
            Button {
                importShortcuts()
            } label: {
                Label("Nhập", systemImage: "square.and.arrow.down")
            }
            .help("Nhập danh sách gõ tắt từ file")
            
            Button {
                exportShortcuts()
            } label: {
                Label("Xuất", systemImage: "square.and.arrow.up")
            }
            .help("Xuất danh sách gõ tắt ra file")
            
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
            HStack {
                Text("Viết tắt")
                    .font(.headline)
                    .frame(width: 120, alignment: .leading)
                
                Text("Thay thế")
                    .font(.headline)
                    .frame(maxWidth: .infinity, alignment: .leading)
                
                Text("Bật")
                    .font(.headline)
                    .frame(width: 50, alignment: .center)
                
                Text("Thao tác")
                    .font(.headline)
                    .frame(width: 100, alignment: .trailing)
            }
            .padding()
            .background(Color(NSColor.controlBackgroundColor))
            
            Divider()
            
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
            Text(shortcut.trigger)
                .font(.system(.body, design: .monospaced))
                .fontWeight(.medium)
                .frame(width: 120, alignment: .leading)
            
            Text(shortcut.replacement)
                .font(.body)
                .frame(maxWidth: .infinity, alignment: .leading)
                .lineLimit(2)
            
            Toggle("", isOn: Binding(
                get: { shortcut.isEnabled },
                set: { newValue in
                    if let index = settingsManager.shortcuts.firstIndex(where: { $0.id == shortcut.id }) {
                        settingsManager.shortcuts[index].isEnabled = newValue
                    }
                }
            ))
            .toggleStyle(.checkbox)
            .labelsHidden()
            .frame(width: 50, alignment: .center)
            
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
            .frame(width: 100, alignment: .trailing)
        }
        .padding()
        .background(Color(NSColor.textBackgroundColor))
    }
    
    private var footerView: some View {
        HStack {
            VStack(alignment: .leading) {
                Text("\(enabledShortcutCount)/\(shortcutCount) gõ tắt đang bật")
                    .font(.caption)
                    .foregroundColor(.secondary)
                if let date = lastSaveTime {
                    Text("Đã lưu: \(date, formatter: itemFormatter)")
                        .font(.caption)
                        .foregroundColor(.green)
                } else {
                    Text("Chưa lưu")
                        .font(.caption)
                        .foregroundColor(.gray)
                }
            }
            
            Spacer()
        }
    }
    
    // MARK: - Actions
    
    private func importShortcuts() {
        let panel = NSOpenPanel()
        panel.allowedContentTypes = [.plainText]
        panel.allowsMultipleSelection = false
        panel.canChooseDirectories = false
        panel.message = "Chọn file chứa danh sách gõ tắt (định dạng: viết_tắt:nội_dung)"
        
        panel.begin { response in
            guard response == .OK, let url = panel.url else { return }
            
            do {
                let content = try String(contentsOf: url, encoding: .utf8)
                let count = settingsManager.importShortcuts(from: content)
                
                if count > 0 {
                    Log.info("Imported \(count) shortcuts from \(url.lastPathComponent)")
                } else {
                    importErrorMessage = "Không tìm thấy gõ tắt hợp lệ trong file"
                    showingImportError = true
                }
            } catch {
                importErrorMessage = "Không thể đọc file: \(error.localizedDescription)"
                showingImportError = true
            }
        }
    }
    
    private func exportShortcuts() {
        let content = settingsManager.exportShortcutsToString()
        
        let panel = NSSavePanel()
        panel.allowedContentTypes = [.plainText]
        panel.nameFieldStringValue = "goxviet-shortcuts.txt"
        panel.message = "Lưu danh sách gõ tắt"
        
        panel.begin { response in
            guard response == .OK, let url = panel.url else { return }
            
            do {
                try content.write(to: url, atomically: true, encoding: .utf8)
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
    formatter.dateStyle = .short
    formatter.timeStyle = .short
    return formatter
}()

// MARK: - Preview

#Preview {
    TextExpansionSettingsView()
        .frame(width: 700, height: 600)
}
