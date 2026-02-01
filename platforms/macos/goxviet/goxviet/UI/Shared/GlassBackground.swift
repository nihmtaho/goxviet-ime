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
        ZStack {
            // Base color with slight gradient
            LinearGradient(
                gradient: Gradient(colors: [
                    Color(nsColor: .windowBackgroundColor).opacity(opacity),
                    Color(nsColor: .windowBackgroundColor).opacity(opacity * 0.9)
                ]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            
            // Subtle texture overlay
            Rectangle()
                .fill(Color.white.opacity(0.02))
                .blendMode(.overlay)
        }
        .background(.ultraThinMaterial)
        .cornerRadius(12)
    }
}

struct GlassCard: View {
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 12)
                .fill(.ultraThinMaterial)
                .shadow(color: Color.black.opacity(0.1), radius: 10, x: 0, y: 5)
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
