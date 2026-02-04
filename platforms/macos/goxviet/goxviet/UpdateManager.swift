//
//  UpdateManager.swift
//  GoxViet
//
//  Auto-update checker with DMG-based installer (mount, copy, restart).
//

import Foundation
import AppKit
import Combine

// MARK: - Update State

enum UpdateState: Equatable {
    case idle
    case checking
    case updateAvailable
    case downloading
    case readyToInstall
    case upToDate
    case error
}

final class UpdateManager: NSObject, ObservableObject, LifecycleManaged {
    static let shared = UpdateManager()

    @Published private(set) var latestVersion: String?
    @Published private(set) var statusMessage: String = "Not checked yet"
    @Published private(set) var isChecking: Bool = false
    @Published private(set) var updateAvailable: Bool = false
    @Published private(set) var lastChecked: Date?
    @Published private(set) var downloadURL: URL?
    @Published private(set) var isInstalling: Bool = false
    
    // New properties for enhanced UI
    @Published private(set) var updateState: UpdateState = .idle
    @Published private(set) var downloadProgress: Double = 0.0

    private var apiSession: URLSession?
    private var downloadSession: URLSession?
    private var downloadTask: URLSessionDownloadTask?
    private var downloadingVersion: String?
    private var timer: Timer?
    private(set) var isRunning: Bool = false
    private let defaults = UserDefaults.standard
    private var isUserCancelledDownload: Bool = false

    private let apiURL = URL(string: "https://api.github.com/repos/nihmtaho/goxviet-ime/releases/latest")!
    private let checkInterval: TimeInterval = 6 * 60 * 60  // Every 6 hours
    private let minCheckGap: TimeInterval = 20 * 60        // Skip redundant checks within 20 minutes

    private func releasePageURL(for version: String) -> URL {
        URL(string: "https://github.com/nihmtaho/goxviet-ime/releases/tag/v\(version)") ?? URL(string: "https://github.com/nihmtaho/goxviet-ime/releases")!
    }
    
    /// Lazy initialization of API session - only created when needed
    private func makeAPISession() -> URLSession {
        if let existing = apiSession { return existing }
        
        let apiConfig = URLSessionConfiguration.default
        apiConfig.waitsForConnectivity = true
        apiConfig.timeoutIntervalForRequest = 12
        apiConfig.timeoutIntervalForResource = 12
        
        let session = URLSession(configuration: apiConfig)
        apiSession = session
        Log.info("API URLSession created (lazy)")
        return session
    }

    private override init() {
        super.init()
    }
    
    deinit {
        // CRITICAL FIX: Do NOT call stop() in deinit for singleton
        // UpdateManager is a singleton that lives for the entire app lifecycle.
        // Calling stop() in deinit causes race condition:
        // 1. deinit starts deallocating UpdateManager
        // 2. stop() schedules async work via DispatchQueue.main.async
        // 3. Weak self reference becomes invalid mid-deallocation
        // 4. EXC_BAD_ACCESS when async block tries to access deallocated memory
        // 
        // Instead, let ResourceManager handle cleanup via its lifecycle management.
        // Only call stop() explicitly when app is actually terminating.
        Log.info("UpdateManager would be deinitialized (singleton - skipping stop)")
    }


    // MARK: - Public API

    func start() {
        guard !isRunning else { return }
        isRunning = true
        refreshSchedule(triggerImmediate: true)
    }

    /// Pauses the check timer without stopping other operations.
    /// Called when Update window is closed to prevent timer callbacks
    /// after window deallocation (which causes EXC_BAD_ACCESS).
    /// Pauses the check timer without stopping other operations.
    /// Called when Update window is closed to prevent timer callbacks
    /// after window deallocation (which causes EXC_BAD_ACCESS).
    func pauseChecking() {
        // CRITICAL FIX: Run synchronously if on main thread to ensure timer
        // is invalidated BEFORE window/view deallocation continues.
        // Async dispatch allows window close to finish before timer is stopped,
        // reopening the race condition window.
        if Thread.isMainThread {
            self._pauseCheckingInternal()
        } else {
            DispatchQueue.main.async { [weak self] in
                self?._pauseCheckingInternal()
            }
        }
    }
    
