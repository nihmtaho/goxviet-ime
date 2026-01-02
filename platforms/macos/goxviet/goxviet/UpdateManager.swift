//
//  UpdateManager.swift
//  GoxViet
//
//  Auto-update checker with DMG-based installer (mount, copy, restart).
//

import Foundation
import AppKit
import Combine

final class UpdateManager: NSObject, ObservableObject {
    static let shared = UpdateManager()

    @Published private(set) var latestVersion: String?
    @Published private(set) var statusMessage: String = "Not checked yet"
    @Published private(set) var isChecking: Bool = false
    @Published private(set) var updateAvailable: Bool = false
    @Published private(set) var lastChecked: Date?
    @Published private(set) var downloadURL: URL?
    @Published private(set) var isInstalling: Bool = false

    private let session: URLSession
    private var downloadTask: URLSessionDownloadTask?
    private var downloadingVersion: String?
    private var timer: Timer?
    private let defaults = UserDefaults.standard

    private let apiURL = URL(string: "https://api.github.com/repos/nihmtaho/goxviet-ime/releases/latest")!
    private let checkInterval: TimeInterval = 6 * 60 * 60  // Every 6 hours
    private let minCheckGap: TimeInterval = 20 * 60        // Skip redundant checks within 20 minutes

    private func releasePageURL(for version: String) -> URL {
        URL(string: "https://github.com/nihmtaho/goxviet-ime/releases/tag/v\(version)") ?? URL(string: "https://github.com/nihmtaho/goxviet-ime/releases")!
    }

    private override init() {
        let config = URLSessionConfiguration.ephemeral
        config.timeoutIntervalForRequest = 12
        config.timeoutIntervalForResource = 12
        session = URLSession(configuration: config)
        super.init()
    }

    // MARK: - Public API

    func start() {
        refreshSchedule(triggerImmediate: true)
    }

    func refreshSchedule(triggerImmediate: Bool = false) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            self.timer?.invalidate()
            self.timer = nil

            self.timer = Timer.scheduledTimer(withTimeInterval: self.checkInterval, repeats: true) { [weak self] _ in
                self?.maybeCheck()
            }

