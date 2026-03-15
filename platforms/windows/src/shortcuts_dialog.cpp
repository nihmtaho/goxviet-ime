#include "shortcuts_dialog.h"
#include "settings.h"
#include "modern_ui.h"
#include "resource.h"
#include "utils.h"
#include <commctrl.h>
#include <commdlg.h>
#include <strsafe.h>

namespace goxviet {

ShortcutsDialog& ShortcutsDialog::Instance() { static ShortcutsDialog i; return i; }
ShortcutsDialog::~ShortcutsDialog() { if (hwnd_) DestroyWindow(hwnd_); }

void ShortcutsDialog::Show(void* engine) {
    engine_ = engine;
    if (!hwnd_) Create();
    if (!hwnd_)  return;
    Rebuild();
    ShowWindow(hwnd_, SW_SHOWNORMAL);
    SetForegroundWindow(hwnd_);
    visible_ = true;
}

void ShortcutsDialog::Create() {
    HINSTANCE hInst = GetModuleHandleW(nullptr);
    static const wchar_t* CLASS = L"GoxVietShortcuts";
    WNDCLASSEXW wc = {};
    wc.cbSize = sizeof(wc); wc.lpfnWndProc = WndProc;
    wc.hInstance = hInst; wc.hCursor = LoadCursor(nullptr, IDC_ARROW);
    wc.lpszClassName = CLASS;
    RegisterClassExW(&wc);

    hwnd_ = CreateWindowExW(WS_EX_DLGMODALFRAME, CLASS, L"Từ viết tắt — Gõ Việt",
                            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME,
                            CW_USEDEFAULT, CW_USEDEFAULT, 540, 440,
                            nullptr, nullptr, hInst, this);
    if (!hwnd_) return;

    listView_ = CreateWindowExW(WS_EX_CLIENTEDGE, WC_LISTVIEW, L"",
                                WS_CHILD | WS_VISIBLE | LVS_REPORT | LVS_SINGLESEL | LVS_SHOWSELALWAYS,
                                10, 10, 510, 290, hwnd_,
                                reinterpret_cast<HMENU>(IDC_SHORTCUTS_LIST), hInst, nullptr);
    ListView_SetExtendedListViewStyle(listView_, LVS_EX_FULLROWSELECT | LVS_EX_GRIDLINES);

    LVCOLUMNW col = {};
    col.mask = LVCF_TEXT | LVCF_WIDTH;
    col.cx = 160; col.pszText = const_cast<wchar_t*>(L"Viết tắt");
    ListView_InsertColumn(listView_, 0, &col);
    col.cx = 330; col.pszText = const_cast<wchar_t*>(L"Thay thế");
    ListView_InsertColumn(listView_, 1, &col);

    CreateWindowW(L"STATIC", L"Từ tắt:", WS_CHILD | WS_VISIBLE,
                  10, 310, 55, 22, hwnd_, nullptr, hInst, nullptr);
    editTrig_ = CreateWindowExW(WS_EX_CLIENTEDGE, L"EDIT", L"",
                                WS_CHILD | WS_VISIBLE | ES_AUTOHSCROLL,
                                70, 310, 130, 22, hwnd_,
                                reinterpret_cast<HMENU>(IDC_EDIT_TRIGGER), hInst, nullptr);
    CreateWindowW(L"STATIC", L"Thay thế:", WS_CHILD | WS_VISIBLE,
                  215, 310, 65, 22, hwnd_, nullptr, hInst, nullptr);
    editRepl_ = CreateWindowExW(WS_EX_CLIENTEDGE, L"EDIT", L"",
                                WS_CHILD | WS_VISIBLE | ES_AUTOHSCROLL,
                                285, 310, 160, 22, hwnd_,
                                reinterpret_cast<HMENU>(IDC_EDIT_REPLACEMENT), hInst, nullptr);
    CreateWindowW(L"BUTTON", L"Thêm", WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
                  455, 310, 68, 22, hwnd_,
                  reinterpret_cast<HMENU>(IDC_BTN_ADD), hInst, nullptr);

    CreateWindowW(L"BUTTON", L"Xoá", WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
                  10, 346, 80, 28, hwnd_,
                  reinterpret_cast<HMENU>(IDC_BTN_DELETE), hInst, nullptr);
    CreateWindowW(L"BUTTON", L"Nhập (trigger:value)", WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
                  100, 346, 160, 28, hwnd_,
                  reinterpret_cast<HMENU>(IDC_BTN_IMPORT), hInst, nullptr);
    CreateWindowW(L"BUTTON", L"Xuất", WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
                  270, 346, 80, 28, hwnd_,
                  reinterpret_cast<HMENU>(IDC_BTN_EXPORT), hInst, nullptr);
}

void ShortcutsDialog::Rebuild() {
    if (!listView_) return;
    ListView_DeleteAllItems(listView_);
    const auto& shortcuts = Settings::Instance().shortcuts;
    for (int i = 0; i < (int)shortcuts.size(); ++i) {
        LVITEMW item = {}; item.mask = LVIF_TEXT; item.iItem = i;
        item.pszText = const_cast<wchar_t*>(shortcuts[i].trigger.c_str());
        ListView_InsertItem(listView_, &item);
        ListView_SetItemText(listView_, i, 1,
                             const_cast<wchar_t*>(shortcuts[i].replacement.c_str()));
    }
}

void ShortcutsDialog::AddEntry() {
    wchar_t trig[128] = {}, repl[512] = {};
    GetWindowTextW(editTrig_, trig, 128);
    GetWindowTextW(editRepl_, repl, 512);
    if (!trig[0] || !repl[0]) return;
    Settings::Instance().AddShortcut(trig, repl);
    if (engine_) Settings::Instance().SyncShortcutsToEngine(engine_);
    SetWindowTextW(editTrig_, L""); SetWindowTextW(editRepl_, L"");
    Rebuild();
}

void ShortcutsDialog::DeleteSelected() {
    int idx = ListView_GetNextItem(listView_, -1, LVNI_SELECTED);
    if (idx < 0) return;
    const auto& sc = Settings::Instance().shortcuts;
    if (idx < (int)sc.size()) {
        Settings::Instance().RemoveShortcut(sc[idx].trigger);
        if (engine_) Settings::Instance().SyncShortcutsToEngine(engine_);
    }
    Rebuild();
}

void ShortcutsDialog::ImportFile() {
    wchar_t path[MAX_PATH] = {};
    OPENFILENAMEW ofn = {};
    ofn.lStructSize = sizeof(ofn); ofn.hwndOwner = hwnd_;
    ofn.lpstrFilter = L"Shortcut files (*.txt;*.csv)\0*.txt;*.csv\0All\0*.*\0";
    ofn.lpstrFile = path; ofn.nMaxFile = MAX_PATH;
    ofn.Flags = OFN_FILEMUSTEXIST;
    if (!GetOpenFileNameW(&ofn)) return;
    int n = Settings::Instance().ImportShortcuts(path);
    if (engine_) Settings::Instance().SyncShortcutsToEngine(engine_);
    wchar_t msg[64]; StringCchPrintfW(msg, 64, L"Đã nhập %d từ viết tắt.", n);
    MessageBoxW(hwnd_, msg, L"Nhập thành công", MB_OK);
    Rebuild();
}

void ShortcutsDialog::ExportFile() {
    wchar_t path[MAX_PATH] = {};
    OPENFILENAMEW ofn = {};
    ofn.lStructSize = sizeof(ofn); ofn.hwndOwner = hwnd_;
    ofn.lpstrFilter = L"Shortcut files (*.txt)\0*.txt\0";
    ofn.lpstrFile = path; ofn.nMaxFile = MAX_PATH;
    ofn.Flags = OFN_OVERWRITEPROMPT; ofn.lpstrDefExt = L"txt";
    if (!GetSaveFileNameW(&ofn)) return;
    Settings::Instance().ExportShortcuts(path);
}

LRESULT CALLBACK ShortcutsDialog::WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam) {
    ShortcutsDialog* dlg = nullptr;
    if (msg == WM_CREATE) {
        dlg = reinterpret_cast<ShortcutsDialog*>(
            reinterpret_cast<CREATESTRUCTW*>(lParam)->lpCreateParams);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, reinterpret_cast<LONG_PTR>(dlg));
    } else {
        dlg = reinterpret_cast<ShortcutsDialog*>(GetWindowLongPtrW(hwnd, GWLP_USERDATA));
    }
    switch (msg) {
    case WM_COMMAND:
        if (!dlg) return 0;
        switch (LOWORD(wParam)) {
        case IDC_BTN_ADD:    dlg->AddEntry();    break;
        case IDC_BTN_DELETE: dlg->DeleteSelected(); break;
        case IDC_BTN_IMPORT: dlg->ImportFile();  break;
        case IDC_BTN_EXPORT: dlg->ExportFile();  break;
        }
        return 0;
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