    private func _pauseCheckingInternal() {
        // Unregister timer from ResourceManager to stop it from firing
        ResourceManager.shared.unregister(timerIdentifier: "UpdateManager.checkTimer")
        self.timer = nil
        
        Log.info("UpdateManager checking paused (no more periodic checks)")
    }

    /// Resumes the check timer after it was paused.
    /// Called when Update window is reopened.
    func resumeChecking() {
        DispatchQueue.main.async { [weak self] in
            guard let self = self, self.isRunning else { return }
            
            // Re-schedule the timer
            self.refreshSchedule(triggerImmediate: false)
            
            Log.info("UpdateManager checking resumed")
        }
    }

    /// Stops UpdateManager and cleans up resources.
    /// WARNING: This method ONLY cleans up UpdateManager resources.
    /// It does NOT close windows or terminate the app.
    /// Window lifecycle is managed by the window controllers/SwiftUI views.
    func stop() {
        DispatchQueue.main.async { [weak self] in
            guard let self = self, self.isRunning else { return }
            
            // Unregister timer from ResourceManager
            ResourceManager.shared.unregister(timerIdentifier: "UpdateManager.checkTimer")
            self.timer = nil
            
            // Cancel ongoing downloads (does not affect other app operations)
            self.downloadTask?.cancel()
            self.downloadTask = nil
            
            // Invalidate and nil out sessions to release network resources
            // Note: This only affects UpdateManager's network sessions,
            // not app windows or other managers
            self.downloadSession?.invalidateAndCancel()
            self.downloadSession = nil
            self.apiSession?.invalidateAndCancel()
            self.apiSession = nil
            
            // Reset state (internal state only, does not affect app UI)
            self.isChecking = false
            self.isInstalling = false
            self.updateState = .idle
            self.isRunning = false
            
            Log.info("UpdateManager stopped and cleaned up (windows unaffected)")
        }
    }

    func refreshSchedule(triggerImmediate: Bool = false) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            // Create new timer
            let newTimer = Timer.scheduledTimer(withTimeInterval: self.checkInterval, repeats: true) { [weak self] _ in
                self?.maybeCheck()
            }
            
            // Register with ResourceManager for automatic cleanup
            ResourceManager.shared.register(timer: newTimer, identifier: "UpdateManager.checkTimer")
            self.timer = newTimer