            if triggerImmediate {
                self.maybeCheck()
            }
        }
    }

    func checkForUpdates(userInitiated: Bool) {
        // Prevent overlapping checks
        if isChecking { return }

        DispatchQueue.main.async {
            self.isChecking = true
            self.statusMessage = "Checking for updates..."
        }

        var request = URLRequest(url: apiURL)
        request.setValue("application/vnd.github+json", forHTTPHeaderField: "Accept")
        request.setValue("GoxViet-Update-Agent", forHTTPHeaderField: "User-Agent")

        session.dataTask(with: request) { [weak self] data, response, error in
            guard let self = self else { return }

            if let error = error {
                self.finishCheckWithError("Network error: \(error.localizedDescription)", userInitiated: userInitiated)
                return
            }

            guard let httpResponse = response as? HTTPURLResponse, (200...299).contains(httpResponse.statusCode) else {
                self.finishCheckWithError("Unexpected response from update server", userInitiated: userInitiated)
                return
            }

            guard let data = data else {
                self.finishCheckWithError("Empty response from update server", userInitiated: userInitiated)
                return
            }

            do {
                let release = try JSONDecoder().decode(ReleaseResponse.self, from: data)
                self.handleRelease(release, userInitiated: userInitiated)
            } catch {
                self.finishCheckWithError("Unable to parse update information", userInitiated: userInitiated)
            }
        }.resume()
    }

    func downloadUpdate() {
        guard let downloadURL = downloadURL else {
            presentAlert(title: "Error", message: "Download URL not available")
            return
        }

        DispatchQueue.main.async {
            self.isInstalling = true
            self.statusMessage = "Downloading..."
        }

        downloadingVersion = latestVersion ?? "unknown"
        let config = URLSessionConfiguration.default
        let customSession = URLSession(configuration: config, delegate: self, delegateQueue: .main)
        downloadTask = customSession.downloadTask(with: downloadURL)
        downloadTask?.resume()
    }

    func cancelDownload() {
        downloadTask?.cancel()
        downloadTask = nil
        DispatchQueue.main.async {
            self.isInstalling = false
            self.statusMessage = "Download cancelled"
        }
    }

    // MARK: - Internal Helpers

    private func maybeCheck() {
        // Avoid noisy checks if a recent one just happened
        if let last = lastUpdateCheckDate, Date().timeIntervalSince(last) < minCheckGap {
            return
        }

        checkForUpdates(userInitiated: false)
    }

    private func handleRelease(_ release: ReleaseResponse, userInitiated: Bool) {
        let latest = normalizeVersion(release.tagName)
        let current = currentVersion()

        DispatchQueue.main.async {
            self.latestVersion = latest
            self.lastChecked = Date()
            self.lastUpdateCheckDate = self.lastChecked
            self.downloadURL = release.preferredDownloadURL
            self.isChecking = false
        }

        guard let latestVersion = latest, let currentVersion = current else {
            finishCheckWithError("Unable to read version information", userInitiated: userInitiated)
            return
        }

        if isNewerVersion(latestVersion, than: currentVersion) {
            DispatchQueue.main.async {
                self.updateAvailable = true
                self.statusMessage = "New version available: \(latestVersion)"
            }
            handleNewVersion(latest: latestVersion, release: release, userInitiated: userInitiated)
        } else {
            DispatchQueue.main.async {
                self.updateAvailable = false
                self.statusMessage = "You are up to date"
                if userInitiated {
                    self.presentAlert(title: "Up to Date", message: "GoxViet \(currentVersion) is the latest version available.")
                }
            }
        }
    }

    private func handleNewVersion(latest: String, release: ReleaseResponse, userInitiated: Bool) {
        let alreadyNotified = lastNotifiedVersion == latest

        // Show alert only if user asked explicitly or we have not notified for this version yet
        if userInitiated || !alreadyNotified {
            lastNotifiedVersion = latest
            presentUpdateAvailableAlert(version: latest, release: release)
        } else {
            DispatchQueue.main.async {
                self.statusMessage = "Update available: \(latest)"
            }
        }
    }

    private func finishCheckWithError(_ message: String, userInitiated: Bool) {
        DispatchQueue.main.async {
            self.isChecking = false
            self.statusMessage = message
            if userInitiated {
                self.presentAlert(title: "Update Check Failed", message: message)
            }
        }
    }

    private func presentUpdateAvailableAlert(version: String, release: ReleaseResponse) {
        DispatchQueue.main.async {
            let alert = NSAlert()
            alert.messageText = "Update Available"
            alert.informativeText = "GoxViet \(version) is available. Current version: \(self.currentVersion() ?? "unknown")."
            alert.alertStyle = .informational
            alert.addButton(withTitle: "Download & Install")
            alert.addButton(withTitle: "Later")

            let response = alert.runModal()
            switch response {
            case .alertFirstButtonReturn:
                self.downloadUpdate()
            default:
                break
            }
        }
    }

    private func presentAlert(title: String, message: String) {
        DispatchQueue.main.async {
            let alert = NSAlert()
            alert.messageText = title
            alert.informativeText = message
            alert.alertStyle = .informational
            alert.runModal()
        }
    }

    // MARK: - Version Helpers

    private func currentVersion() -> String? {
        guard let version = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String else {
            return nil
        }
        return normalizeVersion(version)
    }

    private func normalizeVersion(_ version: String?) -> String? {
        guard let version = version else { return nil }
        return version.trimmingCharacters(in: CharacterSet(charactersIn: "vV"))
    }

    private func isNewerVersion(_ latest: String, than current: String) -> Bool {
        let latestParts = latest.split(separator: ".").compactMap { Int($0) }
        let currentParts = current.split(separator: ".").compactMap { Int($0) }

        let maxCount = max(latestParts.count, currentParts.count)
        for idx in 0..<maxCount {
            let latestValue = idx < latestParts.count ? latestParts[idx] : 0
            let currentValue = idx < currentParts.count ? currentParts[idx] : 0
            if latestValue > currentValue { return true }
            if latestValue < currentValue { return false }
        }
        return false
    }

    // MARK: - Homebrew Helpers

    private func resolveBrewPath() -> String? {
        let candidates = ["/opt/homebrew/bin/brew", "/usr/local/bin/brew"]
        for path in candidates where FileManager.default.isExecutableFile(atPath: path) {
            return path
        }
        return nil
    }

    private func runProcess(executable: String, arguments: [String]) -> Bool {
        let process = Process()
        process.executableURL = URL(fileURLWithPath: executable)
        process.arguments = arguments

        let pipe = Pipe()
        process.standardOutput = pipe
        process.standardError = pipe

        do {
            try process.run()
        } catch {
            return false
        }

        process.waitUntilExit()

        if process.terminationStatus != 0 {
            return false
        }

        return true
    }

    private func downloadAndOpenInstaller(from url: URL) {
        DispatchQueue.main.async {
            self.isChecking = true
            self.statusMessage = "Downloading update..."
        }

        session.downloadTask(with: url) { [weak self] tempURL, response, error in
            guard let self = self else { return }

            if let error = error {
                self.finishCheckWithError("Download failed: \(error.localizedDescription)", userInitiated: true)
                return
            }

            guard let tempURL = tempURL else {
                self.finishCheckWithError("Download failed: no data", userInitiated: true)
                return
            }

            let destination = FileManager.default.temporaryDirectory.appendingPathComponent(url.lastPathComponent)
            try? FileManager.default.removeItem(at: destination)

            do {
                try FileManager.default.moveItem(at: tempURL, to: destination)
                self.installDMG(at: destination)
            } catch {
                self.finishCheckWithError("Cannot save installer", userInitiated: true)
            }
        }.resume()
    }

    private func installDMG(at dmgURL: URL) {
        DispatchQueue.global(qos: .userInitiated).async {
            let mountPoint = FileManager.default.temporaryDirectory.appendingPathComponent("goxviet-mount-\(UUID().uuidString)")
            try? FileManager.default.createDirectory(at: mountPoint, withIntermediateDirectories: true)

            let attached = self.runShell("hdiutil attach '\(dmgURL.path)' -nobrowse -quiet -mountpoint '\(mountPoint.path)'")
            guard attached.ok else {
                self.finishCheckWithError("Cannot mount installer", userInitiated: true)
                DispatchQueue.main.async { NSWorkspace.shared.open(dmgURL) }
                return
            }

            guard let appBundle = self.findAppBundle(in: mountPoint) else {
                self.finishCheckWithError("Installer missing app bundle", userInitiated: true)
                _ = self.runShell("hdiutil detach '\(mountPoint.path)' -force -quiet")
                DispatchQueue.main.async { NSWorkspace.shared.open(dmgURL) }
                return
            }

            // Copy to temp for replacement
            let tempApp = "/tmp/GoxViet-update-\(UUID().uuidString).app"
            let copyResult = self.runShell("cp -R '\(appBundle.path)' '\(tempApp)'")
            guard copyResult.ok else {
                self.finishCheckWithError("Cannot prepare update", userInitiated: true)
                _ = self.runShell("hdiutil detach '\(mountPoint.path)' -force -quiet")
                return
            }

            // Unmount
            _ = self.runShell("hdiutil detach '\(mountPoint.path)' -force -quiet")

            // Close current app and install new one
            self.relaunchWithNewApp(tempApp: tempApp)
        }
    }

    private func findAppBundle(in mountPoint: URL) -> URL? {
        let enumerator = FileManager.default.enumerator(at: mountPoint, includingPropertiesForKeys: nil)
        while let item = enumerator?.nextObject() as? URL {
            if item.pathExtension.lowercased() == "app" {
                return item
            }
        }
        return nil
    }

    private func relaunchWithNewApp(tempApp: String) {
        DispatchQueue.main.async {
            self.isInstalling = true
            self.statusMessage = "Installing..."
        }

        let destApp = "/Applications/goxviet.app"
        let script = "sleep 0.5 && rm -rf '\(destApp)' && mv '\(tempApp)' '\(destApp)' && open '\(destApp)'"

        let task = Process()
        task.launchPath = "/bin/sh"
        task.arguments = ["-c", script]
        try? task.run()

        NSApp.terminate(nil)
    }

    @discardableResult
    private func runShell(_ command: String) -> (output: String, ok: Bool) {
        let process = Process()
        let pipe = Pipe()
        process.executableURL = URL(fileURLWithPath: "/bin/bash")
        process.arguments = ["-c", command]
        process.standardOutput = pipe
        process.standardError = pipe
        try? process.run()
        process.waitUntilExit()
        let data = pipe.fileHandleForReading.readDataToEndOfFile()
        let output = String(data: data, encoding: .utf8) ?? ""
        return (output.trimmingCharacters(in: .whitespacesAndNewlines), process.terminationStatus == 0)
    }
}

