#include "keyboard_hook.h"
#include "rust_bridge.h"
#include "settings.h"
#include "vk_mapper.h"
#include "app_compat.h"
#include "input_language_monitor.h"
#include "restore_shortcut.h"
#include "notifications.h"
#include "utils.h"
#include <string>

namespace goxviet {

static constexpr ULONG_PTR GOXVIET_MARKER = 0x474F5856;  // "GOXV"

static void*  s_engine    = nullptr;
static bool   s_processing = false;   // reentrancy guard

static RestoreShortcutDetector s_restoreDetector;

// ============================================================================
// BackspaceInjectionStrategy
// ============================================================================

void BackspaceInjectionStrategy::MicrosecondSleep(uint32_t us) {
    if (us == 0) return;
    if (us < 2000) {
        LARGE_INTEGER freq, start, now;
        QueryPerformanceFrequency(&freq);
        QueryPerformanceCounter(&start);
        double target = static_cast<double>(us) * freq.QuadPart / 1e6;
        do { QueryPerformanceCounter(&now); }
        while ((now.QuadPart - start.QuadPart) < target);
    } else {
        Sleep((us + 999) / 1000);
    }
}

void BackspaceInjectionStrategy::SendBackspaces(int count, uint32_t delayUs) {
    for (int i = 0; i < count && i < 64; ++i) {
        INPUT in[2] = {};
        in[0].type = in[1].type = INPUT_KEYBOARD;
        in[0].ki.wVk = in[1].ki.wVk = VK_BACK;
        in[0].ki.dwExtraInfo = in[1].ki.dwExtraInfo = GOXVIET_MARKER;
        in[1].ki.dwFlags = KEYEVENTF_KEYUP;
        SendInput(2, in, sizeof(INPUT));
        MicrosecondSleep(delayUs);
    }
}

void BackspaceInjectionStrategy::SendUnicodeText(const wchar_t* text, int len, uint32_t delayUs) {
    for (int i = 0; i < len; ++i) {
        INPUT in[2] = {};
        in[0].type = in[1].type = INPUT_KEYBOARD;
        in[0].ki.wScan = in[1].ki.wScan = text[i];
        in[0].ki.dwFlags = KEYEVENTF_UNICODE;
        in[1].ki.dwFlags = KEYEVENTF_UNICODE | KEYEVENTF_KEYUP;
        in[0].ki.dwExtraInfo = in[1].ki.dwExtraInfo = GOXVIET_MARKER;
        SendInput(2, in, sizeof(INPUT));
        MicrosecondSleep(delayUs);
    }
}

void BackspaceInjectionStrategy::Inject(int backspaces, const char* utf8Text,
                                         const InjectionTiming& timing) {
    if (!utf8Text) return;
    EnterCriticalSection(&cs_);
    std::wstring wtext = RustBridge::Utf8ToUtf16(utf8Text);
    SendBackspaces(backspaces, timing.backspaceDelayUs);
    MicrosecondSleep(timing.waitDelayUs);
    SendUnicodeText(wtext.c_str(), static_cast<int>(wtext.size()), timing.textDelayUs);
    LeaveCriticalSection(&cs_);
}

// ============================================================================
// SelectionInjectionStrategy
// ============================================================================

void SelectionInjectionStrategy::Inject(int backspaces, const char* utf8Text,
                                          const InjectionTiming& timing) {
    if (!utf8Text) return;
    std::wstring wtext = RustBridge::Utf8ToUtf16(utf8Text);

    // Shift+Left × backspaces to select existing chars
    for (int i = 0; i < backspaces && i < 64; ++i) {
        INPUT shift[2] = {}; shift[0].type = shift[1].type = INPUT_KEYBOARD;
        shift[0].ki.wVk = shift[1].ki.wVk = VK_SHIFT;
        shift[0].ki.dwExtraInfo = shift[1].ki.dwExtraInfo = GOXVIET_MARKER;
        shift[1].ki.dwFlags = KEYEVENTF_KEYUP;

        INPUT left[2] = {}; left[0].type = left[1].type = INPUT_KEYBOARD;
        left[0].ki.wVk = left[1].ki.wVk = VK_LEFT;
        left[0].ki.dwFlags = left[0].ki.dwFlags | KEYEVENTF_EXTENDEDKEY;
        left[1].ki.dwFlags = KEYEVENTF_KEYUP | KEYEVENTF_EXTENDEDKEY;
        left[0].ki.dwExtraInfo = left[1].ki.dwExtraInfo = GOXVIET_MARKER;

        SendInput(1, &shift[0], sizeof(INPUT));
        SendInput(1, &left[0],  sizeof(INPUT));
        SendInput(1, &left[1],  sizeof(INPUT));
        SendInput(1, &shift[1], sizeof(INPUT));
        if (timing.backspaceDelayUs) Sleep((timing.backspaceDelayUs + 999) / 1000);
    }

    // Type replacement
    for (wchar_t ch : wtext) {
        INPUT in[2] = {};
        in[0].type = in[1].type = INPUT_KEYBOARD;
        in[0].ki.wScan = in[1].ki.wScan = ch;
        in[0].ki.dwFlags = KEYEVENTF_UNICODE;
        in[1].ki.dwFlags = KEYEVENTF_UNICODE | KEYEVENTF_KEYUP;
        in[0].ki.dwExtraInfo = in[1].ki.dwExtraInfo = GOXVIET_MARKER;
        SendInput(2, in, sizeof(INPUT));
        if (timing.textDelayUs) Sleep((timing.textDelayUs + 999) / 1000);
    }
}

// ============================================================================
// TextInjector — context
// ============================================================================

TextInjector& TextInjector::Instance() {
    static TextInjector instance;
    return instance;
}

TextInjector::TextInjector() = default;

void TextInjector::Inject(int backspaces, const char* utf8Text) {
    if (!utf8Text) return;
    auto det = AppCompat::Instance().GetInjectionMethod();
    if (det.method == InjectionMethod::Selection && backspaces > 0)
        selection_.Inject(backspaces, utf8Text, det.timing);
    else
        fast_.Inject(backspaces, utf8Text, det.timing);
}

// ============================================================================
// KeyboardHook
// ============================================================================

KeyboardHook& KeyboardHook::Instance() {
    static KeyboardHook instance;
    return instance;
}

bool KeyboardHook::Install(void* engine) {
    if (hook_) return true;
    s_engine = engine;
    hook_ = SetWindowsHookExW(WH_KEYBOARD_LL, LowLevelProc, nullptr, 0);
    return hook_ != nullptr;
}

void KeyboardHook::Uninstall() {
    if (hook_) { UnhookWindowsHookEx(hook_); hook_ = nullptr; }
}

static void DoRestore() {
    if (!s_engine) return;
    FfiProcessResult_v2 result{};
    auto status = RustBridge::Instance().RestoreToRaw(s_engine, &result);
    if (status == FfiStatusCode::Success && result.consumed && result.text) {
        s_processing = true;
        TextInjector::Instance().Inject(result.backspace_count, result.text);
        s_processing = false;
    }
    RustBridge::Instance().FreeString(result.text);
}

LRESULT CALLBACK KeyboardHook::LowLevelProc(int nCode, WPARAM wParam, LPARAM lParam) {
    if (nCode < 0) return CallNextHookEx(nullptr, nCode, wParam, lParam);
    if (s_processing) return CallNextHookEx(nullptr, nCode, wParam, lParam);

    const auto* kb = reinterpret_cast<const KBDLLHOOKSTRUCT*>(lParam);
    if (kb->dwExtraInfo == GOXVIET_MARKER)
        return CallNextHookEx(nullptr, nCode, wParam, lParam);

    if (wParam != WM_KEYDOWN && wParam != WM_SYSKEYDOWN)
        return CallNextHookEx(nullptr, nCode, wParam, lParam);

    DWORD vk = kb->vkCode;
    auto& settings = Settings::Instance();

    bool ctrlDown  = (GetAsyncKeyState(VK_CONTROL) & 0x8000) != 0;
    bool altDown   = (GetAsyncKeyState(VK_MENU)    & 0x8000) != 0;
    bool shiftDown = (GetAsyncKeyState(VK_SHIFT)   & 0x8000) != 0;
    bool capsLock  = (GetKeyState(VK_CAPITAL) & 0x0001) != 0;

    // ---- Ctrl+Space: toggle -------------------------------------------------
    if (ctrlDown && vk == VK_SPACE) {
        s_processing = true;
        settings.SetEnabled(!settings.enabled);
        if (s_engine) {
            RustBridge::Instance().SetConfig(s_engine, new FfiConfig_v2(settings.ToFfiConfig()));
            RustBridge::Instance().ResetAll(s_engine);
        }
        if (settings.enabled && settings.sound) PlayToggleSound();
        s_processing = false;
        return 1;
    }

    // ---- RestoreShortcut: double-tap modifier --------------------------------
    if (settings.restoreShortcut.enabled) {
        if (s_restoreDetector.OnKeyDown(vk, settings.restoreShortcut)) {
            DoRestore();
            return 1;
        }
    }

    if (!settings.enabled) return CallNextHookEx(nullptr, nCode, wParam, lParam);

    // Auto-disable for non-Latin
    if (settings.autoDisableNonLatin &&
        !InputLanguageMonitor::Instance().IsCurrentLayoutLatin())
        return CallNextHookEx(nullptr, nCode, wParam, lParam);

    // Ctrl/Alt combos — reset buffer, pass through
    if (ctrlDown || altDown) {
        if (s_engine) RustBridge::Instance().ResetBuffer(s_engine);
        return CallNextHookEx(nullptr, nCode, wParam, lParam);
    }

    // Shift+Backspace: delete previous word
    if (shiftDown && vk == VK_BACK && settings.shiftBackspace) {
        if (s_engine) RustBridge::Instance().ResetAll(s_engine);
        s_processing = true;
        // Send Ctrl+Backspace (delete word) then reset
        INPUT in[2] = {};
        in[0].type = in[1].type = INPUT_KEYBOARD;
        in[0].ki.wVk = VK_BACK; in[0].ki.dwFlags = 0;
        in[1].ki.wVk = VK_BACK; in[1].ki.dwFlags = KEYEVENTF_KEYUP;
        in[0].ki.dwExtraInfo = in[1].ki.dwExtraInfo = GOXVIET_MARKER;
        // Simulate Ctrl held
        INPUT ctrl[2] = {};
        ctrl[0].type = ctrl[1].type = INPUT_KEYBOARD;
        ctrl[0].ki.wVk = VK_CONTROL;
        ctrl[1].ki.wVk = VK_CONTROL; ctrl[1].ki.dwFlags = KEYEVENTF_KEYUP;
        ctrl[0].ki.dwExtraInfo = ctrl[1].ki.dwExtraInfo = GOXVIET_MARKER;
        SendInput(1, &ctrl[0], sizeof(INPUT));
        SendInput(2, in, sizeof(INPUT));
        SendInput(1, &ctrl[1], sizeof(INPUT));
        s_processing = false;
        return 1;
    }

    // Map VK → char
    const auto& mapper = VkMapper::Instance();
    auto key = mapper.Map(vk, capsLock, shiftDown);

    if (key.isBreak) {
        if (s_engine) RustBridge::Instance().ResetBuffer(s_engine);
        return CallNextHookEx(nullptr, nCode, wParam, lParam);
    }

    // ESC restore
    if (vk == VK_ESCAPE && settings.escRestore) {
        DoRestore();
        return 1;
    }

    if (key.ch == 0 || !s_engine)
        return CallNextHookEx(nullptr, nCode, wParam, lParam);

    // Process key through engine
    FfiProcessResult_v2 result{};
    auto status = RustBridge::Instance().ProcessKeyExt(
        s_engine, key.ch, capsLock, shiftDown, false, &result);

    if (status == FfiStatusCode::Success && result.consumed) {
        s_processing = true;
        TextInjector::Instance().Inject(result.backspace_count, result.text);
        RustBridge::Instance().FreeString(result.text);
        s_processing = false;
        return 1;
    }

    RustBridge::Instance().FreeString(result.text);
    return CallNextHookEx(nullptr, nCode, wParam, lParam);
}

}  // namespace goxviet
