#pragma once
#include <windows.h>
#include <string>

namespace goxviet {

class DebugConsole {
public:
    static DebugConsole& Instance();

    void Create();
    void Log(const std::wstring& message);
    bool IsAvailable() const { return available_; }

private:
    DebugConsole() = default;
    ~DebugConsole();
    DebugConsole(const DebugConsole&) = delete;
    DebugConsole& operator=(const DebugConsole&) = delete;

    bool available_ = false;
};

}  // namespace goxviet
