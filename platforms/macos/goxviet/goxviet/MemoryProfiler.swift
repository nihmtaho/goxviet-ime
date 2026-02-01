//
//  MemoryProfiler.swift
//  GoxViet
//
//  Real-time memory profiling and monitoring
//

import Foundation
import Combine
import os.log

/// Memory usage statistics
struct MemoryStats: Codable, Equatable {
    let timestamp: Date
    let usedMemoryMB: Double
    let availableMemoryMB: Double
    let peakMemoryMB: Double
    let allocatedObjectsCount: Int
    
    var usagePercentage: Double {
        guard availableMemoryMB > 0 else { return 0 }
        return (usedMemoryMB / (usedMemoryMB + availableMemoryMB)) * 100
    }
}

/// Memory profiling manager
final class MemoryProfiler: ObservableObject {
    static let shared = MemoryProfiler()
    
    @Published private(set) var currentStats: MemoryStats
    @Published private(set) var history: [MemoryStats] = []
    @Published private(set) var isMonitoring: Bool = false
    
    private var monitoringTimer: Timer?
    private var peakMemory: Double = 0.0
    private let maxHistoryCount = 60 // Keep last 60 samples
    
    private init() {
        self.currentStats = MemoryProfiler.captureCurrentStats()
        self.peakMemory = self.currentStats.usedMemoryMB
    }
    
    // MARK: - Public API
    
    /// Start real-time monitoring
    func startMonitoring(interval: TimeInterval = 0.5) {
        guard !isMonitoring else { return }
        
        isMonitoring = true
        history.removeAll()
        peakMemory = 0.0
        
        Log.info("Starting memory profiling with interval: \(interval)s")
        
        // Capture initial stats immediately
        updateStats()
        
        // Schedule timer on main run loop for UI updates
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }
            
            self.monitoringTimer = Timer.scheduledTimer(withTimeInterval: interval, repeats: true) { [weak self] _ in
                self?.updateStats()
            }
            
            // Ensure timer fires while scrolling
            if let timer = self.monitoringTimer {
                RunLoop.main.add(timer, forMode: .common)
                ResourceManager.shared.register(timer: timer, identifier: "MemoryProfiler.monitoringTimer")
            }
            
            Log.info("Memory profiling started successfully")
        }
    }
    
    /// Stop monitoring
    func stopMonitoring() {
        guard isMonitoring else { return }
        
        isMonitoring = false
        ResourceManager.shared.unregister(timerIdentifier: "MemoryProfiler.monitoringTimer")
        monitoringTimer?.invalidate()
        monitoringTimer = nil
        
        Log.info("Memory profiling stopped")
    }
    
    /// Take a single snapshot
    func captureSnapshot() -> MemoryStats {
        let stats = MemoryProfiler.captureCurrentStats(peakMemory: peakMemory)
        updatePeakMemory(stats.usedMemoryMB)
        return stats
    }
    
    /// Reset peak memory tracking
    func resetPeak() {
        peakMemory = currentStats.usedMemoryMB
        Log.info("Peak memory reset to: \(peakMemory) MB")
    }
    
    /// Export history to JSON
    func exportHistory() -> Data? {
        return try? JSONEncoder().encode(history)
    }
    
    // MARK: - Private Helpers
    
    private func updateStats() {
        let stats = MemoryProfiler.captureCurrentStats(peakMemory: self.peakMemory)
        
        // Update on main thread for UI binding
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }
            
            self.currentStats = stats
            self.updatePeakMemory(stats.usedMemoryMB)
            
            // Add to history
            self.history.append(stats)
            
            // Keep only recent samples
            if self.history.count > self.maxHistoryCount {
                self.history.removeFirst()
            }
            
            // Log every 10 samples for debugging
            if self.history.count % 10 == 0 {
                Log.info("Memory profiling: \(self.history.count) samples, current: \(stats.formattedUsedMemory)")
            }
        }
    }
    
    private func updatePeakMemory(_ current: Double) {
        if current > peakMemory {
            peakMemory = current
        }
    }
    
    // MARK: - System Memory Capture
    
    private static func captureCurrentStats(peakMemory: Double = 0.0) -> MemoryStats {
        var info = mach_task_basic_info()
        var count = mach_msg_type_number_t(MemoryLayout<mach_task_basic_info>.size) / 4
        
        let kerr = withUnsafeMutablePointer(to: &info) {
            $0.withMemoryRebound(to: integer_t.self, capacity: 1) {
                task_info(mach_task_self_, task_flavor_t(MACH_TASK_BASIC_INFO), $0, &count)
            }
        }
        
        let usedMB: Double
        if kerr == KERN_SUCCESS {
            usedMB = Double(info.resident_size) / 1024.0 / 1024.0
        } else {
            usedMB = 0.0
        }
        
        // Get system memory info
        let availableMB = getAvailableMemory()
        
        // Get malloc zone statistics
        let allocatedObjects = getMallocStats()
        
        return MemoryStats(
            timestamp: Date(),
            usedMemoryMB: usedMB,
            availableMemoryMB: availableMB,
            peakMemoryMB: max(peakMemory, usedMB),
            allocatedObjectsCount: allocatedObjects
        )
    }
    
    private static func getAvailableMemory() -> Double {
        var vmStats = vm_statistics64()
        var count = mach_msg_type_number_t(MemoryLayout<vm_statistics64>.size / MemoryLayout<integer_t>.size)
        
        let result = withUnsafeMutablePointer(to: &vmStats) {
            $0.withMemoryRebound(to: integer_t.self, capacity: Int(count)) {
                host_statistics64(mach_host_self(), HOST_VM_INFO64, $0, &count)
            }
        }
        
        if result == KERN_SUCCESS {
            let freeMB = Double(vmStats.free_count) * Double(vm_page_size) / 1024.0 / 1024.0
            let inactiveMB = Double(vmStats.inactive_count) * Double(vm_page_size) / 1024.0 / 1024.0
            return freeMB + inactiveMB
        }
        
        return 0.0
    }
    
    private static func getMallocStats() -> Int {
        // Simplified: Return approximate count based on memory usage
        // Full malloc zone enumeration requires complex pointer handling
        // For profiling purposes, we'll use a simplified metric
        
        var info = mach_task_basic_info()
        var count = mach_msg_type_number_t(MemoryLayout<mach_task_basic_info>.size) / 4
        
        let kerr = withUnsafeMutablePointer(to: &info) {
            $0.withMemoryRebound(to: integer_t.self, capacity: 1) {
                task_info(mach_task_self_, task_flavor_t(MACH_TASK_BASIC_INFO), $0, &count)
            }
        }
        
        if kerr == KERN_SUCCESS {
            // Rough approximation: 1 object per 1KB of resident memory
            return Int(info.resident_size / 1024)
        }
        
        return 0
    }
}

// MARK: - Formatted Display

extension MemoryStats {
    var formattedUsedMemory: String {
        return String(format: "%.1f MB", usedMemoryMB)
    }
    
    var formattedAvailableMemory: String {
        return String(format: "%.1f MB", availableMemoryMB)
    }
    
    var formattedPeakMemory: String {
        return String(format: "%.1f MB", peakMemoryMB)
    }
    
    var formattedUsagePercentage: String {
        return String(format: "%.1f%%", usagePercentage)
    }
}
