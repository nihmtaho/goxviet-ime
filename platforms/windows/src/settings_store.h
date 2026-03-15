#pragma once
// SettingsStore — Registry persistence ONLY (Single Responsibility Principle).
// Does not know about engine, UI, or business logic.
// Equivalent to UserDefaults I/O in macOS SettingsManager.

#include <windows.h>
#include <string>
#include <vector>

namespace goxviet {

struct ShortcutEntry {
    std::wstring trigger;
    std::wstring replacement;
    bool enabled = true;
};

class SettingsStore {
public:
    static SettingsStore& Instance();

    // Scalar reads — returns defaultValue if key is absent
    bool   ReadBool  (const wchar_t* key, bool   defaultValue = false) const;
    DWORD  ReadDword (const wchar_t* key, DWORD  defaultValue = 0)     const;
    std::wstring ReadString(const wchar_t* key, const wchar_t* defaultValue = L"") const;

    // Scalar writes
    void WriteBool  (const wchar_t* key, bool  value);
    void WriteDword (const wchar_t* key, DWORD value);
    void WriteString(const wchar_t* key, const std::wstring& value);

    // Shortcuts sub-tree
    std::vector<ShortcutEntry> ReadShortcuts() const;
    void WriteShortcuts(const std::vector<ShortcutEntry>& shortcuts);

    // Auto-start in Run key
    void SetAutoStart(bool enabled);

    // Per-app modes (flat: appName → 0/1)
    bool ReadPerApp(const std::wstring& appName, bool defaultValue) const;
    void WritePerApp(const std::wstring& appName, bool enabled);
    void RemovePerApp(const std::wstring& appName);
    std::vector<std::pair<std::wstring, bool>> ReadAllPerApp() const;

private:
    SettingsStore() = default;
    SettingsStore(const SettingsStore&) = delete;
    SettingsStore& operator=(const SettingsStore&) = delete;

    static constexpr const wchar_t* ROOT     = L"Software\\GoxViet";
    static constexpr const wchar_t* RUN_KEY  = L"Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    static constexpr const wchar_t* PERAPP   = L"Software\\GoxViet\\PerApp";

    HKEY OpenRoot(REGSAM access) const;
};

}  // namespace goxviet
