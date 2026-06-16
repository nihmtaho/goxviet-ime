#pragma once
#include <windows.h>
#include <cstdint>
#include <string>

namespace goxviet {

// ---- FFI types: must exactly match Rust repr(C) structs in goxviet_core ----

enum class FfiInputMethod : int32_t {
    Telex = 0,
    Vni   = 1,
};

enum class FfiToneStyle : int32_t {
    Old = 0,   // Traditional (Truyền thống)
    New = 1,   // Modern (Hiện đại)
};

enum class FfiStatusCode : int32_t {
    Success        = 0,
    InvalidHandle  = 1,
    InvalidConfig  = 2,
    BufferTooSmall = 3,
    NullPointer    = 4,
    NotInitialized = 5,
    InternalError  = 6,
};

#pragma pack(push, 1)
struct FfiConfig_v2 {
    FfiInputMethod input_method;      // int32, 4 bytes
    FfiToneStyle   tone_style;        // int32, 4 bytes
    uint8_t        smart_mode;        // bool, 1 byte
    uint8_t        instant_restore;   // bool, 1 byte
    uint8_t        esc_restore;       // bool, 1 byte
    uint8_t        enable_shortcuts;  // bool, 1 byte
};  // 12 bytes total

struct FfiProcessResult_v2 {
    const char* text;            // UTF-8, nullptr if no output
    uint8_t     backspace_count;
    uint8_t     consumed;        // bool
};

struct FfiVersionInfo {
    uint32_t major;
    uint32_t minor;
    uint32_t patch;
    uint32_t api_version;
};
#pragma pack(pop)

// Loads goxviet_core.dll dynamically and exposes the v2 API
class RustBridge {
public:
    static RustBridge& Instance();

    // Load DLL. Pass nullptr to search next to the executable (then PATH).
    bool Load(const wchar_t* dllPath = nullptr);
    void Unload();
    bool IsLoaded() const { return hDll_ != nullptr; }

    // Engine lifecycle
    void* CreateEngine(const FfiConfig_v2* config);
    void  DestroyEngine(void* engine);

    // Key processing (prefers the ext variant; falls back to basic)
    FfiStatusCode ProcessKeyExt(void* engine, int8_t keyChar,
                                bool caps, bool shift, bool ctrl,
                                FfiProcessResult_v2* result);

    // String management — caller must invoke FreeString after consuming text
    void FreeString(const char* str);

    // Config
    FfiStatusCode GetConfig(void* engine, FfiConfig_v2* config);
    FfiStatusCode SetConfig(void* engine, const FfiConfig_v2* config);

    // Buffer management
    FfiStatusCode ResetBuffer(void* engine);
    FfiStatusCode ResetAll(void* engine);
    FfiStatusCode RestoreToRaw(void* engine, FfiProcessResult_v2* result);

    // Version query
    FfiStatusCode GetVersion(FfiVersionInfo* info);

    // Shortcuts
    bool AddShortcut       (void* engine, const char* trigger, const char* replacement);
    bool ClearShortcuts    (void* engine);   // ime_clear_shortcuts_v2
    bool SetShortcutsEnabled(void* engine, bool enabled);  // ime_set_shortcuts_enabled_v2

    // String conversion helpers
    static std::string  Utf16ToUtf8(const std::wstring& wstr);
    static std::wstring Utf8ToUtf16(const char* utf8);

private:
    RustBridge() = default;
    ~RustBridge() { Unload(); }
    RustBridge(const RustBridge&) = delete;
    RustBridge& operator=(const RustBridge&) = delete;

    HMODULE hDll_ = nullptr;

    using PfnCreate        = void*   (__cdecl*)(const FfiConfig_v2*);
    using PfnDestroy       = void    (__cdecl*)(void*);
    using PfnProcessKey    = int32_t (__cdecl*)(void*, int8_t, FfiProcessResult_v2*);
    using PfnProcessKeyExt = int32_t (__cdecl*)(void*, int8_t, uint8_t, uint8_t, uint8_t, FfiProcessResult_v2*);
    using PfnFreeString    = void    (__cdecl*)(const char*);
    using PfnGetConfig     = int32_t (__cdecl*)(void*, FfiConfig_v2*);
    using PfnSetConfig     = int32_t (__cdecl*)(void*, const FfiConfig_v2*);
    using PfnResetBuffer   = int32_t (__cdecl*)(void*);
    using PfnResetAll      = int32_t (__cdecl*)(void*);
    using PfnRestoreToRaw  = int32_t (__cdecl*)(void*, FfiProcessResult_v2*);
    using PfnGetVersion    = int32_t (__cdecl*)(FfiVersionInfo*);
    using PfnAddShortcut   = int32_t (__cdecl*)(void*, const char*, const char*);

    PfnCreate        pfnCreate_        = nullptr;
    PfnDestroy       pfnDestroy_       = nullptr;
    PfnProcessKey    pfnProcessKey_    = nullptr;
    PfnProcessKeyExt pfnProcessKeyExt_ = nullptr;  // optional ext variant
    PfnFreeString    pfnFreeString_    = nullptr;
    PfnGetConfig     pfnGetConfig_     = nullptr;
    PfnSetConfig     pfnSetConfig_     = nullptr;
    PfnResetBuffer   pfnResetBuffer_   = nullptr;
    PfnResetAll      pfnResetAll_      = nullptr;
    PfnRestoreToRaw  pfnRestoreToRaw_  = nullptr;
    PfnGetVersion    pfnGetVersion_    = nullptr;
    PfnAddShortcut   pfnAddShortcut_         = nullptr;  // optional

    using PfnClearShortcuts      = int32_t (__cdecl*)(void*);
    using PfnSetShortcutsEnabled = int32_t (__cdecl*)(void*, uint8_t);
    PfnClearShortcuts      pfnClearShortcuts_      = nullptr;  // optional
    PfnSetShortcutsEnabled pfnSetShortcutsEnabled_ = nullptr;  // optional
};

}  // namespace goxviet
