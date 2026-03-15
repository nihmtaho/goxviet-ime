#pragma once
#include <windows.h>
#include <shellapi.h>

namespace goxviet {

class SystemTray {
public:
    static SystemTray& Instance();

    bool Create(HWND hwnd);
    void Destroy();
    void UpdateIcon();
    void HandleMessage(WPARAM wParam, LPARAM lParam);

private:
    SystemTray() = default;
    ~SystemTray() { Destroy(); }
    SystemTray(const SystemTray&) = delete;
    SystemTray& operator=(const SystemTray&) = delete;

    void ShowContextMenu();

    HWND             hwnd_    = nullptr;
    NOTIFYICONDATAW  nid_     = {};
    bool             created_ = false;
};

}  // namespace goxviet
