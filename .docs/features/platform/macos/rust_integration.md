# Rust Core Integration (FFI v2)

Describes the connection between the Swift macOS layer and the Rust core library via the **v2 FFI API**.

> **v1 API removed.** Old functions (`ime_init`, `ime_key`, `ime_key_ext`, `ime_free`, `ime_method`, etc.) no longer exist. See [lib.md](../../core-engine/lib.md) for migration reference.

---

## Bridging Header

`goxviet-Bridging-Header.h` imports the compiled static library (`libgoxviet_core.a`). The actual Swift FFI declarations live in `Core/RustBridgeV2.swift` using `@_silgen_name` — no Objective-C bridging header entries are needed for v2.

---

## Swift FFI Type Declarations (`Core/RustBridgeV2.swift`)

### Why Structs, Not Enums

Swift enums store a 1-byte discriminator, not the `Int32` raw value. Using `enum` for `FfiInputMethod` would make `FfiConfig_v2` 3 bytes instead of the 12 bytes Rust expects via `#[repr(C)]`. **Always use `struct` wrappers for FFI integer types.**

```swift
// ✅ Correct — struct wrapping Int32 matches Rust repr(C)
struct FfiInputMethod: Equatable {
    let rawValue: Int32
    static let telex = FfiInputMethod(rawValue: 0)
    static let vni   = FfiInputMethod(rawValue: 1)
}

// ❌ Wrong — Swift enum would break the ABI
// enum FfiInputMethod: Int32 { case telex = 0, vni = 1 }
```

### Key Types

```swift
struct FfiConfig_v2 {
    var input_method: FfiInputMethod        // Telex=0, VNI=1
    var tone_style: FfiToneStyle            // Traditional=0, Modern=1
    var smart_mode: Bool
    var instant_restore_enabled: Bool
    var esc_restore_enabled: Bool
    var enable_shortcuts: Bool
}

struct FfiProcessResult_v2 {
    var text: UnsafeMutablePointer<CChar>?  // UTF-8, must free
    var backspace_count: UInt8
    var consumed: Bool
}
```

### Function Bindings

```swift
@_silgen_name("ime_create_engine_v2")
func ime_create_engine_v2(_ config: UnsafePointer<FfiConfig_v2>?) -> FfiEnginePtr?

@_silgen_name("ime_destroy_engine_v2")
func ime_destroy_engine_v2(_ engine: FfiEnginePtr?)

@_silgen_name("ime_process_key_v2")
func ime_process_key_v2(_ engine: FfiEnginePtr?, _ key: CChar,
                         _ out: UnsafeMutablePointer<FfiProcessResult_v2>) -> Int32

@_silgen_name("ime_set_config_v2")
func ime_set_config_v2(_ engine: FfiEnginePtr?,
                        _ config: UnsafePointer<FfiConfig_v2>) -> Int32

@_silgen_name("ime_free_string_v2")
func ime_free_string_v2(_ s: UnsafeMutablePointer<CChar>?)
```

---

## Swift Wrapper (`Core/RustEngineV2.swift`)

`RustEngineV2` is a thread-safe Swift class that owns the engine pointer and exposes a clean Swift API. **No other Swift file should call FFI functions directly.**

### Lifecycle

```swift
class RustEngineV2 {
    private var enginePtr: FfiEnginePtr?
    private let lock = NSLock()

    init(config: FfiConfig_v2 = FfiConfig_v2.defaults) {
        var cfg = config
        enginePtr = ime_create_engine_v2(&cfg)
    }

    deinit {
        ime_destroy_engine_v2(enginePtr)
    }
}
```

### Processing a Key

```swift
struct ProcessResult {
    let text: String?
    let backspaceCount: Int
    let consumed: Bool
}

func processKey(_ char: Character) -> ProcessResult {
    lock.lock()
    defer { lock.unlock() }

    guard let engine = enginePtr,
          let ascii = char.asciiValue else {
        return ProcessResult(text: nil, backspaceCount: 0, consumed: false)
    }

    var result = FfiProcessResult_v2()
    let status = ime_process_key_v2(engine, CChar(bitPattern: ascii), &result)

    defer { ime_free_string_v2(result.text) }  // CRITICAL: always free

    guard status == FfiStatusCode.success.rawValue, result.consumed else {
        return ProcessResult(text: nil, backspaceCount: 0, consumed: false)
    }

    let text = result.text.map { String(cString: $0) }
    return ProcessResult(text: text, backspaceCount: Int(result.backspaceCount), consumed: true)
}
```

### Applying Config

```swift
func applyConfig(_ config: FfiConfig_v2) {
    lock.lock()
    defer { lock.unlock() }
    var cfg = config
    _ = ime_set_config_v2(enginePtr, &cfg)
}
```

---

## Memory Contract

| Action | Rule |
|---|---|
| Engine creation | `ime_create_engine_v2` returns an opaque pointer. Owner must call `ime_destroy_engine_v2`. |
| Key processing | `ime_process_key_v2` writes `result.text` (may be null). **Always** call `ime_free_string_v2(result.text)` after reading the string. Use `defer`. |
| Config | `FfiConfig_v2` is passed by pointer (read-only). Swift owns the struct on the stack. |
| Shortcuts | Trigger/replacement `const char *` strings are copied by Rust. Swift strings can be released immediately. |

---

## Initialization Sequence

```
AppDelegate.applicationDidFinishLaunching
    └── InputManager.shared.start()
            └── ime_init_v2()                    # calls ime_create_engine_v2 internally
                    └── loads saved config from SettingsManager
                    └── syncs shortcuts to engine (ime_add_shortcut_v2 per entry)
```

`ime_init_v2()` is a Swift-level helper in `RustEngineV2` that wraps engine creation + initial config application.

---

## Building the Static Library

```bash
# Universal binary (arm64 + x86_64) for Xcode
./scripts/rust_build_lib_universal_for_macos.sh
# Output: platforms/macos/goxviet/libgoxviet_core.a
```

The library is linked as a static library in the Xcode project. The bridging header must be set in Build Settings → `SWIFT_OBJC_BRIDGING_HEADER`.
