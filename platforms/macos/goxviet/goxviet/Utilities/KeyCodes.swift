//
//  KeyCodes.swift
//  GoxViet
//
//  Shared key code definitions following SOLID principles
//  Single Responsibility: Only key code constants
//

import CoreGraphics

// MARK: - KeyCode Protocol
/// Protocol defining keyboard key code interface
/// Following Interface Segregation Principle - small, focused interface
protocol KeyCodeProtocol {
    static var backspace: CGKeyCode { get }
    static var forwardDelete: CGKeyCode { get }
    static var leftArrow: CGKeyCode { get }
    static var rightArrow: CGKeyCode { get }
    static var downArrow: CGKeyCode { get }
    static var upArrow: CGKeyCode { get }
    static var home: CGKeyCode { get }
    static var end: CGKeyCode { get }
    static var pageUp: CGKeyCode { get }
    static var pageDown: CGKeyCode { get }
    static var space: CGKeyCode { get }
    static var tab: CGKeyCode { get }
    static var returnKey: CGKeyCode { get }
    static var enter: CGKeyCode { get }
    static var esc: CGKeyCode { get }
    static var delete: CGKeyCode { get }
}

// MARK: - KeyCode Implementation
/// Centralized key code definitions
/// Following Single Responsibility Principle - only contains key codes
/// Following Open/Closed Principle - can be extended without modification
public enum KeyCodes {
    // MARK: Navigation Keys
    public static let backspace: CGKeyCode = 0x33
    public static let forwardDelete: CGKeyCode = 0x75
    public static let leftArrow: CGKeyCode = 0x7B
    public static let rightArrow: CGKeyCode = 0x7C
    public static let downArrow: CGKeyCode = 0x7D
    public static let upArrow: CGKeyCode = 0x7E
    public static let home: CGKeyCode = 0x73
    public static let end: CGKeyCode = 0x77
    public static let pageUp: CGKeyCode = 0x74
    public static let pageDown: CGKeyCode = 0x79
    public static let space: CGKeyCode = 0x31
    public static let tab: CGKeyCode = 0x30
    public static let returnKey: CGKeyCode = 0x24
    public static let enter: CGKeyCode = 0x4C
    public static let esc: CGKeyCode = 0x35
    public static let delete: CGKeyCode = 0x33

    // MARK: Punctuation Keys
    public static let dot: CGKeyCode = 0x2F
    public static let comma: CGKeyCode = 0x2B
    public static let slash: CGKeyCode = 0x2C
    public static let semicolon: CGKeyCode = 0x29
    public static let quote: CGKeyCode = 0x27
    public static let lbracket: CGKeyCode = 0x21
    public static let rbracket: CGKeyCode = 0x1E
    public static let backslash: CGKeyCode = 0x2A
    public static let minus: CGKeyCode = 0x1B
    public static let equal: CGKeyCode = 0x18
    public static let backquote: CGKeyCode = 0x32

    // MARK: Number Keys (shifted = !@#$%^&*())
    public static let n0: CGKeyCode = 0x1D
    public static let n1: CGKeyCode = 0x12
    public static let n2: CGKeyCode = 0x13
    public static let n3: CGKeyCode = 0x14
    public static let n4: CGKeyCode = 0x15
    public static let n5: CGKeyCode = 0x17
    public static let n6: CGKeyCode = 0x16
    public static let n7: CGKeyCode = 0x1A
    public static let n8: CGKeyCode = 0x1C
    public static let n9: CGKeyCode = 0x19

    // MARK: Letter Keys
    public static let a: CGKeyCode = 0x00
    public static let b: CGKeyCode = 0x0B
    public static let c: CGKeyCode = 0x08
    public static let d: CGKeyCode = 0x02
    public static let e: CGKeyCode = 0x0E
    public static let f: CGKeyCode = 0x03
    public static let g: CGKeyCode = 0x05
    public static let h: CGKeyCode = 0x04
    public static let i: CGKeyCode = 0x22
    public static let j: CGKeyCode = 0x26
    public static let k: CGKeyCode = 0x28
    public static let l: CGKeyCode = 0x25
    public static let m: CGKeyCode = 0x2E
    public static let n: CGKeyCode = 0x2D
    public static let o: CGKeyCode = 0x1F
    public static let p: CGKeyCode = 0x23
    public static let q: CGKeyCode = 0x0C
    public static let r: CGKeyCode = 0x0F
    public static let s: CGKeyCode = 0x01
    public static let t: CGKeyCode = 0x11
    public static let u: CGKeyCode = 0x20
    public static let v: CGKeyCode = 0x09
    public static let w: CGKeyCode = 0x0D
    public static let x: CGKeyCode = 0x07
    public static let y: CGKeyCode = 0x10
    public static let z: CGKeyCode = 0x06

