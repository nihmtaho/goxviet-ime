//
//  SmartModeIndicator.swift
//  GoxViet
//
//  Menu bar indicator for Smart Mode status with quick app info
//

import SwiftUI

/// Menu bar content showing Smart Mode status and current app info
struct SmartModeIndicator: View {
    
    @ObservedObject private var appState = AppState.shared
    @State private var currentApp: CurrentAppInfo?
    @State private var recentApps: [String] = []
    @State private var metrics: PerAppModeManagerEnhanced.PerformanceMetrics?
    
    struct CurrentAppInfo {
        let bundleId: String
        let name: String
        let icon: NSImage?
        let isVietnamese: Bool
    }
    
    var body: some View {
        VStack(spacing: 0) {
            // Header
            headerSection
            
            Divider()
            
            // Current App
            if let app = currentApp {
                currentAppSection(app)
                Divider()
            }
            
            // Quick Toggle
            quickToggleSection
            
            Divider()
            
            // Recent Apps
            if !recentApps.isEmpty {
                recentAppsSection
                Divider()
            }
            
            // Performance Metrics (Debug)
            if appState.isDebugMode, let metrics = metrics {
                metricsSection(metrics)
                Divider()
            }
            
            // Actions
            actionsSection
        }
        .frame(width: 280)
        .onAppear {
            loadCurrentApp()
            loadRecentApps()
            loadMetrics()
        }
    }
    
    // MARK: - Sections
    
    private var headerSection: some View {
        HStack {
            Image(systemName: "brain.head.profile")
                .font(.title2)
                .foregroundColor(appState.isSmartModeEnabled ? .green : .gray)
            
            VStack(alignment: .leading, spacing: 2) {
                Text("Smart Mode")
                    .font(.headline)
                
                Text(appState.isSmartModeEnabled ? "Active" : "Inactive")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            
            Spacer()
            
            Toggle("", isOn: Binding(
                get: { appState.isSmartModeEnabled },
                set: { enabled in
                    appState.isSmartModeEnabled = enabled
                    if enabled {
                        PerAppModeManagerEnhanced.shared.refresh()
                    }
                }
            ))
            .toggleStyle(.switch)
            .labelsHidden()
        }
        .padding()
    }
    
    private func currentAppSection(_ app: CurrentAppInfo) -> some View {
        HStack(spacing: 12) {
            if let icon = app.icon {
                Image(nsImage: icon)
                    .resizable()
                    .frame(width: 32, height: 32)
            } else {
                Image(systemName: "app.dashed")
                    .font(.largeTitle)
                    .foregroundColor(.secondary)
            }
            
            VStack(alignment: .leading, spacing: 4) {
                Text(app.name)
                    .font(.subheadline)
                    .fontWeight(.medium)
                    .lineLimit(1)
                
                HStack(spacing: 4) {
                    Circle()
                        .fill(app.isVietnamese ? Color.green : Color.orange)
                        .frame(width: 6, height: 6)
                    
                    Text(app.isVietnamese ? "Vietnamese" : "English")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
            
            Spacer()
        }
        .padding()
        .contentShape(Rectangle())
    }
    
    private var quickToggleSection: some View {
        VStack(spacing: 8) {
            if let app = currentApp {
                Button(action: {
                    let newState = !app.isVietnamese
                    AppState.shared.isEnabled = newState
                    PerAppModeManagerEnhanced.shared.setStateForCurrentApp(newState)
                    loadCurrentApp()
                }) {
                    HStack {
                        Image(systemName: app.isVietnamese ? "globe.americas" : "character.textbox")
                            .font(.body)
                        
                        Text("Switch to \(app.isVietnamese ? "English" : "Vietnamese")")
                            .font(.subheadline)
                        
                        Spacer()
                        
                        Text(app.isVietnamese ? "⌘E" : "⌘V")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                    .contentShape(Rectangle())
                }
                .buttonStyle(.plain)
                .padding(.horizontal)
            }
        }
        .padding(.vertical, 8)
    }
    
    private var recentAppsSection: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Recent Apps")
                .font(.caption)
                .foregroundColor(.secondary)
                .padding(.horizontal)
            
            ScrollView {
                VStack(spacing: 4) {
                    ForEach(recentApps.prefix(5), id: \.self) { bundleId in
                        recentAppRow(bundleId)
                    }
                }
            }
            .frame(maxHeight: 150)
        }
        .padding(.vertical, 8)
    }
    
    private func recentAppRow(_ bundleId: String) -> some View {
        HStack(spacing: 8) {
            if let icon = PerAppModeManagerEnhanced.shared.getAppIcon(bundleId) {
                Image(nsImage: icon)
                    .resizable()
                    .frame(width: 20, height: 20)
            }
            
            Text(PerAppModeManagerEnhanced.shared.getAppName(bundleId))
                .font(.caption)
                .lineLimit(1)
            
            Spacer()
            
            let isEnabled = AppState.shared.getPerAppMode(bundleId: bundleId)
            Circle()
                .fill(isEnabled ? Color.green : Color.orange)
                .frame(width: 6, height: 6)
        }
        .padding(.horizontal)
        .padding(.vertical, 4)
        .contentShape(Rectangle())
    }
    
    private func metricsSection(_ metrics: PerAppModeManagerEnhanced.PerformanceMetrics) -> some View {
        VStack(alignment: .leading, spacing: 4) {
            Text("Performance")
                .font(.caption)
                .foregroundColor(.secondary)
            
            HStack {
                metricItem("Switches", "\(metrics.totalSwitches)")
                metricItem("Hit Rate", String(format: "%.1f%%", metrics.cacheHitRate * 100))
                metricItem("Cached", "\(metrics.cachedAppsCount)")
            }
        }
        .padding()
    }
    
    private func metricItem(_ label: String, _ value: String) -> some View {
        VStack(spacing: 2) {
            Text(value)
                .font(.caption)
                .fontWeight(.semibold)
            Text(label)
                .font(.caption2)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity)
    }
    
    private var actionsSection: some View {
        VStack(spacing: 8) {
            Button("Refresh") {
                PerAppModeManagerEnhanced.shared.refresh()
                loadCurrentApp()
                loadRecentApps()
                loadMetrics()
            }
            .buttonStyle(.plain)
            
            Button("Open Settings...") {
                NotificationCenter.default.post(name: .openSettings, object: nil)
            }
            .buttonStyle(.plain)
        }
        .padding()
    }
    
    // MARK: - Data Loading
    
    private func loadCurrentApp() {
        guard let bundleId = PerAppModeManagerEnhanced.shared.getCurrentBundleId() else {
            currentApp = nil
            return
        }
        
        let name = PerAppModeManagerEnhanced.shared.getAppName(bundleId)
        let icon = PerAppModeManagerEnhanced.shared.getAppIcon(bundleId)
        let isVietnamese = AppState.shared.getPerAppMode(bundleId: bundleId)
        
        currentApp = CurrentAppInfo(
            bundleId: bundleId,
            name: name,
            icon: icon,
            isVietnamese: isVietnamese
        )
    }
    
    private func loadRecentApps() {
        recentApps = PerAppModeManagerEnhanced.shared.getRecentlyUsedApps()
    }
    
    private func loadMetrics() {
        metrics = PerAppModeManagerEnhanced.shared.getPerformanceMetrics()
    }
}

// MARK: - Menu Bar Status Item

class SmartModeMenuBarItem {
    
    static let shared = SmartModeMenuBarItem()
    
    private var statusItem: NSStatusItem?
    private var popover: NSPopover?
    
    private init() {}
    
    func setup() {
        // Create status item
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        
        guard let button = statusItem?.button else { return }
        
        // Set icon
        updateIcon()
        
        // Set action
        button.action = #selector(togglePopover)
        button.target = self
        
        // Create popover
        popover = NSPopover()
        popover?.contentViewController = NSHostingController(
            rootView: SmartModeIndicator()
        )
        popover?.behavior = .transient
        
        // Observe state changes
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(stateChanged),
            name: .updateStateChanged,
            object: nil
        )
        
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(currentAppChanged),
            name: .currentAppChanged,
            object: nil
        )
        
        Log.info("SmartModeMenuBarItem setup complete")
    }
    
