// P/Invoke declarations for the Rust core engine.
// RULE: This is the ONLY file in the project that may contain [DllImport].
// All other code must go through RustBridge.

using System.Runtime.InteropServices;

namespace GoxViet.FFI;

internal static class NativeMethods
{
    private const string DllName = "goxviet_core";

    /// Create engine with optional config (NULL = use defaults).
    /// Caller must call ime_destroy_engine_v2 when done.
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr ime_create_engine_v2(IntPtr config);

    /// Destroy engine and free all resources.
    /// Safe to call with NULL.
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern void ime_destroy_engine_v2(IntPtr engine);

    /// Process a key event. Returns FfiStatusCode as int.
    /// keyChar: ASCII char (a-z, 0-9, backspace=0x08, space=0x20, ESC=0x1B)
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern int ime_process_key_v2(
        IntPtr engine,
        sbyte keyChar,
        ref FfiProcessResult_v2 result);

    /// Extended process key with modifier state.
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern int ime_process_key_ext_v2(
        IntPtr engine,
        sbyte keyChar,
        [MarshalAs(UnmanagedType.U1)] bool caps,
        [MarshalAs(UnmanagedType.U1)] bool shift,
        [MarshalAs(UnmanagedType.U1)] bool ctrl,
        ref FfiProcessResult_v2 result);

    /// Free a string allocated by the Rust engine.
    /// MUST be called for every non-null Text pointer from FfiProcessResult_v2.
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern void ime_free_string_v2(IntPtr str);

    /// Get current engine config.
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern int ime_get_config_v2(IntPtr engine, ref FfiConfig_v2 config);

    /// Update engine config.
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern int ime_set_config_v2(IntPtr engine, ref FfiConfig_v2 config);

    /// Reset the current word buffer (e.g. on focus change within a word).
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern int ime_reset_buffer_v2(IntPtr engine);

    /// Reset all engine state (buffer + shortcuts + any pending state).
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern int ime_reset_all_v2(IntPtr engine);

    /// Restore current buffer to raw (un-transformed) keystrokes.
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern int ime_restore_to_raw_v2(IntPtr engine, ref FfiProcessResult_v2 result);

    /// Get engine version information.
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern int ime_get_version_v2(ref FfiVersionInfo info);
}
