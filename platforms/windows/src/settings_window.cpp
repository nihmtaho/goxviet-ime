#include "settings_window.h"
#include "settings.h"
#include "settings_store.h"
#include "modern_ui.h"
#include "system_tray.h"
#include "resource.h"
#include "rust_bridge.h"
#include "restore_shortcut.h"
#include "per_app.h"
#include <strsafe.h>
#include <shellapi.h>
#include <commdlg.h>
#include <commctrl.h>

namespace goxviet {

// ── Window sizing ──────────────────────────────────────────────────────────────
static constexpr int DEF_W  = 760;
static constexpr int DEF_H  = 560;
static constexpr int MIN_W  = 620;
static constexpr int MIN_H  = 440;
static constexpr int SIDE_W = 176;   // wide enough for Vietnamese sidebar text

// ── Layout (computed at runtime from font metrics) ─────────────────────────────
struct Layout {
    int fontH   = 0;   // text height of system font
    int pad     = 0;   // outer padding
    int rowS    = 0;   // row height, label-only
    int rowT    = 0;   // row height, label + sublabel
    int secH    = 0;   // section header height (bold label + separator)
    int lblH    = 0;   // label control height
    int subH    = 0;   // sublabel height
    int togY    = 0;   // toggle vertical offset from row top (centre toggle in rowT)
    int togX    = 0;   // toggle left edge (= panelW - pad - 48)
    int panelW  = 0;   // usable content width of a panel
};

static Layout g_lay{};

// ── Fonts ─────────────────────────────────────────────────────────────────────
static HFONT g_font     = nullptr;   // system message font
static HFONT g_fontBold = nullptr;   // message font + semibold
static HFONT g_fontLg   = nullptr;   // large bold for "Gõ Việt" heading

static void BuildFonts(HWND hwnd) {
    if (g_font) return;

    // System message font (respects user accessibility/DPI settings)
    NONCLIENTMETRICSW ncm = {};
    ncm.cbSize = sizeof(ncm);
    SystemParametersInfoW(SPI_GETNONCLIENTMETRICS, sizeof(ncm), &ncm, 0);
    g_font = CreateFontIndirectW(&ncm.lfMessageFont);

    // Bold variant of the same face/size
    LOGFONTW lf = ncm.lfMessageFont;
    lf.lfWeight = FW_SEMIBOLD;
    g_fontBold = CreateFontIndirectW(&lf);

    // Large title font
    LOGFONTW lfLg = ncm.lfMessageFont;
    lfLg.lfWeight  = FW_BOLD;
    lfLg.lfHeight  = ncm.lfMessageFont.lfHeight * 2;  // roughly 2× normal size
    g_fontLg = CreateFontIndirectW(&lfLg);

    // Measure actual font pixel height
    HDC hdc = GetDC(hwnd);
    HFONT old = static_cast<HFONT>(SelectObject(hdc, g_font));
    TEXTMETRICW tm = {};
    GetTextMetricsW(hdc, &tm);
    SelectObject(hdc, old);
    ReleaseDC(hwnd, hdc);

    int fh = tm.tmHeight + tm.tmExternalLeading;

    // Compute layout from font metrics
    g_lay.fontH  = fh;
    g_lay.pad    = fh + 2;          // outer padding ≈ 1 line
    g_lay.lblH   = fh + 4;
    g_lay.subH   = fh + 2;
    g_lay.rowS   = fh + 20;         // single-label row
    g_lay.rowT   = fh * 2 + 26;     // label + sublabel row
    g_lay.secH   = fh + 18;         // section header (bold) + separator
}

static void BuildPanelWidth(HWND hwnd) {
    // Panel content width = client_W - SIDE_W - 1(divider) - scrollbar
    RECT cr; GetClientRect(hwnd, &cr);
    int scrollW = GetSystemMetrics(SM_CXVSCROLL);
    g_lay.panelW = cr.right - SIDE_W - 1 - scrollW - 2;
    g_lay.pad    = max(g_lay.pad, 14);  // at least 14px padding
    g_lay.togX   = g_lay.panelW - g_lay.pad - 48;
}

static BOOL CALLBACK ApplyFont(HWND child, LPARAM font) {
    SendMessageW(child, WM_SETFONT, font, FALSE);
    return TRUE;
}

// ── Panel scroll state ─────────────────────────────────────────────────────────
struct PanelState { int scrollY = 0; int contentH = 0; };
static PanelState* PS(HWND p) {
    return reinterpret_cast<PanelState*>(GetWindowLongPtrW(p, GWLP_USERDATA));
}
static void ApplyScrollDelta(HWND p, int delta) {
    auto* ps = PS(p); if (!ps) return;
    RECT r; GetClientRect(p, &r);
    int maxY = max(0, ps->contentH - (r.bottom - r.top));
    int oldY = ps->scrollY;
    ps->scrollY = max(0, min(oldY + delta, maxY));
    int dy = oldY - ps->scrollY;
    if (dy) {
        ScrollWindowEx(p, 0, dy, nullptr, nullptr, nullptr, nullptr,
                       SW_SCROLLCHILDREN | SW_INVALIDATE | SW_ERASE);
        SCROLLINFO si = {};
        si.cbSize = sizeof(si); si.fMask = SIF_RANGE | SIF_PAGE | SIF_POS;
        si.nMin = 0; si.nMax = ps->contentH; si.nPage = r.bottom - r.top;
        si.nPos = ps->scrollY;
        SetScrollInfo(p, SB_VERT, &si, TRUE);
    }
}
static void SetPanelContentH(HWND p, int h) {
    auto* ps = PS(p); if (!ps) return;
    ps->contentH = h;
    RECT r; GetClientRect(p, &r);
    SCROLLINFO si = {};
    si.cbSize = sizeof(si); si.fMask = SIF_RANGE | SIF_PAGE | SIF_POS;
    si.nMin = 0; si.nMax = h; si.nPage = r.bottom - r.top;
    si.nPos = 0;
    SetScrollInfo(p, SB_VERT, &si, TRUE);
}

// ── Panel window proc ──────────────────────────────────────────────────────────
static LRESULT CALLBACK PanelProc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
    switch (msg) {
    case WM_CREATE:
        SetWindowLongPtrW(hwnd, GWLP_USERDATA,
                          reinterpret_cast<LONG_PTR>(new PanelState{}));
        return 0;
    case WM_DESTROY:
        delete PS(hwnd);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
        return 0;

    case WM_ERASEBKGND: {
        RECT r; GetClientRect(hwnd, &r);
        HBRUSH b = CreateSolidBrush(ui::GetTheme().windowBg);
        FillRect(reinterpret_cast<HDC>(wp), &r, b); DeleteObject(b);
        return 1;
    }
    case WM_PAINT: { PAINTSTRUCT ps; BeginPaint(hwnd, &ps); EndPaint(hwnd, &ps); return 0; }

    case WM_CTLCOLORSTATIC:
    case WM_CTLCOLOREDIT: {
        HDC hdc = reinterpret_cast<HDC>(wp);
        HWND ctrl = reinterpret_cast<HWND>(lp);
        const auto& th = ui::GetTheme();
        SetTextColor(hdc, GetWindowLongPtrW(ctrl, GWLP_USERDATA) == 1
                         ? th.textSecondary : th.textPrimary);
        SetBkMode(hdc, TRANSPARENT);
        static HBRUSH bg = nullptr;
        if (bg) DeleteObject(bg);
        bg = CreateSolidBrush(th.windowBg);
        return reinterpret_cast<LRESULT>(bg);
    }
    case WM_VSCROLL: {
        int line = g_lay.rowS > 0 ? g_lay.rowS : 24;
        switch (LOWORD(wp)) {
        case SB_LINEUP:      ApplyScrollDelta(hwnd, -line);    break;
        case SB_LINEDOWN:    ApplyScrollDelta(hwnd,  line);    break;
        case SB_PAGEUP:      ApplyScrollDelta(hwnd, -180);     break;
        case SB_PAGEDOWN:    ApplyScrollDelta(hwnd,  180);     break;
        case SB_THUMBTRACK: {
            auto* ps = PS(hwnd);
            if (ps) {
                int old = ps->scrollY; ps->scrollY = HIWORD(wp);
                int dy = old - ps->scrollY;
                if (dy) ScrollWindowEx(hwnd, 0, dy, nullptr, nullptr, nullptr,
                                       nullptr, SW_SCROLLCHILDREN|SW_INVALIDATE|SW_ERASE);
            }
            break;
        }
        }
        return 0;
    }
    case WM_MOUSEWHEEL:
        ApplyScrollDelta(hwnd, -GET_WHEEL_DELTA_WPARAM(wp) / 3);
        return 0;

    case WM_SIZE: {
        auto* ps = PS(hwnd);
        if (!ps || ps->contentH == 0) return 0;
        RECT r; GetClientRect(hwnd, &r);
        int h = r.bottom - r.top;
        SCROLLINFO si = {};
        si.cbSize = sizeof(si); si.fMask = SIF_RANGE | SIF_PAGE | SIF_POS;
        si.nMin = 0; si.nMax = ps->contentH; si.nPage = h; si.nPos = ps->scrollY;
        SetScrollInfo(hwnd, SB_VERT, &si, TRUE);
        return 0;
    }
    case WM_COMMAND:
    case WM_TOGGLE_CHANGED:
        SendMessageW(GetParent(hwnd), msg, wp, lp);
        return 0;
    }
    return DefWindowProcW(hwnd, msg, wp, lp);
}

