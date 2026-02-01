//
//  MetricsChartView.swift
//  GoxViet
//
//  Visualize engine metrics with charts
//

import SwiftUI
import Charts

struct EngineMetrics {
    let totalKeystrokes: UInt64
    let backspaceCount: UInt64
    let avgBufferLength: Double
}

struct MetricDataPoint: Identifiable {
    let id = UUID()
    let label: String
    let value: Double
}

struct MetricsChartView: View {
    let metrics: EngineMetrics
    
    var chartData: [MetricDataPoint] {
        [
            MetricDataPoint(label: "Keystrokes", value: Double(metrics.totalKeystrokes)),
            MetricDataPoint(label: "Backspaces", value: Double(metrics.backspaceCount)),
            MetricDataPoint(label: "Avg Buffer × 100", value: metrics.avgBufferLength * 100)
        ]
    }
    
    var backspaceRatio: Double {
        guard metrics.totalKeystrokes > 0 else { return 0 }
        return Double(metrics.backspaceCount) / Double(metrics.totalKeystrokes) * 100
    }
    
    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            // Summary Cards
            HStack(spacing: 12) {
                MetricCard(
                    title: "Total Keystrokes",
                    value: formatNumber(metrics.totalKeystrokes),
                    icon: "keyboard",
                    color: .blue
                )
                
                MetricCard(
                    title: "Backspaces",
                    value: formatNumber(metrics.backspaceCount),
                    icon: "delete.backward",
                    color: .orange
                )
                
                MetricCard(
                    title: "Backspace Ratio",
                    value: String(format: "%.1f%%", backspaceRatio),
                    icon: "percent",
                    color: backspaceRatio > 20 ? .red : .green
                )
                
                MetricCard(
                    title: "Avg Buffer Length",
                    value: String(format: "%.2f", metrics.avgBufferLength),
                    icon: "text.alignleft",
                    color: .purple
                )
            }
            
            Divider()
            
            // Bar Chart
            if #available(macOS 13.0, *) {
                VStack(alignment: .leading, spacing: 8) {
                    Text("Metrics Comparison")
                        .font(.system(size: 14, weight: .semibold))
                    
                    Chart(chartData) { item in
                        BarMark(
                            x: .value("Metric", item.label),
                            y: .value("Value", item.value)
                        )
                        .foregroundStyle(Color.accentColor.gradient)
                        .cornerRadius(4)
                    }
                    .frame(height: 200)
                    .chartYAxis {
                        AxisMarks(position: .leading)
                    }
                }
                .padding(12)
                .background(
                    RoundedRectangle(cornerRadius: 8)
                        .fill(Color(nsColor: .controlBackgroundColor).opacity(0.5))
                )
            }
        }
    }
    
    private func formatNumber(_ num: UInt64) -> String {
        if num >= 1_000_000 {
            return String(format: "%.1fM", Double(num) / 1_000_000)
        } else if num >= 1_000 {
            return String(format: "%.1fK", Double(num) / 1_000)
        } else {
            return "\(num)"
        }
    }
}

struct MetricCard: View {
    let title: String
    let value: String
    let icon: String
    let color: Color
    
    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Image(systemName: icon)
                    .foregroundColor(color)
                    .font(.system(size: 16))
                Spacer()
            }
            
            Text(value)
                .font(.system(size: 24, weight: .bold))
                .foregroundColor(.primary)
            
            Text(title)
                .font(.system(size: 11))
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(12)
        .background(
            RoundedRectangle(cornerRadius: 8)
                .fill(Color(nsColor: .controlBackgroundColor).opacity(0.5))
        )
    }
}

#Preview {
    MetricsChartView(
        metrics: EngineMetrics(
            totalKeystrokes: 12345,
            backspaceCount: 2345,
            avgBufferLength: 4.25
        )
    )
    .frame(width: 600)
    .padding()
}
