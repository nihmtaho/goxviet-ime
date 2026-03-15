#include "about_dialog.h"
#include "rust_bridge.h"
#include "modern_ui.h"
#include "resource.h"
#include <shellapi.h>
#include <strsafe.h>

namespace goxviet {

AboutDialog& AboutDialog::Instance() {
    static AboutDialog instance;
    return instance;
}

AboutDialog::~AboutDialog() {
    if (hwnd_) DestroyWindow(hwnd_);
}

void AboutDialog::Show() {
    if (!hwnd_) Create();
    if (!hwnd_)  return;
    ShowWindow(hwnd_, SW_SHOWNORMAL);
    SetForegroundWindow(hwnd_);
    visible_ = true;
}

void AboutDialog::Create() {
    HINSTANCE hInst = GetModuleHandleW(nullptr);
    static const wchar_t* CLASS = L"GoxVietAbout";
    WNDCLASSEXW wc = {};
    wc.cbSize        = sizeof(wc);
    wc.lpfnWndProc   = WndProc;
    wc.hInstance     = hInst;
    wc.hCursor       = LoadCursor(nullptr, IDC_ARROW);
    wc.lpszClassName = CLASS;
    wc.hIcon         = static_cast<HICON>(LoadImageW(hInst,
                           MAKEINTRESOURCEW(IDI_APP_ICON), IMAGE_ICON, 32, 32, LR_DEFAULTCOLOR));
    RegisterClassExW(&wc);

    hwnd_ = CreateWindowExW(WS_EX_DLGMODALFRAME, CLASS, L"Về Gõ Việt",
                            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU,
                            CW_USEDEFAULT, CW_USEDEFAULT, 320, 220,
                            nullptr, nullptr, hInst, this);
}

LRESULT CALLBACK AboutDialog::WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam) {
    AboutDialog* dlg = nullptr;
    if (msg == WM_CREATE) {
        auto* cs = reinterpret_cast<CREATESTRUCTW*>(lParam);
        dlg = reinterpret_cast<AboutDialog*>(cs->lpCreateParams);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, reinterpret_cast<LONG_PTR>(dlg));
    } else {
        dlg = reinterpret_cast<AboutDialog*>(GetWindowLongPtrW(hwnd, GWLP_USERDATA));
    }

    switch (msg) {
    case WM_PAINT: {
        PAINTSTRUCT ps;
        HDC hdc = BeginPaint(hwnd, &ps);
        RECT r; GetClientRect(hwnd, &r);
        const auto& theme = ui::GetTheme();
        HBRUSH bg = CreateSolidBrush(theme.windowBg);
        FillRect(hdc, &r, bg);
        DeleteObject(bg);

        // App name
        RECT nameR = { 20, 20, r.right - 20, 55 };
        ui::DrawTextW(hdc, L"Gõ Việt", nameR, theme.textPrimary, 18, true, DT_LEFT | DT_VCENTER | DT_SINGLELINE);

        // Engine version
        FfiVersionInfo vi{};
        wchar_t ver[64] = L"v?.?.?";
        if (RustBridge::Instance().GetVersion(&vi) == FfiStatusCode::Success)
            StringCchPrintfW(ver, 64, L"Phiên bản lõi: %u.%u.%u", vi.major, vi.minor, vi.patch);
        RECT verR = { 20, 58, r.right - 20, 82 };
        ui::DrawTextW(hdc, ver, verR, theme.textSecondary, 12, false, DT_LEFT | DT_VCENTER | DT_SINGLELINE);

        // Description
        RECT descR = { 20, 85, r.right - 20, 130 };
        ui::DrawTextW(hdc, L"Bộ gõ tiếng Việt cho Windows.\nHỗ trợ Telex, VNI và nhiều tính năng hiện đại.",
                      descR, theme.textSecondary, 12, false, DT_LEFT | DT_WORDBREAK);

        // GitHub link hint
        RECT linkR = { 20, 140, r.right - 20, 160 };
        ui::DrawTextW(hdc, L"Mã nguồn mở — MIT License", linkR, theme.accent, 11, false, DT_LEFT);

        EndPaint(hwnd, &ps);
        return 0;
    }
    case WM_CTLCOLORDLG:
    case WM_CTLCOLORSTATIC: {
        auto* hdc = reinterpret_cast<HDC>(wParam);
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, ui::GetTheme().textPrimary);
        return reinterpret_cast<LRESULT>(GetStockObject(NULL_BRUSH));
    }
    case WM_CLOSE:
        ShowWindow(hwnd, SW_HIDE);
        if (dlg) dlg->visible_ = false;
        return 0;
    case WM_DESTROY:
        if (dlg) { dlg->hwnd_ = nullptr; dlg->visible_ = false; }
        return 0;
    }
    return DefWindowProcW(hwnd, msg, wParam, lParam);
}

}  // namespace goxviet
