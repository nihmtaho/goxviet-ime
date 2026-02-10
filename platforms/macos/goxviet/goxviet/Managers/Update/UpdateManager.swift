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
    case available(UpdateInfo)
    case downloading(progress: Double)
    case readyToInstall
    case installing
    case upToDate
    case error(String)
}

final class UpdateManager: NSObject, ObservableObject, LifecycleManaged {
    static let shared = UpdateManager()

    @Published private(set) var state: UpdateState = .idle {
        didSet {
            NotificationCenter.default.post(name: .updateStateChanged, object: nil)
        }
    }
    
    // Helper properties for UI binding
    var isChecking: Bool {
        if case .checking = state { return true }
        return false
    }
    

    
    var latestVersion: String? {
        if case .available(let info) = state { return info.version }
        return nil
    }
    
    var statusMessage: String {
        switch state {
        case .idle: return "Not checked yet"
        case .checking: return "Checking for updates..."
        case .available(let info): return "New version available: \(info.version)"
        case .downloading(let progress): return "Downloading: \(Int(progress * 100))%"
        case .readyToInstall: return "Ready to install"
        case .installing: return "Installing..."
        case .upToDate: return "You are up to date"
        case .error(let msg): return msg
        }
    }
    
    var downloadProgress: Double {
        if case .downloading(let progress) = state { return progress }
        return 0.0
    }

    @Published private(set) var lastChecked: Date?
    
    private var downloadSession: URLSession?
    private var downloadTask: URLSessionDownloadTask?
    private var downloadingVersion: String?
    private var timer: Timer?
    private(set) var isRunning: Bool = false
    private let defaults = UserDefaults.standard
    private var isUserCancelledDownload: Bool = false
    
    private let autoCheckInterval: TimeInterval = 6 * 60 * 60  // Every 6 hours
    private let autoCheckKey = "com.goxviet.ime.lastUpdateCheck"
    private let skipVersionKey = "com.goxviet.ime.skipVersion"

    private override init() {
        super.init()
        let timestamp = defaults.double(forKey: autoCheckKey)
        if timestamp > 0 {
            lastChecked = Date(timeIntervalSince1970: timestamp)
        }
    }
    
    deinit {
        // ResourceManager handles cleanup
        Log.info("UpdateManager would be deinitialized (singleton - skipping stop)")
    }

    // MARK: - Public API

    func start() {
        guard !isRunning else { return }
        isRunning = true
        refreshSchedule()
    }
    
    func refreshSchedule() {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }
            
            // Invalidate old timer
            ResourceManager.shared.unregister(timerIdentifier: "UpdateManager.checkTimer")
            self.timer = nil

            // Create new timer
            let newTimer = Timer.scheduledTimer(withTimeInterval: self.autoCheckInterval, repeats: true) { [weak self] _ in
                self?.checkForUpdatesSilently()
            }
            
            ResourceManager.shared.register(timer: newTimer, identifier: "UpdateManager.checkTimer")
            self.timer = newTimer
            
