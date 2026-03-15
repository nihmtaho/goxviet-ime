#include "settings_store.h"

namespace goxviet {

SettingsStore& SettingsStore::Instance() {
    static SettingsStore instance;
    return instance;
}

HKEY SettingsStore::OpenRoot(REGSAM access) const {
    HKEY hKey = nullptr;
    RegCreateKeyExW(HKEY_CURRENT_USER, ROOT, 0, nullptr,
                    REG_OPTION_NON_VOLATILE, access, nullptr, &hKey, nullptr);
    return hKey;
}

bool SettingsStore::ReadBool(const wchar_t* key, bool def) const {
    HKEY hKey = OpenRoot(KEY_READ);
    if (!hKey) return def;
    DWORD v = 0, size = sizeof(v);
    bool ok = RegQueryValueExW(hKey, key, nullptr, nullptr,
                               reinterpret_cast<BYTE*>(&v), &size) == ERROR_SUCCESS;
    RegCloseKey(hKey);
    return ok ? (v != 0) : def;
}

DWORD SettingsStore::ReadDword(const wchar_t* key, DWORD def) const {
    HKEY hKey = OpenRoot(KEY_READ);
    if (!hKey) return def;
    DWORD v = 0, size = sizeof(v);
    bool ok = RegQueryValueExW(hKey, key, nullptr, nullptr,
                               reinterpret_cast<BYTE*>(&v), &size) == ERROR_SUCCESS;
    RegCloseKey(hKey);
    return ok ? v : def;
}

std::wstring SettingsStore::ReadString(const wchar_t* key, const wchar_t* def) const {
    HKEY hKey = OpenRoot(KEY_READ);
    if (!hKey) return def;
    DWORD size = 0;
    if (RegQueryValueExW(hKey, key, nullptr, nullptr, nullptr, &size) != ERROR_SUCCESS) {
        RegCloseKey(hKey); return def;
    }
    std::wstring result(size / sizeof(wchar_t), L'\0');
    RegQueryValueExW(hKey, key, nullptr, nullptr,
                     reinterpret_cast<BYTE*>(result.data()), &size);
    if (!result.empty() && result.back() == L'\0') result.pop_back();
    RegCloseKey(hKey);
    return result;
}

void SettingsStore::WriteBool(const wchar_t* key, bool value) {
    WriteDword(key, value ? 1 : 0);
}

void SettingsStore::WriteDword(const wchar_t* key, DWORD value) {
    HKEY hKey = OpenRoot(KEY_WRITE);
    if (!hKey) return;
    RegSetValueExW(hKey, key, 0, REG_DWORD,
                   reinterpret_cast<const BYTE*>(&value), sizeof(DWORD));
    RegCloseKey(hKey);
}

void SettingsStore::WriteString(const wchar_t* key, const std::wstring& value) {
    HKEY hKey = OpenRoot(KEY_WRITE);
    if (!hKey) return;
    RegSetValueExW(hKey, key, 0, REG_SZ,
                   reinterpret_cast<const BYTE*>(value.c_str()),
                   static_cast<DWORD>((value.size() + 1) * sizeof(wchar_t)));
    RegCloseKey(hKey);
}

std::vector<ShortcutEntry> SettingsStore::ReadShortcuts() const {
    std::vector<ShortcutEntry> result;
    HKEY hRoot = OpenRoot(KEY_READ);
    if (!hRoot) return result;

    HKEY hSub = nullptr;
    if (RegOpenKeyExW(hRoot, L"Shortcuts", 0, KEY_READ, &hSub) == ERROR_SUCCESS) {
        wchar_t name[64]; DWORD index = 0, nameLen;
        while (true) {
            nameLen = 64;
            if (RegEnumKeyW(hSub, index++, name, nameLen) != ERROR_SUCCESS) break;
            HKEY hEntry;
            if (RegOpenKeyExW(hSub, name, 0, KEY_READ, &hEntry) == ERROR_SUCCESS) {
                ShortcutEntry e;
                // trigger
                DWORD sz = 0;
                if (RegQueryValueExW(hEntry, L"Trigger", nullptr, nullptr, nullptr, &sz) == ERROR_SUCCESS) {
                    e.trigger.resize(sz / sizeof(wchar_t));
                    RegQueryValueExW(hEntry, L"Trigger", nullptr, nullptr,
                                     reinterpret_cast<BYTE*>(e.trigger.data()), &sz);
                    if (!e.trigger.empty() && e.trigger.back() == L'\0') e.trigger.pop_back();
                }
                // replacement
                sz = 0;
                if (RegQueryValueExW(hEntry, L"Replacement", nullptr, nullptr, nullptr, &sz) == ERROR_SUCCESS) {
                    e.replacement.resize(sz / sizeof(wchar_t));
                    RegQueryValueExW(hEntry, L"Replacement", nullptr, nullptr,
                                     reinterpret_cast<BYTE*>(e.replacement.data()), &sz);
                    if (!e.replacement.empty() && e.replacement.back() == L'\0') e.replacement.pop_back();
                }
                DWORD en = 1, ensz = sizeof(en);
                RegQueryValueExW(hEntry, L"Enabled", nullptr, nullptr,
                                 reinterpret_cast<BYTE*>(&en), &ensz);
                e.enabled = (en != 0);
                if (!e.trigger.empty()) result.push_back(e);
                RegCloseKey(hEntry);
            }
        }
        RegCloseKey(hSub);
    }
    RegCloseKey(hRoot);
    return result;
}

void SettingsStore::WriteShortcuts(const std::vector<ShortcutEntry>& shortcuts) {
    HKEY hRoot = OpenRoot(KEY_WRITE);
    if (!hRoot) return;

    RegDeleteTreeW(hRoot, L"Shortcuts");
    HKEY hSub;
    if (RegCreateKeyExW(hRoot, L"Shortcuts", 0, nullptr,
                        REG_OPTION_NON_VOLATILE, KEY_WRITE, nullptr,
                        &hSub, nullptr) == ERROR_SUCCESS) {
        for (size_t i = 0; i < shortcuts.size(); ++i) {
            wchar_t name[32]; swprintf_s(name, L"%04zu", i);
            HKEY hEntry;
            if (RegCreateKeyExW(hSub, name, 0, nullptr,
                                REG_OPTION_NON_VOLATILE, KEY_WRITE, nullptr,
                                &hEntry, nullptr) == ERROR_SUCCESS) {
                auto& sc = shortcuts[i];
                RegSetValueExW(hEntry, L"Trigger", 0, REG_SZ,
                    reinterpret_cast<const BYTE*>(sc.trigger.c_str()),
                    static_cast<DWORD>((sc.trigger.size() + 1) * sizeof(wchar_t)));
                RegSetValueExW(hEntry, L"Replacement", 0, REG_SZ,
                    reinterpret_cast<const BYTE*>(sc.replacement.c_str()),
                    static_cast<DWORD>((sc.replacement.size() + 1) * sizeof(wchar_t)));
                DWORD en = sc.enabled ? 1 : 0;
                RegSetValueExW(hEntry, L"Enabled", 0, REG_DWORD,
                               reinterpret_cast<const BYTE*>(&en), sizeof(en));
                RegCloseKey(hEntry);
            }
        }
        RegCloseKey(hSub);
    }
    RegCloseKey(hRoot);
}

void SettingsStore::SetAutoStart(bool enabled) {
    HKEY hRun;
    if (RegCreateKeyExW(HKEY_CURRENT_USER, RUN_KEY, 0, nullptr,
                        REG_OPTION_NON_VOLATILE, KEY_WRITE, nullptr,
                        &hRun, nullptr) == ERROR_SUCCESS) {
        if (enabled) {
            wchar_t exe[MAX_PATH]; GetModuleFileNameW(nullptr, exe, MAX_PATH);
            RegSetValueExW(hRun, L"GoxViet", 0, REG_SZ,
                           reinterpret_cast<const BYTE*>(exe),
                           static_cast<DWORD>((wcslen(exe) + 1) * sizeof(wchar_t)));
        } else {
            RegDeleteValueW(hRun, L"GoxViet");
        }
        RegCloseKey(hRun);
    }
}

bool SettingsStore::ReadPerApp(const std::wstring& app, bool def) const {
    HKEY hKey;
    if (RegOpenKeyExW(HKEY_CURRENT_USER, PERAPP, 0, KEY_READ, &hKey) != ERROR_SUCCESS)
        return def;
    DWORD v = 0, size = sizeof(v);
    bool ok = RegQueryValueExW(hKey, app.c_str(), nullptr, nullptr,
                               reinterpret_cast<BYTE*>(&v), &size) == ERROR_SUCCESS;
    RegCloseKey(hKey);
    return ok ? (v != 0) : def;
}

void SettingsStore::WritePerApp(const std::wstring& app, bool enabled) {
    HKEY hKey;
    if (RegCreateKeyExW(HKEY_CURRENT_USER, PERAPP, 0, nullptr,
                        REG_OPTION_NON_VOLATILE, KEY_WRITE, nullptr,
                        &hKey, nullptr) != ERROR_SUCCESS) return;
    DWORD v = enabled ? 1 : 0;
    RegSetValueExW(hKey, app.c_str(), 0, REG_DWORD,
                   reinterpret_cast<const BYTE*>(&v), sizeof(v));
    RegCloseKey(hKey);
}

void SettingsStore::RemovePerApp(const std::wstring& app) {
    HKEY hKey;
    if (RegOpenKeyExW(HKEY_CURRENT_USER, PERAPP, 0, KEY_WRITE, &hKey) != ERROR_SUCCESS) return;
    RegDeleteValueW(hKey, app.c_str());
    RegCloseKey(hKey);
}

std::vector<std::pair<std::wstring, bool>> SettingsStore::ReadAllPerApp() const {
    std::vector<std::pair<std::wstring, bool>> result;
    HKEY hKey;
    if (RegOpenKeyExW(HKEY_CURRENT_USER, PERAPP, 0, KEY_READ, &hKey) != ERROR_SUCCESS)
        return result;
    wchar_t name[512]; DWORD index = 0, nameLen, type, value, sz;
    while (true) {
        nameLen = 512; sz = sizeof(DWORD);
        if (RegEnumValueW(hKey, index++, name, &nameLen, nullptr, &type,
                          reinterpret_cast<BYTE*>(&value), &sz) != ERROR_SUCCESS) break;
        if (type == REG_DWORD) result.push_back({ name, value != 0 });
    }
    RegCloseKey(hKey);
    return result;
}

}  // namespace goxviet
