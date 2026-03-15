#pragma once
#include <windows.h>
#include <string>
#include <unordered_map>

namespace goxviet {

// Per-application IME enable/disable state (max 100 entries)
class PerAppMode {
public:
    static PerAppMode& Instance();

    void Load();
    void Save();

    bool GetAppState(const std::wstring& appName) const;
    void SetAppState(const std::wstring& appName, bool enabled);
    void SwitchToApp(const std::wstring& appName, void* engine);

    bool HasEntry(const std::wstring& appName) const;
    void RemoveEntry(const std::wstring& appName);

    const std::unordered_map<std::wstring, bool>& GetAll() const { return states_; }

private:
    PerAppMode();
    ~PerAppMode();
    PerAppMode(const PerAppMode&) = delete;
    PerAppMode& operator=(const PerAppMode&) = delete;

    static constexpr size_t MAX_ENTRIES = 100;
    // Persistence delegated to SettingsStore

    std::unordered_map<std::wstring, bool> states_;
    std::wstring currentApp_;
    CRITICAL_SECTION lock_;
};

}  // namespace goxviet
