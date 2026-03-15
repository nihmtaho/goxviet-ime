#include "utils.h"
#include <sstream>
#include <iomanip>
#include <ctime>

namespace goxviet {

void PlayToggleSound() {
    MessageBeep(MB_OK);
}

static std::wstring GetTimestamp() {
    auto now = std::time(nullptr);
    struct tm tm;
    localtime_s(&tm, &now);
    std::wostringstream wss;
    wss << std::put_time(&tm, L"%H:%M:%S");
    return wss.str();
}

void LogInfo(const std::wstring& message) {
    OutputDebugStringW((L"[GoxViet INFO][" + GetTimestamp() + L"] " + message + L"\n").c_str());
}

void LogError(const std::wstring& message) {
    OutputDebugStringW((L"[GoxViet ERR ][" + GetTimestamp() + L"] " + message + L"\n").c_str());
}

}  // namespace goxviet
