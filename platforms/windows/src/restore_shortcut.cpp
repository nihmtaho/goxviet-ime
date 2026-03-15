#include "restore_shortcut.h"
#include "settings_store.h"

namespace goxviet {

static constexpr const wchar_t* KEY_MOD      = L"RestoreModifier";
static constexpr const wchar_t* KEY_INTERVAL = L"RestoreInterval";
static constexpr const wchar_t* KEY_ENABLED  = L"RestoreShortcutEnabled";

RestoreShortcut RestoreShortcut::Load() {
    auto& store = SettingsStore::Instance();
    RestoreShortcut rs;
    rs.modifier   = static_cast<RestoreModifier>(store.ReadDword(KEY_MOD,      0));
    rs.intervalMs = store.ReadDword(KEY_INTERVAL, 400);
    rs.enabled    = store.ReadBool (KEY_ENABLED,  true);
    return rs;
}

void RestoreShortcut::Save() const {
    auto& store = SettingsStore::Instance();
    store.WriteDword(KEY_MOD,      static_cast<DWORD>(modifier));
    store.WriteDword(KEY_INTERVAL, intervalMs);
    store.WriteBool (KEY_ENABLED,  enabled);
}

const wchar_t* RestoreShortcut::DisplayName() const {
    switch (modifier) {
    case RestoreModifier::RightAlt:   return L"Double Right-Alt";
    case RestoreModifier::RightShift: return L"Double Right-Shift";
    case RestoreModifier::RightCtrl:  return L"Double Right-Ctrl";
    case RestoreModifier::LeftAlt:    return L"Double Left-Alt";
    default:                          return L"Double Right-Alt";
    }
}

DWORD RestoreShortcut::VirtualKey() const {
    switch (modifier) {
    case RestoreModifier::RightAlt:   return VK_RMENU;
    case RestoreModifier::RightShift: return VK_RSHIFT;
    case RestoreModifier::RightCtrl:  return VK_RCONTROL;
    case RestoreModifier::LeftAlt:    return VK_LMENU;
    default:                          return VK_RMENU;
    }
}

bool RestoreShortcutDetector::OnKeyDown(DWORD vk, const RestoreShortcut& shortcut) {
    if (!shortcut.enabled) return false;
    if (vk != shortcut.VirtualKey()) { Reset(); return false; }

    DWORD now = GetTickCount();

    if (!waitingSecond_) {
        // First tap
        lastVk_        = vk;
        lastTickMs_    = now;
        waitingSecond_ = true;
        return false;
    }

    // Second tap
    DWORD elapsed = now - lastTickMs_;
    if (elapsed <= shortcut.intervalMs && elapsed > 0) {
        Reset();
        return true;  // confirmed double-tap
    }

    // Too slow — restart sequence
    lastTickMs_    = now;
    waitingSecond_ = true;
    return false;
}

void RestoreShortcutDetector::Reset() {
    lastVk_ = 0; lastTickMs_ = 0; waitingSecond_ = false;
}

}  // namespace goxviet
