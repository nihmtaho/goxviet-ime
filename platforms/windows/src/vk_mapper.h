#pragma once
// VkMapper — maps Windows Virtual Key codes to ASCII chars expected by goxviet_core.
// Extracted as its own unit (Single Responsibility Principle).
// Equivalent to KeyCodes.swift on macOS.

#include <windows.h>
#include <cstdint>

namespace goxviet {

struct KeyInfo {
    int8_t  ch       = 0;     // ASCII char for the engine, 0 if unmappable
    bool    isBreak  = false; // resets composition buffer
};

class VkMapper {
public:
    static const VkMapper& Instance();

    // Map a VK code to the info the engine expects.
    // caps = CapsLock state, shift = Shift held.
    KeyInfo Map(DWORD vk, bool caps, bool shift) const;

    // True if this VK should reset the composition buffer and pass through.
    bool IsBreak(DWORD vk) const;

private:
    VkMapper() = default;
};

}  // namespace goxviet
