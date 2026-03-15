#include <windows.h>
#include <commctrl.h>
#include "rust_bridge.h"
#include "keyboard_hook.h"
#include "system_tray.h"
#include "settings.h"
#include "settings_window.h"
#include "app_compat.h"
#include "per_app.h"
#include "modern_ui.h"
#include "input_language_monitor.h"
#include "notifications.h"
#include "utils.h"
#include "debug_console.h"
#include "resource.h"

static const wchar_t* WINDOW_CLASS   = L"GoxVietHidden";
static HWINEVENTHOOK  g_fgHook       = nullptr;
static HWINEVENTHOOK  g_focusHook    = nullptr;
static void*          g_engine       = nullptr;
static HWND           g_msgWnd       = nullptr;

namespace goxviet { void* GetEngine() { return g_engine; } }

// ---- WinEvent hook callbacks -----------------------------------------------

static void CALLBACK WinEventProc(HWINEVENTHOOK, DWORD event, HWND,
                                   LONG idObject, LONG, DWORD, DWORD) {
    auto& appCompat = goxviet::AppCompat::Instance();
    if (event == EVENT_SYSTEM_FOREGROUND) {
        if (g_engine) goxviet::RustBridge::Instance().ResetAll(g_engine);
        appCompat.ClearDetectionCache();

        if (goxviet::Settings::Instance().perApp) {
            auto app = appCompat.GetForegroundAppName();
            if (!app.empty())
                goxviet::PerAppMode::Instance().SwitchToApp(app, g_engine);
        }
    } else if (event == EVENT_OBJECT_FOCUS) {
        if (idObject == OBJID_WINDOW || idObject == OBJID_CLIENT) {
            if (g_engine) goxviet::RustBridge::Instance().ResetBuffer(g_engine);
            appCompat.ClearDetectionCache();
        }
    }
}

// ---- EventBus subscribers --------------------------------------------------

static void SubscribeEvents() {
    auto& bus = goxviet::EventBus::Instance();

    // Re-sync engine config whenever a setting changes.
    // Each event registered separately (MSVC doesn't deduce element type from braced list).
    auto syncEngine = [](goxviet::AppEvent) {
        if (!g_engine) return;
        auto& s = goxviet::Settings::Instance();
        s.ApplyToEngine(g_engine);
        if (s.enableShortcuts) s.SyncShortcutsToEngine(g_engine);
        goxviet::SystemTray::Instance().UpdateIcon();
    };
    bus.Subscribe(goxviet::AppEvent::InputMethodChanged,   &g_engine, syncEngine);
    bus.Subscribe(goxviet::AppEvent::ToneStyleChanged,     &g_engine, syncEngine);
    bus.Subscribe(goxviet::AppEvent::SmartModeChanged,     &g_engine, syncEngine);
    bus.Subscribe(goxviet::AppEvent::InstantRestoreChanged,&g_engine, syncEngine);
    bus.Subscribe(goxviet::AppEvent::EscRestoreChanged,    &g_engine, syncEngine);
    bus.Subscribe(goxviet::AppEvent::FreeToneChanged,      &g_engine, syncEngine);
    bus.Subscribe(goxviet::AppEvent::ShortcutsChanged,     &g_engine, syncEngine);

    // Auto-disable for non-Latin keyboard
    bus.Subscribe(goxviet::AppEvent::InputLanguageChanged, &g_msgWnd,
                  [](goxviet::AppEvent) {
        auto& s   = goxviet::Settings::Instance();
        auto& mon = goxviet::InputLanguageMonitor::Instance();
        if (s.autoDisableNonLatin) {
            bool latin = mon.IsCurrentLayoutLatin();
            s.SetEnabled(latin, /*silent=*/false);
            if (g_engine) {
                goxviet::FfiConfig_v2 cfg = s.ToFfiConfig();
                goxviet::RustBridge::Instance().SetConfig(g_engine, &cfg);
            }
            goxviet::SystemTray::Instance().UpdateIcon();
        }
    });

    // Update tray icon on enable/disable
    bus.Subscribe(goxviet::AppEvent::EnabledChanged, &g_msgWnd, [](goxviet::AppEvent) {
        goxviet::SystemTray::Instance().UpdateIcon();
    });
}

// ---- Message window --------------------------------------------------------

