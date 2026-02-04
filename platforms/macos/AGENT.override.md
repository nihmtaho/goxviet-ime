# AGENT OVERRIDE: PLATFORM macOS (SWIFT)

## Context
You are working on the **macOS Platform Layer** of GoxViet. This uses Swift (AppKit/SwiftUI) to interface with the operating system and the Rust Core Engine.

## Rules & Standards

### 1. UI & Concurrency
- **Main Thread:** All UI updates MUST happen on the Main Actor (`DispatchQueue.main` or `@MainActor`).
- **Blocking calls:** Engine calls are synchronous. Keep them extremely fast.

### 2. Memory Management (FFI)
- **Explicit Freeing:** Any pointer returned from Rust FFI (`ImeResult*`) MUST be freed using `ime_free`.
    - Use `defer { ime_free(...) }` immediately after receiving the pointer.

### 3. Architecture
- **InputManager:** The heart of the event tap. Modifications here are high-risk.
- **RustBridge:** The only place where raw FFI calls should occur.

### 4. Design
- **Native Polish:** Use standard macOS UI patterns (Settings window, Menu bar).
- **Glassmorphism:** Use modern sidebar styles in Settings.

### 5. Documentation
- Update `.docs/features/platform/macos/` when changing platform features.
