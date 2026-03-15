# GoxViet Windows Platform Architecture

## Overview

The Windows platform is a .NET 8 WPF tray application that loads the Rust core
(`goxviet_core.dll`) via P/Invoke and intercepts all keystrokes using `WH_KEYBOARD_LL`.

## Layer Map

| Layer | Location | Responsibility |
|---|---|---|
| **FFI bridge** | `FFI/` | Raw P/Invoke, struct layout, string lifecycle |
| **Input pipeline** | `Input/` | Hook install, VK→ASCII mapping, SendInput injection |
| **Settings** | `Settings/` | JSON persistence, engine config sync |
| **UI** | `UI/` | System tray icon, settings window (WPF) |

## Keystroke Pipeline

```
WH_KEYBOARD_LL callback
  → KeyboardHook.HookCallback()
    → Skip if dwExtraInfo == INJECTED_MARKER (our own injected events)
    → Skip if Ctrl/Alt held (reset engine state, pass through)
    → Skip if IME disabled (reset engine state, pass through)
    → VkMapper.ToAscii(vk)           // VK_* → ASCII sbyte
    → RustBridge.ProcessKey()         // P/Invoke → Rust engine
      → NativeMethods.ime_process_key_ext_v2()
      → Marshal.PtrToStringUTF8() + ime_free_string_v2() in finally
    → if consumed:
        TextInjector.Inject(bs, text) // SendInput: backspace×N + Unicode chars
        return 1                      // suppress original keystroke
```

## Critical Rules

1. `NativeMethods.cs` is the **only** file with `[DllImport]`. All other code uses `RustBridge`.
2. `ime_free_string_v2` **must** be called in a `finally` block after every `ime_process_key*` call — even on error, because Rust may have allocated a partial result.
3. Never store `FfiProcessResult_v2.Text` (IntPtr) beyond the `finally` block. Convert to `string` and free immediately.
4. The hook callback **must** `catch (Exception)` — an unhandled exception silently kills the hook permanently (no error visible to user).
5. All events injected by `TextInjector` carry `dwExtraInfo = INJECTED_MARKER` (`0x564E4945`). The hook checks this to avoid re-processing its own output.
6. On Ctrl/Alt combos, call `RustBridge.ResetAll()` before passing through — same as macOS behavior.
7. `KeyboardHook.Install()` must be called from the WPF UI thread (which pumps the Windows message loop).

## Struct Layout Notes

`FfiConfig_v2` layout (must match Rust `#[repr(C)]`):

| Field | Type | Size | Offset |
|---|---|---|---|
| `InputMethod` | `FfiInputMethod` (i32) | 4 | 0 |
| `ToneStyle` | `FfiToneStyle` (i32) | 4 | 4 |
| `SmartMode` | `bool` (u8) | 1 | 8 |
| `InstantRestoreEnabled` | `bool` (u8) | 1 | 9 |
| `EscRestoreEnabled` | `bool` (u8) | 1 | 10 |
| `EnableShortcuts` | `bool` (u8) | 1 | 11 |
| **Total** | | **12** | |

C# `bool` is 4 bytes by default — `[MarshalAs(UnmanagedType.U1)]` is required on every bool
field to match Rust's 1-byte `bool`.

## DLL Build

```powershell
# From repo root (Windows with MSVC toolchain)
.\scripts\rust_build_dll_for_windows.ps1

# Output: core/target/x86_64-pc-windows-msvc/release/goxviet_core.dll
# The .csproj <None> item copies it to the app output directory automatically.
```

Required Cargo.toml change: `crate-type = ["staticlib", "cdylib", "rlib"]`

## Settings File

`%APPDATA%\GoxViet\settings.json` — equivalent to macOS `UserDefaults` for GoxViet keys.

```json
{
  "IsEnabled": true,
  "InputMethod": 0,
  "ToneStyle": 1,
  "SmartMode": true,
  "InstantRestore": true,
  "EscRestore": false
}
```

## Toggle Hotkey

`Ctrl+Space` registered via `RegisterHotKey` (Win32 API). This fires even when the IME is
disabled — it does not go through the keyboard hook.

## Architecture Decision: WH_KEYBOARD_LL vs TSF

This foundation uses `WH_KEYBOARD_LL` (low-level keyboard hook) rather than TSF (Text Services
Framework). This is consistent with how production Vietnamese IMEs on Windows operate (EVKey,
Unikey, VPSKeys). TSF requires a full COM server and is a separate milestone (v2.2.0).

The WinHookEx approach:
- Works in all Win32, WPF, and Electron apps
- Simpler integration, no COM registration
- Requires `SetWindowsHookEx` from the message-loop thread
- Marked by some AV software (mitigated by `asInvoker` in app.manifest)

TSF will be layered on top in a future release for better compatibility with UWP apps and
improved IME candidate window support.

## CI/CD

See `.github/workflows/windows-ci.yml` — two jobs:
1. **build-rust-dll**: Rust tests + `cargo build --release --target x86_64-pc-windows-msvc`
2. **build-dotnet**: Download DLL artifact + `dotnet build` + artifact upload

Triggered on push/PR to `develop`/`main` when `core/**` or `platforms/windows/**` changes.
Release packaging (ZIP) runs only on version tags (`v*`).
