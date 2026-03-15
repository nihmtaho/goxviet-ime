#include "app_compat.h"
#include <psapi.h>
#include <shlwapi.h>
#include <algorithm>
#include <cwctype>
#pragma comment(lib, "shlwapi.lib")

namespace goxviet {

AppCompat& AppCompat::Instance() {
    static AppCompat instance;
    return instance;
}

std::wstring AppCompat::GetForegroundAppName() {
    HWND hwnd = GetForegroundWindow();
    if (!hwnd) return {};

    DWORD pid;
    GetWindowThreadProcessId(hwnd, &pid);
    HANDLE hProc = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid);
    if (!hProc) return {};

    wchar_t path[MAX_PATH];
    DWORD size = MAX_PATH;
    bool ok = QueryFullProcessImageNameW(hProc, 0, path, &size) != 0;
    CloseHandle(hProc);
    if (!ok) return {};

    // Return only the filename without extension, lower-cased
    wchar_t* name = PathFindFileNameW(path);
    std::wstring result(name);
    std::transform(result.begin(), result.end(), result.begin(), ::towlower);
    // Remove .exe suffix
    if (result.size() > 4 && result.substr(result.size() - 4) == L".exe")
        result.resize(result.size() - 4);
    return result;
}

DetectionResult AppCompat::GetInjectionMethod() {
    DWORD now = GetTickCount();
    if (hasCached_ && (now - cachedResult_.timestamp) < CACHE_TTL_MS)
        return cachedResult_;

    std::wstring app = GetForegroundAppName();
    cachedResult_ = Detect(app);
    cachedApp_    = app;
    hasCached_    = true;
    return cachedResult_;
}

void AppCompat::ClearDetectionCache() {
    hasCached_ = false;
    cachedApp_.clear();
}

DetectionResult AppCompat::Detect(const std::wstring& app) {
    // Terminals — slow injection
    static const wchar_t* slow[] = {
        L"windowsterminal", L"cmd", L"powershell", L"pwsh",
        L"wt", L"alacritty", L"mintty", L"conhost",
        L"wezterm-gui", L"hyper",
    };
    for (auto* s : slow)
        if (app == s)
            return DetectionResult(InjectionMethod::Slow, 8000, 25000, 8000);

    // Browser address bars — selection method (handled dynamically)
    if (NeedsSelectionMethod())
        return DetectionResult(InjectionMethod::Selection, 200, 800, 500);

    // IDEs / code editors — fast
    static const wchar_t* fast[] = {
        L"code", L"cursor", L"devenv", L"rider64", L"idea64",
        L"pycharm64", L"webstorm64", L"clion64", L"goland64",
        L"sublime_text", L"notepad++", L"notepad", L"wordpad",
        L"atom", L"zed",
    };
    for (auto* f : fast)
        if (app == f)
            return DetectionResult(InjectionMethod::Fast, 100, 400, 200);

    // Default: fast timing
    return DetectionResult(InjectionMethod::Fast, 200, 800, 500);
}

bool AppCompat::NeedsSelectionMethod() {
    HWND hwnd = GetForegroundWindow();
    if (!hwnd) return false;

    // Check if focused child control is an Edit/ComboBox in a browser's address bar
    HWND focused = GetFocus();
    if (!focused) return false;

    wchar_t className[128] = {};
    GetClassNameW(focused, className, 128);

    // Chrome/Edge omnibox uses OmniboxViewViews
    if (wcsstr(className, L"OmniboxView") != nullptr) return true;
    if (wcscmp(className, L"Chrome_OmniboxView") == 0)  return true;

    return false;
}

}  // namespace goxviet