static void RegPanel() {
    static bool done = false; if (done) return; done = true;
    WNDCLASSEXW wc = {};
    wc.cbSize = sizeof(wc); wc.lpfnWndProc = PanelProc;
    wc.hInstance = GetModuleHandleW(nullptr);
    wc.lpszClassName = L"GoxVietPanel";
    wc.hCursor = LoadCursor(nullptr, IDC_ARROW);
    RegisterClassExW(&wc);
}

static HWND MakePanel(HWND parent) {
    RegPanel();
    RECT r; GetClientRect(parent, &r);
    int pw = r.right - SIDE_W - 1, ph = r.bottom;
    return CreateWindowExW(0, L"GoxVietPanel", L"",
                           WS_CHILD | WS_VSCROLL | WS_CLIPCHILDREN,
                           SIDE_W + 1, 0, max(pw, 400), max(ph, 300),
                           parent, nullptr, GetModuleHandleW(nullptr), nullptr);
}

// ── Control helpers ────────────────────────────────────────────────────────────
static HINSTANCE HI() { return GetModuleHandleW(nullptr); }

static HWND Lbl(HWND p, const wchar_t* t, int x, int y, int w, HFONT f = nullptr) {
    HWND h = CreateWindowW(L"STATIC", t, WS_CHILD | WS_VISIBLE,
                           x, y, w, g_lay.lblH, p, nullptr, HI(), nullptr);
    SendMessageW(h, WM_SETFONT, reinterpret_cast<WPARAM>(f ? f : g_font), FALSE);
    return h;
}
static HWND SubLbl(HWND p, const wchar_t* t, int x, int y, int w) {
    HWND h = CreateWindowW(L"STATIC", t, WS_CHILD | WS_VISIBLE,
                           x, y, w, g_lay.subH, p, nullptr, HI(), nullptr);
    SetWindowLongPtrW(h, GWLP_USERDATA, 1);
    SendMessageW(h, WM_SETFONT, reinterpret_cast<WPARAM>(g_font), FALSE);
    return h;
}
static HWND Sep(HWND p, int y) {
    return CreateWindowW(L"STATIC", L"", WS_CHILD | WS_VISIBLE | SS_ETCHEDHORZ,
                         g_lay.pad, y, g_lay.panelW - g_lay.pad * 2, 1, p,
                         nullptr, HI(), nullptr);
}
static HWND Cmb(HWND p, int x, int y, int w, int id) {
    HWND h = CreateWindowW(WC_COMBOBOX, L"",
                           WS_CHILD | WS_VISIBLE | CBS_DROPDOWNLIST | WS_VSCROLL,
                           x, y, w, 260, p,
                           reinterpret_cast<HMENU>(static_cast<UINT_PTR>(id)),
                           HI(), nullptr);
    SendMessageW(h, WM_SETFONT, reinterpret_cast<WPARAM>(g_font), FALSE);
    return h;
}
static HWND Btn(HWND p, const wchar_t* t, int x, int y, int w, int h, int id) {
    HWND hw = CreateWindowW(L"BUTTON", t, WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
                            x, y, w, h, p,
                            reinterpret_cast<HMENU>(static_cast<UINT_PTR>(id)),
                            HI(), nullptr);
    SendMessageW(hw, WM_SETFONT, reinterpret_cast<WPARAM>(g_font), FALSE);
    return hw;
}
static HWND Edt(HWND p, int x, int y, int w, int id) {
    HWND h = CreateWindowExW(WS_EX_CLIENTEDGE, L"EDIT", L"",
                             WS_CHILD | WS_VISIBLE | ES_AUTOHSCROLL,
                             x, y, w, g_lay.lblH + 4, p,
                             reinterpret_cast<HMENU>(static_cast<UINT_PTR>(id)),
                             HI(), nullptr);
    SendMessageW(h, WM_SETFONT, reinterpret_cast<WPARAM>(g_font), FALSE);
    return h;
}
static HWND Tog(HWND p, int y, int id) {
    // Centre toggle vertically in a rowT
    return ui::CreateToggle(p, g_lay.togX, y + (g_lay.rowT - 24) / 2, id);
}

