import Cocoa
import SwiftUI

/// Bridge to capture and expose `openSettingsAction` for non-SwiftUI callers (e.g., AppDelegate menu item).
final class SettingsActionBridge {
    static let shared = SettingsActionBridge()

    private var openAction: OpenSettingsAction?
    private var hostingController: NSHostingController<SettingsActionInstaller>?
    private var hiddenWindow: NSWindow?

    private init() {}

    /// Install a hidden SwiftUI host to capture `openSettingsAction` from the environment.
    func installIfNeeded() {
        guard hostingController == nil else { return }

        let installer = SettingsActionInstaller()
        let controller = NSHostingController(rootView: installer)
        controller.view.isHidden = true
        controller.view.frame = .zero
        controller.view.alphaValue = 0

        let window = NSWindow(contentViewController: controller)
        window.setIsVisible(false)
        window.level = .statusBar
        window.isOpaque = false
        window.hasShadow = false
        window.backgroundColor = .clear
        window.titleVisibility = .hidden
        window.titlebarAppearsTransparent = true
        window.isReleasedWhenClosed = false

        hostingController = controller
        hiddenWindow = window
    }

    /// Register the action captured from SwiftUI environment.
    func register(action: OpenSettingsAction) {
        openAction = action
    }

    /// Invoke settings action if available. Returns true if handled.
    func open() -> Bool {
        guard let action = openAction else { return false }
        action()
        return true
    }
}

/// Invisible installer view to capture `openSettingsAction`.
struct SettingsActionInstaller: View {
    @Environment(\.openSettings) private var openSettingsAction

    var body: some View {
        Color.clear
            .onAppear {
                SettingsActionBridge.shared.register(action: openSettingsAction)
            }
    }
}
