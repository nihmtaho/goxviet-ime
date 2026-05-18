import AppKit

final class SoundFeedbackService {
    static let shared = SoundFeedbackService()

    private let enableSound: NSSound?
    private let disableSound: NSSound?

    private init() {
        // Cache at init to avoid first-play latency
        enableSound = NSSound(named: "Tink")
        disableSound = NSSound(named: "Pop")
    }

    func playEnable() {
        guard SettingsManager.shared.soundEnabled else { return }
        enableSound?.play()
    }

    func playDisable() {
        guard SettingsManager.shared.soundEnabled else { return }
        disableSound?.play()
    }
}
