#include "modern_ui.h"
#include "resource.h"
#include <uxtheme.h>
#include <dwmapi.h>
#pragma comment(lib, "uxtheme.lib")
#pragma comment(lib, "dwmapi.lib")

namespace goxviet {
namespace ui {

static ULONG_PTR g_gdiplusToken = 0;

void InitGdiPlus() {
    Gdiplus::GdiplusStartupInput si;
    Gdiplus::GdiplusStartup(&g_gdiplusToken, &si, nullptr);
}

void ShutdownGdiPlus() {
    if (g_gdiplusToken) {
        Gdiplus::GdiplusShutdown(g_gdiplusToken);
        g_gdiplusToken = 0;
    }
}

bool IsDarkMode() {
    DWORD value = 0, size = sizeof(DWORD);
    RegGetValueW(HKEY_CURRENT_USER,
        L"Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
        L"AppsUseLightTheme", RRF_RT_REG_DWORD, nullptr, &value, &size);
    return (value == 0);
}

const Theme& GetTheme() {
    return IsDarkMode() ? DarkTheme : LightTheme;
}

float GetDpiScale(HWND hwnd) {
    UINT dpi = GetDpiForWindow(hwnd);
    return dpi / 96.0f;
}

int Scale(int value, HWND hwnd) {
    return static_cast<int>(value * GetDpiScale(hwnd));
}

void DrawRoundedRect(HDC hdc, const RECT& r, int radius, COLORREF fill, COLORREF border) {
    Gdiplus::Graphics g(hdc);
    g.SetSmoothingMode(Gdiplus::SmoothingModeAntiAlias);

    Gdiplus::GraphicsPath path;
    int x = r.left, y = r.top, w = r.right - r.left, h = r.bottom - r.top;
    path.AddArc(x, y, radius * 2, radius * 2, 180, 90);
    path.AddArc(x + w - radius * 2, y, radius * 2, radius * 2, 270, 90);
    path.AddArc(x + w - radius * 2, y + h - radius * 2, radius * 2, radius * 2, 0, 90);
    path.AddArc(x, y + h - radius * 2, radius * 2, radius * 2, 90, 90);
    path.CloseFigure();

    {
        Gdiplus::SolidBrush brush(Gdiplus::Color(
            GetRValue(fill), GetGValue(fill), GetBValue(fill)));
        g.FillPath(&brush, &path);
    }
    if (border != CLR_NONE) {
        Gdiplus::Pen pen(Gdiplus::Color(GetRValue(border), GetGValue(border), GetBValue(border)), 1.0f);
        g.DrawPath(&pen, &path);
    }
}

void DrawTextW(HDC hdc, const wchar_t* text, const RECT& r, COLORREF color,
               int fontSize, bool bold, UINT dtFlags) {
    HFONT font = CreateFontW(-fontSize, 0, 0, 0,
                             bold ? FW_SEMIBOLD : FW_NORMAL,
                             FALSE, FALSE, FALSE,
                             DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
                             CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY,
                             DEFAULT_PITCH, L"Segoe UI");
    HFONT old = static_cast<HFONT>(SelectObject(hdc, font));
    SetTextColor(hdc, color);
    SetBkMode(hdc, TRANSPARENT);
    RECT rc = r;
    DrawTextW(hdc, text, -1, &rc, dtFlags);
    SelectObject(hdc, old);
    DeleteObject(font);
}

void DrawToggle(HDC hdc, int x, int y, int w, int h, bool on, bool hovered) {
    const auto& theme = GetTheme();
    COLORREF track = on ? theme.toggleOn : theme.toggleOff;
    if (hovered && !on) track = RGB(130, 130, 130);

    RECT track_r = { x, y, x + w, y + h };
    DrawRoundedRect(hdc, track_r, h / 2, track);

    int knobSize  = h - 4;
    int knobX     = on ? (x + w - knobSize - 2) : (x + 2);
    RECT knob_r   = { knobX, y + 2, knobX + knobSize, y + 2 + knobSize };
    DrawRoundedRect(hdc, knob_r, knobSize / 2, theme.toggleKnob);
}

// ---- Toggle switch custom control ----

struct ToggleData {
    bool state   = false;
    bool hovered = false;
};

static LRESULT CALLBACK ToggleWndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam) {
    auto* d = reinterpret_cast<ToggleData*>(GetWindowLongPtrW(hwnd, GWLP_USERDATA));

    switch (msg) {
    case WM_CREATE: {
        d = new ToggleData{};
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, reinterpret_cast<LONG_PTR>(d));
        return 0;
    }
    case WM_DESTROY:
        delete d;
        return 0;

    case WM_PAINT: {
        PAINTSTRUCT ps;
        HDC hdc = BeginPaint(hwnd, &ps);
        RECT r; GetClientRect(hwnd, &r);

        // Fill parent background
        HWND parent = GetParent(hwnd);
        const auto& theme = GetTheme();
        HBRUSH bg = CreateSolidBrush(theme.windowBg);
        FillRect(hdc, &r, bg);
        DeleteObject(bg);

        DrawToggle(hdc, r.left, r.top, r.right - r.left, r.bottom - r.top,
                   d ? d->state : false, d ? d->hovered : false);
        EndPaint(hwnd, &ps);
        return 0;
    }
    case WM_LBUTTONUP:
        if (d) {
            d->state = !d->state;
            InvalidateRect(hwnd, nullptr, TRUE);
            SendMessageW(GetParent(hwnd), WM_TOGGLE_CHANGED,
                         static_cast<WPARAM>(GetDlgCtrlID(hwnd)),
                         static_cast<LPARAM>(d->state ? 1 : 0));
        }
        return 0;

    case WM_MOUSEMOVE:
        if (d && !d->hovered) {
            d->hovered = true;
            InvalidateRect(hwnd, nullptr, TRUE);
            TRACKMOUSEEVENT tme{ sizeof(tme), TME_LEAVE, hwnd, 0 };
            TrackMouseEvent(&tme);
        }
        return 0;

    case WM_MOUSELEAVE:
        if (d) { d->hovered = false; InvalidateRect(hwnd, nullptr, TRUE); }
        return 0;

    // Custom messages
    case WM_USER + 10:   // get state
        return d ? (d->state ? 1 : 0) : 0;
    case WM_USER + 11:   // set state
        if (d) {
            d->state = (wParam != 0);
            InvalidateRect(hwnd, nullptr, TRUE);
            if (lParam) {
                SendMessageW(GetParent(hwnd), WM_TOGGLE_CHANGED,
                             static_cast<WPARAM>(GetDlgCtrlID(hwnd)),
                             static_cast<LPARAM>(d->state ? 1 : 0));
            }
        }
        return 0;
    }
    return DefWindowProcW(hwnd, msg, wParam, lParam);
}

