// Thread-safe wrapper around the Rust core engine.
// Mirrors the contract of RustBridgeV2.swift on macOS:
//   - All calls are lock-guarded
//   - ime_free_string_v2 is ALWAYS called in a finally block (never stored as IntPtr)
//   - Engine is initialized once; re-initialize resets state

using System.Diagnostics;
using System.Runtime.InteropServices;

namespace GoxViet.FFI;

public sealed class RustBridge : IDisposable
{
    public static readonly RustBridge Instance = new();

    private IntPtr _engine = IntPtr.Zero;
    private readonly object _lock = new();
    private bool _disposed;

    private RustBridge() { }

    /// Initialize the engine with the given config (or defaults if null).
    public void Initialize(FfiConfig_v2? config = null)
    {
        lock (_lock)
        {
            if (_engine != IntPtr.Zero)
            {
                NativeMethods.ime_destroy_engine_v2(_engine);
                _engine = IntPtr.Zero;
            }

            if (config.HasValue)
            {
                var cfg = config.Value;
                unsafe
                {
                    _engine = NativeMethods.ime_create_engine_v2((IntPtr)(&cfg));
                }
            }
            else
            {
                _engine = NativeMethods.ime_create_engine_v2(IntPtr.Zero);
            }

            if (_engine == IntPtr.Zero)
                throw new InvalidOperationException("ime_create_engine_v2 returned NULL.");

            // Validate struct sizes match Rust layout in debug builds
            Debug.Assert(Marshal.SizeOf<FfiConfig_v2>() == 12,
                $"FfiConfig_v2 size mismatch: got {Marshal.SizeOf<FfiConfig_v2>()} expected 12");
        }
    }

    /// Process a key event. Returns the transformation result.
    public ProcessResult ProcessKey(sbyte asciiChar, bool caps = false, bool shift = false, bool ctrl = false)
    {
        lock (_lock)
        {
            EnsureEngine();
            var raw = new FfiProcessResult_v2();
            int status = NativeMethods.ime_process_key_ext_v2(_engine, asciiChar, caps, shift, ctrl, ref raw);
            return ExtractResult(status, ref raw);
        }
    }

    /// Restore current buffer to raw keystrokes (e.g. on ESC).
    public ProcessResult RestoreToRaw()
    {
        lock (_lock)
        {
            EnsureEngine();
            var raw = new FfiProcessResult_v2();
            int status = NativeMethods.ime_restore_to_raw_v2(_engine, ref raw);
            return ExtractResult(status, ref raw);
        }
    }

    /// Update engine config (e.g. when user changes settings).
    public void SetConfig(FfiConfig_v2 config)
    {
        lock (_lock)
        {
            EnsureEngine();
            int status = NativeMethods.ime_set_config_v2(_engine, ref config);
            if (status != (int)FfiStatusCode.Success)
                throw new InvalidOperationException($"ime_set_config_v2 returned {(FfiStatusCode)status}");
        }
    }

    /// Reset word buffer (on focus change within a word).
    public void ResetBuffer()
    {
        lock (_lock)
        {
            if (_engine == IntPtr.Zero) return;
            NativeMethods.ime_reset_buffer_v2(_engine);
        }
    }

    /// Reset all engine state (on app switch, Ctrl/Alt combos, etc.).
    public void ResetAll()
    {
        lock (_lock)
        {
            if (_engine == IntPtr.Zero) return;
            NativeMethods.ime_reset_all_v2(_engine);
        }
    }

    /// Get engine version info.
    public (uint Major, uint Minor, uint Patch) GetVersion()
    {
        var info = new FfiVersionInfo();
        NativeMethods.ime_get_version_v2(ref info);
        return (info.Major, info.Minor, info.Patch);
    }

    // Convert raw FFI result to managed type, freeing the native string immediately.
    // The finally block ensures the pointer is freed even if an exception is thrown.
    private static ProcessResult ExtractResult(int statusCode, ref FfiProcessResult_v2 raw)
    {
        try
        {
            if (statusCode != (int)FfiStatusCode.Success)
                throw new InvalidOperationException($"FFI error: {(FfiStatusCode)statusCode}");

            string text = raw.Text != IntPtr.Zero
                ? Marshal.PtrToStringUTF8(raw.Text) ?? string.Empty
                : string.Empty;

            return new ProcessResult(text, raw.BackspaceCount, raw.Consumed);
        }
        finally
        {
            // CRITICAL: free even if exception was thrown above
            if (raw.Text != IntPtr.Zero)
            {
                NativeMethods.ime_free_string_v2(raw.Text);
                raw.Text = IntPtr.Zero;
            }
        }
    }

    private void EnsureEngine()
    {
        if (_engine == IntPtr.Zero)
            throw new InvalidOperationException("Engine not initialized. Call Initialize() first.");
    }

    public void Dispose()
    {
        if (_disposed) return;
        _disposed = true;
        lock (_lock)
        {
            if (_engine != IntPtr.Zero)
            {
                NativeMethods.ime_destroy_engine_v2(_engine);
                _engine = IntPtr.Zero;
            }
        }
    }
}

/// Managed representation of a processed keystroke result.
public sealed record ProcessResult(string Text, byte BackspaceCount, bool Consumed);
