#include "settings.h"
#include "settings_store.h"
#include "notifications.h"
#include "utils.h"
#include <fstream>
#include <sstream>
#include <algorithm>

namespace goxviet {

Settings& Settings::Instance() {
    static Settings instance;
    return instance;
}

// ---- Initialize (like SettingsManager::initialize() on macOS) ---------------

void Settings::Initialize() {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (initialised_) return;
    initialised_ = true;

    auto& store = SettingsStore::Instance();

    // First-launch detection — register defaults if key is absent
    bool firstLaunch = !store.ReadBool(L"HasLaunchedBefore", false);

    if (!firstLaunch) {
        Load();
    } else {
        // First launch: keep compiled-in defaults, persist them
        store.WriteBool (L"HasLaunchedBefore",  true);
        store.WriteBool (L"Enabled",             enabled);
        store.WriteDword(L"Method",              method);
        store.WriteBool (L"ModernTone",          modernTone);
        store.WriteBool (L"SmartMode",           smartMode);
        store.WriteBool (L"InstantRestore",      instantRestore);
        store.WriteBool (L"EscRestore",          escRestore);
        store.WriteBool (L"FreeTone",            freeTone);
        store.WriteBool (L"EnableShortcuts",     enableShortcuts);
        store.WriteBool (L"ShiftBackspace",      shiftBackspace);
        store.WriteBool (L"AutoCapitalize",      autoCapitalize);
        store.WriteBool (L"AutoDisableNonLatin", autoDisableNonLatin);
        store.WriteDword(L"OutputEncoding",      static_cast<DWORD>(outputEncoding));
        store.WriteBool (L"AutoStart",           autoStart);
        store.WriteBool (L"Sound",               sound);
        store.WriteBool (L"PerApp",              perApp);
        restoreShortcut.Save();
        LogInfo(L"First launch — defaults saved");
    }

    // Always load shortcuts (and seed defaults if empty)
    auto stored = store.ReadShortcuts();
    if (stored.empty()) {
        LoadDefaultShortcuts();
    } else {
        shortcuts.clear();
        for (auto& s : stored)
            shortcuts.push_back({ s.trigger, s.replacement, s.enabled });
    }
}

void Settings::Load() {
    auto& store = SettingsStore::Instance();
    enabled             = store.ReadBool (L"Enabled",             true);
    method              = static_cast<uint8_t>(store.ReadDword(L"Method", 0));
    modernTone          = store.ReadBool (L"ModernTone",          false);
    smartMode           = store.ReadBool (L"SmartMode",           true);
    instantRestore      = store.ReadBool (L"InstantRestore",      true);
    escRestore          = store.ReadBool (L"EscRestore",          false);
    freeTone            = store.ReadBool (L"FreeTone",            false);
    enableShortcuts     = store.ReadBool (L"EnableShortcuts",     true);
    shiftBackspace      = store.ReadBool (L"ShiftBackspace",      false);
    autoCapitalize      = store.ReadBool (L"AutoCapitalize",      false);
    autoDisableNonLatin = store.ReadBool (L"AutoDisableNonLatin", true);
    outputEncoding      = static_cast<OutputEncoding>(store.ReadDword(L"OutputEncoding", 0));
    autoStart           = store.ReadBool (L"AutoStart",           false);
    sound               = store.ReadBool (L"Sound",               true);
    perApp              = store.ReadBool (L"PerApp",              false);
    restoreShortcut     = RestoreShortcut::Load();
}

void Settings::LoadDefaultShortcuts() {
    // Default shortcuts matching macOS defaults
    static const struct { const wchar_t* t; const wchar_t* r; } kDefaults[] = {
        { L"vn",  L"Việt Nam" },
        { L"hcm", L"Hồ Chí Minh" },
        { L"hn",  L"Hà Nội" },
        { L"dc",  L"được" },
        { L"ko",  L"không" },
    };
    shortcuts.clear();
    for (auto& d : kDefaults)
        shortcuts.push_back({ d.t, d.r, true });

    std::vector<ShortcutEntry> entries;
    for (auto& s : shortcuts) entries.push_back({ s.trigger, s.replacement, s.enabled });
    SettingsStore::Instance().WriteShortcuts(entries);
    LogInfo(L"Default shortcuts loaded");
}

// ---- Save -------------------------------------------------------------------

void Settings::Save() {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    auto& store = SettingsStore::Instance();
    store.WriteBool (L"Enabled",             enabled);
    store.WriteDword(L"Method",              method);
    store.WriteBool (L"ModernTone",          modernTone);
    store.WriteBool (L"SmartMode",           smartMode);
    store.WriteBool (L"InstantRestore",      instantRestore);
    store.WriteBool (L"EscRestore",          escRestore);
    store.WriteBool (L"FreeTone",            freeTone);
    store.WriteBool (L"EnableShortcuts",     enableShortcuts);
    store.WriteBool (L"ShiftBackspace",      shiftBackspace);
    store.WriteBool (L"AutoCapitalize",      autoCapitalize);
    store.WriteBool (L"AutoDisableNonLatin", autoDisableNonLatin);
    store.WriteDword(L"OutputEncoding",      static_cast<DWORD>(outputEncoding));
    store.WriteBool (L"AutoStart",           autoStart);
    store.WriteBool (L"Sound",               sound);
    store.WriteBool (L"PerApp",              perApp);
    store.SetAutoStart(autoStart);
    restoreShortcut.Save();

    std::vector<ShortcutEntry> entries;
    for (auto& s : shortcuts) entries.push_back({ s.trigger, s.replacement, s.enabled });
    store.WriteShortcuts(entries);
}

// ---- Engine sync ------------------------------------------------------------

FfiConfig_v2 Settings::ToFfiConfig() const {
    FfiConfig_v2 cfg{};
    cfg.input_method     = (method == 1) ? FfiInputMethod::Vni : FfiInputMethod::Telex;
    cfg.tone_style       = modernTone      ? FfiToneStyle::New : FfiToneStyle::Old;
    cfg.smart_mode       = smartMode       ? 1u : 0u;
    cfg.instant_restore  = instantRestore  ? 1u : 0u;
    cfg.esc_restore      = escRestore      ? 1u : 0u;
    cfg.enable_shortcuts = enableShortcuts ? 1u : 0u;
    return cfg;
}

void Settings::ApplyToEngine(void* engine) {
    if (!engine) return;
    FfiConfig_v2 cfg = ToFfiConfig();
    RustBridge::Instance().SetConfig(engine, &cfg);
}

void Settings::SyncShortcutsToEngine(void* engine) {
    if (!engine) return;
    auto& bridge = RustBridge::Instance();
    bridge.ClearShortcuts(engine);
    for (const auto& sc : shortcuts) {
        if (!sc.enabled) continue;
        auto t = RustBridge::Utf16ToUtf8(sc.trigger);
        auto r = RustBridge::Utf16ToUtf8(sc.replacement);
        bridge.AddShortcut(engine, t.c_str(), r.c_str());
    }
    bridge.SetShortcutsEnabled(engine, enableShortcuts);
}

// ---- Setters ----------------------------------------------------------------

void Settings::PostEvent(AppEvent ev) {
    EventBus::Instance().Post(ev);
}

void Settings::SetEnabled(bool v, bool silent) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (v == enabled) return;
    enabled = v;
    SettingsStore::Instance().WriteBool(L"Enabled", v);
    if (!silent) PostEvent(AppEvent::EnabledChanged);
}

