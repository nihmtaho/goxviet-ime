#pragma once
// Settings — data model + business logic + observer notifications.
// Persistence is delegated to SettingsStore (SRP).
// Equivalent to SettingsManager.swift on macOS.

#include <string>
#include <vector>
#include <functional>
#include <mutex>
#include "notifications.h"
#include "rust_bridge.h"
#include "restore_shortcut.h"

namespace goxviet {

struct Shortcut {
    std::wstring trigger;
    std::wstring replacement;
    bool enabled = true;
};

enum class OutputEncoding : uint8_t {
    Unicode = 0,
    Tcvn3   = 1,
    Vni     = 2,
    Cp1258  = 3,
};

class Settings {
public:
    static Settings& Instance();

    // ---- Core input ----
    bool           enabled             = true;
    uint8_t        method              = 0;      // 0=Telex, 1=VNI
    bool           modernTone          = false;

    // ---- Behaviour ----
    bool           smartMode           = true;
    bool           instantRestore      = true;   // restore shortcut (multi-tap modifier)
    bool           escRestore          = false;
    bool           freeTone            = false;  // free-tone marking
    bool           enableShortcuts     = true;
    bool           shiftBackspace      = false;  // Shift+Backspace deletes previous word
    bool           autoCapitalize      = false;
    bool           autoDisableNonLatin = true;   // auto-disable for non-Latin keyboards

    // ---- Restore shortcut (double-tap modifier, like macOS double-Option) ----
    RestoreShortcut restoreShortcut;

    // ---- Output ----
    OutputEncoding outputEncoding      = OutputEncoding::Unicode;

    // ---- App settings ----
    bool           autoStart           = false;
    bool           sound               = true;
    bool           perApp              = false;

    // ---- Text shortcuts ----
    std::vector<Shortcut> shortcuts;

    // ---- Lifecycle ----
    void Initialize();   // load + first-launch defaults + observer setup
    void Save();
    void ApplyToEngine(void* engine);
    void SyncShortcutsToEngine(void* engine);

    // ---- Setters (fire EventBus notifications) ----
    void SetEnabled            (bool v, bool silent = false);
    void SetMethod             (uint8_t v);
    void SetModernTone         (bool v);
    void SetSmartMode          (bool v);
    void SetInstantRestore     (bool v);
    void SetEscRestore         (bool v);
    void SetFreeTone           (bool v);
    void SetShiftBackspace     (bool v);
    void SetEnableShortcuts    (bool v);
    void SetAutoDisableNonLatin(bool v);
    void SetOutputEncoding     (OutputEncoding v);
    void SetAutoStart          (bool v);
    void SetRestoreShortcut    (const RestoreShortcut& rs);

    // ---- Shortcuts management ----
    bool AddShortcut   (const std::wstring& trigger, const std::wstring& replacement);
    bool UpdateShortcut(const std::wstring& oldTrigger, const std::wstring& newTrigger,
                        const std::wstring& replacement);
    void RemoveShortcut(const std::wstring& trigger);
    int  ImportShortcuts(const wchar_t* path);   // trigger:replacement per line
    bool ExportShortcuts(const wchar_t* path) const;

    // ---- Conversion ----
    FfiConfig_v2 ToFfiConfig() const;

    // ---- Per-app (delegates to SettingsStore) ----
    bool GetPerApp(const std::wstring& appName) const;
    void SetPerApp(const std::wstring& appName, bool enabled);
    void RemovePerApp(const std::wstring& appName);

    void ResetToDefaults();

private:
    Settings() = default;
    Settings(const Settings&) = delete;
    Settings& operator=(const Settings&) = delete;

    void Load();
    void LoadDefaultShortcuts();
    void PostEvent(AppEvent ev);

    mutable std::recursive_mutex mutex_;
    bool initialised_ = false;
};


}  // namespace goxviet
