#include "vk_mapper.h"
#include <algorithm>

namespace goxviet {

const VkMapper& VkMapper::Instance() {
    static VkMapper instance;
    return instance;
}

static constexpr DWORD kBreakKeys[] = {
    VK_RETURN, VK_TAB,
    VK_LEFT, VK_RIGHT, VK_UP, VK_DOWN,
    VK_HOME, VK_END, VK_PRIOR, VK_NEXT,
    VK_DELETE, VK_INSERT,
    VK_F1, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6,
    VK_F7, VK_F8, VK_F9, VK_F10, VK_F11, VK_F12,
    VK_OEM_PERIOD, VK_OEM_COMMA, VK_OEM_2, VK_OEM_1, VK_OEM_7,
    VK_OEM_4, VK_OEM_6, VK_OEM_3, VK_OEM_5,
    VK_OEM_PLUS, VK_OEM_MINUS, VK_OEM_102,
};

bool VkMapper::IsBreak(DWORD vk) const {
    for (DWORD bk : kBreakKeys) if (vk == bk) return true;
    return false;
}

KeyInfo VkMapper::Map(DWORD vk, bool caps, bool shift) const {
    bool upper = shift ^ caps;

    // a–z
    if (vk >= 'A' && vk <= 'Z')
        return { static_cast<int8_t>(upper ? (int)vk : (int)vk + 32), false };

    // 0–9
    if (vk >= '0' && vk <= '9') {
        if (!shift) return { static_cast<int8_t>(vk), false };
        static const char shiftDigits[] = ")!@#$%^&*(";
        return { static_cast<int8_t>(shiftDigits[vk - '0']), false };
    }

    if (vk == VK_SPACE)   return { 0x20, false };
    if (vk == VK_BACK)    return { 0x08, false };
    if (vk == VK_ESCAPE)  return { 0x1B, false };

    if (IsBreak(vk)) return { 0, true };

    return { 0, false };
}

}  // namespace goxviet