void Settings::SetMethod(uint8_t v) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (v == method) return;
    method = v;
    SettingsStore::Instance().WriteDword(L"Method", v);
    PostEvent(AppEvent::InputMethodChanged);
}

void Settings::SetModernTone(bool v) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (v == modernTone) return;
    modernTone = v;
    SettingsStore::Instance().WriteBool(L"ModernTone", v);
    PostEvent(AppEvent::ToneStyleChanged);
}

void Settings::SetSmartMode(bool v) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (v == smartMode) return;
    smartMode = v;
    SettingsStore::Instance().WriteBool(L"SmartMode", v);
    PostEvent(AppEvent::SmartModeChanged);
}

void Settings::SetInstantRestore(bool v) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (v == instantRestore) return;
    instantRestore = v;
    SettingsStore::Instance().WriteBool(L"InstantRestore", v);
    PostEvent(AppEvent::InstantRestoreChanged);
}

void Settings::SetEscRestore(bool v) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (v == escRestore) return;
    escRestore = v;
    SettingsStore::Instance().WriteBool(L"EscRestore", v);
    PostEvent(AppEvent::EscRestoreChanged);
}

void Settings::SetFreeTone(bool v) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (v == freeTone) return;
    freeTone = v;
    SettingsStore::Instance().WriteBool(L"FreeTone", v);
    PostEvent(AppEvent::FreeToneChanged);
}

void Settings::SetShiftBackspace(bool v) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (v == shiftBackspace) return;
    shiftBackspace = v;
    SettingsStore::Instance().WriteBool(L"ShiftBackspace", v);
    PostEvent(AppEvent::ShiftBackspaceChanged);
}

void Settings::SetEnableShortcuts(bool v) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (v == enableShortcuts) return;
    enableShortcuts = v;
    SettingsStore::Instance().WriteBool(L"EnableShortcuts", v);
    PostEvent(AppEvent::ShortcutsChanged);
}

void Settings::SetAutoDisableNonLatin(bool v) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (v == autoDisableNonLatin) return;
    autoDisableNonLatin = v;
    SettingsStore::Instance().WriteBool(L"AutoDisableNonLatin", v);
}

void Settings::SetOutputEncoding(OutputEncoding v) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (v == outputEncoding) return;
    outputEncoding = v;
    SettingsStore::Instance().WriteDword(L"OutputEncoding", static_cast<DWORD>(v));
    PostEvent(AppEvent::OutputEncodingChanged);
}

