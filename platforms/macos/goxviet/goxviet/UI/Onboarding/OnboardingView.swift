//
//  OnboardingView.swift
//  GoxViet
//
//  First-launch setup wizard: welcome → accessibility permission → input method.
//

import SwiftUI
import ApplicationServices

struct OnboardingView: View {
    @State private var step: Int = 0
    @State private var accessibilityGranted: Bool = false
    var onComplete: () -> Void

    var body: some View {
        VStack(spacing: 24) {
            switch step {
            case 0: welcomeStep
            case 1: permissionsStep
            case 2: inputMethodStep
            default: doneStep
            }
        }
        .padding(32)
        .frame(width: 480, height: 360)
    }

    private var welcomeStep: some View {
        VStack(spacing: 16) {
            Image(nsImage: NSApp.applicationIconImage)
                .resizable()
                .frame(width: 80, height: 80)
            Text("Chào mừng đến với Gõ Việt")
                .font(.title2.bold())
            Text("Bộ gõ tiếng Việt hiệu năng cao cho macOS.")
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
            Button("Bắt đầu") { step = 1 }
                .buttonStyle(.borderedProminent)
                .controlSize(.large)
        }
    }

    private var permissionsStep: some View {
        VStack(spacing: 16) {
            Image(systemName: "keyboard")
                .font(.system(size: 48))
                .foregroundColor(.blue)
            Text("Cấp quyền Accessibility")
                .font(.title3.bold())
            Text("Gõ Việt cần quyền Accessibility để bắt phím và chèn văn bản.")
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
            if accessibilityGranted {
                Label("Đã cấp quyền", systemImage: "checkmark.circle.fill")
                    .foregroundColor(.green)
                Button("Tiếp tục") { step = 2 }
                    .buttonStyle(.borderedProminent)
            } else {
                Button("Mở System Settings") {
                    if let url = URL(string: "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility") {
                        NSWorkspace.shared.open(url)
                    }
                }
                .buttonStyle(.borderedProminent)
                Button("Kiểm tra lại") {
                    accessibilityGranted = AXIsProcessTrusted()
                }
                .buttonStyle(.bordered)
            }
        }
        .onAppear { accessibilityGranted = AXIsProcessTrusted() }
    }

    private var inputMethodStep: some View {
        VStack(spacing: 16) {
            Image(systemName: "character.cursor.ibeam")
                .font(.system(size: 48))
                .foregroundColor(.blue)
            Text("Chọn kiểu gõ")
                .font(.title3.bold())
            HStack(spacing: 16) {
                methodButton(title: "Telex", subtitle: "aa→â, aw→ă, s→sắc", method: 0)
                methodButton(title: "VNI", subtitle: "6→â, 8→ă, 1→sắc", method: 1)
            }
            Button("Tiếp tục") { step = 3 }
                .buttonStyle(.bordered)
        }
    }

    private var doneStep: some View {
        VStack(spacing: 16) {
            Image(systemName: "checkmark.seal.fill")
                .font(.system(size: 48))
                .foregroundColor(.green)
            Text("Sẵn sàng!")
                .font(.title2.bold())
            Text("Gõ Việt đã được cấu hình. Bắt đầu gõ tiếng Việt ngay bây giờ.")
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
            Button("Hoàn tất") {
                SettingsManager.shared.hasCompletedOnboarding = true
                onComplete()
            }
            .buttonStyle(.borderedProminent)
            .controlSize(.large)
        }
    }

    @ViewBuilder
    private func methodButton(title: String, subtitle: String, method: Int) -> some View {
        Button {
            SettingsManager.shared.inputMethod = method
        } label: {
            VStack(alignment: .leading, spacing: 4) {
                Text(title).font(.headline)
                Text(subtitle).font(.caption).foregroundColor(.secondary)
            }
            .frame(width: 160, alignment: .leading)
            .padding(12)
            .overlay(
                RoundedRectangle(cornerRadius: 8)
                    .stroke(SettingsManager.shared.inputMethod == method ? Color.accentColor : Color.secondary.opacity(0.3))
            )
        }
        .buttonStyle(.plain)
    }
}
