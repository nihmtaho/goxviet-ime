//
//  UpdateSimulator.swift
//  GoxViet
//
//  Simulator for testing update flow without requiring actual GitHub releases
//

#if DEBUG
import Foundation
import Combine

final class UpdateSimulator {
    static let shared = UpdateSimulator()
    
    private var progressTimer: Timer?
    private var currentProgress: Double = 0.0
    
    private init() {}
    
    // MARK: - Public API
    
    /// Simulate a complete update flow with progress animation
    func simulateUpdateFlow() {
        let updateManager = UpdateManager.shared
        
        // Step 1: Simulate checking for updates
        DispatchQueue.main.async {
            updateManager.setValue(UpdateState.checking, forKey: "updateState")
            updateManager.setValue("Checking for updates...", forKey: "statusMessage")
        }
        
        // Step 2: After 2 seconds, show update available
        DispatchQueue.main.asyncAfter(deadline: .now() + 2.0) {
            let currentVersion = self.getCurrentVersion()
            let simulatedVersion = self.getSimulatedNewerVersion(current: currentVersion)
            
            updateManager.setValue(simulatedVersion, forKey: "latestVersion")
            updateManager.setValue(UpdateState.updateAvailable, forKey: "updateState")
            updateManager.setValue("New version available: \(simulatedVersion)", forKey: "statusMessage")
            updateManager.setValue(true, forKey: "updateAvailable")
            updateManager.setValue(Date(), forKey: "lastChecked")
        }
    }
    
    /// Simulate download progress (call this after user clicks Download)
    func simulateDownload() {
        let updateManager = UpdateManager.shared
        
        // Reset progress
        currentProgress = 0.0
        
        DispatchQueue.main.async {
            updateManager.setValue(UpdateState.downloading, forKey: "updateState")
            updateManager.setValue(0.0, forKey: "downloadProgress")
            updateManager.setValue("Downloading: 0%", forKey: "statusMessage")
        }
        
        // Animate progress over 5 seconds
        progressTimer?.invalidate()
        progressTimer = Timer.scheduledTimer(withTimeInterval: 0.1, repeats: true) { [weak self] timer in
            guard let self = self else {
                timer.invalidate()
                return
            }
            
            self.currentProgress += 0.02 // Increment by 2% each 0.1s (5 seconds total)
            
            if self.currentProgress >= 1.0 {
                self.currentProgress = 1.0
                timer.invalidate()
                self.finishDownload()
            }
            
            DispatchQueue.main.async {
                updateManager.setValue(self.currentProgress, forKey: "downloadProgress")
                updateManager.setValue("Downloading: \(Int(self.currentProgress * 100))%", forKey: "statusMessage")
            }
        }
    }
    
    /// Simulate installation ready state
    private func finishDownload() {
        let updateManager = UpdateManager.shared
        
        DispatchQueue.main.async {
            updateManager.setValue(UpdateState.readyToInstall, forKey: "updateState")
            updateManager.setValue(1.0, forKey: "downloadProgress")
            updateManager.setValue("Download complete - Ready to install", forKey: "statusMessage")
        }
    }
    
    /// Reset to idle state
    func reset() {
        progressTimer?.invalidate()
        progressTimer = nil
        currentProgress = 0.0
        
        let updateManager = UpdateManager.shared
        DispatchQueue.main.async {
            updateManager.setValue(UpdateState.idle, forKey: "updateState")
            updateManager.setValue(0.0, forKey: "downloadProgress")
            updateManager.setValue("Not checked yet", forKey: "statusMessage")
            updateManager.setValue(false, forKey: "updateAvailable")
        }
    }
    
    // MARK: - Helpers
    
    private func getCurrentVersion() -> String {
        guard let version = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String else {
            return "1.0.0"
        }
        return version
    }
    
    private func getSimulatedNewerVersion(current: String) -> String {
        // Parse current version and increment minor version
        let components = current.split(separator: ".").compactMap { Int($0) }
        
        if components.count >= 3 {
            return "\(components[0]).\(components[1] + 1).0"
        } else if components.count == 2 {
            return "\(components[0]).\(components[1] + 1).0"
        } else {
            return "2.0.0"
        }
    }
}

// MARK: - NSObject Extension for setValue

private extension NSObject {
    func setValue(_ value: Any?, forKey key: String) {
        // Use KVO to set private properties for testing
        setValue(value, forKeyPath: key)
    }
}

#endif
