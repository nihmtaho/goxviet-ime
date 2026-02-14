# GoxViet Core - FFI API Reference

**Version:** 1.0.0  
**Last Updated:** 2026-02-11  
**Language:** C-compatible API for platform integration

---

## Overview

This document describes the C FFI (Foreign Function Interface) API for the GoxViet Vietnamese IME core engine. The API is designed to be:

- **Simple**: Minimal API surface, easy to integrate
- **Safe**: No panics, proper error handling
- **Backward Compatible**: Works with existing client code
- **Cross-Platform**: Works on macOS, Windows, Linux

---

## Table of Contents

1. [Types](#types)
2. [Engine Lifecycle](#engine-lifecycle)
3. [Configuration](#configuration)
4. [Text Processing](#text-processing)
5. [Memory Management](#memory-management)
6. [Error Handling](#error-handling)
7. [Examples](#examples)

---

## Types

### FfiResult

Operation result type.

```c
typedef struct {
    bool success;      // true if operation succeeded
    int32_t error_code; // 0 on success, error code otherwise
} FfiResult;
```

**Error Codes:**
- `0`: Success
- `1`: Invalid input
- `2`: Invalid handle
- `3`: Invalid UTF-8
- `4`: Invalid action
- `5`: Processing error

---

### FfiInputMethod

Input method selection.

```c
typedef enum {
    Telex = 0,  // Telex input method (default)
    Vni = 1,    // VNI input method
} FfiInputMethod;
```

---

### FfiToneStyle

Tone placement style.

```c
typedef enum {
    Old = 0,  // Old style: hòa, thủy
    New = 1,  // New style: hoà, thuỷ (default)
} FfiToneStyle;
```

---

### FfiConfig

Engine configuration.

```c
typedef struct {
    FfiInputMethod input_method;  // Input method to use
    FfiToneStyle tone_style;      // Tone placement style
    bool smart_mode;              // Enable smart Vietnamese/English detection
    bool enable_shortcuts;        // Enable text expansion shortcuts
} FfiConfig;
```

---

### FfiProcessResult

Result from keystroke processing.

```c
typedef struct {
    char* text;              // Output text (UTF-8, null-terminated)
                             // MUST be freed with ime_free_string()
    int32_t backspace_count; // Number of characters to delete before insert
    bool consumed;           // true if input was consumed by engine
    FfiResult result;        // Operation result
} FfiProcessResult;
```

---

### FfiEngineHandle

Opaque engine handle.

```c
typedef void* FfiEngineHandle;
```

**Lifecycle:**
1. Create with `ime_engine_new()` or `ime_engine_new_with_config()`
2. Use with process/config functions
3. Free with `ime_engine_free()`

---

## Engine Lifecycle

### ime_engine_new

Create new engine with default configuration.

```c
FfiEngineHandle ime_engine_new(void);
```

**Returns:** Engine handle (non-null on success)

**Example:**
```c
FfiEngineHandle engine = ime_engine_new();
if (engine == NULL) {
    // Failed to create engine
    return;
}

// Use engine...

ime_engine_free(engine);
```

---

### ime_engine_new_with_config

Create new engine with custom configuration.

```c
FfiEngineHandle ime_engine_new_with_config(FfiConfig config);
```

**Parameters:**
- `config`: Engine configuration

**Returns:** Engine handle (non-null on success)

**Example:**
```c
FfiConfig config = {
    .input_method = Vni,
    .tone_style = Old,
    .smart_mode = true,
    .enable_shortcuts = true
};

FfiEngineHandle engine = ime_engine_new_with_config(config);
// Use engine...
ime_engine_free(engine);
```

---

### ime_engine_free

Free engine handle and associated resources.

```c
void ime_engine_free(FfiEngineHandle handle);
```

**Parameters:**
- `handle`: Engine handle to free

**Safety:**
- Can be called with NULL handle (no-op)
- Must only be called once per handle
- After this call, handle is invalid

**Example:**
```c
FfiEngineHandle engine = ime_engine_new();
// Use engine...
ime_engine_free(engine);
// engine is now invalid, don't use!
```

---

## Configuration

### ime_get_config

Get current engine configuration.

```c
FfiConfig ime_get_config(FfiEngineHandle handle);
```

**Parameters:**
- `handle`: Engine handle

**Returns:** Current configuration (default config if handle is NULL)

**Example:**
```c
FfiEngineHandle engine = ime_engine_new();
FfiConfig config = ime_get_config(engine);

printf("Input method: %d\n", config.input_method);
printf("Smart mode: %s\n", config.smart_mode ? "ON" : "OFF");

ime_engine_free(engine);
```

---

### ime_set_config

Update engine configuration.

```c
FfiResult ime_set_config(FfiEngineHandle handle, FfiConfig config);
```

**Parameters:**
- `handle`: Engine handle
- `config`: New configuration

**Returns:** Result (success=true on success)

**Example:**
```c
FfiEngineHandle engine = ime_engine_new();

// Switch to VNI input method
FfiConfig config = ime_get_config(engine);
config.input_method = Vni;

FfiResult result = ime_set_config(engine, config);
if (result.success) {
    printf("Config updated successfully\n");
} else {
    printf("Failed to update config: %d\n", result.error_code);
}

ime_engine_free(engine);
```

---

## Text Processing

### ime_process_key

Process a keystroke.

```c
FfiProcessResult ime_process_key(
    FfiEngineHandle handle,
    const char* key_char,
    int32_t action
);
```

**Parameters:**
- `handle`: Engine handle
- `key_char`: Input character (UTF-8, null-terminated)
- `action`: Key action
  - `0`: Text input (normal key press)
  - `1`: Backspace
  - `2`: Commit (finalize current input)

**Returns:** Processing result

**Important:** Caller MUST free `result.text` using `ime_free_string()`

**Example: Normal input**
```c
FfiEngineHandle engine = ime_engine_new();

// User types 'v', 'i', 'e', 't'
const char* keys[] = {"v", "i", "e", "t"};

for (int i = 0; i < 4; i++) {
    FfiProcessResult result = ime_process_key(engine, keys[i], 0);
    
    if (result.consumed && result.text != NULL) {
        // Delete backspace_count characters
        for (int j = 0; j < result.backspace_count; j++) {
            send_backspace();
        }
        
        // Insert result text
        insert_text(result.text);
        
        // Free string
        ime_free_string(result.text);
    }
}

ime_engine_free(engine);
```

**Example: Vietnamese transformation**
```c
// Type "viet" with tone mark
const char* sequence[] = {"v", "i", "e", "t", "s"}; // 's' = sắc tone in Telex

for (int i = 0; i < 5; i++) {
    FfiProcessResult result = ime_process_key(engine, sequence[i], 0);
    
    if (result.consumed && result.text != NULL) {
        // On final 's', result will be:
        // - text: "viết"
        // - backspace_count: 4 (delete "viet")
        // - consumed: true
        
        // Delete previous text
        for (int j = 0; j < result.backspace_count; j++) {
            send_backspace();
        }
        
        // Insert transformed text
        insert_text(result.text); // "viết"
        
        ime_free_string(result.text);
    }
}
```

**Example: Backspace**
```c
// User presses backspace
FfiProcessResult result = ime_process_key(engine, "", 1);

if (result.consumed) {
    // Engine handled backspace (e.g., undoing transformation)
    if (result.text != NULL) {
        // Replace with result.text
        for (int i = 0; i < result.backspace_count; i++) {
            send_backspace();
        }
        insert_text(result.text);
        ime_free_string(result.text);
    }
} else {
    // Engine didn't handle it, let OS handle backspace
    send_backspace();
}
```

---

## Memory Management

### ime_free_string

Free a string returned by the IME API.

```c
void ime_free_string(char* ptr);
```

**Parameters:**
- `ptr`: String pointer to free (can be NULL)

**Safety:**
- Can be called with NULL (no-op)
- Must only be called once per pointer
- Only for strings returned by IME API

**Example:**
```c
FfiProcessResult result = ime_process_key(engine, "a", 0);

if (result.text != NULL) {
    printf("Output: %s\n", result.text);
    ime_free_string(result.text); // MUST free!
}
```

**⚠️ Memory Leak Warning:**
```c
// ❌ WRONG - Memory leak!
FfiProcessResult result = ime_process_key(engine, "a", 0);
// Forgot to free result.text

// ✅ CORRECT
FfiProcessResult result = ime_process_key(engine, "a", 0);
if (result.text != NULL) {
    use_text(result.text);
    ime_free_string(result.text); // Always free!
}
```

---

### ime_get_version

Get engine version string.

```c
char* ime_get_version(void);
```

**Returns:** Version string (e.g., "1.0.0"). Caller MUST free with `ime_free_string()`.

**Example:**
```c
char* version = ime_get_version();
printf("Engine version: %s\n", version);
ime_free_string(version);
```

---

## Error Handling

### Error Codes

| Code | Name | Description |
|------|------|-------------|
| 0 | Success | Operation completed successfully |
| 1 | Invalid Input | Input string contains invalid UTF-8 or null bytes |
| 2 | Invalid Handle | Engine handle is NULL or corrupted |
| 3 | Invalid UTF-8 | Input is not valid UTF-8 |
| 4 | Invalid Action | Action code is not 0, 1, or 2 |
| 5 | Processing Error | Internal processing error |

### Checking Results

```c
FfiProcessResult result = ime_process_key(engine, "a", 0);

if (!result.result.success) {
    switch (result.result.error_code) {
        case 2:
            fprintf(stderr, "Invalid engine handle\n");
            break;
        case 3:
            fprintf(stderr, "Invalid UTF-8 input\n");
            break;
        case 4:
            fprintf(stderr, "Invalid action code\n");
            break;
        case 5:
            fprintf(stderr, "Processing error\n");
            break;
        default:
            fprintf(stderr, "Unknown error: %d\n", result.result.error_code);
    }
    return;
}

// Success - use result
if (result.consumed && result.text != NULL) {
    use_text(result.text);
    ime_free_string(result.text);
}
```

---

## Examples

### Complete Integration Example

```c
#include <stdio.h>
#include <stdbool.h>

// Forward declarations (from your platform integration)
void send_backspace(void);
void insert_text(const char* text);

int main() {
    // 1. Initialize engine
    FfiConfig config = {
        .input_method = Telex,
        .tone_style = New,
        .smart_mode = true,
        .enable_shortcuts = true
    };
    
    FfiEngineHandle engine = ime_engine_new_with_config(config);
    if (engine == NULL) {
        fprintf(stderr, "Failed to create engine\n");
        return 1;
    }
    
    // 2. Process keystroke sequence: "viet" + "s" (tone mark)
    const char* sequence[] = {"v", "i", "e", "t", "s"};
    
    for (int i = 0; i < 5; i++) {
        FfiProcessResult result = ime_process_key(engine, sequence[i], 0);
        
        if (!result.result.success) {
            fprintf(stderr, "Processing error: %d\n", result.result.error_code);
            continue;
        }
        
        if (result.consumed && result.text != NULL) {
            // Delete previous characters
            for (int j = 0; j < result.backspace_count; j++) {
                send_backspace();
            }
            
            // Insert new text
            insert_text(result.text);
            printf("Output: %s (deleted %d chars)\n", 
                   result.text, result.backspace_count);
            
            // Clean up
            ime_free_string(result.text);
        } else if (result.consumed) {
            // Input consumed but no output
            printf("Input consumed (no output)\n");
        } else {
            // Input not consumed - pass through to OS
            printf("Pass-through: %s\n", sequence[i]);
            insert_text(sequence[i]);
        }
    }
    
    // 3. Clean up
    ime_engine_free(engine);
    
    return 0;
}
```

**Expected Output:**
```
Pass-through: v
Pass-through: i
Pass-through: e
Pass-through: t
Output: viết (deleted 4 chars)
```

---

### Swift Integration (macOS)

```swift
import Foundation

class VietnameseIME {
    private var engineHandle: FfiEngineHandle?
    
    init() {
        engineHandle = ime_engine_new()
    }
    
    deinit {
        if let handle = engineHandle {
            ime_engine_free(handle)
        }
    }
    
    func processKey(_ char: Character, action: Int32 = 0) -> String? {
        guard let handle = engineHandle else { return nil }
        
        let keyString = String(char)
        let cString = keyString.cString(using: .utf8)!
        
        let result = cString.withUnsafeBufferPointer { ptr in
            ime_process_key(handle, ptr.baseAddress, action)
        }
        
        defer {
            if result.text != nil {
                ime_free_string(result.text)
            }
        }
        
        guard result.result.success else {
            print("Error: \(result.result.error_code)")
            return nil
        }
        
        if result.consumed, let text = result.text {
            return String(cString: text)
        }
        
        return nil
    }
}

// Usage
let ime = VietnameseIME()
let output = ime.processKey("s", action: 0)
if let text = output {
    print("Output: \(text)")
}
```

---

### C# Integration (Windows)

```csharp
using System;
using System.Runtime.InteropServices;
using System.Text;

public class VietnameseIME : IDisposable
{
    private IntPtr engineHandle;
    
    [DllImport("goxviet_core.dll", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr ime_engine_new();
    
    [DllImport("goxviet_core.dll", CallingConvention = CallingConvention.Cdecl)]
    private static extern void ime_engine_free(IntPtr handle);
    
    [DllImport("goxviet_core.dll", CallingConvention = CallingConvention.Cdecl)]
    private static extern FfiProcessResult ime_process_key(
        IntPtr handle, 
        [MarshalAs(UnmanagedType.LPStr)] string key, 
        int action
    );
    
    [DllImport("goxviet_core.dll", CallingConvention = CallingConvention.Cdecl)]
    private static extern void ime_free_string(IntPtr ptr);
    
    public VietnameseIME()
    {
        engineHandle = ime_engine_new();
    }
    
    public string? ProcessKey(char key, int action = 0)
    {
        var result = ime_process_key(engineHandle, key.ToString(), action);
        
        try
        {
            if (!result.result.success)
            {
                Console.WriteLine($"Error: {result.result.error_code}");
                return null;
            }
            
            if (result.consumed && result.text != IntPtr.Zero)
            {
                return Marshal.PtrToStringUTF8(result.text);
            }
            
            return null;
        }
        finally
        {
            if (result.text != IntPtr.Zero)
            {
                ime_free_string(result.text);
            }
        }
    }
    
    public void Dispose()
    {
        if (engineHandle != IntPtr.Zero)
        {
            ime_engine_free(engineHandle);
            engineHandle = IntPtr.Zero;
        }
    }
}

// Usage
using var ime = new VietnameseIME();
var output = ime.ProcessKey('s');
if (output != null)
{
    Console.WriteLine($"Output: {output}");
}
```

---

## Thread Safety

The engine is **NOT thread-safe** by default. If you need to use the same engine from multiple threads:

1. **Option 1:** Use one engine per thread
2. **Option 2:** Synchronize access with mutex/lock

**Example (C with pthread):**
```c
pthread_mutex_t engine_mutex = PTHREAD_MUTEX_INITIALIZER;
FfiEngineHandle engine;

void process_key_threadsafe(const char* key) {
    pthread_mutex_lock(&engine_mutex);
    FfiProcessResult result = ime_process_key(engine, key, 0);
    // Use result...
    if (result.text != NULL) {
        ime_free_string(result.text);
    }
    pthread_mutex_unlock(&engine_mutex);
}
```

---

## Performance Considerations

- **Engine Creation**: ~1ms (do once at startup)
- **Keystroke Processing**: <1ms per keystroke (target)
- **Memory**: ~1MB per engine instance
- **Concurrency**: Create separate engines for better parallelism

**Best Practices:**
- Reuse engine instances (don't create/destroy per keystroke)
- Process keystrokes on main/input thread (avoid context switching)
- Free strings immediately after use (avoid accumulating memory)

---

## Troubleshooting

### Engine returns NULL

**Cause:** Memory allocation failed  
**Solution:** Check available memory, reduce concurrent engine instances

### Processing always returns error code 2

**Cause:** Invalid engine handle (NULL or freed)  
**Solution:** Check that engine was created successfully and not already freed

### Memory leak

**Cause:** Not freeing `result.text`  
**Solution:** Always call `ime_free_string()` on non-NULL text pointers

### Incorrect Vietnamese output

**Cause:** Wrong input method or tone style  
**Solution:** Check `ime_get_config()` and update with `ime_set_config()`

---

## References

- [Architecture Documentation](./ARCHITECTURE.md)
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [Vietnamese Input Methods](./vietnamese-logic.md)

---

## Support

For issues, questions, or contributions:
- GitHub Issues: https://github.com/goxviet/goxviet/issues
- Documentation: https://goxviet.github.io/docs

---

**Last Updated:** 2026-02-11  
**Version:** 1.0.0  
**License:** MIT