// MARK: - Preferences Helpers

private extension UpdateManager {
    enum PrefKeys {
        static let lastUpdateCheck = "com.goxviet.ime.lastUpdateCheck"
        static let lastNotifiedVersion = "com.goxviet.ime.lastNotifiedUpdateVersion"
    }

    var lastUpdateCheckDate: Date? {
        get {
            let timestamp = defaults.double(forKey: PrefKeys.lastUpdateCheck)
            return timestamp > 0 ? Date(timeIntervalSince1970: timestamp) : nil
        }
        set {
            if let date = newValue {
                defaults.set(date.timeIntervalSince1970, forKey: PrefKeys.lastUpdateCheck)
            } else {
                defaults.removeObject(forKey: PrefKeys.lastUpdateCheck)
            }
        }
    }

    var lastNotifiedVersion: String? {
        get {
            let value = defaults.string(forKey: PrefKeys.lastNotifiedVersion) ?? ""
            return value.isEmpty ? nil : value
        }
        set {
            if let newValue = newValue, !newValue.isEmpty {
                defaults.set(newValue, forKey: PrefKeys.lastNotifiedVersion)
            } else {
                defaults.removeObject(forKey: PrefKeys.lastNotifiedVersion)
            }
        }
    }

}

// MARK: - Release Models

