//
//  LifecycleManaged.swift
//  GoxViet
//
//  Protocol for managers with explicit lifecycle management
//  Ensures consistent start/stop/cleanup patterns across the app
//

import Foundation

/// Protocol for components with explicit lifecycle management
protocol LifecycleManaged: AnyObject {
    /// Whether the component is currently running
    var isRunning: Bool { get }
    
    /// Start the component (register observers, start timers, etc.)
    func start()
    
    /// Stop the component (cleanup observers, stop timers, etc.)
    func stop()
}

// MARK: - Default Implementations

extension LifecycleManaged {
    /// Helper to ensure idempotent start
    func guardedStart(_ action: () -> Void) {
        guard !isRunning else {
            Log.info("\(type(of: self)) already running")
            return
        }
        action()
    }
    
    /// Helper to ensure idempotent stop
    func guardedStop(_ action: () -> Void) {
        guard isRunning else {
            Log.info("\(type(of: self)) already stopped")
            return
        }
        action()
    }
}
