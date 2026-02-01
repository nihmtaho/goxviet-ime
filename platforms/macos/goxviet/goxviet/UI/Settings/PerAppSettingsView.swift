//
//  PerAppSettingsView.swift
//  GoxViet
//
//  Enhanced Per-App Settings with search, grouping, and app icons
//

import SwiftUI
import AppKit

struct PerAppSettingsView: View {
    @Binding var smartModeEnabled: Bool
    @Binding var perAppModes: [String: Bool]
    @Binding var showClearConfirmation: Bool
    let reloadAction: () -> Void
    
    @State private var searchText = ""
    @State private var sortOption: SortOption = .alphabetical
    @State private var filterOption: FilterOption = .all
    @State private var selectedApps: Set<String> = []
    @State private var showBulkActions = false
    
    enum SortOption: String, CaseIterable {
        case alphabetical = "Alphabetical"
        case recentlyUsed = "Recently Used"
        case status = "Status"
    }
    
    enum FilterOption: String, CaseIterable {
        case all = "All Apps"
        case enabled = "Enabled"
        case disabled = "Disabled"
    }
    
    var filteredAndSortedApps: [(String, Bool)] {
        var apps = Array(perAppModes)
        
        // Filter by search
        if !searchText.isEmpty {
            apps = apps.filter { bundleId, _ in
                let appName = getAppName(from: bundleId)
                return appName.localizedCaseInsensitiveContains(searchText) ||
                       bundleId.localizedCaseInsensitiveContains(searchText)
            }
        }
        
        // Filter by status
        switch filterOption {
        case .all:
            break
        case .enabled:
            apps = apps.filter { $0.1 == true }
        case .disabled:
            apps = apps.filter { $0.1 == false }
        }
        
        // Sort
        switch sortOption {
        case .alphabetical:
            apps.sort { getAppName(from: $0.0) < getAppName(from: $1.0) }
        case .recentlyUsed:
            // For now, same as alphabetical. Could track usage time later
            apps.sort { getAppName(from: $0.0) < getAppName(from: $1.0) }
        case .status:
            apps.sort { $0.1 && !$1.1 } // Enabled first
        }
        
        return apps
    }
    
