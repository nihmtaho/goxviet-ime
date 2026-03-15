#pragma once
#include <windows.h>

namespace goxviet {

class AboutDialog {
public:
    static AboutDialog& Instance();
    void Show();

private:
    AboutDialog() = default;
    ~AboutDialog();
    AboutDialog(const AboutDialog&) = delete;
    AboutDialog& operator=(const AboutDialog&) = delete;

    void Create();
    static LRESULT CALLBACK WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam);

    HWND hwnd_    = nullptr;
    bool visible_ = false;
};

}  // namespace goxviet
