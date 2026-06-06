//
//  DebugLogView.swift
//  GoxViet
//
//  Displays live-refreshing content of debug.log with a Clear button.
//

import SwiftUI

struct DebugLogView: View {
    @State private var logContent: String = ""
    @State private var refreshTimer: Timer?

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Text("Debug Log")
                    .font(.headline)
                Spacer()
                Button("Clear") {
                    DebugLogger.shared.clear()
                    logContent = ""
                }
                .buttonStyle(.bordered)
            }

            ScrollViewReader { proxy in
                ScrollView {
                    Text(logContent.isEmpty ? "(no log entries)" : logContent)
                        .font(.system(.caption, design: .monospaced))
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .padding(8)
                        .id("bottom")
                }
                .background(Color(NSColor.textBackgroundColor))
                .cornerRadius(6)
                .frame(minHeight: 200, maxHeight: 400)
                .onChange(of: logContent, perform: { _ in
                    proxy.scrollTo("bottom", anchor: .bottom)
                })
            }
        }
        .onAppear {
            logContent = DebugLogger.shared.readLog()
            refreshTimer = Timer.scheduledTimer(withTimeInterval: 1.0, repeats: true) { _ in
                logContent = DebugLogger.shared.readLog()
            }
        }
        .onDisappear {
            refreshTimer?.invalidate()
            refreshTimer = nil
        }
    }
}
