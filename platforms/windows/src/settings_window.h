#pragma once
// Settings window — four-tab layout matching macOS:
//   Cài đặt chung | Theo ứng dụng | Gõ tắt | Về ứng dụng
//
// Each tab is a child panel window. SwitchTab() just shows/hides panels.

#include <windows.h>
#include <commctrl.h>
#include "settings.h"

namespace goxviet {

enum class SettingsTab { General = 0, PerApp, TextExpansion, About };

class SettingsWindow {
public:
    static SettingsWindow& Instance();
    void Show(void* engine, SettingsTab tab = SettingsTab::General);

private:
    SettingsWindow() = default;
    ~SettingsWindow();
    SettingsWindow(const SettingsWindow&) = delete;
    SettingsWindow& operator=(const SettingsWindow&) = delete;

    void Create();
    void BuildSidebar();
    void SwitchTab(SettingsTab tab);

    // Each builder returns a panel window containing all controls for that tab
    HWND BuildGeneralPanel();
    HWND BuildPerAppPanel();
    HWND BuildTextExpansionPanel();
    HWND BuildAboutPanel();

    // Refresh helpers (reload settings → controls)
    void RefreshGeneral();
    void RefreshPerApp();
    void RefreshTextExpansion();

    // Save / action helpers
    void SaveFromGeneral();
    void PerAppRebuildList();
    void PerAppToggleSelected();
    void PerAppRemoveSelected();
    void TxRebuildList();
    void TxAddEntry();
    void TxDeleteSelected();
    void TxImport();
    void TxExport();

    static LRESULT CALLBACK WndProc(HWND, UINT, WPARAM, LPARAM);
    static LRESULT CALLBACK PanelProc(HWND, UINT, WPARAM, LPARAM);

    // ── Main window ──────────────────────────────────────────────────────────
    HWND hwnd_    = nullptr;
    HWND sidebar_ = nullptr;
    bool visible_ = false;
    void* engine_ = nullptr;
    SettingsTab tab_ = SettingsTab::General;

    // ── Panels (one per tab) ─────────────────────────────────────────────────
    HWND panels_[4] = {};

    // ── General tab controls ─────────────────────────────────────────────────
    HWND gTogEnabled_   = nullptr;
    HWND gCmbMethod_    = nullptr;
    HWND gCmbTone_      = nullptr;
    HWND gTogFreeTone_  = nullptr;
    HWND gTogInstant_   = nullptr;
    HWND gTogAutoNL_    = nullptr;
    HWND gTogEsc_       = nullptr;
    HWND gCmbRestore_   = nullptr;
    HWND gTogShiftBS_   = nullptr;
    HWND gCmbEncoding_  = nullptr;
    HWND gTogShortcuts_ = nullptr;
    HWND gTogAutoStart_ = nullptr;
    HWND gTogSound_     = nullptr;
    HWND gLblVersion_   = nullptr;

    // ── Per-App tab controls ─────────────────────────────────────────────────
    HWND paTog_         = nullptr;
    HWND paList_        = nullptr;
    HWND paBtnToggle_   = nullptr;
    HWND paBtnRemove_   = nullptr;
    HWND paLblCount_    = nullptr;

    // ── Text Expansion tab controls ──────────────────────────────────────────
    HWND txTog_         = nullptr;
    HWND txList_        = nullptr;
    HWND txEditTrig_    = nullptr;
    HWND txEditRepl_    = nullptr;
    HWND txBtnAdd_      = nullptr;
    HWND txBtnDel_      = nullptr;
    HWND txBtnImp_      = nullptr;
    HWND txBtnExp_      = nullptr;
    HWND txLblCount_    = nullptr;

    // ── About tab controls ───────────────────────────────────────────────────
    HWND abLblVer_      = nullptr;
};

}  // namespace goxviet
