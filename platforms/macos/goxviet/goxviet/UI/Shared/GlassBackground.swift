//
//  GlassBackground.swift
//  GoxViet
//
//  Reusable glass/translucent background effect for modern macOS UI
//

import SwiftUI

struct GlassBackground: View {
    var opacity: Double = 0.95
    var blur: CGFloat = 20

    var body: some View {
        if #available(macOS 26, *) {
            Color.clear.glassEffect(in: .rect(cornerRadius: 12))
        } else {
            ZStack {
                LinearGradient(
                    gradient: Gradient(colors: [
                        Color(NSColor.windowBackgroundColor).opacity(opacity),
                        Color(NSColor.windowBackgroundColor).opacity(opacity * 0.9)
                    ]),
                    startPoint: .topLeading,
                    endPoint: .bottomTrailing
                )
                Rectangle()
                    .fill(Color.white.opacity(0.02))
                    .blendMode(.overlay)
            }
            .background(.ultraThinMaterial)
            .cornerRadius(12)
        }
    }
}

struct GlassCard: View {
    var body: some View {
        if #available(macOS 26, *) {
            Color.clear
                .glassEffect(in: .rect(cornerRadius: 12))
                .shadow(color: Color.black.opacity(0.1), radius: 10, x: 0, y: 5)
        } else {
            RoundedRectangle(cornerRadius: 12)
                .fill(.ultraThinMaterial)
                .shadow(color: Color.black.opacity(0.1), radius: 10, x: 0, y: 5)
        }
    }
}

// MARK: - Adaptive Button Style Helpers

extension View {
    /// Applies `.glass` button style on macOS 26+, falls back to `.bordered`.
    @ViewBuilder func adaptiveGlassButton() -> some View {
        if #available(macOS 26, *) {
            self.buttonStyle(.glass)
        } else {
            self.buttonStyle(.bordered)
        }
    }

    /// Applies `.glassProminent` button style on macOS 26+, falls back to `.borderedProminent`.
    @ViewBuilder func adaptiveGlassProminentButton() -> some View {
        if #available(macOS 26, *) {
            self.buttonStyle(.glassProminent)
        } else {
            self.buttonStyle(.borderedProminent)
        }
    }
}

#Preview {
    VStack {
        Text("Glass Background")
            .font(.title)
            .padding()
            .background(GlassBackground())
        
        Text("Glass Card")
            .font(.title)
            .padding()
            .background(GlassCard())
    }
    .frame(width: 400, height: 300)
    .background(Color.gray.opacity(0.3))
}
