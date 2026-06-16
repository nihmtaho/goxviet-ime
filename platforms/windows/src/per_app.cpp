#include "per_app.h"
#include "settings.h"
#include "settings_store.h"
#include "utils.h"

namespace goxviet {

PerAppMode& PerAppMode::Instance() {
    static PerAppMode instance;
    return instance;
}

PerAppMode::PerAppMode() { InitializeCriticalSection(&lock_); }
PerAppMode::~PerAppMode() { DeleteCriticalSection(&lock_); }

void PerAppMode::Load() {
    EnterCriticalSection(&lock_);
    states_.clear();
    for (auto& [app, en] : SettingsStore::Instance().ReadAllPerApp())
        states_[app] = en;
    LeaveCriticalSection(&lock_);
}

void PerAppMode::Save() {
    // No-op: individual SetAppState writes immediately via SettingsStore.
}

bool PerAppMode::GetAppState(const std::wstring& appName) const {
    auto it = states_.find(appName);
    return (it == states_.end()) ? Settings::Instance().enabled : it->second;
}

void PerAppMode::SetAppState(const std::wstring& appName, bool enabled) {
    EnterCriticalSection(&lock_);
    if (states_.size() >= MAX_ENTRIES && states_.find(appName) == states_.end())
        states_.erase(states_.begin());
    states_[appName] = enabled;
    LeaveCriticalSection(&lock_);
    SettingsStore::Instance().WritePerApp(appName, enabled);
}

void PerAppMode::SwitchToApp(const std::wstring& appName, void* engine) {
    if (appName == currentApp_) return;
    currentApp_ = appName;

    bool appEnabled = GetAppState(appName);
    // Silent: don't post EnabledChanged to avoid tray icon flicker on every switch
    Settings::Instance().SetEnabled(appEnabled, /*silent=*/true);

    if (engine) {
        FfiConfig_v2 cfg = Settings::Instance().ToFfiConfig();
        RustBridge::Instance().SetConfig(engine, &cfg);
        RustBridge::Instance().ResetAll(engine);
    }
}

bool PerAppMode::HasEntry(const std::wstring& appName) const {
    return states_.find(appName) != states_.end();
}

void PerAppMode::RemoveEntry(const std::wstring& appName) {
    EnterCriticalSection(&lock_);
    states_.erase(appName);
    LeaveCriticalSection(&lock_);
    SettingsStore::Instance().RemovePerApp(appName);
}

}  // namespace goxviet
