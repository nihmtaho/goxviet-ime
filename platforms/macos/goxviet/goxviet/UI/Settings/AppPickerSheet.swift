//
//  AppPickerSheet.swift
//  GoxViet
//
//  Sheet for picking a running app by name or bundle ID,
//  used in per-app injection configuration.
//

import SwiftUI
import AppKit

struct AppPickerSheet: View {
    @Binding var isPresented: Bool
    var onSelect: (String, String) -> Void  // (bundleId, appName)

    @State private var searchText: String = ""
    @State private var apps: [AppEntry] = []

    private struct AppEntry: Identifiable {
        let id: String  // bundleId
        let name: String
        let icon: NSImage?
    }

    private var filtered: [AppEntry] {
        if searchText.isEmpty { return apps }
        return apps.filter {
            $0.name.localizedCaseInsensitiveContains(searchText) ||
            $0.id.localizedCaseInsensitiveContains(searchText)
        }
    }

    var body: some View {
        VStack(spacing: 0) {
            HStack {
                Image(systemName: "magnifyingglass").foregroundColor(.secondary)
                TextField("Tìm app...", text: $searchText)
                    .textFieldStyle(.plain)
            }
            .padding(10)
            .background(Color(NSColor.controlBackgroundColor))

            Divider()

            List(filtered) { app in
                Button {
                    onSelect(app.id, app.name)
                    isPresented = false
                } label: {
                    HStack(spacing: 10) {
                        if let icon = app.icon {
                            Image(nsImage: icon).resizable().frame(width: 24, height: 24)
                        } else {
                            Image(systemName: "app").frame(width: 24, height: 24)
                        }
                        VStack(alignment: .leading, spacing: 2) {
                            Text(app.name).font(.body)
                            Text(app.id).font(.caption).foregroundColor(.secondary)
                        }
                    }
                }
                .buttonStyle(.plain)
            }
            .frame(minHeight: 300)

            Divider()

            HStack {
                Spacer()
                Button("Huỷ") { isPresented = false }
                    .keyboardShortcut(.cancelAction)
            }
            .padding(12)
        }
        .frame(width: 400, height: 440)
        .onAppear { loadApps() }
    }

    private func loadApps() {
        let running = NSWorkspace.shared.runningApplications
            .filter { $0.activationPolicy == .regular }
            .compactMap { app -> AppEntry? in
                guard let bundleId = app.bundleIdentifier,
                      let name = app.localizedName else { return nil }
                return AppEntry(id: bundleId, name: name, icon: app.icon)
            }
        var seen = Set<String>()
        apps = running.filter { seen.insert($0.id).inserted }
            .sorted { $0.name < $1.name }
    }
}
