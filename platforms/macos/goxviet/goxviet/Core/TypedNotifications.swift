//
//  TypedNotifications.swift
//  GoxViet
//
//  Typed notification system to replace raw NotificationCenter usage
//  Provides type-safe notifications with structured payloads
//

import Foundation

// MARK: - Notification Payload Protocols

/// Base protocol for all notification payloads
protocol NotificationPayload {
    var timestamp: Date { get }
}

// MARK: - Concrete Notification Types

/// Input method changed notification
struct InputMethodChangedNotification: NotificationPayload {
    let method: Int  // 0 = Telex, 1 = VNI
    let timestamp: Date
    
    init(method: Int) {
        self.method = method
        self.timestamp = Date()
    }
    
    var methodName: String {
        method == 0 ? "Telex" : "VNI"
    }
}

/// Tone style changed notification
struct ToneStyleChangedNotification: NotificationPayload {
    let isModern: Bool
    let timestamp: Date
    
    init(isModern: Bool) {
        self.isModern = isModern
        self.timestamp = Date()
    }
    
    var styleName: String {
        isModern ? "Modern" : "Traditional"
    }
}

/// Smart mode changed notification
struct SmartModeChangedNotification: NotificationPayload {
    let enabled: Bool
    let timestamp: Date
    
    init(enabled: Bool) {
        self.enabled = enabled
        self.timestamp = Date()
    }
}

/// Per-app modes changed notification
struct PerAppModesChangedNotification: NotificationPayload {
    let bundleId: String?
    let enabled: Bool?
    let timestamp: Date
    
    init(bundleId: String? = nil, enabled: Bool? = nil) {
        self.bundleId = bundleId
        self.enabled = enabled
        self.timestamp = Date()
    }
}

/// ESC restore changed notification
struct EscRestoreChangedNotification: NotificationPayload {
    let enabled: Bool
    let timestamp: Date
    
    init(enabled: Bool) {
        self.enabled = enabled
        self.timestamp = Date()
    }
}

/// Free tone changed notification
struct FreeToneChangedNotification: NotificationPayload {
    let enabled: Bool
    let timestamp: Date
    
    init(enabled: Bool) {
        self.enabled = enabled
        self.timestamp = Date()
    }
}

/// Instant restore changed notification
struct InstantRestoreChangedNotification: NotificationPayload {
    let enabled: Bool
    let timestamp: Date
    
    init(enabled: Bool) {
        self.enabled = enabled
        self.timestamp = Date()
    }
}

/// Text expansion enabled/disabled notification
struct TextExpansionEnabledChangedNotification: NotificationPayload {
    let enabled: Bool
    let timestamp: Date
    
    init(enabled: Bool) {
        self.enabled = enabled
        self.timestamp = Date()
    }
}

// MARK: - Typed Notification Center

/// Type-safe wrapper around NotificationCenter
final class TypedNotificationCenter {
    
    static let shared = TypedNotificationCenter()
    
    private let center = NotificationCenter.default
    private let queue = DispatchQueue.main
    
    private init() {}
    
    // MARK: - Post Methods
    
    func post(_ notification: InputMethodChangedNotification) {
        queue.async {
            self.center.post(
                name: .inputMethodChanged,
                object: notification,
                userInfo: ["method": notification.method, "timestamp": notification.timestamp]
            )
        }
    }
    
    func post(_ notification: ToneStyleChangedNotification) {
        queue.async {
            self.center.post(
                name: .toneStyleChanged,
                object: notification,
                userInfo: ["isModern": notification.isModern, "timestamp": notification.timestamp]
            )
        }
    }
    
    func post(_ notification: SmartModeChangedNotification) {
        queue.async {
            self.center.post(
                name: .smartModeChanged,
                object: notification,
                userInfo: ["enabled": notification.enabled, "timestamp": notification.timestamp]
            )
        }
    }
    
    func post(_ notification: PerAppModesChangedNotification) {
        queue.async {
            var userInfo: [String: Any] = ["timestamp": notification.timestamp]
            if let bundleId = notification.bundleId {
                userInfo["bundleId"] = bundleId
            }
            if let enabled = notification.enabled {
                userInfo["enabled"] = enabled
            }
            
            self.center.post(
                name: .perAppModesChanged,
                object: notification,
                userInfo: userInfo
            )
        }
    }
    
    func post(_ notification: EscRestoreChangedNotification) {
        queue.async {
            self.center.post(
                name: .escRestoreChanged,
                object: notification,
                userInfo: ["enabled": notification.enabled, "timestamp": notification.timestamp]
            )
        }
    }
    
    func post(_ notification: FreeToneChangedNotification) {
        queue.async {
            self.center.post(
                name: .freeToneChanged,
                object: notification,
                userInfo: ["enabled": notification.enabled, "timestamp": notification.timestamp]
            )
        }
    }
    
    func post(_ notification: InstantRestoreChangedNotification) {
        queue.async {
            self.center.post(
                name: .instantRestoreChanged,
                object: notification,
                userInfo: ["enabled": notification.enabled, "timestamp": notification.timestamp]
            )
        }
    }
    