// Section header (bold text + separator line), returns new y
static int SH(HWND p, const wchar_t* title, int y) {
    Lbl(p, title, g_lay.pad, y, g_lay.panelW - g_lay.pad * 2, g_fontBold);
    Sep(p, y + g_lay.lblH + 4);
    return y + g_lay.secH;
}

// Toggle row with optional sublabel; returns new y
static int TR(HWND p, const wchar_t* lbl, const wchar_t* sub,
              int y, int id, HWND* out) {
    int lw = g_lay.togX - g_lay.pad * 2 - 4;
    if (sub) {
        Lbl(p, lbl,  g_lay.pad, y + 4, lw);
        SubLbl(p, sub, g_lay.pad + 8, y + g_lay.lblH + 8, lw - 8);
        *out = Tog(p, y, id);
        return y + g_lay.rowT;
    } else {
        Lbl(p, lbl, g_lay.pad, y + (g_lay.rowS - g_lay.lblH) / 2, lw);
        *out = ui::CreateToggle(p, g_lay.togX, y + (g_lay.rowS - 24) / 2, id);
        return y + g_lay.rowS;
    }
}

// Label + combobox row, returns new y
static int CR(HWND p, const wchar_t* lbl, int y, int cmbW, int id, HWND* out) {
    int lw = g_lay.togX - g_lay.pad - 8 - cmbW;
    Lbl(p, lbl, g_lay.pad, y + 4, max(lw, 80));
    *out = Cmb(p, g_lay.pad + max(lw, 80) + 8, y, cmbW, id);
    return y + g_lay.rowS;
}

// Gap between sections
static constexpr int GAP = 8;

// ── Singleton ──────────────────────────────────────────────────────────────────
SettingsWindow& SettingsWindow::Instance() { static SettingsWindow i; return i; }
SettingsWindow::~SettingsWindow() { if (hwnd_) DestroyWindow(hwnd_); }

// ── Show ───────────────────────────────────────────────────────────────────────
void SettingsWindow::Show(void* engine, SettingsTab tab) {
    engine_ = engine;
    if (!hwnd_) Create();
    if (!hwnd_)  return;
    SwitchTab(tab);
    ShowWindow(hwnd_, SW_SHOWNORMAL);
    SetForegroundWindow(hwnd_);
    visible_ = true;
}

// ── Create main window ─────────────────────────────────────────────────────────
void SettingsWindow::Create() {
    HINSTANCE hInst = GetModuleHandleW(nullptr);
    ui::RegisterToggleClass(hInst);

    static bool reg = false;
    if (!reg) {
        WNDCLASSEXW wc = {};
        wc.cbSize = sizeof(wc); wc.lpfnWndProc = WndProc;
        wc.hInstance = hInst; wc.hCursor = LoadCursor(nullptr, IDC_ARROW);
        wc.lpszClassName = L"GoxVietSettings";
        wc.hIcon = static_cast<HICON>(LoadImageW(hInst, MAKEINTRESOURCEW(IDI_APP_ICON),
                                                 IMAGE_ICON, 32, 32, LR_DEFAULTCOLOR));
        RegisterClassExW(&wc);
        reg = true;
    }

    int sw = GetSystemMetrics(SM_CXSCREEN), sh = GetSystemMetrics(SM_CYSCREEN);
    // Create the window first so we have an HWND for font metrics
    hwnd_ = CreateWindowExW(WS_EX_DLGMODALFRAME, L"GoxVietSettings", L"Gõ Việt — Cài đặt",
                            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU |
                            WS_THICKFRAME | WS_MINIMIZEBOX,
                            (sw - DEF_W) / 2, (sh - DEF_H) / 2,
                            DEF_W, DEF_H, nullptr, nullptr, hInst, this);
    if (!hwnd_) return;

    // Initialise fonts and layout metrics BEFORE building panels
    BuildFonts(hwnd_);
    BuildPanelWidth(hwnd_);

    BuildSidebar();
    panels_[0] = BuildGeneralPanel();
    panels_[1] = BuildPerAppPanel();
    panels_[2] = BuildTextExpansionPanel();
    panels_[3] = BuildAboutPanel();
}

// ── Sidebar ────────────────────────────────────────────────────────────────────
void SettingsWindow::BuildSidebar() {
    RECT cr; GetClientRect(hwnd_, &cr);
    sidebar_ = CreateWindowW(WC_LISTBOX, L"",
                             WS_CHILD | WS_VISIBLE | LBS_OWNERDRAWFIXED | LBS_HASSTRINGS |
                             LBS_NOINTEGRALHEIGHT | LBS_NOTIFY,
                             0, 0, SIDE_W, cr.bottom, hwnd_,
                             reinterpret_cast<HMENU>(9001), HI(), nullptr);
    SendMessageW(sidebar_, LB_ADDSTRING, 0, (LPARAM)L"Cài đặt chung");
    SendMessageW(sidebar_, LB_ADDSTRING, 0, (LPARAM)L"Theo ứng dụng");
    SendMessageW(sidebar_, LB_ADDSTRING, 0, (LPARAM)L"Gõ tắt");
    SendMessageW(sidebar_, LB_ADDSTRING, 0, (LPARAM)L"Về ứng dụng");
    int itemH = max(g_lay.fontH + 16, 36);
    SendMessageW(sidebar_, LB_SETITEMHEIGHT, 0, itemH);
    SendMessageW(sidebar_, LB_SETCURSEL, 0, 0);
    SendMessageW(sidebar_, WM_SETFONT, reinterpret_cast<WPARAM>(g_font), FALSE);
}

