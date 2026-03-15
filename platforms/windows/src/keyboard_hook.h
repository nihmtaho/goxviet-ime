#pragma once
// KeyboardHook — installs WH_KEYBOARD_LL and delegates key events to KeyProcessor.
// Single Responsibility: hook lifecycle only.

#include <windows.h>
#include "iinjection_strategy.h"

namespace goxviet {

class KeyboardHook {
public:
    static KeyboardHook& Instance();

    // engine: opaque handle from RustBridge::CreateEngine.
    bool Install(void* engine);
    void Uninstall();

    bool IsInstalled() const { return hook_ != nullptr; }

private:
    KeyboardHook() = default;
    ~KeyboardHook() { Uninstall(); }
    KeyboardHook(const KeyboardHook&) = delete;
    KeyboardHook& operator=(const KeyboardHook&) = delete;

    static LRESULT CALLBACK LowLevelProc(int nCode, WPARAM wParam, LPARAM lParam);

    HHOOK hook_ = nullptr;
};

// ---- Injection strategies (Open/Closed Principle) ---------------------------

// Fast/slow — backspace then unicode text
class BackspaceInjectionStrategy final : public IInjectionStrategy {
public:
    void Inject(int backspaces, const char* utf8Text, const InjectionTiming& timing) override;
private:
    void SendBackspaces(int count, uint32_t delayUs);
    void SendUnicodeText(const wchar_t* text, int len, uint32_t delayUs);
    static void MicrosecondSleep(uint32_t us);
    CRITICAL_SECTION cs_ = {};
public:
    BackspaceInjectionStrategy()  { InitializeCriticalSection(&cs_); }
    ~BackspaceInjectionStrategy() { DeleteCriticalSection(&cs_); }
};

// Selection — Shift+Left×N then type replacement (browser address bars)
class SelectionInjectionStrategy final : public IInjectionStrategy {
public:
    void Inject(int backspaces, const char* utf8Text, const InjectionTiming& timing) override;
};

// ---- TextInjector (context — picks strategy based on AppCompat) -------------

class TextInjector {
public:
    static TextInjector& Instance();
    void Inject(int backspaces, const char* utf8Text);

private:
    TextInjector();
    TextInjector(const TextInjector&) = delete;
    TextInjector& operator=(const TextInjector&) = delete;

    BackspaceInjectionStrategy  fast_;
    SelectionInjectionStrategy  selection_;
};

}  // namespace goxviet