    func teardown() {
        NSStatusBar.system.removeStatusItem(statusItem!)
        statusItem = nil
        popover = nil
        
        NotificationCenter.default.removeObserver(self)
        
        Log.info("SmartModeMenuBarItem removed")
    }
    
    @objc private func togglePopover() {
        guard let button = statusItem?.button else { return }
        
        if let popover = popover, popover.isShown {
            popover.performClose(nil)
        } else {
            popover?.show(relativeTo: button.bounds, of: button, preferredEdge: .minY)
        }
    }
    
    @objc private func stateChanged() {
        DispatchQueue.main.async {
            self.updateIcon()
        }
    }
    
    @objc private func currentAppChanged() {
        DispatchQueue.main.async {
            self.updateIcon()
        }
    }
    
    private func updateIcon() {
        guard let button = statusItem?.button else { return }
        
        let appState = AppState.shared
        let isSmartMode = appState.isSmartModeEnabled
        let isEnabled = appState.isEnabled
        
        // Choose icon based on state
        let iconName: String
        if isSmartMode {
            iconName = isEnabled ? "brain.head.profile.fill" : "brain.head.profile"
        } else {
            iconName = isEnabled ? "character.textbox" : "globe.americas"
        }
        
        // Update icon
        if let icon = NSImage(systemSymbolName: iconName, accessibilityDescription: nil) {
            icon.isTemplate = true
            button.image = icon
        }
        
        // Update tooltip
        var tooltip = isEnabled ? "Vietnamese" : "English"
        if isSmartMode, let appName = PerAppModeManagerEnhanced.shared.getCurrentAppName() {
            tooltip += " • \(appName)"
        }
        button.toolTip = tooltip
    }
}

// MARK: - Notification Names

extension Notification.Name {
    static let openSettings = Notification.Name("com.goxviet.openSettings")
}

// MARK: - Preview

struct SmartModeIndicator_Previews: PreviewProvider {
    static var previews: some View {
        SmartModeIndicator()
            .frame(width: 280)
    }
}