// ── SwitchTab ──────────────────────────────────────────────────────────────────
void SettingsWindow::SwitchTab(SettingsTab tab) {
    tab_ = tab;
    SendMessageW(sidebar_, LB_SETCURSEL, static_cast<int>(tab), 0);
    for (int i = 0; i < 4; ++i)
        if (panels_[i])
            ShowWindow(panels_[i], i == static_cast<int>(tab) ? SW_SHOW : SW_HIDE);
    switch (tab) {
    case SettingsTab::General:       RefreshGeneral();       break;
    case SettingsTab::PerApp:        RefreshPerApp();        break;
    case SettingsTab::TextExpansion: RefreshTextExpansion(); break;
    default: break;
    }
    InvalidateRect(hwnd_, nullptr, TRUE);
}

// ── General Panel ──────────────────────────────────────────────────────────────
HWND SettingsWindow::BuildGeneralPanel() {
    HWND p = MakePanel(hwnd_);
    int y = g_lay.pad;

    y = SH(p, L"Bật / Tắt", y);
    y = TR(p, L"Bật Gõ Việt", nullptr, y, IDC_TOGGLE_ENABLED, &gTogEnabled_);
    y += GAP;

    y = SH(p, L"Kiểu gõ", y);
    y = CR(p, L"Phương thức nhập", y, 160, IDC_CMB_METHOD, &gCmbMethod_);
    SendMessageW(gCmbMethod_, CB_ADDSTRING, 0, (LPARAM)L"Telex");
    SendMessageW(gCmbMethod_, CB_ADDSTRING, 0, (LPARAM)L"VNI");
    y = CR(p, L"Kiểu dấu thanh", y, 180, IDC_CMB_TONE, &gCmbTone_);
    SendMessageW(gCmbTone_, CB_ADDSTRING, 0, (LPARAM)L"Truyền thống");
    SendMessageW(gCmbTone_, CB_ADDSTRING, 0, (LPARAM)L"Hiện đại");
    y = TR(p, L"Tự do dấu thanh",
           L"Cho phép dấu thanh trên mọi ký tự",
           y, IDC_TOGGLE_FREE_TONE, &gTogFreeTone_);
    y += GAP;

    y = SH(p, L"Tính năng thông minh", y);
    y = TR(p, L"Khôi phục tức thì",
           L"Tự động khôi phục về ký tự gốc khi gõ sai",
           y, IDC_TOGGLE_INSTANT, &gTogInstant_);
    y = TR(p, L"Tắt tự động (bàn phím không Latin)",
           L"Tắt IME khi dùng bàn phím CJK, Ả Rập, v.v.",
           y, IDC_TOGGLE_AUTO_DISABLE_NL, &gTogAutoNL_);
    y = TR(p, L"ESC khôi phục",
           L"Nhấn ESC để hoàn tác về ký tự gốc",
           y, IDC_TOGGLE_ESC, &gTogEsc_);
    y = CR(p, L"Khôi phục nhanh (nhấn đôi)", y, 192, IDC_CMB_RESTORE_MOD, &gCmbRestore_);
    SendMessageW(gCmbRestore_, CB_ADDSTRING, 0, (LPARAM)L"Double Right-Alt");
    SendMessageW(gCmbRestore_, CB_ADDSTRING, 0, (LPARAM)L"Double Right-Shift");
    SendMessageW(gCmbRestore_, CB_ADDSTRING, 0, (LPARAM)L"Double Right-Ctrl");
    SendMessageW(gCmbRestore_, CB_ADDSTRING, 0, (LPARAM)L"Double Left-Alt");
    y += GAP;

    y = SH(p, L"Chỉnh sửa", y);
    y = TR(p, L"Shift+Backspace xoá từ  (Beta)",
           L"Xoá cả từ với Shift+Backspace",
           y, IDC_TOGGLE_SHIFT_BS, &gTogShiftBS_);
    y += GAP;

    y = SH(p, L"Bảng mã đầu ra", y);
    y = CR(p, L"Mã hoá", y, 220, IDC_CMB_ENCODING, &gCmbEncoding_);
    SendMessageW(gCmbEncoding_, CB_ADDSTRING, 0, (LPARAM)L"Unicode (Default)");
    SendMessageW(gCmbEncoding_, CB_ADDSTRING, 0, (LPARAM)L"TCVN3 (Legacy)");
    SendMessageW(gCmbEncoding_, CB_ADDSTRING, 0, (LPARAM)L"VNI Windows (Legacy)");
    SendMessageW(gCmbEncoding_, CB_ADDSTRING, 0, (LPARAM)L"CP1258 (Windows-1258)");
    y += GAP;

    y = SH(p, L"Hệ thống", y);
    y = TR(p, L"Từ viết tắt", nullptr,       y, IDC_TOGGLE_SHORTCUTS, &gTogShortcuts_);
    y = TR(p, L"Chạy cùng Windows", nullptr,  y, IDC_TOGGLE_AUTOSTART, &gTogAutoStart_);
    y = TR(p, L"Âm thanh bật/tắt", nullptr,   y, IDC_TOGGLE_SOUND, &gTogSound_);
    y += GAP;

    // Version string + Reset button
    int btnW = 158, btnH = g_lay.rowS - 4;
    gLblVersion_ = Lbl(p, L"", g_lay.pad, y + 4,
                       g_lay.panelW - g_lay.pad * 2 - btnW - 8);
    Btn(p, L"Đặt lại mặc định",
        g_lay.panelW - g_lay.pad - btnW, y - 1, btnW, btnH, IDC_BTN_RESET);
    y += g_lay.rowS + g_lay.pad;

    SetPanelContentH(p, y);
    return p;
}

