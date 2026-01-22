# Rust Core Integration (FFI)

The connection between the macOS platform (Swift) and the Core Engine (Rust) handles the crossing of the Foreign Function Interface boundary.

## Bridging Header (`goxviet-Bridging-Header.h`)

Exposes standard C signatures for the Rust functions.

```c
// Key processing
ImeResult *ime_key_ext(uint16_t key, bool caps, bool ctrl, bool shift);
void ime_free(ImeResult *result);

// Configuration
void ime_init(void);
void ime_method(uint8_t method); // 0=Telex, 1=VNI
void ime_clear(void);
```

### `ImeResult` Struct
The critical data structure returned by the engine.
-   `chars`: A pointer (`*mut u32`) to a heap-allocated array of UTF-32 codepoints.
-   `backspace`: The number of characters to delete from the current cursor position.
-   `action`: The type of action (`Send`, `Restore`, `None`).

## Rust Bridge Wrapper (`RustBridge.swift`)

A Swift class that wraps the unsafe C calls into a safe, idiomatic API.

### Key Responsibilities

1.  **Initialization**: Calls `ime_init()` once on startup.
2.  **Memory Management**:
    -   The `ime_key` functions return a raw pointer to an `ImeResult`.
    -   Swift **must** call `ime_free(result)` in a `defer` block immediately after receiving the result to prevent memory leaks.
    -   The `chars` buffer inside `ImeResult` is converted to a Swift `String` or `[Character]` array before being freed.

3.  **String Passing**:
    -   Converts Swift `String` to C-strings (`const char*`) when adding shortcuts (`ime_add_shortcut`).
    -   Uses `.withUnsafeBufferPointer` to pass string data safely without copying where possible.

### Example Flow: Processing a Key

```swift
// Swift
let result = ime_key_ext(keyCode, caps, ctrl, shift)
guard let r = result else { return }
defer { ime_free(r) } // CRITICAL: Free memory

if r.pointee.action == 1 {
    // Read data before freeing
    let backspaceCount = Int(r.pointee.backspace)
    let text = extractString(from: r.pointee.chars, length: r.pointee.count)
    // Inject...
}
```
