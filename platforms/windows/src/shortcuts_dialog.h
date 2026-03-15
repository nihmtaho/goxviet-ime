#pragma once
#include <windows.h>
#include <commctrl.h>

namespace goxviet {

class ShortcutsDialog {
public:
    static ShortcutsDialog& Instance();
    void Show(void* engine);

private:
    ShortcutsDialog() = default;
    ~ShortcutsDialog();
    ShortcutsDialog(const ShortcutsDialog&) = delete;
    ShortcutsDialog& operator=(const ShortcutsDialog&) = delete;

    void Create();
    void Rebuild();
    void AddEntry();
    void DeleteSelected();
    void ImportFile();
    void ExportFile();
    void ApplyAndReload();

    static LRESULT CALLBACK WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam);

    HWND hwnd_      = nullptr;
    HWND listView_  = nullptr;
    HWND editTrig_  = nullptr;
    HWND editRepl_  = nullptr;
    bool visible_   = false;
    void* engine_   = nullptr;
};

}  // namespace goxviet
