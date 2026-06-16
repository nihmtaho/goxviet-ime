# AGENT OVERRIDE: PLATFORM WINDOWS (C++)

## Context
You are working on the **Windows Platform Layer** of GoxViet. This is a **native C++20 Win32 application** that loads `goxviet_core.dll` at runtime via `LoadLibrary` and uses the v2 FFI API.

## Build System
- **CMake 3.22+** — primary build system (`CMakeLists.txt` at `platforms/windows/`)
- Target: `goxviet.exe` (WIN32 subsystem, static CRT)
- Architectures: x86 (`-A Win32`), x64 (`-A x64`), ARM64 (`-A ARM64`)
- No Corrosion / no static Rust linking — the DLL is loaded dynamically

## Rules & Standards

### 1. Rust Core Integration
- **Dynamic loading only**: `RustBridge::Load()` calls `LoadLibrary` + `GetProcAddress`
- **DLL name**: `goxviet_core.dll` — never reference any other IME core
- **API**: All calls go through `RustBridge` in `src/rust_bridge.h`
- **Struct layout**: FFI structs use `#pragma pack(push, 1)` — must match Rust `repr(C)`
- **String ownership**: always call `RustBridge::FreeString(result.text)` after consuming

### 2. Interop & Encoding
- **UTF-16** on all Windows API surfaces
- **UTF-8** on all FFI boundaries (engine expects/returns UTF-8)
- Use `RustBridge::Utf16ToUtf8` / `RustBridge::Utf8ToUtf16` for conversion

### 3. Keyboard Hook
- `WH_KEYBOARD_LL` via `SetWindowsHookExW` — must run on the main thread's message loop
- Injected events are stamped with `GOXVIET_MARKER` (`0x474F5856`) in `dwExtraInfo`
- Never re-process events that carry the marker
- Reset buffer on Ctrl/Alt, break keys, and foreground app switches

### 4. Architecture
- Tray-only application — no main window, `HWND_MESSAGE` hidden window
- All singletons — `Settings`, `RustBridge`, `KeyboardHook`, `SystemTray`, etc.
- Engine pointer (`g_engine`) lives in `main.cpp`; passed explicitly to subsystems

### 5. Stability
- Reentrancy guard (`processing_`) in `KeyboardHook` to prevent recursive hook calls
- `CRITICAL_SECTION` in `TextInjector` for thread-safe injection

### 6. Documentation
- Documentation path: `.docs/features/platform/windows/`