            // Initial check
            self.checkForUpdatesSilently()
        }
    }

    func pauseChecking() {
        if Thread.isMainThread {
            self._pauseCheckingInternal()
        } else {
            DispatchQueue.main.async { [weak self] in
                self?._pauseCheckingInternal()
            }
        }
    }
    
    private func _pauseCheckingInternal() {
        ResourceManager.shared.unregister(timerIdentifier: "UpdateManager.checkTimer")
        self.timer = nil
        Log.info("UpdateManager checking paused")
    }

    func resumeChecking() {
        if isRunning {
            refreshSchedule()
        }
    }

    func stop() {
        DispatchQueue.main.async { [weak self] in
            guard let self = self, self.isRunning else { return }
            
            ResourceManager.shared.unregister(timerIdentifier: "UpdateManager.checkTimer")
            self.timer = nil
            
            self.downloadTask?.cancel()
            self.downloadTask = nil
            
            self.downloadSession?.invalidateAndCancel()
            self.downloadSession = nil
            
            self.state = .idle
            self.isRunning = false
            
            Log.info("UpdateManager stopped")
        }
    }
    
    // MARK: - Update Checks

    func checkForUpdates(userInitiated: Bool) {
        if isChecking { 
            Log.warning("Update check already in progress, skipping")
            return 
        }

        DispatchQueue.main.async {
            self.state = .checking
        }

        UpdateChecker.shared.checkForUpdates { [weak self] result in
            guard let self = self else { return }
            
            self.lastChecked = Date()
            self.defaults.set(self.lastChecked!.timeIntervalSince1970, forKey: self.autoCheckKey)
            
            switch result {
            case .available(let info):
                let skipped = self.defaults.string(forKey: self.skipVersionKey)
                if !userInitiated && skipped == info.version {
                    // Silent check and version skipped -> ignore
                    self.state = .idle
                    return
                }
                self.state = .available(info)
                if !userInitiated {
                    // Could show notification here
                    Log.info("New update available in background: \(info.version)")
                }
                
            case .upToDate:
                self.state = .upToDate
                
            case .error(let message):
                self.state = .error(message)
                Log.error("Update check failed: \(message)")
            }
        }
    }
    
    func checkForUpdatesManually() {
        checkForUpdates(userInitiated: true)
    }
    
    func checkForUpdatesSilently() {
        checkForUpdates(userInitiated: false)
    }
    
    func skipVersion(_ version: String) {
        defaults.set(version, forKey: skipVersionKey)
        state = .idle
    }

    func downloadUpdate() {
        guard case .available(let info) = state else {
             Log.warning("Cannot download: state is not .available")
             return
        }

        DispatchQueue.main.async {
            self.state = .downloading(progress: 0.0)
        }

        isUserCancelledDownload = false
        downloadingVersion = info.version
        let session = makeDownloadSession()
        downloadTask = session.downloadTask(with: info.downloadURL)
        downloadTask?.resume()
    }

    func cancelDownload() {
        isUserCancelledDownload = true
        downloadTask?.cancel()
        downloadTask = nil
        
        DispatchQueue.main.async { [weak self] in
            self?.state = .idle
            Log.info("Download cancelled by user")
        }
    }

    // MARK: - Install
    
    private func installDMG(at dmgURL: URL) {
        DispatchQueue.global(qos: .userInitiated).async {
            let mountPoint = FileManager.default.temporaryDirectory.appendingPathComponent("goxviet-mount-\(UUID().uuidString)")
            try? FileManager.default.createDirectory(at: mountPoint, withIntermediateDirectories: true)

            let attached = self.runShell("hdiutil attach '\(dmgURL.path)' -nobrowse -quiet -mountpoint '\(mountPoint.path)'")
            guard attached.ok else {
                self.finishError("Cannot mount installer")
                DispatchQueue.main.async { NSWorkspace.shared.open(dmgURL) }
                return
            }

            guard let appBundle = self.findAppBundle(in: mountPoint) else {
                self.finishError("Installer missing app bundle")
                _ = self.runShell("hdiutil detach '\(mountPoint.path)' -force -quiet")
                DispatchQueue.main.async { NSWorkspace.shared.open(dmgURL) }
                return
            }

            let tempApp = "/tmp/GoxViet-update-\(UUID().uuidString).app"
            let copyResult = self.runShell("cp -R '\(appBundle.path)' '\(tempApp)'")
            guard copyResult.ok else {
                self.finishError("Cannot prepare update")
                _ = self.runShell("hdiutil detach '\(mountPoint.path)' -force -quiet")
                return
            }

            _ = self.runShell("hdiutil detach '\(mountPoint.path)' -force -quiet")

            self.relaunchWithNewApp(tempApp: tempApp)
        }
    }

    private func findAppBundle(in mountPoint: URL) -> URL? {
        let enumerator = FileManager.default.enumerator(at: mountPoint, includingPropertiesForKeys: nil)
        while let item = enumerator?.nextObject() as? URL {
            if item.pathExtension.lowercased() == "app" { return item }
        }
        return nil
    }

    private func finishError(_ msg: String) {
        DispatchQueue.main.async {
            self.state = .error(msg)
        }
    }

#if DEBUG
    func simulateState(_ newState: UpdateState) {
        DispatchQueue.main.async {
            self.state = newState
        }
    }