void SettingsWindow::RefreshGeneral() {
    const auto& s = Settings::Instance();
    ui::SetToggleState(gTogEnabled_,    s.enabled);
    ui::SetToggleState(gTogFreeTone_,   s.freeTone);
    ui::SetToggleState(gTogInstant_,    s.instantRestore);
    ui::SetToggleState(gTogAutoNL_,     s.autoDisableNonLatin);
    ui::SetToggleState(gTogEsc_,        s.escRestore);
    ui::SetToggleState(gTogShiftBS_,    s.shiftBackspace);
    ui::SetToggleState(gTogShortcuts_,  s.enableShortcuts);
    ui::SetToggleState(gTogAutoStart_,  s.autoStart);
    ui::SetToggleState(gTogSound_,      s.sound);
    SendMessageW(gCmbMethod_,   CB_SETCURSEL, s.method, 0);
    SendMessageW(gCmbTone_,     CB_SETCURSEL, s.modernTone ? 1 : 0, 0);
    SendMessageW(gCmbEncoding_, CB_SETCURSEL, static_cast<int>(s.outputEncoding), 0);
    SendMessageW(gCmbRestore_,  CB_SETCURSEL, static_cast<int>(s.restoreShortcut.modifier), 0);
    FfiVersionInfo vi{};
    wchar_t vs[80] = L"";
    if (RustBridge::Instance().GetVersion(&vi) == FfiStatusCode::Success)
        StringCchPrintfW(vs, 80, L"goxviet_core v%u.%u.%u", vi.major, vi.minor, vi.patch);
    SetWindowTextW(gLblVersion_, vs);
}

void SettingsWindow::SaveFromGeneral() {
    auto& s = Settings::Instance();
    s.SetEnabled          (ui::GetToggleState(gTogEnabled_));
    s.SetFreeTone         (ui::GetToggleState(gTogFreeTone_));
    s.SetInstantRestore   (ui::GetToggleState(gTogInstant_));
    s.SetAutoDisableNonLatin(ui::GetToggleState(gTogAutoNL_));
    s.SetEscRestore       (ui::GetToggleState(gTogEsc_));
    s.SetShiftBackspace   (ui::GetToggleState(gTogShiftBS_));
    s.SetEnableShortcuts  (ui::GetToggleState(gTogShortcuts_));
    s.SetAutoStart        (ui::GetToggleState(gTogAutoStart_));
    s.sound               = ui::GetToggleState(gTogSound_);
    s.SetMethod           (static_cast<uint8_t>(SendMessageW(gCmbMethod_, CB_GETCURSEL, 0, 0)));
    s.SetModernTone       (SendMessageW(gCmbTone_, CB_GETCURSEL, 0, 0) == 1);
    s.SetOutputEncoding   (static_cast<OutputEncoding>(SendMessageW(gCmbEncoding_, CB_GETCURSEL, 0, 0)));
    RestoreShortcut rs = s.restoreShortcut;
    rs.modifier = static_cast<RestoreModifier>(SendMessageW(gCmbRestore_, CB_GETCURSEL, 0, 0));
    s.SetRestoreShortcut(rs);
    s.Save();
    s.ApplyToEngine(engine_);
    if (engine_) s.SyncShortcutsToEngine(engine_);
    SystemTray::Instance().UpdateIcon();
}

// ── Per-App Panel ──────────────────────────────────────────────────────────────
HWND SettingsWindow::BuildPerAppPanel() {
    HWND p = MakePanel(hwnd_);
    int y = g_lay.pad;

    y = SH(p, L"Chế độ riêng theo ứng dụng", y);
    y = TR(p, L"Nhớ trạng thái IME riêng mỗi ứng dụng",
           L"Mỗi ứng dụng có trạng thái Bật/Tắt IME riêng",
           y, IDC_TOGGLE_PERAPP, &paTog_);
    y += GAP;

    RECT cr; GetClientRect(hwnd_, &cr);
    int listH = max(cr.bottom - y - g_lay.rowS - g_lay.pad * 3, 180);
    paList_ = CreateWindowExW(WS_EX_CLIENTEDGE, WC_LISTVIEW, L"",
                              WS_CHILD | WS_VISIBLE | LVS_REPORT | LVS_SINGLESEL | LVS_SHOWSELALWAYS,
                              g_lay.pad, y, g_lay.panelW - g_lay.pad * 2, listH,
                              p, reinterpret_cast<HMENU>(9100), HI(), nullptr);
    ListView_SetExtendedListViewStyle(paList_, LVS_EX_FULLROWSELECT | LVS_EX_GRIDLINES);
    SendMessageW(paList_, WM_SETFONT, reinterpret_cast<WPARAM>(g_font), FALSE);
    LVCOLUMNW col = {}; col.mask = LVCF_TEXT | LVCF_WIDTH;
    col.cx = g_lay.panelW - g_lay.pad * 2 - 130; col.pszText = const_cast<wchar_t*>(L"Ứng dụng");
    ListView_InsertColumn(paList_, 0, &col);
    col.cx = 126; col.pszText = const_cast<wchar_t*>(L"Trạng thái IME");
    ListView_InsertColumn(paList_, 1, &col);
    y += listH + GAP;

    int bh = g_lay.rowS - 4;
    paBtnToggle_ = Btn(p, L"Đổi trạng thái", g_lay.pad,       y, 144, bh, IDC_PA_BTN_TOGGLE);
    paBtnRemove_ = Btn(p, L"Xoá",             g_lay.pad + 152, y,  72, bh, IDC_PA_BTN_REMOVE);
    paLblCount_  = Lbl(p, L"", g_lay.pad + 236, y + 4,
                       g_lay.panelW - g_lay.pad - 240);
    y += g_lay.rowS + g_lay.pad;

    SetPanelContentH(p, y);
    return p;
}

void SettingsWindow::RefreshPerApp() {
    ui::SetToggleState(paTog_, Settings::Instance().perApp);
    PerAppRebuildList();
}

void SettingsWindow::PerAppRebuildList() {
    if (!paList_) return;
    ListView_DeleteAllItems(paList_);
    auto all = SettingsStore::Instance().ReadAllPerApp();
    wchar_t cnt[64]; StringCchPrintfW(cnt, 64, L"%d ứng dụng", (int)all.size());
    SetWindowTextW(paLblCount_, cnt);
    int i = 0;
    for (auto& [app, en] : all) {
        LVITEMW item = {}; item.mask = LVIF_TEXT; item.iItem = i;
        item.pszText = const_cast<wchar_t*>(app.c_str());
        ListView_InsertItem(paList_, &item);
        ListView_SetItemText(paList_, i, 1, const_cast<wchar_t*>(en ? L"✓ Bật" : L"✗ Tắt"));
        ++i;
    }
}

