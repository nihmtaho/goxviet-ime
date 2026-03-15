#include "rust_bridge.h"
#include <shlwapi.h>
#pragma comment(lib, "shlwapi.lib")

namespace goxviet {

RustBridge& RustBridge::Instance() {
    static RustBridge instance;
    return instance;
}

bool RustBridge::Load(const wchar_t* dllPath) {
    if (hDll_) return true;

    if (dllPath) {
        hDll_ = LoadLibraryW(dllPath);
    } else {
        wchar_t exeDir[MAX_PATH];
        GetModuleFileNameW(nullptr, exeDir, MAX_PATH);
        PathRemoveFileSpecW(exeDir);

        wchar_t fullPath[MAX_PATH];
        PathCombineW(fullPath, exeDir, L"goxviet_core.dll");
        hDll_ = LoadLibraryW(fullPath);

        if (!hDll_) hDll_ = LoadLibraryW(L"goxviet_core.dll");
    }

    if (!hDll_) return false;

#define LOAD_REQUIRED(type, member, name) \
    member = reinterpret_cast<type>(GetProcAddress(hDll_, name)); \
    if (!member) { Unload(); return false; }

    LOAD_REQUIRED(PfnCreate,       pfnCreate_,       "ime_create_engine_v2")
    LOAD_REQUIRED(PfnDestroy,      pfnDestroy_,      "ime_destroy_engine_v2")
    LOAD_REQUIRED(PfnProcessKey,   pfnProcessKey_,   "ime_process_key_v2")
    LOAD_REQUIRED(PfnFreeString,   pfnFreeString_,   "ime_free_string_v2")
    LOAD_REQUIRED(PfnGetConfig,    pfnGetConfig_,    "ime_get_config_v2")
    LOAD_REQUIRED(PfnSetConfig,    pfnSetConfig_,    "ime_set_config_v2")
    LOAD_REQUIRED(PfnResetBuffer,  pfnResetBuffer_,  "ime_reset_buffer_v2")
    LOAD_REQUIRED(PfnResetAll,     pfnResetAll_,     "ime_reset_all_v2")
    LOAD_REQUIRED(PfnRestoreToRaw, pfnRestoreToRaw_, "ime_restore_to_raw_v2")
    LOAD_REQUIRED(PfnGetVersion,   pfnGetVersion_,   "ime_get_version_v2")
#undef LOAD_REQUIRED

    // Optional: extended key processing (preferred when available)
    pfnProcessKeyExt_ = reinterpret_cast<PfnProcessKeyExt>(
        GetProcAddress(hDll_, "ime_process_key_ext_v2"));

    // Optional shortcut functions
    pfnAddShortcut_ = reinterpret_cast<PfnAddShortcut>(
        GetProcAddress(hDll_, "ime_add_shortcut_v2"));
    pfnClearShortcuts_ = reinterpret_cast<PfnClearShortcuts>(
        GetProcAddress(hDll_, "ime_clear_shortcuts_v2"));
    pfnSetShortcutsEnabled_ = reinterpret_cast<PfnSetShortcutsEnabled>(
        GetProcAddress(hDll_, "ime_set_shortcuts_enabled_v2"));

    return true;
}

void RustBridge::Unload() {
    pfnCreate_ = nullptr; pfnDestroy_ = nullptr;
    pfnProcessKey_ = nullptr; pfnProcessKeyExt_ = nullptr;
    pfnFreeString_ = nullptr; pfnGetConfig_ = nullptr;
    pfnSetConfig_ = nullptr; pfnResetBuffer_ = nullptr;
    pfnResetAll_ = nullptr; pfnRestoreToRaw_ = nullptr;
    pfnGetVersion_ = nullptr; pfnAddShortcut_ = nullptr;
    if (hDll_) { FreeLibrary(hDll_); hDll_ = nullptr; }
}

void* RustBridge::CreateEngine(const FfiConfig_v2* config) {
    return pfnCreate_ ? pfnCreate_(config) : nullptr;
}

void RustBridge::DestroyEngine(void* engine) {
    if (pfnDestroy_ && engine) pfnDestroy_(engine);
}

FfiStatusCode RustBridge::ProcessKeyExt(void* engine, int8_t keyChar,
                                         bool caps, bool shift, bool ctrl,
                                         FfiProcessResult_v2* result) {
    if (!engine || !result) return FfiStatusCode::NullPointer;
    if (pfnProcessKeyExt_) {
        return static_cast<FfiStatusCode>(
            pfnProcessKeyExt_(engine, keyChar,
                              caps  ? 1u : 0u,
                              shift ? 1u : 0u,
                              ctrl  ? 1u : 0u,
                              result));
    }
    // Fall back to basic single-arg variant
    if (pfnProcessKey_) {
        return static_cast<FfiStatusCode>(pfnProcessKey_(engine, keyChar, result));
    }
    return FfiStatusCode::NotInitialized;
}

void RustBridge::FreeString(const char* str) {
    if (pfnFreeString_ && str) pfnFreeString_(str);
}

FfiStatusCode RustBridge::GetConfig(void* engine, FfiConfig_v2* config) {
    if (!engine || !config) return FfiStatusCode::NullPointer;
    return static_cast<FfiStatusCode>(pfnGetConfig_(engine, config));
}

FfiStatusCode RustBridge::SetConfig(void* engine, const FfiConfig_v2* config) {
    if (!engine || !config) return FfiStatusCode::NullPointer;
    return static_cast<FfiStatusCode>(pfnSetConfig_(engine, config));
}

FfiStatusCode RustBridge::ResetBuffer(void* engine) {
    if (!engine) return FfiStatusCode::NullPointer;
    return static_cast<FfiStatusCode>(pfnResetBuffer_(engine));
}

FfiStatusCode RustBridge::ResetAll(void* engine) {
    if (!engine) return FfiStatusCode::NullPointer;
    return static_cast<FfiStatusCode>(pfnResetAll_(engine));
}

FfiStatusCode RustBridge::RestoreToRaw(void* engine, FfiProcessResult_v2* result) {
    if (!engine || !result) return FfiStatusCode::NullPointer;
    return static_cast<FfiStatusCode>(pfnRestoreToRaw_(engine, result));
}

FfiStatusCode RustBridge::GetVersion(FfiVersionInfo* info) {
    if (!info) return FfiStatusCode::NullPointer;
    return static_cast<FfiStatusCode>(pfnGetVersion_(info));
}

bool RustBridge::AddShortcut(void* engine, const char* trigger, const char* replacement) {
    if (!pfnAddShortcut_ || !engine || !trigger || !replacement) return false;
    return pfnAddShortcut_(engine, trigger, replacement) == 0;
}

bool RustBridge::ClearShortcuts(void* engine) {
    if (!pfnClearShortcuts_ || !engine) return false;
    return pfnClearShortcuts_(engine) == 0;
}

bool RustBridge::SetShortcutsEnabled(void* engine, bool enabled) {
    if (!pfnSetShortcutsEnabled_ || !engine) return false;
    return pfnSetShortcutsEnabled_(engine, enabled ? 1u : 0u) == 0;
}

std::string RustBridge::Utf16ToUtf8(const std::wstring& wstr) {
    if (wstr.empty()) return {};
    int size = WideCharToMultiByte(CP_UTF8, 0, wstr.c_str(), -1, nullptr, 0, nullptr, nullptr);
    if (size <= 1) return {};
    std::string result(size - 1, '\0');
    WideCharToMultiByte(CP_UTF8, 0, wstr.c_str(), -1, result.data(), size, nullptr, nullptr);
    return result;
}

std::wstring RustBridge::Utf8ToUtf16(const char* utf8) {
    if (!utf8 || !*utf8) return {};
    int size = MultiByteToWideChar(CP_UTF8, 0, utf8, -1, nullptr, 0);
    if (size <= 1) return {};
    std::wstring result(size - 1, L'\0');
    MultiByteToWideChar(CP_UTF8, 0, utf8, -1, result.data(), size);
    return result;
}

}  // namespace goxviet