            if triggerImmediate {
                self.maybeCheck()
            }
        }
    }

    func checkForUpdates(userInitiated: Bool) {
        // Prevent overlapping checks
        if isChecking { 
            Log.warning("Update check already in progress, skipping")
            return 
        }

        DispatchQueue.main.async {
            self.isChecking = true
            self.updateState = .checking
            self.statusMessage = "Checking for updates..."
        }

        var request = URLRequest(url: apiURL)
        request.setValue("application/vnd.github+json", forHTTPHeaderField: "Accept")
        request.setValue("GoxViet-Update-Agent", forHTTPHeaderField: "User-Agent")
        request.cachePolicy = .reloadIgnoringLocalCacheData // Always fetch fresh data

        makeAPISession().dataTask(with: request) { [weak self] data, response, error in
            guard let self = self else { return }

            if let error = error {
                let nsError = error as NSError
                var errorMessage = "Network error: \(error.localizedDescription)"
                
                // Provide more specific error messages
                if nsError.code == NSURLErrorNotConnectedToInternet {
                    errorMessage = "No internet connection. Please check your network."
                } else if nsError.code == NSURLErrorTimedOut {
                    errorMessage = "Request timed out. Please try again."
                } else if nsError.code == NSURLErrorCannotFindHost {
                    errorMessage = "Cannot reach update server. Please try again later."
                }
                
                self.finishCheckWithError(errorMessage, userInitiated: userInitiated)
                Log.error("Update check failed: \(error)")
                return
            }

            guard let httpResponse = response as? HTTPURLResponse else {
                self.finishCheckWithError("Invalid response from update server", userInitiated: userInitiated)
                return
            }
            
            guard (200...299).contains(httpResponse.statusCode) else {
                let errorMessage: String
                switch httpResponse.statusCode {
                case 404:
                    errorMessage = "Update information not found"
                case 403:
                    errorMessage = "Access to update server denied"
                case 500...599:
                    errorMessage = "Update server is temporarily unavailable"
                default:
                    errorMessage = "Unexpected response from update server (HTTP \(httpResponse.statusCode))"
                }
                self.finishCheckWithError(errorMessage, userInitiated: userInitiated)
                Log.error("Update check HTTP error: \(httpResponse.statusCode)")
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
                Log.error("Update check parse error: \(error)")
            }
        }.resume()
    }

    func downloadUpdate() {
        guard let downloadURL = downloadURL else {
            DispatchQueue.main.async {
                self.updateState = .error
                self.statusMessage = "Download URL not available"
            }
            return
        }

        DispatchQueue.main.async {
            self.isInstalling = true
            self.updateState = .downloading
            self.downloadProgress = 0.0
            self.statusMessage = "Downloading..."
        }

        // Reset cancel flag when starting new download
        isUserCancelledDownload = false
        downloadingVersion = latestVersion ?? "unknown"
        let session = makeDownloadSession()
        downloadTask = session.downloadTask(with: downloadURL)
        downloadTask?.resume()
    }

    /// Cancels the ongoing download.
    /// WARNING: This method ONLY cancels the download task and updates state.
    /// It does NOT close any windows (Update window or Settings window).
    /// Window closing is handled by the view layer (UpdateWindowView).
    /// CRITICAL: No long-running operations to prevent InputManager interruption.
    func cancelDownload() {
        // Set flag to indicate user-initiated cancel
        isUserCancelledDownload = true
        
        // Only cancel the task, let delegate handle cleanup
        downloadTask?.cancel()
        downloadTask = nil
        
        // Update UI state (internal state only, does not affect windows)
        // Use immediate sync update to avoid async timing issues with InputManager
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }
            self.isInstalling = false
            self.updateState = .idle
            self.downloadProgress = 0.0
            self.statusMessage = "Download cancelled"
            Log.info("Download cancelled by user - no InputManager impact")
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
                self.updateState = .updateAvailable
                self.statusMessage = "New version available: \(latestVersion)"
            }
            handleNewVersion(latest: latestVersion, release: release, userInitiated: userInitiated)
        } else {
            DispatchQueue.main.async {
                self.updateAvailable = false
                self.updateState = .upToDate
                self.statusMessage = "You are up to date"
            }
        }
    }

    private func handleNewVersion(latest: String, release: ReleaseResponse, userInitiated: Bool) {
        let alreadyNotified = lastNotifiedVersion == latest

        if userInitiated || !alreadyNotified {
            lastNotifiedVersion = latest
            // If it's a background block, we might want to notify via Notification Center
            // but for now, we just ensure the window is open if manual, 
            // and background just updates the statusMessage.
            if !userInitiated {
                 // For background, we could trigger a system notification here in the future
                 Log.info("New version available in background: \(latest)")
            }
        }
        
        DispatchQueue.main.async {
            self.statusMessage = "Update available: \(latest)"
        }
    }

    private func finishCheckWithError(_ message: String, userInitiated: Bool) {
        DispatchQueue.main.async {
            self.isChecking = false
            self.updateState = .error
            self.statusMessage = message
        }
    }

    // Old alert methods removed in favor of dedicated UpdateWindowView flow
    
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

    /// Installs the new app version and relaunches.
    /// WARNING: This is the ONLY method in UpdateManager that terminates the app.
    /// It terminates the app (NSApp.terminate) to complete the installation.
    /// This is expected behavior when installing updates.
    private func relaunchWithNewApp(tempApp: String) {
        DispatchQueue.main.async {
            self.isInstalling = true
            self.statusMessage = "Installing..."
        }

        let destApp = Bundle.main.bundlePath
        // CRITICAL FIX: Use rsync for in-place update instead of rm -rf
        // This preserves the TCC (Accessibility) permissions of the app bundle
        // by keeping the directory inode intact and only updating contents.
        // We use 'rsync -a --delete' to mirror the new app into the existing one.
        let script = "sleep 0.5 && rsync -a --delete '\(tempApp)/' '\(destApp)/' && rm -rf '\(tempApp)' && open '\(destApp)'"

        let task = Process()
        task.launchPath = "/bin/sh"
        task.arguments = ["-c", script]
        try? task.run()

        // This is the only place where we terminate the app
        // (required for update installation)
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

private extension UpdateManager {
    func makeDownloadSession() -> URLSession {
        if let existing = downloadSession { return existing }
        let config = URLSessionConfiguration.default
        config.waitsForConnectivity = true
        config.timeoutIntervalForRequest = 20
        config.timeoutIntervalForResource = 120
        config.allowsExpensiveNetworkAccess = true
        config.allowsConstrainedNetworkAccess = true
        let session = URLSession(configuration: config, delegate: self, delegateQueue: .main)
        downloadSession = session
        return session
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

    nonisolated init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        self.tagName = try container.decode(String.self, forKey: .tagName)
        self.htmlURL = try container.decode(String.self, forKey: .htmlURL)
        self.assets = try container.decode([ReleaseAsset].self, forKey: .assets)
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

    nonisolated init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        self.name = try container.decode(String.self, forKey: .name)
        self.browserDownloadURL = try container.decode(String.self, forKey: .browserDownloadURL)
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
            
            // CRITICAL FIX: Use copy-and-nil pattern to prevent use-after-free
            let sessionToCleanup = self.downloadSession
            self.downloadSession = nil
            if let session = sessionToCleanup {
                session.finishTasksAndInvalidate()
            }
            
            // Update state to ready to install
            DispatchQueue.main.async {
                self.updateState = .readyToInstall
                self.downloadProgress = 1.0
                self.statusMessage = "Download complete - Ready to install"
            }
            
            // Auto-install after a brief delay to show completion
            DispatchQueue.main.asyncAfter(deadline: .now() + 1.5) {
                self.installDMG(at: dmgPath)
            }
        } catch {
            finishCheckWithError("Cannot save installer: \(error.localizedDescription)", userInitiated: true)
        }
    }

    func urlSession(_ session: URLSession, downloadTask: URLSessionDownloadTask,
                    didWriteData bytesWritten: Int64, totalBytesWritten: Int64,
                    totalBytesExpectedToWrite: Int64) {
        let progress = Double(totalBytesWritten) / Double(totalBytesExpectedToWrite)
        DispatchQueue.main.async {
            self.downloadProgress = progress
            self.statusMessage = "Downloading: \(Int(progress * 100))%"
        }
    }

    func urlSession(_ session: URLSession, task: URLSessionTask, didCompleteWithError error: Error?) {
        defer {
            // CRITICAL FIX: Use copy-and-nil pattern to prevent double cleanup/use-after-free
            // Store reference, clear immediately, then invalidate
            let sessionToCleanup = self.downloadSession
            self.downloadSession = nil
            
            if let session = sessionToCleanup {
                session.finishTasksAndInvalidate()
            }
        }
        
        guard let error = error else { return }
        
        if (error as NSError).code == NSURLErrorCancelled {
            // Only update UI if not already cancelled by user
            if !isUserCancelledDownload {
                DispatchQueue.main.async {
                    self.isInstalling = false
                    self.updateState = .idle
                    self.downloadProgress = 0.0
                    self.statusMessage = "Download cancelled"
                }
            }
            // Reset flag
            isUserCancelledDownload = false
        } else {
            finishCheckWithError("Download failed: \(error.localizedDescription)", userInitiated: true)
        }
    }
}