void Settings::SetAutoStart(bool v) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    autoStart = v;
    SettingsStore::Instance().SetAutoStart(v);
}

void Settings::SetRestoreShortcut(const RestoreShortcut& rs) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    restoreShortcut = rs;
    restoreShortcut.Save();
}

// ---- Shortcuts management ---------------------------------------------------

bool Settings::AddShortcut(const std::wstring& trigger, const std::wstring& replacement) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    auto t = trigger; std::wstring trimmed = t;
    // Check for duplicate
    for (auto& sc : shortcuts)
        if (sc.trigger == trimmed) return false;
    shortcuts.push_back({ trimmed, replacement, true });
    std::vector<ShortcutEntry> entries;
    for (auto& s : shortcuts) entries.push_back({ s.trigger, s.replacement, s.enabled });
    SettingsStore::Instance().WriteShortcuts(entries);
    PostEvent(AppEvent::ShortcutsChanged);
    return true;
}

bool Settings::UpdateShortcut(const std::wstring& oldTrigger, const std::wstring& newTrigger,
                               const std::wstring& replacement) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    shortcuts.erase(std::remove_if(shortcuts.begin(), shortcuts.end(),
        [&](const Shortcut& s){ return s.trigger == oldTrigger; }), shortcuts.end());
    shortcuts.push_back({ newTrigger, replacement, true });
    std::vector<ShortcutEntry> entries;
    for (auto& s : shortcuts) entries.push_back({ s.trigger, s.replacement, s.enabled });
    SettingsStore::Instance().WriteShortcuts(entries);
    PostEvent(AppEvent::ShortcutsChanged);
    return true;
}

void Settings::RemoveShortcut(const std::wstring& trigger) {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    shortcuts.erase(std::remove_if(shortcuts.begin(), shortcuts.end(),
        [&](const Shortcut& s){ return s.trigger == trigger; }), shortcuts.end());
    std::vector<ShortcutEntry> entries;
    for (auto& s : shortcuts) entries.push_back({ s.trigger, s.replacement, s.enabled });
    SettingsStore::Instance().WriteShortcuts(entries);
    PostEvent(AppEvent::ShortcutsChanged);
}

// trigger:replacement format (macOS-compatible, ';' for comment lines)
int Settings::ImportShortcuts(const wchar_t* path) {
    std::wifstream f(path);
    if (!f.is_open()) return 0;
    f.imbue(std::locale(""));
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    int count = 0;
    std::wstring line;
    while (std::getline(f, line)) {
        if (line.empty() || line[0] == L';') continue;
        auto colon = line.find(L':');
        if (colon == std::wstring::npos) continue;
        std::wstring t = line.substr(0, colon);
        std::wstring r = line.substr(colon + 1);
        if (t.empty()) continue;
        auto it = std::find_if(shortcuts.begin(), shortcuts.end(),
                               [&](const Shortcut& s){ return s.trigger == t; });
        if (it != shortcuts.end()) { it->replacement = r; it->enabled = true; }
        else shortcuts.push_back({ t, r, true });
        ++count;
    }
    std::vector<ShortcutEntry> entries;
    for (auto& s : shortcuts) entries.push_back({ s.trigger, s.replacement, s.enabled });
    SettingsStore::Instance().WriteShortcuts(entries);
    PostEvent(AppEvent::ShortcutsChanged);
    return count;
}

bool Settings::ExportShortcuts(const wchar_t* path) const {
    std::wofstream f(path);
    if (!f.is_open()) return false;
    f << L";Gõ Việt - Bảng gõ tắt\n";
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    for (const auto& s : shortcuts)
        if (!s.trigger.empty()) f << s.trigger << L":" << s.replacement << L"\n";
    return true;
}

void Settings::ResetToDefaults() {
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    enabled = true; method = 0; modernTone = false;
    smartMode = true; instantRestore = true; escRestore = false;
    freeTone = false; enableShortcuts = true; shiftBackspace = false;
    autoCapitalize = false; autoDisableNonLatin = true;
    outputEncoding = OutputEncoding::Unicode;
    autoStart = false; sound = true; perApp = false;
    restoreShortcut = RestoreShortcut::Default();
    shortcuts.clear();
    LoadDefaultShortcuts();
    Save();
    PostEvent(AppEvent::EnabledChanged);
}

// ---- Per-app delegation -----------------------------------------------------

bool Settings::GetPerApp(const std::wstring& app) const {
    return SettingsStore::Instance().ReadPerApp(app, enabled);
}

void Settings::SetPerApp(const std::wstring& app, bool en) {
    SettingsStore::Instance().WritePerApp(app, en);
    PostEvent(AppEvent::PerAppModesChanged);
}

void Settings::RemovePerApp(const std::wstring& app) {
    SettingsStore::Instance().RemovePerApp(app);
    PostEvent(AppEvent::PerAppModesChanged);
}

}  // namespace goxviet
