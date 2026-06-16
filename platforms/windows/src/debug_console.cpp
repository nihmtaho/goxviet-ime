#include "debug_console.h"

namespace goxviet {

DebugConsole& DebugConsole::Instance() {
    static DebugConsole instance;
    return instance;
}

DebugConsole::~DebugConsole() {
    if (available_) FreeConsole();
}

void DebugConsole::Create() {
    if (available_) return;
    if (!AllocConsole()) return;

    FILE* fp;
    freopen_s(&fp, "CONOUT$", "w", stdout);
    freopen_s(&fp, "CONOUT$", "w", stderr);
    SetConsoleTitleW(L"Gõ Việt Debug Console");
    SetConsoleOutputCP(CP_UTF8);
    available_ = true;

    wprintf(L"══════════════════════════════════════\n");
    wprintf(L"  Gõ Việt — Debug Console\n");
    wprintf(L"══════════════════════════════════════\n\n");
}

void DebugConsole::Log(const std::wstring& message) {
    OutputDebugStringW(message.c_str());
    OutputDebugStringW(L"\n");
    if (available_) {
        wprintf(L"%s\n", message.c_str());
        fflush(stdout);
    }
}

}  // namespace goxviet
