#pragma once
// InputLanguageMonitor — detects non-Latin keyboard layouts and posts
// AppEvent::InputLanguageChanged so Settings can auto-disable the IME.
// Equivalent to InputSourceMonitor.swift on macOS.
//
// Usage: call OnInputLangChange() from WM_INPUTLANGCHANGE in the message window.

#include <windows.h>

namespace goxviet {

class InputLanguageMonitor {
public:
    static InputLanguageMonitor& Instance();

    // Call from WM_INPUTLANGCHANGE handler. Posts InputLanguageChanged when
    // the new layout is non-Latin, so keyboard_hook can suppress input.
    void OnInputLangChange(HKL newLayout);

    bool IsCurrentLayoutLatin() const { return currentIsLatin_; }

private:
    InputLanguageMonitor() = default;
    InputLanguageMonitor(const InputLanguageMonitor&) = delete;
    InputLanguageMonitor& operator=(const InputLanguageMonitor&) = delete;

    static bool IsLatin(HKL hkl);

    bool currentIsLatin_ = true;
};

}  // namespace goxviet