    var body: some View {
        VStack(spacing: 0) {
            // Header
            VStack(alignment: .leading, spacing: 16) {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Per-App Settings")
                        .font(.system(size: 20, weight: .semibold))
                    Text("Configure IME behavior for individual applications")
                        .font(.system(size: 13))
                        .foregroundColor(.secondary)
                }
                
                // Smart Mode Toggle
                ToggleRow(
                    title: "Smart Mode",
                    description: "Remember IME state per application",
                    systemImage: "brain.head.profile",
                    isOn: $smartModeEnabled
                )
                // SettingsManager handles notification
                
                if !smartModeEnabled {
                    HStack {
                        Image(systemName: "info.circle")
                            .foregroundColor(.orange)
                        Text("Smart Mode is disabled. Per-app settings will not be applied.")
                            .font(.system(size: 12))
                            .foregroundColor(.secondary)
                    }
                    .padding(12)
                    .background(
                        RoundedRectangle(cornerRadius: 8)
                            .fill(Color.orange.opacity(0.1))
                    )
                }
            }
            .padding(24)
            .background(Color(nsColor: .controlBackgroundColor).opacity(0.3))
            
            Divider()
            
            // Toolbar
            if smartModeEnabled {
                HStack(spacing: 12) {
                    // Search
                    HStack {
                        Image(systemName: "magnifyingglass")
                            .foregroundColor(.secondary)
                        TextField("Search apps...", text: $searchText)
                            .textFieldStyle(.plain)
                    }
                    .padding(8)
                    .background(
                        RoundedRectangle(cornerRadius: 8)
                            .fill(Color(nsColor: .textBackgroundColor))
                    )
                    
                    Divider()
                        .frame(height: 20)
                    
                    // Filter
                    Picker("Filter", selection: $filterOption) {
                        ForEach(FilterOption.allCases, id: \.self) { option in
                            Text(option.rawValue).tag(option)
                        }
                    }
                    .pickerStyle(.menu)
                    .frame(width: 120)
                    
                    // Sort
                    Picker("Sort", selection: $sortOption) {
                        ForEach(SortOption.allCases, id: \.self) { option in
                            Text(option.rawValue).tag(option)
                        }
                    }
                    .pickerStyle(.menu)
                    .frame(width: 140)
                    
                    Spacer()
                    
                    // Bulk Actions
                    if !selectedApps.isEmpty {
                        Menu {
                            Button("Enable Selected") {
                                bulkAction(enable: true)
                            }
                            Button("Disable Selected") {
                                bulkAction(enable: false)
                            }
                            Divider()
                            Button("Deselect All") {
                                selectedApps.removeAll()
                            }
                        } label: {
                            Label("\(selectedApps.count) selected", systemImage: "checkmark.circle")
                        }
                    }
                    
                    // Clear All
                    Button {
                        showClearConfirmation = true
                    } label: {
                        Label("Clear All", systemImage: "trash")
                    }
                    .buttonStyle(.bordered)
                    .alert("Clear All Settings", isPresented: $showClearConfirmation) {
                        Button("Cancel", role: .cancel) { }
                        Button("Clear", role: .destructive) {
                            clearAllSettings()
                        }
                    } message: {
                        Text("This will remove all per-app settings. This action cannot be undone.")
                    }
                }
                .padding(.horizontal, 24)
                .padding(.vertical, 12)
                .background(Color(nsColor: .controlBackgroundColor).opacity(0.2))
            }
            
            Divider()
            
            // App List
            if smartModeEnabled {
                if filteredAndSortedApps.isEmpty {
                    VStack(spacing: 12) {
                        Image(systemName: searchText.isEmpty ? "app.badge" : "magnifyingglass")
                            .font(.system(size: 48))
                            .foregroundColor(.secondary)
                        Text(searchText.isEmpty ? "No apps configured yet" : "No apps found")
                            .font(.title3)
                        Text(searchText.isEmpty ? "Start typing in apps to save their IME state" : "Try a different search term")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                } else {
                    ScrollView {
                        LazyVStack(spacing: 8) {
                            ForEach(filteredAndSortedApps, id: \.0) { bundleId, enabled in
                                AppRow(
                                    bundleId: bundleId,
                                    enabled: enabled,
                                    isSelected: selectedApps.contains(bundleId),
                                    onToggle: {
                                        toggleApp(bundleId)
                                    },
                                    onSelect: {
                                        if selectedApps.contains(bundleId) {
                                            selectedApps.remove(bundleId)
                                        } else {
                                            selectedApps.insert(bundleId)
                                        }
                                    }
                                )
                            }
                        }
                        .padding(24)
                    }
                }
            } else {
                Spacer()
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
    
    private func getAppName(from bundleId: String) -> String {
        return PerAppModeManagerEnhanced.shared.getAppName(bundleId)
    }
    
    private func toggleApp(_ bundleId: String) {
        perAppModes[bundleId]?.toggle()
        PerAppModeManagerEnhanced.shared.setPerAppMode(bundleId: bundleId, enabled: perAppModes[bundleId]!)
        reloadAction()
    }
    
    private func bulkAction(enable: Bool) {
        for bundleId in selectedApps {
            perAppModes[bundleId] = enable
            PerAppModeManagerEnhanced.shared.setPerAppMode(bundleId: bundleId, enabled: enable)
        }
        selectedApps.removeAll()
        reloadAction()
    }
    
    private func clearAllSettings() {
        PerAppModeManagerEnhanced.shared.clearAllPerAppModes()
        perAppModes.removeAll()
        selectedApps.removeAll()
        reloadAction()
    }
}

// App Row Component
struct AppRow: View {
    let bundleId: String
    let enabled: Bool
    let isSelected: Bool
    let onToggle: () -> Void
    let onSelect: () -> Void
    
    @State private var appIcon: NSImage?
    @State private var appName: String = ""
    
    var body: some View {
        HStack(spacing: 12) {
            // Selection checkbox
            Button {
                onSelect()
            } label: {
                Image(systemName: isSelected ? "checkmark.circle.fill" : "circle")
                    .foregroundColor(isSelected ? .accentColor : .secondary)
            }
            .buttonStyle(.plain)
            
            // App Icon
            if let icon = appIcon {
                Image(nsImage: icon)
                    .resizable()
                    .frame(width: 32, height: 32)
            } else {
                Image(systemName: "app")
                    .font(.system(size: 24))
                    .foregroundColor(.secondary)
                    .frame(width: 32, height: 32)
            }
            
            // App Info
            VStack(alignment: .leading, spacing: 2) {
                Text(appName.isEmpty ? bundleId : appName)
                    .font(.system(size: 13, weight: .medium))
                
                if !appName.isEmpty {
                    Text(bundleId)
                        .font(.system(size: 11))
                        .foregroundColor(.secondary)
                }
            }
            
            Spacer()
            
            // Status Badge
            HStack(spacing: 6) {
                Circle()
                    .fill(enabled ? Color.green : Color.gray)
                    .frame(width: 8, height: 8)
                Text(enabled ? "Enabled" : "Disabled")
                    .font(.system(size: 11))
                    .foregroundColor(.secondary)
            }
            .padding(.horizontal, 8)
            .padding(.vertical, 4)
            .background(
                Capsule()
                    .fill(Color(nsColor: .controlBackgroundColor))
            )
            
            // Toggle
            Toggle("", isOn: Binding(
                get: { enabled },
                set: { _ in onToggle() }
            ))
            .labelsHidden()
            .toggleStyle(.switch)
        }
        .padding(12)
        .background(
            RoundedRectangle(cornerRadius: 8)
                .fill(isSelected ? Color.accentColor.opacity(0.1) : Color(nsColor: .controlBackgroundColor).opacity(0.5))
        )
        .contentShape(Rectangle())
        .onTapGesture {
            onSelect()
        }
        .onAppear {
            loadAppInfo()
        }
    }
    
    private func loadAppInfo() {
        if let url = NSWorkspace.shared.urlForApplication(withBundleIdentifier: bundleId) {
            appIcon = NSWorkspace.shared.icon(forFile: url.path)
            
            if let bundle = Bundle(url: url),
               let name = bundle.object(forInfoDictionaryKey: "CFBundleName") as? String {
                appName = name
            } else {
                appName = url.deletingPathExtension().lastPathComponent
            }
        }
    }
}

#Preview {
    PerAppSettingsView(
        smartModeEnabled: .constant(true),
        perAppModes: .constant([
            "com.apple.Safari": true,
            "com.microsoft.VSCode": false,
            "com.google.Chrome": true
        ]),
        showClearConfirmation: .constant(false),
        reloadAction: { }
    )
    .frame(width: 700, height: 600)
}