    func post(_ notification: TextExpansionEnabledChangedNotification) {
        queue.async {
            self.center.post(
                name: .textExpansionEnabledChanged,
                object: notification,
                userInfo: ["enabled": notification.enabled, "timestamp": notification.timestamp]
            )
        }
    }
    
    // MARK: - Subscribe Methods
    
    func subscribe(
        to: InputMethodChangedNotification.Type,
        handler: @escaping (InputMethodChangedNotification) -> Void
    ) -> NSObjectProtocol {
        return center.addObserver(
            forName: .inputMethodChanged,
            object: nil,
            queue: .main
        ) { notification in
            if let typed = notification.object as? InputMethodChangedNotification {
                handler(typed)
            }
        }
    }
    
    func subscribe(
        to: ToneStyleChangedNotification.Type,
        handler: @escaping (ToneStyleChangedNotification) -> Void
    ) -> NSObjectProtocol {
        return center.addObserver(
            forName: .toneStyleChanged,
            object: nil,
            queue: .main
        ) { notification in
            if let typed = notification.object as? ToneStyleChangedNotification {
                handler(typed)
            }
        }
    }
    
    func subscribe(
        to: SmartModeChangedNotification.Type,
        handler: @escaping (SmartModeChangedNotification) -> Void
    ) -> NSObjectProtocol {
        return center.addObserver(
            forName: .smartModeChanged,
            object: nil,
            queue: .main
        ) { notification in
            if let typed = notification.object as? SmartModeChangedNotification {
                handler(typed)
            }
        }
    }
    
    func subscribe(
        to: PerAppModesChangedNotification.Type,
        handler: @escaping (PerAppModesChangedNotification) -> Void
    ) -> NSObjectProtocol {
        return center.addObserver(
            forName: .perAppModesChanged,
            object: nil,
            queue: .main
        ) { notification in
            if let typed = notification.object as? PerAppModesChangedNotification {
                handler(typed)
            }
        }
    }
    
    // MARK: - Unsubscribe
    
    func unsubscribe(_ observer: NSObjectProtocol) {
        center.removeObserver(observer)
    }
}

// MARK: - Debouncer for Rapid Updates

/// Debounces rapid notification posts
final class NotificationDebouncer {
    private var workItem: DispatchWorkItem?
    private let delay: TimeInterval
    private let queue: DispatchQueue
    
    init(delay: TimeInterval = 0.3, queue: DispatchQueue = .main) {
        self.delay = delay
        self.queue = queue
    }
    
    func debounce(_ action: @escaping () -> Void) {
        workItem?.cancel()
        
        let newWorkItem = DispatchWorkItem(block: action)
        workItem = newWorkItem
        
        queue.asyncAfter(deadline: .now() + delay, execute: newWorkItem)
    }
    
    func cancel() {
        workItem?.cancel()
        workItem = nil
    }
}

// MARK: - Usage Examples (in comments)

/*
 // POSTING NOTIFICATIONS
 
 // Old way (unsafe):
 NotificationCenter.default.post(name: .inputMethodChanged, object: 0)
 
 // New way (type-safe):
 let notification = InputMethodChangedNotification(method: 0)
 TypedNotificationCenter.shared.post(notification)
 
 // SUBSCRIBING TO NOTIFICATIONS
 
 // Old way:
 NotificationCenter.default.addObserver(
     forName: .inputMethodChanged,
     object: nil,
     queue: .main
 ) { notification in
     if let method = notification.object as? Int {
         print("Method changed: \(method)")
     }
 }
 
 // New way:
 let observer = TypedNotificationCenter.shared.subscribe(
     to: InputMethodChangedNotification.self
 ) { notification in
     print("Method changed: \(notification.methodName)")
     print("Timestamp: \(notification.timestamp)")
 }
 
 // DEBOUNCING
 
 let debouncer = NotificationDebouncer(delay: 0.5)
 
 for i in 0..<100 {
     debouncer.debounce {
         print("Only called once after 0.5s: \(i)")
     }
 }
 */

// MARK: - Notification Name Extensions

extension Notification.Name {
    static let inputMethodChanged = Notification.Name("inputMethodChanged")
    static let toneStyleChanged = Notification.Name("toneStyleChanged")
    static let smartModeChanged = Notification.Name("smartModeChanged")
    static let updateStateChanged = Notification.Name("updateStateChanged")
    static let shortcutChanged = Notification.Name("shortcutChanged")
    static let perAppModesChanged = Notification.Name("perAppModesChanged")
    static let toggleVietnamese = Notification.Name("toggleVietnamese")
    static let settingsChanged = Notification.Name("com.goxviet.settingsChanged")
    static let outputEncodingChanged = Notification.Name("com.goxviet.outputEncodingChanged")
    static let shiftBackspaceEnabledChanged = Notification.Name("com.goxviet.shiftBackspaceEnabledChanged")
    static let textExpansionEnabledChanged = Notification.Name("com.goxviet.ime.textExpansionEnabledChanged")

    
    // Debugging
    static let didSaveShortcuts = Notification.Name("com.goxviet.ime.didSaveShortcuts")
}

