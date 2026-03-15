// FFI Type Definitions
// Mirrors core/src/presentation/ffi/types.rs #[repr(C)] types.
// CRITICAL: bool fields use [MarshalAs(UnmanagedType.U1)] — Rust bool is 1 byte,
// C# bool is 4 bytes by default. Mismatch causes silent data corruption.

using System.Runtime.InteropServices;

namespace GoxViet.FFI;

public enum FfiInputMethod : int
{
    Telex = 0,
    Vni   = 1,
}

public enum FfiToneStyle : int
{
    Old = 0,
    New = 1,
}

public enum FfiStatusCode : int
{
    Success               =   0,
    ErrorNullEngine       =  -1,
    ErrorNullOutput       =  -2,
    ErrorNullConfig       =  -3,
    ErrorInvalidKey       =  -4,
    ErrorInvalidArgument  =  -5,
    ErrorProcessingFailed = -10,
    ErrorInvalidUtf8      = -11,
    ErrorParseError       = -12,
    ErrorOutOfMemory      = -20,
    ErrorAlreadyExists    = -30,
    ErrorNotFound         = -31,
    ErrorUnknown          = -98,
    ErrorPanic            = -99,
}

/// Mirrors FfiConfig_v2 from types.rs.
/// Layout: input_method(4) + tone_style(4) + 4×bool(1 each) = 12 bytes total.
[StructLayout(LayoutKind.Sequential)]
public struct FfiConfig_v2
{
    public FfiInputMethod InputMethod;
    public FfiToneStyle   ToneStyle;
    [MarshalAs(UnmanagedType.U1)] public bool SmartMode;
    [MarshalAs(UnmanagedType.U1)] public bool InstantRestoreEnabled;
    [MarshalAs(UnmanagedType.U1)] public bool EscRestoreEnabled;
    [MarshalAs(UnmanagedType.U1)] public bool EnableShortcuts;
}

/// Mirrors FfiProcessResult_v2 from types.rs.
/// text is an IntPtr (not string) — must be freed via ime_free_string_v2 immediately after use.
[StructLayout(LayoutKind.Sequential)]
public struct FfiProcessResult_v2
{
    public IntPtr Text;           // char* UTF-8, caller must free with ime_free_string_v2
    public byte   BackspaceCount;
    [MarshalAs(UnmanagedType.U1)] public bool Consumed;
}

/// Mirrors FfiVersionInfo from types.rs.
[StructLayout(LayoutKind.Sequential)]
public struct FfiVersionInfo
{
    public uint Major;
    public uint Minor;
    public uint Patch;
    public uint ApiVersion;
}
