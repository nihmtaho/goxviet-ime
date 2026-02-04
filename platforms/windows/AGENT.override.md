# AGENT OVERRIDE: PLATFORM WINDOWS (C#/.NET)

## Context
You are working on the **Windows Platform Layer** of GoxViet. This uses C# (.NET) to interface with the operating system (Win32 API) and the Rust Core Engine.

## Rules & Standards

### 1. Interop & Encoding
- **UTF-16:** Windows APIs use UTF-16 (Wide Chars).
    - Convert Rust UTF-8 strings to UTF-16 before passing to Windows APIs.
- **P/Invoke:** Use `DllImport` safely. Match calling conventions (`Cdecl`).

### 2. System Integration
- **Low-Level Hooks:** `SetWindowsHookEx` is used for key interception.
- **Key codes:** Map Virtual Key Codes (VK) correctly to the Engine's expected format.

### 3. Architecture
- **Tray Application:** Runs in background. Minimized to system tray.
- **IPC:** Communication with settings UI (if separate process).

### 4. Stability
- **Exception Handling:** Catch exceptions at the boundary of hooks to prevent bringing down the input system.

### 5. Documentation
- Documentation path: `.docs/features/platform/windows/` (to be created if missing).
