// Maps Windows Virtual Key (VK_*) codes to ASCII chars the Rust engine accepts.
// The engine's key event system handles: a-z, 0-9, backspace (0x08), ESC (0x1B), space (0x20).
// Any VK code not in _map is not processable and must pass through unmodified.

namespace GoxViet.Input;

internal static class VkMapper
{
    // VK → lowercase ASCII char.
    // The engine receives lowercase; caps/shift state is passed as separate bool parameters.
    private static readonly Dictionary<int, char> _map = new()
    {
        // VK_A (0x41) .. VK_Z (0x5A) — Windows VK codes for letters equal ASCII uppercase
        [0x41] = 'a', [0x42] = 'b', [0x43] = 'c', [0x44] = 'd', [0x45] = 'e',
        [0x46] = 'f', [0x47] = 'g', [0x48] = 'h', [0x49] = 'i', [0x4A] = 'j',
        [0x4B] = 'k', [0x4C] = 'l', [0x4D] = 'm', [0x4E] = 'n', [0x4F] = 'o',
        [0x50] = 'p', [0x51] = 'q', [0x52] = 'r', [0x53] = 's', [0x54] = 't',
        [0x55] = 'u', [0x56] = 'v', [0x57] = 'w', [0x58] = 'x', [0x59] = 'y',
        [0x5A] = 'z',
        // VK_0 (0x30) .. VK_9 (0x39) — same as ASCII digits
        [0x30] = '0', [0x31] = '1', [0x32] = '2', [0x33] = '3', [0x34] = '4',
        [0x35] = '5', [0x36] = '6', [0x37] = '7', [0x38] = '8', [0x39] = '9',
        // Special keys the engine understands
        [0x08] = '\b',    // VK_BACK — backspace (engine handles undo)
        [0x1B] = '\x1B',  // VK_ESCAPE — restore to raw
        [0x20] = ' ',     // VK_SPACE — word boundary flush
    };

    // Keys that terminate a word without being part of it.
    // Engine must receive these to finalize the current word, but they pass through unchanged.
    private static readonly HashSet<int> _breakKeys = new()
    {
        0x0D,             // VK_RETURN
        0x09,             // VK_TAB
        0x25, 0x26, 0x27, 0x28, // VK_LEFT, VK_UP, VK_RIGHT, VK_DOWN
        0xBE,             // VK_OEM_PERIOD (.)
        0xBC,             // VK_OEM_COMMA (,)
        0xBF,             // VK_OEM_2 (/)
        0xBA,             // VK_OEM_1 (;)
        0xDE,             // VK_OEM_7 (')
        0xDB,             // VK_OEM_4 ([)
        0xDD,             // VK_OEM_6 (])
        0xC0,             // VK_OEM_3 (`)
        0xBB,             // VK_OEM_PLUS (=)
        0xBD,             // VK_OEM_MINUS (-)
    };

    /// Returns the ASCII char for this VK code, or null if not processable by the engine.
    public static char? ToAscii(int vkCode) =>
        _map.TryGetValue(vkCode, out var c) ? c : null;

    /// True if this key ends the current word (pass through but flush the buffer).
    public static bool IsBreakKey(int vkCode) => _breakKeys.Contains(vkCode);
}