    // MARK: Modifier Keys
    public static let leftShift: CGKeyCode = 0x38
    public static let rightShift: CGKeyCode = 0x3C
    public static let leftCommand: CGKeyCode = 0x37
    public static let rightCommand: CGKeyCode = 0x36
    public static let leftOption: CGKeyCode = 0x3A
    public static let rightOption: CGKeyCode = 0x3D
    public static let leftControl: CGKeyCode = 0x3B
    public static let rightControl: CGKeyCode = 0x3E
    public static let capsLock: CGKeyCode = 0x39
    public static let fn: CGKeyCode = 0x3F

    // MARK: Function Keys
    public static let f1: CGKeyCode = 0x7A
    public static let f2: CGKeyCode = 0x78
    public static let f3: CGKeyCode = 0x63
    public static let f4: CGKeyCode = 0x76
    public static let f5: CGKeyCode = 0x60
    public static let f6: CGKeyCode = 0x61
    public static let f7: CGKeyCode = 0x62
    public static let f8: CGKeyCode = 0x64
    public static let f9: CGKeyCode = 0x65
    public static let f10: CGKeyCode = 0x6D
    public static let f11: CGKeyCode = 0x67
    public static let f12: CGKeyCode = 0x6F
    public static let f13: CGKeyCode = 0x69
    public static let f14: CGKeyCode = 0x6B
    public static let f15: CGKeyCode = 0x71
    public static let f16: CGKeyCode = 0x6A
    public static let f17: CGKeyCode = 0x40
    public static let f18: CGKeyCode = 0x4F
    public static let f19: CGKeyCode = 0x50
    public static let f20: CGKeyCode = 0x5A
}

// MARK: - KeyCode Collections
/// Pre-computed key code sets for efficient lookups
/// Following Performance Optimization - O(1) lookups
public enum KeyCodeSets {
    /// All navigation keys
    public static let navigation: Set<CGKeyCode> = [
        KeyCodes.leftArrow, KeyCodes.rightArrow, KeyCodes.upArrow, KeyCodes.downArrow,
        KeyCodes.home, KeyCodes.end, KeyCodes.pageUp, KeyCodes.pageDown
    ]

    /// All punctuation keys
    public static let punctuation: Set<CGKeyCode> = [
        KeyCodes.dot, KeyCodes.comma, KeyCodes.slash, KeyCodes.semicolon,
        KeyCodes.quote, KeyCodes.lbracket, KeyCodes.rbracket, KeyCodes.backslash,
        KeyCodes.minus, KeyCodes.equal, KeyCodes.backquote
    ]

    /// All number keys
    public static let numbers: Set<CGKeyCode> = [
        KeyCodes.n0, KeyCodes.n1, KeyCodes.n2, KeyCodes.n3, KeyCodes.n4,
        KeyCodes.n5, KeyCodes.n6, KeyCodes.n7, KeyCodes.n8, KeyCodes.n9
    ]

    /// All break keys (keys that should reset/break input)
    public static let breakKeys: Set<CGKeyCode> = Set([
        KeyCodes.space, KeyCodes.tab, KeyCodes.returnKey, KeyCodes.enter, KeyCodes.esc
    ]).union(navigation).union(punctuation)

    /// All modifier keys
    public static let modifiers: Set<CGKeyCode> = [
        KeyCodes.leftShift, KeyCodes.rightShift, KeyCodes.leftCommand, KeyCodes.rightCommand,
        KeyCodes.leftOption, KeyCodes.rightOption, KeyCodes.leftControl, KeyCodes.rightControl,
        KeyCodes.capsLock, KeyCodes.fn
    ]
}

// MARK: - KeyCode Extensions
public extension CGKeyCode {
    /// Check if this key code is a break key
    @inline(__always)
    var isBreakKey: Bool {
        return KeyCodeSets.breakKeys.contains(self)
    }

    /// Check if this key code is a punctuation key
    @inline(__always)
    var isPunctuation: Bool {
        return KeyCodeSets.punctuation.contains(self)
    }

    /// Check if this key code is a navigation key
    @inline(__always)
    var isNavigation: Bool {
        return KeyCodeSets.navigation.contains(self)
    }

    /// Check if this key code is a modifier key
    @inline(__always)
    var isModifier: Bool {
        return KeyCodeSets.modifiers.contains(self)
    }

    /// Check if this key code is a number key
    @inline(__always)
    var isNumber: Bool {
        return KeyCodeSets.numbers.contains(self)
    }
}