void SettingsWindow::PerAppToggleSelected() {
    int idx = ListView_GetNextItem(paList_, -1, LVNI_SELECTED); if (idx < 0) return;
    wchar_t name[MAX_PATH] = {}; ListView_GetItemText(paList_, idx, 0, name, MAX_PATH);
    if (!name[0]) return;
    bool cur = SettingsStore::Instance().ReadPerApp(name, true);
    PerAppMode::Instance().SetAppState(name, !cur);
    PerAppRebuildList();
}

void SettingsWindow::PerAppRemoveSelected() {
    int idx = ListView_GetNextItem(paList_, -1, LVNI_SELECTED); if (idx < 0) return;
    wchar_t name[MAX_PATH] = {}; ListView_GetItemText(paList_, idx, 0, name, MAX_PATH);
    if (!name[0]) return;
    PerAppMode::Instance().RemoveEntry(name);
    PerAppRebuildList();
}

// ── Text Expansion Panel ───────────────────────────────────────────────────────
HWND SettingsWindow::BuildTextExpansionPanel() {
    HWND p = MakePanel(hwnd_);
    int y = g_lay.pad;

    y = SH(p, L"Gõ tắt (Text Expansion)", y);
    y = TR(p, L"Bật tính năng gõ tắt",
           L"Gõ từ tắt sẽ tự động được thay thế bằng nội dung đầy đủ",
           y, IDC_TOGGLE_SHORTCUTS, &txTog_);
    y += GAP;

    RECT cr; GetClientRect(hwnd_, &cr);
    int editRowH = g_lay.lblH + 8;
    int btnRowH  = g_lay.rowS;
    int listH    = max(cr.bottom - y - editRowH - btnRowH - g_lay.pad * 4, 160);

    txList_ = CreateWindowExW(WS_EX_CLIENTEDGE, WC_LISTVIEW, L"",
                              WS_CHILD | WS_VISIBLE | LVS_REPORT | LVS_SINGLESEL | LVS_SHOWSELALWAYS,
                              g_lay.pad, y, g_lay.panelW - g_lay.pad * 2, listH,
                              p, reinterpret_cast<HMENU>(9200), HI(), nullptr);
    ListView_SetExtendedListViewStyle(txList_, LVS_EX_FULLROWSELECT | LVS_EX_GRIDLINES);
    SendMessageW(txList_, WM_SETFONT, reinterpret_cast<WPARAM>(g_font), FALSE);
    LVCOLUMNW col = {}; col.mask = LVCF_TEXT | LVCF_WIDTH;
    col.cx = 170; col.pszText = const_cast<wchar_t*>(L"Viết tắt");
    ListView_InsertColumn(txList_, 0, &col);
    col.cx = g_lay.panelW - g_lay.pad * 2 - 178;
    col.pszText = const_cast<wchar_t*>(L"Thay thế bằng");
    ListView_InsertColumn(txList_, 1, &col);
    y += listH + GAP;

    // Add row
    int ex = g_lay.pad;
    int ew1 = 130, gap = 8;
    int ew2 = g_lay.panelW - ex * 2 - ew1 - gap * 3 - 70 - 80;
    Lbl(p, L"Viết tắt:", ex, y + 4, 60);
    txEditTrig_ = Edt(p, ex + 64, y, ew1, IDC_EDIT_TRIGGER);
    Lbl(p, L"Thay thế:", ex + 64 + ew1 + gap, y + 4, 68);
    txEditRepl_ = Edt(p, ex + 64 + ew1 + gap + 72, y,
                      max(ew2, 80), IDC_EDIT_REPLACEMENT);
    txBtnAdd_ = Btn(p, L"Thêm",
                    g_lay.panelW - g_lay.pad - 76, y - 1, 72, g_lay.lblH + 6,
                    IDC_BTN_ADD);
    y += editRowH + GAP;

    int bh = g_lay.rowS - 4;
    txBtnDel_ = Btn(p, L"Xoá",       ex,         y, 76, bh, IDC_BTN_DELETE);
    txBtnImp_ = Btn(p, L"Nhập file", ex + 84,    y, 98, bh, IDC_BTN_IMPORT);
    txBtnExp_ = Btn(p, L"Xuất file", ex + 190,   y, 98, bh, IDC_BTN_EXPORT);
    txLblCount_ = Lbl(p, L"", ex + 298, y + 4,
                      g_lay.panelW - ex - 302);
    y += g_lay.rowS + g_lay.pad;

    SetPanelContentH(p, y);
    return p;
}

void SettingsWindow::RefreshTextExpansion() {
    ui::SetToggleState(txTog_, Settings::Instance().enableShortcuts);
    TxRebuildList();
}

void SettingsWindow::TxRebuildList() {
    if (!txList_) return;
    ListView_DeleteAllItems(txList_);
    const auto& sc = Settings::Instance().shortcuts;
    wchar_t cnt[64]; StringCchPrintfW(cnt, 64, L"%d từ viết tắt", (int)sc.size());
    SetWindowTextW(txLblCount_, cnt);
    for (int i = 0; i < (int)sc.size(); ++i) {
        LVITEMW item = {}; item.mask = LVIF_TEXT; item.iItem = i;
        item.pszText = const_cast<wchar_t*>(sc[i].trigger.c_str());
        ListView_InsertItem(txList_, &item);
        ListView_SetItemText(txList_, i, 1, const_cast<wchar_t*>(sc[i].replacement.c_str()));
    }
}

void SettingsWindow::TxAddEntry() {
    wchar_t t[128] = {}, r[512] = {};
    GetWindowTextW(txEditTrig_, t, 128); GetWindowTextW(txEditRepl_, r, 512);
    if (!t[0] || !r[0]) return;
    Settings::Instance().AddShortcut(t, r);
    if (engine_) Settings::Instance().SyncShortcutsToEngine(engine_);
    SetWindowTextW(txEditTrig_, L""); SetWindowTextW(txEditRepl_, L"");
    TxRebuildList();
}

