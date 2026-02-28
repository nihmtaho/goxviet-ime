import Cocoa
import SwiftUI

/// Bridge to capture and expose `openSettingsAction` for non-SwiftUI callers (e.g., AppDelegate menu item).
final class SettingsActionBridge {
    static let shared = SettingsActionBridge()

    @available(macOS 14.0, *)
    private var openAction: OpenSettingsAction? {
        get { _openAction as? OpenSettingsAction }
        set { _openAction = newValue }
    }
    private var _openAction: Any?
    private var hostingController: NSHostingController<AnyView>?
    private var hiddenWindow: NSWindow?

    private init() {}

    /// Install a hidden SwiftUI host to capture `openSettingsAction` from the environment.
    func installIfNeeded() {
        guard #available(macOS 14.0, *) else { return }
        guard hostingController == nil else { return }

        let installer = AnyView(SettingsActionInstaller())
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
    @available(macOS 14.0, *)
    func register(action: OpenSettingsAction) {
        openAction = action
    }

    /// Invoke settings action if available. Returns true if handled.
    func open() -> Bool {
        guard #available(macOS 14.0, *) else { return false }
        guard let action = openAction else { return false }
        action()
        return true
    }
}

/// Invisible installer view to capture `openSettingsAction`.
@available(macOS 14.0, *)
struct SettingsActionInstaller: View {
    @Environment(\.openSettings) private var openSettingsAction

    var body: some View {
        Color.clear
            .onAppear {
                SettingsActionBridge.shared.register(action: openSettingsAction)
            }
    }
}