void RegisterToggleClass(HINSTANCE hInst) {
    WNDCLASSEXW wc = {};
    wc.cbSize        = sizeof(wc);
    wc.lpfnWndProc   = ToggleWndProc;
    wc.hInstance     = hInst;
    wc.lpszClassName = GOXVIET_TOGGLE_CLASS;
    wc.hCursor       = LoadCursor(nullptr, IDC_HAND);
    RegisterClassExW(&wc);
}

HWND CreateToggle(HWND parent, int x, int y, int id, bool state) {
    HINSTANCE hInst = reinterpret_cast<HINSTANCE>(GetWindowLongPtrW(parent, GWLP_HINSTANCE));
    HWND hwnd = CreateWindowExW(0, GOXVIET_TOGGLE_CLASS, L"",
                                WS_CHILD | WS_VISIBLE,
                                x, y, 44, 24, parent,
                                reinterpret_cast<HMENU>(static_cast<UINT_PTR>(id)),
                                hInst, nullptr);
    if (hwnd) SetToggleState(hwnd, state);
    return hwnd;
}

bool GetToggleState(HWND hwnd) {
    return SendMessageW(hwnd, WM_USER + 10, 0, 0) != 0;
}

void SetToggleState(HWND hwnd, bool state, bool notify) {
    SendMessageW(hwnd, WM_USER + 11, state ? 1 : 0, notify ? 1 : 0);
}

}  // namespace ui
}  // namespace goxviet
