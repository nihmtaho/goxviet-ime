//
//  ResourceManager.swift
//  GoxViet
//
//  Centralized resource management for timers, observers, and memory pressure handling
//  Provides automatic cleanup on memory pressure events
//

import Foundation
import Cocoa

/// Centralized manager for app-wide resources
final class ResourceManager {
    static let shared = ResourceManager()
    
    // MARK: - Properties
    
    /// All managed timers
    private var timers: [String: Timer] = [:]
    
    /// All managed observers
    private var observers: [String: NSObjectProtocol] = [:]
    
    /// Memory pressure observer (thermal state)
    private var memoryPressureObserver: NSObjectProtocol?
    /// Dispatch source for system memory pressure notifications
    private var memoryPressureSource: DispatchSourceMemoryPressure?
    
    /// Lock for thread-safe access
    private let lock = NSLock()
    
    // MARK: - Initialization
    
    private init() {
        setupMemoryPressureMonitoring()
    }
    
    deinit {
        cleanup()
    }
    
    // MARK: - Timer Management
    
    /// Register a timer for automatic invalidation
    func register(timer: Timer, identifier: String) {
        lock.lock()
        defer { lock.unlock() }
        
        // Invalidate existing timer with same identifier
        if let existing = timers[identifier] {
            existing.invalidate()
        }
        
        timers[identifier] = timer
        Log.info("Registered timer: \(identifier)")
    }
    
    /// Unregister and invalidate a timer
    func unregister(timerIdentifier: String) {
        lock.lock()
        defer { lock.unlock() }
        
        if let timer = timers.removeValue(forKey: timerIdentifier) {
            timer.invalidate()
            Log.info("Unregistered timer: \(timerIdentifier)")
        }
    }
    
    /// Invalidate all registered timers
    func invalidateAllTimers() {
        lock.lock()
        defer { lock.unlock() }
        
        for (_, timer) in timers {
            timer.invalidate()
        }
        timers.removeAll()
        Log.info("Invalidated all timers")
    }
    
    // MARK: - Observer Management
    
    /// Register an observer for automatic removal
    func register(observer: NSObjectProtocol, identifier: String, center: NotificationCenter = .default) {
        lock.lock()
        defer { lock.unlock() }
        
        // Remove existing observer with same identifier
        if let existing = observers[identifier] {
            center.removeObserver(existing)
        }
        
        observers[identifier] = observer
        Log.info("Registered observer: \(identifier)")
    }
    
    /// Unregister and remove an observer
    func unregister(observerIdentifier: String, center: NotificationCenter = .default) {
        lock.lock()
        defer { lock.unlock() }
        
        if let observer = observers.removeValue(forKey: observerIdentifier) {
            center.removeObserver(observer)
            Log.info("Unregistered observer: \(observerIdentifier)")
        }
    }
    
    /// Remove all registered observers
    func removeAllObservers(center: NotificationCenter = .default) {
        lock.lock()
        defer { lock.unlock() }
        
        for (_, observer) in observers {
            center.removeObserver(observer)
        }
        observers.removeAll()
        Log.info("Removed all observers")
    }
    
    // MARK: - Memory Pressure Monitoring
    
    private func setupMemoryPressureMonitoring() {
        // Dispatch-based memory pressure signal (warning/critical)
        let source = DispatchSource.makeMemoryPressureSource(eventMask: [.warning, .critical], queue: .main)
        source.setEventHandler { [weak self] in
            self?.handleMemoryPressure()
        }
        source.setCancelHandler { [weak self] in
            self?.memoryPressureSource = nil
        }
        source.resume()
        memoryPressureSource = source

        // Thermal state as a secondary signal for proactive cleanup
        memoryPressureObserver = NotificationCenter.default.addObserver(
            forName: ProcessInfo.thermalStateDidChangeNotification,
            object: nil,
            queue: .main
        ) { [weak self] _ in
            let thermalState = ProcessInfo.processInfo.thermalState
            if thermalState == .serious || thermalState == .critical {
                self?.handleMemoryPressure()
            }
        }
        
        Log.info("Memory pressure monitoring enabled (dispatch source + thermal observer)")
    }
    
    private func handleMemoryPressure() {
        Log.warning("Memory pressure detected - clearing caches")
        
        // Clear special panel detector cache
        SpecialPanelAppDetector.clearCache()
        
        // Post notification for other components to cleanup
        NotificationCenter.default.post(
            name: .memoryPressure,
            object: nil
        )
    }
    
    // MARK: - Cleanup
    
    func cleanup() {
        lock.lock()
        defer { lock.unlock() }
        
        // Invalidate all timers
        for (_, timer) in timers {
            timer.invalidate()
        }
        timers.removeAll()
        
        // Remove all observers
        for (_, observer) in observers {
            NotificationCenter.default.removeObserver(observer)
        }
        observers.removeAll()
        
        // Remove memory pressure observer
        if let observer = memoryPressureObserver {
            NotificationCenter.default.removeObserver(observer)
            memoryPressureObserver = nil
        }

        // Cancel memory pressure dispatch source
        if let source = memoryPressureSource {
            source.cancel()
            memoryPressureSource = nil
        }
        
        Log.info("ResourceManager cleaned up")
    }
}

// MARK: - Notification Names

extension Notification.Name {
    static let memoryPressure = Notification.Name("com.goxviet.memoryPressure")
}