void SettingsWindow::TxDeleteSelected() {
    int idx = ListView_GetNextItem(txList_, -1, LVNI_SELECTED); if (idx < 0) return;
    const auto& sc = Settings::Instance().shortcuts;
    if (idx < (int)sc.size()) {
        Settings::Instance().RemoveShortcut(sc[idx].trigger);
        if (engine_) Settings::Instance().SyncShortcutsToEngine(engine_);
    }
    TxRebuildList();
}

void SettingsWindow::TxImport() {
    wchar_t path[MAX_PATH] = {};
    OPENFILENAMEW ofn = {}; ofn.lStructSize = sizeof(ofn); ofn.hwndOwner = hwnd_;
    ofn.lpstrFilter = L"Shortcut files (*.txt;*.csv)\0*.txt;*.csv\0All\0*.*\0";
    ofn.lpstrFile = path; ofn.nMaxFile = MAX_PATH; ofn.Flags = OFN_FILEMUSTEXIST;
    if (!GetOpenFileNameW(&ofn)) return;
    int n = Settings::Instance().ImportShortcuts(path);
    if (engine_) Settings::Instance().SyncShortcutsToEngine(engine_);
    wchar_t msg[80]; StringCchPrintfW(msg, 80, L"Đã nhập %d từ viết tắt.", n);
    MessageBoxW(hwnd_, msg, L"Nhập thành công", MB_OK | MB_ICONINFORMATION);
    TxRebuildList();
}

void SettingsWindow::TxExport() {
    wchar_t path[MAX_PATH] = {};
    OPENFILENAMEW ofn = {}; ofn.lStructSize = sizeof(ofn); ofn.hwndOwner = hwnd_;
    ofn.lpstrFilter = L"Text files (*.txt)\0*.txt\0";
    ofn.lpstrFile = path; ofn.nMaxFile = MAX_PATH;
    ofn.Flags = OFN_OVERWRITEPROMPT; ofn.lpstrDefExt = L"txt";
    if (!GetSaveFileNameW(&ofn)) return;
    Settings::Instance().ExportShortcuts(path);
}

// ── About Panel ────────────────────────────────────────────────────────────────
HWND SettingsWindow::BuildAboutPanel() {
    HWND p = MakePanel(hwnd_);
    int y = g_lay.pad + g_lay.fontH;

    // App name — large bold
    HWND nameH = CreateWindowW(L"STATIC", L"Gõ Việt", WS_CHILD | WS_VISIBLE,
                               g_lay.pad, y, g_lay.panelW - g_lay.pad * 2,
                               abs(g_lay.fontH) * 2 + 8, p, nullptr, HI(), nullptr);
    SendMessageW(nameH, WM_SETFONT, reinterpret_cast<WPARAM>(g_fontLg), FALSE);
    y += abs(g_lay.fontH) * 2 + 16;

    abLblVer_ = Lbl(p, L"", g_lay.pad, y, g_lay.panelW - g_lay.pad * 2);
    y += g_lay.lblH + GAP;

    Lbl(p, L"Modern Vietnamese Input Method Engine", g_lay.pad, y,
        g_lay.panelW - g_lay.pad * 2);
    y += g_lay.lblH + 4;
    Lbl(p, L"Bộ gõ tiếng Việt hiệu suất cao, đa nền tảng.", g_lay.pad, y,
        g_lay.panelW - g_lay.pad * 2);
    y += g_lay.lblH + g_lay.pad;

    Sep(p, y); y += 12;

    int bh = g_lay.rowS - 2;
    Btn(p, L"GitHub Repository",  g_lay.pad,        y, 158, bh, IDC_ABOUT_GITHUB);
    Btn(p, L"Báo lỗi / Issues",   g_lay.pad + 166,  y, 148, bh, IDC_ABOUT_ISSUES);
    Btn(p, L"Kiểm tra cập nhật",  g_lay.pad + 322,  y, 154, bh, IDC_ABOUT_UPDATE);
    y += bh + g_lay.pad;

    Sep(p, y); y += 12;
    Lbl(p, L"Core Engine: Rust (goxviet_core)", g_lay.pad, y, g_lay.panelW - g_lay.pad * 2);
    y += g_lay.lblH + 6;
    Lbl(p, L"UI: Native Win32 C++20", g_lay.pad, y, g_lay.panelW - g_lay.pad * 2);
    y += g_lay.lblH + 6;
    Lbl(p, L"Hỗ trợ kiến trúc: x86 / x64 / ARM64", g_lay.pad, y, g_lay.panelW - g_lay.pad * 2);
    y += g_lay.lblH + g_lay.pad;

    Sep(p, y); y += 12;
    Lbl(p, L"License: MIT  |  Copyright © 2024–2026 GoxViet Contributors",
        g_lay.pad, y, g_lay.panelW - g_lay.pad * 2);
    y += g_lay.lblH + 6;
    Lbl(p, L"Cảm ơn cộng đồng IME tiếng Việt và các đóng góp mã nguồn mở.",
        g_lay.pad, y, g_lay.panelW - g_lay.pad * 2);
    y += g_lay.lblH + g_lay.pad;

    FfiVersionInfo vi{};
    if (RustBridge::Instance().GetVersion(&vi) == FfiStatusCode::Success) {
        wchar_t vs[80];
        StringCchPrintfW(vs, 80, L"goxviet_core v%u.%u.%u", vi.major, vi.minor, vi.patch);
        SetWindowTextW(abLblVer_, vs);
    }

    SetPanelContentH(p, y);
    return p;
}

