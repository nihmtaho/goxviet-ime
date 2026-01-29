//
//  SettingRow.swift
//  GoxViet
//
//  Reusable setting row component with label, description, and content
//

import SwiftUI

struct SettingRow<Content: View>: View {
    let title: String
    let description: String?
    let systemImage: String?
    @ViewBuilder let content: () -> Content
    
    init(
        title: String,
        description: String? = nil,
        systemImage: String? = nil,
        @ViewBuilder content: @escaping () -> Content
    ) {
        self.title = title
        self.description = description
        self.systemImage = systemImage
        self.content = content
    }
    
    var body: some View {
        HStack(alignment: .center, spacing: 16) {
            // Left side: Icon + Label
            HStack(alignment: .center, spacing: 12) {
                if let icon = systemImage {
                    Image(systemName: icon)
                        .font(.system(size: 16))
                        .foregroundColor(.accentColor)
                        .frame(width: 24, height: 24)
                }
                
                VStack(alignment: .leading, spacing: 2) {
                    Text(title)
                        .font(.system(size: 13, weight: .medium))
                    
                    if let desc = description {
                        Text(desc)
                            .font(.system(size: 11))
                            .foregroundColor(.secondary)
                    }
                }
            }
            
            Spacer()
            
            // Right side: Control
            content()
        }
        .padding(.vertical, 8)
        .padding(.horizontal, 12)
        .background(
            RoundedRectangle(cornerRadius: 8)
                .fill(Color(nsColor: .controlBackgroundColor).opacity(0.5))
        )
        .contentShape(Rectangle())
    }
}

struct ToggleRow: View {
    let title: String
    let description: String?
    let systemImage: String?
    @Binding var isOn: Bool
    
    init(
        title: String,
        description: String? = nil,
        systemImage: String? = nil,
        isOn: Binding<Bool>
    ) {
        self.title = title
        self.description = description
        self.systemImage = systemImage
        self._isOn = isOn
    }
    
    var body: some View {
        SettingRow(title: title, description: description, systemImage: systemImage) {
            Toggle("", isOn: $isOn)
                .labelsHidden()
                .toggleStyle(.switch)
        }
    }
}

struct PickerRow<SelectionValue: Hashable>: View {
    let title: String
    let description: String?
    let systemImage: String?
    @Binding var selection: SelectionValue
    let options: [(SelectionValue, String)]
    
    init(
        title: String,
        description: String? = nil,
        systemImage: String? = nil,
        selection: Binding<SelectionValue>,
        options: [(SelectionValue, String)]
    ) {
        self.title = title
        self.description = description
        self.systemImage = systemImage
        self._selection = selection
        self.options = options
    }
    
    var body: some View {
        SettingRow(title: title, description: description, systemImage: systemImage) {
            Picker("", selection: $selection) {
                ForEach(options, id: \.0) { value, label in
                    Text(label).tag(value)
                }
            }
            .pickerStyle(.menu)
            .frame(minWidth: 150)
        }
    }
}

#Preview {
    VStack(spacing: 12) {
        ToggleRow(
            title: "Enable Feature",
            description: "Turn this feature on or off",
            systemImage: "switch.2",
            isOn: .constant(true)
        )
        
        PickerRow(
            title: "Input Method",
            description: "Choose your preferred method",
            systemImage: "keyboard",
            selection: .constant(0),
            options: [(0, "Telex"), (1, "VNI")]
        )
        
        SettingRow(
            title: "Custom Row",
            description: "With custom content",
            systemImage: "gear"
        ) {
            Button("Action") { }
        }
    }
    .padding()
    .frame(width: 500)
}
