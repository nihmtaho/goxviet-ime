//
//  MemoryProfilingView.swift
//  GoxViet
//
//  Memory profiling visualization with real-time charts
//

import SwiftUI
import Charts
import UniformTypeIdentifiers

struct MemoryProfilingView: View {
    @StateObject private var profiler = MemoryProfiler.shared
    @State private var showExportAlert = false
    @State private var exportSuccess = false
    
    var body: some View {
        VStack(spacing: 20) {
            // Header
            headerView
            
            // Current Stats Cards
            currentStatsView
            
            // Real-time Chart
            if profiler.isMonitoring {
                memoryChartView
            }
            
            // Controls
            controlsView
            
            Spacer()
        }
        .padding(24)
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
    
    // MARK: - Header
    
    private var headerView: some View {
        VStack(alignment: .leading, spacing: 4) {
            HStack {
                Image(systemName: "cpu")
                    .font(.system(size: 24))
                    .foregroundColor(.accentColor)
                Text("Memory Profiling")
                    .font(.system(size: 20, weight: .semibold))
                
                Spacer()
                
                // Status indicator
                HStack(spacing: 6) {
                    Circle()
                        .fill(profiler.isMonitoring ? Color.green : Color.secondary)
                        .frame(width: 8, height: 8)
                    Text(profiler.isMonitoring ? "Monitoring" : "Idle")
                        .font(.system(size: 11, weight: .medium))
                        .foregroundColor(.secondary)
                }
                .padding(.horizontal, 10)
                .padding(.vertical, 4)
                .background(
                    Capsule()
                        .fill(Color(nsColor: .controlBackgroundColor))
                )
            }
            
            Text("Real-time memory usage tracking and analysis")
                .font(.system(size: 13))
                .foregroundColor(.secondary)
        }
    }
    
    // MARK: - Current Stats
    
    private var currentStatsView: some View {
        HStack(spacing: 16) {
            StatCard(
                title: "Used Memory",
                value: profiler.currentStats.formattedUsedMemory,
                subtitle: profiler.currentStats.formattedUsagePercentage,
                icon: "memorychip",
                color: .blue
            )
            
            StatCard(
                title: "Peak Memory",
                value: profiler.currentStats.formattedPeakMemory,
                subtitle: "Highest recorded",
                icon: "arrow.up.circle",
                color: .orange
            )
            
            StatCard(
                title: "Available",
                value: profiler.currentStats.formattedAvailableMemory,
                subtitle: "Free memory",
                icon: "externaldrive",
                color: .green
            )
            
            StatCard(
                title: "Objects",
                value: "\(profiler.currentStats.allocatedObjectsCount)",
                subtitle: "Allocated",
                icon: "cube.box",
                color: .purple
            )
        }
        .id(profiler.currentStats.timestamp) // Force refresh on each update
    }
    
    // MARK: - Chart
    
    private var memoryChartView: some View {
        GroupBox {
            VStack(alignment: .leading, spacing: 12) {
                Text("Memory Usage Over Time")
                    .font(.system(size: 14, weight: .semibold))
                
                if profiler.history.isEmpty {
                    Text("Collecting data...")
                        .font(.system(size: 12))
                        .foregroundColor(.secondary)
                        .frame(height: 200)
                        .frame(maxWidth: .infinity)
                } else {
                    Chart(profiler.history, id: \.timestamp) { stats in
                        LineMark(
                            x: .value("Time", stats.timestamp),
                            y: .value("Memory", stats.usedMemoryMB)
                        )
                        .foregroundStyle(.blue)
                        .interpolationMethod(.catmullRom)
                        
                        AreaMark(
                            x: .value("Time", stats.timestamp),
                            y: .value("Memory", stats.usedMemoryMB)
                        )
                        .foregroundStyle(
                            LinearGradient(
                                colors: [.blue.opacity(0.3), .blue.opacity(0.05)],
                                startPoint: .top,
                                endPoint: .bottom
                            )
                        )
                    }
                    .chartYAxis {
                        AxisMarks(position: .leading) { value in
                            AxisValueLabel {
                                if let mb = value.as(Double.self) {
                                    Text("\(Int(mb)) MB")
                                        .font(.system(size: 10))
                                }
                            }
                        }
                    }
                    .chartXAxis {
                        AxisMarks { value in
                            AxisValueLabel {
                                if let date = value.as(Date.self) {
                                    Text(date, format: .dateTime.second())
                                        .font(.system(size: 10))
                                }
                            }
                        }
                    }
                    .frame(height: 200)
                    .animation(.easeInOut(duration: 0.3), value: profiler.history.count)
                }
            }
            .padding(12)
        }
        .id(profiler.history.count) // Force chart refresh
    }
    
    // MARK: - Controls
    
    private var controlsView: some View {
        GroupBox {
            HStack(spacing: 12) {
                // Start/Stop
                Button {
                    if profiler.isMonitoring {
                        profiler.stopMonitoring()
                    } else {
                        profiler.startMonitoring()
                    }
                } label: {
                    Label(
                        profiler.isMonitoring ? "Stop Monitoring" : "Start Monitoring",
                        systemImage: profiler.isMonitoring ? "stop.circle" : "play.circle"
                    )
                }
                .buttonStyle(.borderedProminent)
                .tint(profiler.isMonitoring ? .red : .green)
                
                // Capture Snapshot
                Button {
                    _ = profiler.captureSnapshot()
                } label: {
                    Label("Snapshot", systemImage: "camera")
                }
                .buttonStyle(.bordered)
                
                // Reset Peak
                Button {
                    profiler.resetPeak()
                } label: {
                    Label("Reset Peak", systemImage: "arrow.counterclockwise")
                }
                .buttonStyle(.bordered)
                
                Spacer()
                
                // Export
                Button {
                    exportHistory()
                } label: {
                    Label("Export", systemImage: "square.and.arrow.up")
                }
                .buttonStyle(.bordered)
                .disabled(profiler.history.isEmpty)
                .alert("Export Successful", isPresented: $showExportAlert) {
                    Button("OK", role: .cancel) { }
                } message: {
                    Text(exportSuccess ? "Memory profile exported successfully" : "Failed to export memory profile")
                }
            }
            .padding(12)
        }
    }
    
    // MARK: - Actions
    
    private func exportHistory() {
        guard let data = profiler.exportHistory() else {
            exportSuccess = false
            showExportAlert = true
            return
        }
        
        let savePanel = NSSavePanel()
        savePanel.allowedContentTypes = [.json]
        savePanel.nameFieldStringValue = "goxviet_memory_profile_\(Date().timeIntervalSince1970).json"
        
        savePanel.begin { response in
            guard response == .OK, let url = savePanel.url else { return }
            
            do {
                try data.write(to: url)
                exportSuccess = true
                Log.info("Memory profile exported to: \(url.path)")
            } catch {
                exportSuccess = false
                Log.error("Failed to export memory profile: \(error)")
            }
            
            showExportAlert = true
        }
    }
}

// MARK: - Stat Card Component

struct StatCard: View {
    let title: String
    let value: String
    let subtitle: String
    let icon: String
    let color: Color
    
    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Image(systemName: icon)
                    .font(.system(size: 20))
                    .foregroundColor(color)
                Spacer()
            }
            
            VStack(alignment: .leading, spacing: 2) {
                Text(value)
                    .font(.system(size: 22, weight: .bold, design: .rounded))
                    .foregroundColor(.primary)
                
                Text(title)
                    .font(.system(size: 11, weight: .medium))
                    .foregroundColor(.secondary)
            }
            
            Text(subtitle)
                .font(.system(size: 10))
                .foregroundColor(.secondary)
        }
        .padding(12)
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(
            RoundedRectangle(cornerRadius: 8)
                .fill(color.opacity(0.08))
        )
        .overlay(
            RoundedRectangle(cornerRadius: 8)
                .stroke(color.opacity(0.2), lineWidth: 1)
        )
    }
}

#Preview {
    MemoryProfilingView()
        .frame(width: 900, height: 700)
}