static LRESULT CALLBACK WindowProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam) {
    switch (msg) {
    case WM_TRAYICON:
        goxviet::SystemTray::Instance().HandleMessage(wParam, lParam);
        return 0;

    case WM_INPUTLANGCHANGE:
        goxviet::InputLanguageMonitor::Instance()
            .OnInputLangChange(reinterpret_cast<HKL>(lParam));
        return DefWindowProcW(hwnd, msg, wParam, lParam);

    case WM_COMMAND: {
        auto& s = goxviet::Settings::Instance();
        switch (LOWORD(wParam)) {
        case IDM_ENABLE:
            s.SetEnabled(!s.enabled);
            if (g_engine) {
                goxviet::FfiConfig_v2 cfg = s.ToFfiConfig();
                goxviet::RustBridge::Instance().SetConfig(g_engine, &cfg);
                goxviet::RustBridge::Instance().ResetAll(g_engine);
            }
            if (s.enabled && s.sound) goxviet::PlayToggleSound();
            break;
        case IDM_TELEX:  s.SetMethod(0); s.ApplyToEngine(g_engine); break;
        case IDM_VNI:    s.SetMethod(1); s.ApplyToEngine(g_engine); break;
        case IDM_SETTINGS:
            goxviet::SettingsWindow::Instance().Show(g_engine, goxviet::SettingsTab::General);
            break;
        case IDM_ABOUT:
            goxviet::SettingsWindow::Instance().Show(g_engine, goxviet::SettingsTab::About);
            break;
        case IDM_EXIT:
            PostQuitMessage(0);
            break;
        }
        return 0;
    }
    case WM_DESTROY:
        PostQuitMessage(0);
        return 0;
    }
    return DefWindowProcW(hwnd, msg, wParam, lParam);
}

// ---- Entry point -----------------------------------------------------------

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE, LPSTR, int) {
#ifdef GOXVIET_DEBUG_CONSOLE
    goxviet::DebugConsole::Instance().Create();
    goxviet::DebugConsole::Instance().Log(L"[STARTUP] Gõ Việt starting...");
#endif

    INITCOMMONCONTROLSEX icc = { sizeof(icc), ICC_LISTVIEW_CLASSES | ICC_STANDARD_CLASSES };
    InitCommonControlsEx(&icc);
    goxviet::ui::InitGdiPlus();
    goxviet::ui::RegisterToggleClass(hInstance);

    // Initialize settings (first-launch detection + load)
    auto& settings = goxviet::Settings::Instance();
    settings.Initialize();
    goxviet::LogInfo(L"Settings initialized");

    goxviet::PerAppMode::Instance().Load();

    // Load DLL
    auto& bridge = goxviet::RustBridge::Instance();
    if (!bridge.Load()) {
        MessageBoxW(nullptr,
            L"Không tìm thấy goxviet_core.dll.\n"
            L"Vui lòng đặt DLL cùng thư mục với goxviet.exe.",
            L"Gõ Việt — Lỗi", MB_ICONERROR);
        return 1;
    }

    // Create engine
    goxviet::FfiConfig_v2 cfg = settings.ToFfiConfig();
    g_engine = bridge.CreateEngine(&cfg);
    if (!g_engine) {
        MessageBoxW(nullptr, L"Không thể khởi tạo lõi IME.",
                    L"Gõ Việt — Lỗi", MB_ICONERROR);
        return 1;
    }
    settings.SyncShortcutsToEngine(g_engine);
    goxviet::LogInfo(L"Engine ready");

    // Create hidden message window
    WNDCLASSEXW wc = {};
    wc.cbSize = sizeof(wc); wc.lpfnWndProc = WindowProc;
    wc.hInstance = hInstance; wc.lpszClassName = WINDOW_CLASS;
    RegisterClassExW(&wc);

    g_msgWnd = CreateWindowExW(0, WINDOW_CLASS, L"GoxVietMsg",
                               0, 0, 0, 0, 0, HWND_MESSAGE,
                               nullptr, hInstance, nullptr);
    if (!g_msgWnd) { goxviet::LogError(L"Failed to create message window"); return 1; }

    // Subscribe to typed events
    SubscribeEvents();

    // WinEvent hooks
    g_fgHook = SetWinEventHook(EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_FOREGROUND,
                               nullptr, WinEventProc, 0, 0,
                               WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS);
    g_focusHook = SetWinEventHook(EVENT_OBJECT_FOCUS, EVENT_OBJECT_FOCUS,
                                  nullptr, WinEventProc, 0, 0,
                                  WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS);

    // Keyboard hook
    if (!goxviet::KeyboardHook::Instance().Install(g_engine)) {
        MessageBoxW(nullptr, L"Không thể cài keyboard hook.",
                    L"Gõ Việt — Lỗi", MB_ICONERROR);
        return 1;
    }

    // System tray
    if (!goxviet::SystemTray::Instance().Create(g_msgWnd)) {
        MessageBoxW(nullptr, L"Không thể tạo tray icon.",
                    L"Gõ Việt — Lỗi", MB_ICONERROR);
        return 1;
    }

    goxviet::LogInfo(L"Gõ Việt started");

    MSG msg;
    while (GetMessageW(&msg, nullptr, 0, 0)) {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }

    // Cleanup
    goxviet::EventBus::Instance().Unsubscribe(&g_engine);
    goxviet::EventBus::Instance().Unsubscribe(&g_msgWnd);
    if (g_fgHook)    UnhookWinEvent(g_fgHook);
    if (g_focusHook) UnhookWinEvent(g_focusHook);
    goxviet::KeyboardHook::Instance().Uninstall();
    goxviet::SystemTray::Instance().Destroy();
    bridge.DestroyEngine(g_engine); g_engine = nullptr;
    goxviet::ui::ShutdownGdiPlus();
    goxviet::LogInfo(L"Gõ Việt shut down");
    return 0;
}
