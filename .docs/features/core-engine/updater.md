# Auto-Update Logic (`updater/`)

The `updater` module provides platform-independent logic for version comparison and update detection. Actual network requests are handled by the host application (Swift/Kotlin/C#).

## `Version` Struct
Represents a Semantic Version (Major.Minor.Patch).

- **`parse(s: &str) -> Option<Version>`**: Parsons strings like "1.0.0" or "v2.1.3".
- **`compare(&self, other: &Version) -> i32`**: Returns -1, 0, or 1.
- **`has_update(&self, other: &Version) -> bool`**: Returns true if `other` is newer than `self`.

## FFI Interface
Exposes version comparison to foreign languages.

- **`version_compare(v1: *const c_char, v2: *const c_char) -> i32`**
    - Compares two version strings.
    - Returns `-99` on error.
- **`version_has_update(current: *const c_char, latest: *const c_char) -> i32`**
    - Returns `1` if an update is available, `0` otherwise.