private struct ReleaseResponse: Decodable {
    let tagName: String
    let htmlURL: String
    let assets: [ReleaseAsset]

    enum CodingKeys: String, CodingKey {
        case tagName = "tag_name"
        case htmlURL = "html_url"
        case assets
    }

    var preferredDownloadURL: URL? {
        if let dmgAsset = assets.first(where: { $0.name.lowercased().hasSuffix(".dmg") }) {
            return URL(string: dmgAsset.browserDownloadURL)
        }
        return URL(string: htmlURL)
    }
}

private struct ReleaseAsset: Decodable {
    let name: String
    let browserDownloadURL: String

    enum CodingKeys: String, CodingKey {
        case name
        case browserDownloadURL = "browser_download_url"
    }
}

// MARK: - URLSession Download Delegate

extension UpdateManager: URLSessionDownloadDelegate {
    func urlSession(_ session: URLSession, downloadTask: URLSessionDownloadTask,
                    didFinishDownloadingTo location: URL) {
        let tempDir = FileManager.default.temporaryDirectory
        let version = downloadingVersion ?? "latest"
        let dmgPath = tempDir.appendingPathComponent("GoxViet-\(version)-unsigned.dmg")

        do {
            if FileManager.default.fileExists(atPath: dmgPath.path) {
                try FileManager.default.removeItem(at: dmgPath)
            }
            try FileManager.default.copyItem(at: location, to: dmgPath)
            installDMG(at: dmgPath)
        } catch {
            finishCheckWithError("Cannot save installer: \(error.localizedDescription)", userInitiated: true)
        }
    }

    func urlSession(_ session: URLSession, downloadTask: URLSessionDownloadTask,
                    didWriteData bytesWritten: Int64, totalBytesWritten: Int64,
                    totalBytesExpectedToWrite: Int64) {
        let progress = Double(totalBytesWritten) / Double(totalBytesExpectedToWrite)
        DispatchQueue.main.async {
            self.statusMessage = "Downloading: \(Int(progress * 100))%"
        }
    }

    func urlSession(_ session: URLSession, task: URLSessionTask, didCompleteWithError error: Error?) {
        guard let error = error else { return }
        if (error as NSError).code == NSURLErrorCancelled {
            DispatchQueue.main.async {
                self.isInstalling = false
                self.statusMessage = "Download cancelled"
            }
        } else {
            finishCheckWithError("Download failed: \(error.localizedDescription)", userInitiated: true)
        }
    }
}
