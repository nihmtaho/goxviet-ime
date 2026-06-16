#pragma once
#include <windows.h>
#include <string>
#include <cstdint>

namespace goxviet {

enum class InjectionMethod : uint8_t {
    Fast      = 0,   // Default — minimal delays
    Slow      = 1,   // Terminals, heavy Electron apps
    Selection = 2,   // Browser address bars (Shift+Left select + retype)
};

struct InjectionTiming {
    uint32_t backspaceDelayUs = 200;
    uint32_t waitDelayUs      = 800;
    uint32_t textDelayUs      = 500;
};

struct DetectionResult {
    InjectionMethod method  = InjectionMethod::Fast;
    InjectionTiming timing;
    DWORD           timestamp = 0;

    DetectionResult() = default;
    DetectionResult(InjectionMethod m, uint32_t bs, uint32_t wait, uint32_t text)
        : method(m), timing{bs, wait, text}, timestamp(GetTickCount()) {}
};

class AppCompat {
public:
    static AppCompat& Instance();

    std::wstring GetForegroundAppName();
    DetectionResult GetInjectionMethod();
    void ClearDetectionCache();

private:
    AppCompat() = default;
    AppCompat(const AppCompat&) = delete;
    AppCompat& operator=(const AppCompat&) = delete;

    DetectionResult Detect(const std::wstring& appName);
    bool NeedsSelectionMethod();

    std::wstring    cachedApp_;
    DetectionResult cachedResult_;
    bool            hasCached_ = false;
    static constexpr DWORD CACHE_TTL_MS = 200;
};

}  // namespace goxviet
