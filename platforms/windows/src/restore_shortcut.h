#pragma once
// RestoreShortcut — configurable double-tap modifier key that triggers restore-to-raw.
// Equivalent to RestoreShortcut.swift on macOS (double Option → double Right-Alt on Windows).

#include <windows.h>
#include <cstdint>
#include <string>

namespace goxviet {

enum class RestoreModifier : uint8_t {
    RightAlt   = 0,  // default — equivalent to macOS double-Option
    RightShift = 1,
    RightCtrl  = 2,
    LeftAlt    = 3,
};

struct RestoreShortcut {
    RestoreModifier modifier     = RestoreModifier::RightAlt;
    uint32_t        intervalMs   = 400;  // max gap between two taps (ms)
    bool            enabled      = true;

    static RestoreShortcut Default() { return {}; }

    // Load / save via SettingsStore
    static RestoreShortcut Load();
    void Save() const;

    const wchar_t* DisplayName() const;
    DWORD VirtualKey() const;
};

// Stateful detector — call OnKeyDown each time the modifier key fires.
// Returns true when a double-tap is confirmed.
class RestoreShortcutDetector {
public:
    bool OnKeyDown(DWORD vk, const RestoreShortcut& shortcut);
    void Reset();

private:
    DWORD lastVk_       = 0;
    DWORD lastTickMs_   = 0;
    bool  waitingSecond_ = false;
};

}  // namespace goxviet
