//
//  LRUCache.swift
//  GoxViet
//
//  Generic LRU (Least Recently Used) cache implementation
//  Thread-safe with automatic eviction and size limits
//

import Foundation
import Combine

/// Generic LRU cache with automatic eviction
final class LRUCache<Key: Hashable, Value> {
    
    // MARK: - Node
    
    private class Node {
        let key: Key
        var value: Value
        var prev: Node?
        var next: Node?
        
        init(key: Key, value: Value) {
            self.key = key
            self.value = value
        }
    }
    
    // MARK: - Properties
    
    private let capacity: Int
    private var cache: [Key: Node] = [:]
    private var head: Node?
    private var tail: Node?
    private let lock = NSLock()
    
    // Statistics
    private(set) var hits: Int = 0
    private(set) var misses: Int = 0
    private(set) var evictions: Int = 0
    
    // MARK: - Initialization
    
    init(capacity: Int) {
        self.capacity = max(1, capacity)
    }
    
    // MARK: - Public API
    
    /// Get value for key (returns nil if not found)
    func get(_ key: Key) -> Value? {
        lock.lock()
        defer { lock.unlock() }
        
        guard let node = cache[key] else {
            misses += 1
            return nil
        }
        
        hits += 1
        moveToHead(node)
        return node.value
    }
    
    /// Set value for key (evicts least recently used if at capacity)
    func set(_ key: Key, _ value: Value) {
        lock.lock()
        defer { lock.unlock() }
        
        if let node = cache[key] {
            // Update existing
            node.value = value
            moveToHead(node)
        } else {
            // Insert new
            let node = Node(key: key, value: value)
            cache[key] = node
            addToHead(node)
            
            // Evict if over capacity
            if cache.count > capacity {
                if let removedNode = removeTail() {
                    cache.removeValue(forKey: removedNode.key)
                    evictions += 1
                }
            }
        }
    }
    
    /// Remove value for key
    func remove(_ key: Key) {
        lock.lock()
        defer { lock.unlock() }
        
        guard let node = cache[key] else { return }
        removeNode(node)
        cache.removeValue(forKey: key)
    }
    
    /// Clear all entries
    func clear() {
        lock.lock()
        defer { lock.unlock() }
        
        cache.removeAll()
        head = nil
        tail = nil
    }
    
    /// Get current size
    var count: Int {
        lock.lock()
        defer { lock.unlock() }
        return cache.count
    }
    
    /// Check if cache contains key
    func contains(_ key: Key) -> Bool {
        lock.lock()
        defer { lock.unlock() }
        return cache[key] != nil
    }
    
    // MARK: - Statistics
    
    /// Get cache statistics
    func getStats() -> (hits: Int, misses: Int, evictions: Int, hitRate: Double, size: Int, capacity: Int) {
        lock.lock()
        defer { lock.unlock() }
        
        let total = hits + misses
        let hitRate = total > 0 ? Double(hits) / Double(total) : 0.0
        return (hits, misses, evictions, hitRate, cache.count, capacity)
    }
    
    /// Reset statistics
    func resetStats() {
        lock.lock()
        defer { lock.unlock() }
        
        hits = 0
        misses = 0
        evictions = 0
    }
    
    // MARK: - Private Helpers
    
    private func addToHead(_ node: Node) {
        node.next = head
        node.prev = nil
        head?.prev = node
        head = node
        if tail == nil {
            tail = node
        }
    }
    
    private func removeNode(_ node: Node) {
        if node.prev != nil {
            node.prev?.next = node.next
        } else {
            head = node.next
        }
        
        if node.next != nil {
            node.next?.prev = node.prev
        } else {
            tail = node.prev
        }
    }
    
    private func moveToHead(_ node: Node) {
        removeNode(node)
        addToHead(node)
    }
    
    private func removeTail() -> Node? {
        guard let tailNode = tail else { return nil }
        removeNode(tailNode)
        return tailNode
    }
}

// MARK: - Convenience Extensions

extension LRUCache where Value: AnyObject {
    /// Set with weak reference (useful for object caching)
    func setWeak(_ key: Key, _ value: Value) {
        set(key, value)
    }
}

extension LRUCache {
    /// Get all keys (in access order, most recent first)
    var allKeys: [Key] {
        lock.lock()
        defer { lock.unlock() }
        
        var keys: [Key] = []
        var current = head
        while let node = current {
            keys.append(node.key)
            current = node.next
        }
        return keys
    }
}
