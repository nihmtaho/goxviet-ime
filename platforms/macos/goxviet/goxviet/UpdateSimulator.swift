//
//  UpdateSimulator.swift
//  GoxViet
//
//  Simulator for testing update flow without requiring actual GitHub releases
//
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
        // Step 1: Simulate checking for updates
        DispatchQueue.main.async {
            self.setManagerState(.checking)
        }
        
        // Step 2: After 2 seconds, show update available
        DispatchQueue.main.asyncAfter(deadline: .now() + 2.0) {
            let currentVersion = self.getCurrentVersion()
            let simulatedVersion = self.getSimulatedNewerVersion(current: currentVersion)
            
            // Create a mock update info
            let mockInfo = UpdateInfo(
                version: simulatedVersion,
                downloadURL: URL(string: "https://example.com/mock.dmg")!,
                releaseNotes: "This is a simulated update for testing purposes.",
                publishedAt: Date()
            )
            
            self.setManagerState(.available(mockInfo))
        }
    }
    
    /// Simulate download progress (call this after user clicks Download)
    func simulateDownload() {
        // Reset progress
        currentProgress = 0.0
        
        DispatchQueue.main.async {
            self.setManagerState(.downloading(progress: 0.0))
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
                self.setManagerState(.downloading(progress: self.currentProgress))
            }
        }
    }
    
    /// Simulate installation ready state
    private func finishDownload() {
        DispatchQueue.main.async {
            self.setManagerState(.readyToInstall)
        }
    }
    
    /// Reset to idle state
    func reset() {
        progressTimer?.invalidate()
        progressTimer = nil
        currentProgress = 0.0
        
        DispatchQueue.main.async {
            self.setManagerState(.idle)
        }
    }
    
    // MARK: - Helpers
    
    private func setManagerState(_ state: UpdateState) {
        // UpdateManager needs a debug setter for this to work
        UpdateManager.shared.simulateState(state)
        #if DEBUG
        print("[UpdateSimulator] Setting state to: \(state)")
        #endif
    }
    
    private func getCurrentVersion() -> String {
        guard let version = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String else {
            return "1.0.0"
        }
        return version
    }
    
    private func getSimulatedNewerVersion(current: String) -> String {
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
#endif
