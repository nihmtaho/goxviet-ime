import SwiftUI

struct CandidateView: View {
    /// List of candidate words to display
    var candidates: [String]
    
    /// The index of the currently highlighted candidate
    var selectedIndex: Int
    
    /// Callback when a candidate is clicked
    var onSelect: ((Int) -> Void)?

    var body: some View {
        HStack(spacing: 0) {
            ForEach(Array(candidates.enumerated()), id: \.offset) { index, candidate in
                HStack(spacing: 2) {
                    // Index number (1, 2, 3...)
                    Text("\(index + 1).")
                        .font(.system(size: 11, weight: .regular))
                        .foregroundColor(index == selectedIndex ? .white.opacity(0.8) : .secondary)
                    
                    // Candidate word
                    Text(candidate)
                        .font(.system(size: 13, weight: .medium))
                        .foregroundColor(index == selectedIndex ? .white : .primary)
                }
                .padding(.horizontal, 6)
                .padding(.vertical, 3)
                .background(
                    RoundedRectangle(cornerRadius: 4)
                        .fill(index == selectedIndex ? Color.accentColor : Color.clear)
                )
                .contentShape(Rectangle()) // Make the whole area tappable
                .onTapGesture {
                    onSelect?(index)
                }
            }
        }
        .padding(4)
        .background(VisualEffectView(material: .popover, blendingMode: .behindWindow))
        .cornerRadius(6)
        .shadow(color: Color.black.opacity(0.1), radius: 2, x: 0, y: 1)
    }
}

/// Helper struct to wrap NSVisualEffectView for SwiftUI
/// This gives the window the native macOS "frosted glass" look.
struct VisualEffectView: NSViewRepresentable {
    let material: NSVisualEffectView.Material
    let blendingMode: NSVisualEffectView.BlendingMode

    func makeNSView(context: Context) -> NSVisualEffectView {
        let visualEffectView = NSVisualEffectView()
        visualEffectView.material = material
        visualEffectView.blendingMode = blendingMode
        visualEffectView.state = .active
        return visualEffectView
    }

    func updateNSView(_ visualEffectView: NSVisualEffectView, context: Context) {
        visualEffectView.material = material
        visualEffectView.blendingMode = blendingMode
    }
}

struct CandidateView_Previews: PreviewProvider {
    static var previews: some View {
        CandidateView(
            candidates: ["trường", "trưởng", "trương"],
            selectedIndex: 0
        )
        .padding()
    }
}