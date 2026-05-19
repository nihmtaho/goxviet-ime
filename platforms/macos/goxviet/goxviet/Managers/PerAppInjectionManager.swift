import Foundation

final class PerAppInjectionManager {
    nonisolated(unsafe) static var shared: PerAppInjectionManager!

    private let key = "perAppInjectionProfiles"
    private var profiles: [String: PerAppInjectionProfile] = [:]

    init() {
        load()
    }

    // MARK: - Profile Access

    func profile(for bundleId: String) -> PerAppInjectionProfile {
        profiles[bundleId] ?? PerAppInjectionProfile(bundleId: bundleId)
    }

    func setProfile(_ profile: PerAppInjectionProfile) {
        profiles[profile.bundleId] = profile
        save()
    }

    func removeProfile(for bundleId: String) {
        profiles.removeValue(forKey: bundleId)
        save()
    }

    func reset(bundleId: String) {
        profiles[bundleId] = PerAppInjectionProfile(bundleId: bundleId)
        save()
    }

    var allProfiles: [PerAppInjectionProfile] {
        Array(profiles.values).sorted { $0.bundleId < $1.bundleId }
    }

    // MARK: - Persistence

    private func save() {
        guard let data = try? JSONEncoder().encode(profiles) else { return }
        UserDefaults.standard.set(data, forKey: key)
    }

    private func load() {
        guard let data = UserDefaults.standard.data(forKey: key),
              let decoded = try? JSONDecoder().decode([String: PerAppInjectionProfile].self, from: data)
        else { return }
        profiles = decoded
    }
}
