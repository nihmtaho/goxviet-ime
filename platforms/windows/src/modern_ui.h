#pragma once
#include <windows.h>
#include <gdiplus.h>
#pragma comment(lib, "gdiplus.lib")

#ifndef CLR_NONE
#define CLR_NONE ((COLORREF)-1)
#endif

namespace goxviet {
namespace ui {

struct Theme {
    COLORREF windowBg;
    COLORREF sidebarBg;
    COLORREF textPrimary;
    COLORREF textSecondary;
    COLORREF accent;       // Windows 11 blue
    COLORREF toggleOn;
    COLORREF toggleOff;
    COLORREF toggleKnob;
    COLORREF cardBg;
    COLORREF border;
};

inline const Theme DarkTheme = {
    RGB(28,  28,  28),   // windowBg
    RGB(38,  38,  38),   // sidebarBg
    RGB(255, 255, 255),  // textPrimary
    RGB(160, 160, 160),  // textSecondary
    RGB(0,   120, 212),  // accent
    RGB(0,   120, 212),  // toggleOn
    RGB(90,  90,  90),   // toggleOff
    RGB(255, 255, 255),  // toggleKnob
    RGB(45,  45,  45),   // cardBg
    RGB(60,  60,  60),   // border
};

inline const Theme LightTheme = {
    RGB(243, 243, 243),  // windowBg
    RGB(230, 230, 230),  // sidebarBg
    RGB(0,   0,   0),    // textPrimary
    RGB(90,  90,  90),   // textSecondary
    RGB(0,   120, 212),  // accent
    RGB(0,   120, 212),  // toggleOn
    RGB(190, 190, 190),  // toggleOff
    RGB(255, 255, 255),  // toggleKnob
    RGB(255, 255, 255),  // cardBg
    RGB(210, 210, 210),  // border
};

bool           IsDarkMode();
const Theme&   GetTheme();
float          GetDpiScale(HWND hwnd);
int            Scale(int value, HWND hwnd);

void InitGdiPlus();
void ShutdownGdiPlus();

void DrawRoundedRect(HDC hdc, const RECT& r, int radius, COLORREF fill, COLORREF border = CLR_NONE);
void DrawTextW(HDC hdc, const wchar_t* text, const RECT& r, COLORREF color,
               int fontSize, bool bold = false, UINT dtFlags = DT_LEFT | DT_VCENTER | DT_SINGLELINE);
void DrawToggle(HDC hdc, int x, int y, int w, int h, bool on, bool hovered = false);

// Custom toggle-switch control
#define GOXVIET_TOGGLE_CLASS L"GoxVietToggle"
void    RegisterToggleClass(HINSTANCE hInst);
HWND    CreateToggle(HWND parent, int x, int y, int id, bool state = false);
bool    GetToggleState(HWND hwnd);
void    SetToggleState(HWND hwnd, bool state, bool notify = false);

}  // namespace ui
}  // namespace goxviet
