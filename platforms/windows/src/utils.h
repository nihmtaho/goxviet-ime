#pragma once
#include <windows.h>
#include <string>

namespace goxviet {

void PlayToggleSound();

void LogInfo(const std::wstring& message);
void LogError(const std::wstring& message);

}  // namespace goxviet
