#include "system_tray.h"
#include "settings.h"
#include "resource.h"
#include "utils.h"
#include <strsafe.h>

namespace goxviet {

SystemTray& SystemTray::Instance() {
    static SystemTray instance;
    return instance;
}

bool SystemTray::Create(HWND hwnd) {
    if (created_) return true;
    hwnd_ = hwnd;

    nid_             = {};
    nid_.cbSize      = sizeof(NOTIFYICONDATAW);
    nid_.hWnd        = hwnd;
    nid_.uID         = 1;
    nid_.uFlags      = NIF_ICON | NIF_MESSAGE | NIF_TIP;
    nid_.uCallbackMessage = WM_TRAYICON;

    UpdateIcon();

    if (!Shell_NotifyIconW(NIM_ADD, &nid_)) {
        LogError(L"Shell_NotifyIconW NIM_ADD failed");
        return false;
    }

    // Request Windows Vista+ behaviour (balloon support etc.)
    NOTIFYICONDATAW ver = {};
    ver.cbSize = sizeof(ver);
    ver.hWnd   = hwnd;
    ver.uID    = 1;
    ver.uVersion = NOTIFYICON_VERSION_4;
    Shell_NotifyIconW(NIM_SETVERSION, &ver);

    created_ = true;
    return true;
}

void SystemTray::Destroy() {
    if (!created_) return;
    Shell_NotifyIconW(NIM_DELETE, &nid_);
    if (nid_.hIcon) { DestroyIcon(nid_.hIcon); nid_.hIcon = nullptr; }
    created_ = false;
}

void SystemTray::UpdateIcon() {
    auto& settings = Settings::Instance();

    if (nid_.hIcon) { DestroyIcon(nid_.hIcon); nid_.hIcon = nullptr; }

    HINSTANCE hInst = GetModuleHandleW(nullptr);
    int iconId = settings.enabled ? IDI_TRAY_ON : IDI_TRAY_OFF;
    nid_.hIcon = static_cast<HICON>(LoadImageW(hInst,
                                               MAKEINTRESOURCEW(iconId),
                                               IMAGE_ICON, 16, 16, LR_DEFAULTCOLOR));

    StringCchPrintfW(nid_.szTip, ARRAYSIZE(nid_.szTip),
                     settings.enabled
                         ? L"Gõ Việt — Đang bật (%s)"
                         : L"Gõ Việt — Đã tắt (%s)",
                     settings.method == 0 ? L"Telex" : L"VNI");

    if (created_) Shell_NotifyIconW(NIM_MODIFY, &nid_);
}

void SystemTray::HandleMessage(WPARAM wParam, LPARAM lParam) {
    UINT msg = LOWORD(lParam);
    if (msg == WM_LBUTTONDBLCLK) {
        // Double-click opens Settings (matches macOS: clicking menu bar icon opens settings)
        PostMessageW(hwnd_, WM_COMMAND, MAKEWPARAM(IDM_SETTINGS, 0), 0);
    } else if (msg == WM_RBUTTONUP || msg == NIN_SELECT) {
        ShowContextMenu();
    }
}

void SystemTray::ShowContextMenu() {
    auto& settings = Settings::Instance();
    HMENU hMenu = CreatePopupMenu();
    if (!hMenu) return;

    // Toggle — show current state with checkmark
    AppendMenuW(hMenu, MF_STRING | (settings.enabled ? MF_CHECKED : 0),
                IDM_ENABLE, L"Bật / Tắt (Ctrl+Space)");
    AppendMenuW(hMenu, MF_SEPARATOR, 0, nullptr);

    // Input method sub-items
    AppendMenuW(hMenu, MF_STRING | (settings.method == 0 ? MF_CHECKED : 0),
                IDM_TELEX, L"Telex");
    AppendMenuW(hMenu, MF_STRING | (settings.method == 1 ? MF_CHECKED : 0),
                IDM_VNI, L"VNI");
    AppendMenuW(hMenu, MF_SEPARATOR, 0, nullptr);

    AppendMenuW(hMenu, MF_STRING, IDM_SETTINGS, L"Cài đặt...");
    AppendMenuW(hMenu, MF_STRING, IDM_ABOUT,    L"Về Gõ Việt...");
    AppendMenuW(hMenu, MF_SEPARATOR, 0, nullptr);
    AppendMenuW(hMenu, MF_STRING, IDM_EXIT,     L"Thoát");

    // Must set foreground window to make the menu dismiss on click-away
    SetForegroundWindow(hwnd_);

    POINT pt;
    GetCursorPos(&pt);
    TrackPopupMenu(hMenu, TPM_RIGHTBUTTON | TPM_BOTTOMALIGN,
                   pt.x, pt.y, 0, hwnd_, nullptr);
    PostMessageW(hwnd_, WM_NULL, 0, 0);
    DestroyMenu(hMenu);
}

}  // namespace goxviet