#endif
    
    // Kept robust implementation
    private func relaunchWithNewApp(tempApp: String) {
        DispatchQueue.main.async {
            self.state = .installing
        }
        
        // Stop InputManager explicitly to release resources immediately
        InputManager.shared.stop()

        let destApp = Bundle.main.bundlePath
        let logFile = "/tmp/goxviet_update.log"


        let currentPID = ProcessInfo.processInfo.processIdentifier
        
        let debugScript = """
        echo "Starting update at $(date)" > "\(logFile)"
        echo "PID to wait for: \(currentPID)" >> "\(logFile)"
        echo "Temp App: \(tempApp)" >> "\(logFile)"
        echo "Dest App: \(destApp)" >> "\(logFile)"
        
        is_running() { kill -0 $1 > /dev/null 2>&1; }
        
        echo "Waiting for PID \(currentPID) to exit..." >> "\(logFile)"
        MAX_RETRIES=100
        COUNT=0
        while is_running \(currentPID); do
            sleep 0.1
            COUNT=$((COUNT+1))
            if [ $COUNT -ge $MAX_RETRIES ]; then
                echo "Timeout waiting for PID \(currentPID)" >> "\(logFile)"
                pkill -P \(currentPID)
                kill -9 \(currentPID)
                break
            fi
        done
        
        echo "Process terminated (or timeout), proceeding with update..." >> "\(logFile)"
        
        log() { echo "$1" >> "\(logFile)"; }
        
        log "Syncing files..."
        if rsync -a --delete "\(tempApp)/" "\(destApp)/"; then
            log "Rsync successful"
        else
            log "Rsync failed with code $?"
            exit 1
        fi
        
        log "Cleaning up temp app..."
        rm -rf "\(tempApp)"
        
        log "Relaunching app..."
        if open -n "\(destApp)"; then
            log "App launch successful"
        else
            log "Failed to open app"
        fi
        """
        
        let scriptPath = FileManager.default.temporaryDirectory.appendingPathComponent("goxviet_update.sh").path
        do {
            try debugScript.write(toFile: scriptPath, atomically: true, encoding: .utf8)
            try FileManager.default.setAttributes([.posixPermissions: 0o755], ofItemAtPath: scriptPath)
        } catch {
            Log.error("Failed to create update script: \(error)")
            return
        }
        
        let command = "nohup /bin/sh \"\(scriptPath)\" > /dev/null 2>&1 &"
        let task = Process()
        task.launchPath = "/bin/sh"
        task.arguments = ["-c", command]
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

private extension UpdateManager {
    func makeDownloadSession() -> URLSession {
        if let existing = downloadSession { return existing }
        let config = URLSessionConfiguration.default
        config.waitsForConnectivity = true
        config.timeoutIntervalForRequest = 20
        config.timeoutIntervalForResource = 120
        let session = URLSession(configuration: config, delegate: self, delegateQueue: .main)
        downloadSession = session
        return session
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
            
            let sessionToCleanup = self.downloadSession
            self.downloadSession = nil
            if let session = sessionToCleanup {
                session.finishTasksAndInvalidate()
            }
            
            DispatchQueue.main.async {
                self.state = .readyToInstall
            }
            
            DispatchQueue.main.asyncAfter(deadline: .now() + 1.5) {
                self.installDMG(at: dmgPath)
            }
        } catch {
            finishError("Cannot save installer: \(error.localizedDescription)")
        }
    }

    func urlSession(_ session: URLSession, downloadTask: URLSessionDownloadTask,
                    didWriteData bytesWritten: Int64, totalBytesWritten: Int64,
                    totalBytesExpectedToWrite: Int64) {
        let progress = Double(totalBytesWritten) / Double(totalBytesExpectedToWrite)
        DispatchQueue.main.async {
            self.state = .downloading(progress: progress)
        }
    }

    func urlSession(_ session: URLSession, task: URLSessionTask, didCompleteWithError error: Error?) {
        defer {
            let sessionToCleanup = self.downloadSession
            self.downloadSession = nil
            if let session = sessionToCleanup {
                session.finishTasksAndInvalidate()
            }
        }
        
        guard let error = error else { return }
        
        if (error as NSError).code == NSURLErrorCancelled {
            if !isUserCancelledDownload {
                DispatchQueue.main.async {
                    self.state = .idle
                }
            }
            isUserCancelledDownload = false
        } else {
            finishError("Download failed: \(error.localizedDescription)")
        }
    }
}