// ── WndProc ────────────────────────────────────────────────────────────────────
LRESULT CALLBACK SettingsWindow::WndProc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
    SettingsWindow* wnd = nullptr;
    if (msg == WM_CREATE) {
        wnd = reinterpret_cast<SettingsWindow*>(
            reinterpret_cast<CREATESTRUCTW*>(lp)->lpCreateParams);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, reinterpret_cast<LONG_PTR>(wnd));
    } else {
        wnd = reinterpret_cast<SettingsWindow*>(GetWindowLongPtrW(hwnd, GWLP_USERDATA));
    }

    switch (msg) {
    case WM_SIZE: {
        if (!wnd) return 0;
        int w = LOWORD(lp), h = HIWORD(lp);
        if (wnd->sidebar_)
            SetWindowPos(wnd->sidebar_, nullptr, 0, 0, SIDE_W, h,
                         SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE);
        for (int i = 0; i < 4; ++i)
            if (wnd->panels_[i])
                SetWindowPos(wnd->panels_[i], nullptr,
                             SIDE_W + 1, 0, max(w - SIDE_W - 1, 400), h,
                             SWP_NOZORDER | SWP_NOACTIVATE);
        return 0;
    }
    case WM_GETMINMAXINFO: {
        auto* mm = reinterpret_cast<MINMAXINFO*>(lp);
        mm->ptMinTrackSize = { MIN_W, MIN_H };
        return 0;
    }
    case WM_ERASEBKGND: {
        HDC hdc = reinterpret_cast<HDC>(wp);
        RECT r; GetClientRect(hwnd, &r);
        const auto& th = ui::GetTheme();
        RECT sr = { 0, 0, SIDE_W, r.bottom };
        HBRUSH sb = CreateSolidBrush(th.sidebarBg); FillRect(hdc, &sr, sb); DeleteObject(sb);
        RECT cr = { SIDE_W, 0, r.right, r.bottom };
        HBRUSH cb = CreateSolidBrush(th.windowBg);  FillRect(hdc, &cr, cb); DeleteObject(cb);
        HPEN pen = CreatePen(PS_SOLID, 1, th.border);
        HPEN old = static_cast<HPEN>(SelectObject(hdc, pen));
        MoveToEx(hdc, SIDE_W, 0, nullptr); LineTo(hdc, SIDE_W, r.bottom);
        SelectObject(hdc, old); DeleteObject(pen);
        return 1;
    }
    case WM_PAINT: { PAINTSTRUCT ps; BeginPaint(hwnd, &ps); EndPaint(hwnd, &ps); return 0; }

    case WM_MEASUREITEM: {
        auto* mis = reinterpret_cast<MEASUREITEMSTRUCT*>(lp);
        if (mis->CtlID == 9001) {
            mis->itemHeight = max(g_lay.fontH + 16, 36);
            return TRUE;
        }
        return FALSE;
    }
    case WM_DRAWITEM: {
        auto* dis = reinterpret_cast<DRAWITEMSTRUCT*>(lp);
        if (dis->CtlID != 9001) return FALSE;
        const auto& th = ui::GetTheme();
        bool sel = (dis->itemState & ODS_SELECTED) != 0;
        HBRUSH hbr = CreateSolidBrush(sel ? th.accent : th.sidebarBg);
        FillRect(dis->hDC, &dis->rcItem, hbr); DeleteObject(hbr);
        wchar_t text[64] = {};
        SendMessageW(dis->hwndItem, LB_GETTEXT, dis->itemID, (LPARAM)text);
        if (g_font) SelectObject(dis->hDC, g_font);
        SetTextColor(dis->hDC, sel ? RGB(255,255,255) : th.textPrimary);
        SetBkMode(dis->hDC, TRANSPARENT);
        RECT tr = dis->rcItem; tr.left += 14;
        DrawTextW(dis->hDC, text, -1, &tr, DT_VCENTER | DT_SINGLELINE | DT_LEFT);
        return TRUE;
    }

    case WM_COMMAND:
        if (!wnd) return 0;
        if (LOWORD(wp) == 9001 && HIWORD(wp) == LBN_SELCHANGE) {
            int sel = (int)SendMessageW(reinterpret_cast<HWND>(lp), LB_GETCURSEL, 0, 0);
            if (sel >= 0) wnd->SwitchTab(static_cast<SettingsTab>(sel));
            return 0;
        }
        switch (LOWORD(wp)) {
        case IDC_CMB_METHOD: case IDC_CMB_TONE: case IDC_CMB_ENCODING: case IDC_CMB_RESTORE_MOD:
            if (HIWORD(wp) == CBN_SELCHANGE) wnd->SaveFromGeneral(); break;
        case IDC_BTN_RESET:
            if (MessageBoxW(hwnd, L"Đặt lại tất cả cài đặt về mặc định?",
                            L"Xác nhận", MB_YESNO | MB_ICONQUESTION) == IDYES) {
                Settings::Instance().ResetToDefaults();
                wnd->RefreshGeneral();
            } break;
        case IDC_TOGGLE_PERAPP:
            Settings::Instance().perApp = ui::GetToggleState(wnd->paTog_);
            Settings::Instance().Save(); break;
        case IDC_PA_BTN_TOGGLE: wnd->PerAppToggleSelected(); break;
        case IDC_PA_BTN_REMOVE: wnd->PerAppRemoveSelected(); break;
        case IDC_BTN_ADD:    wnd->TxAddEntry();       break;
        case IDC_BTN_DELETE: wnd->TxDeleteSelected(); break;
        case IDC_BTN_IMPORT: wnd->TxImport();         break;
        case IDC_BTN_EXPORT: wnd->TxExport();         break;
        case IDC_ABOUT_GITHUB:
            ShellExecuteW(nullptr, L"open", L"https://github.com/nihmtaho/goxviet-ime",
                          nullptr, nullptr, SW_SHOWNORMAL); break;
        case IDC_ABOUT_ISSUES:
            ShellExecuteW(nullptr, L"open", L"https://github.com/nihmtaho/goxviet-ime/issues",
                          nullptr, nullptr, SW_SHOWNORMAL); break;
        case IDC_ABOUT_UPDATE:
            ShellExecuteW(nullptr, L"open", L"https://github.com/nihmtaho/goxviet-ime/releases",
                          nullptr, nullptr, SW_SHOWNORMAL); break;
        }
        return 0;

    case WM_TOGGLE_CHANGED:
        if (wnd) {
            switch (wnd->tab_) {
            case SettingsTab::General:       wnd->SaveFromGeneral(); break;
            case SettingsTab::TextExpansion:
                Settings::Instance().SetEnableShortcuts(ui::GetToggleState(wnd->txTog_));
                break;
            default: break;
            }
        }
        return 0;

    case WM_CLOSE:
        ShowWindow(hwnd, SW_HIDE);
        if (wnd) wnd->visible_ = false;
        return 0;
    case WM_DESTROY:
        if (wnd) { wnd->hwnd_ = nullptr; wnd->visible_ = false; }
        return 0;
    }
    return DefWindowProcW(hwnd, msg, wp, lp);
}

}  // namespace goxviet
