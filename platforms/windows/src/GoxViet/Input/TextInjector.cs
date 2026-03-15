// Injects replacement text via Win32 SendInput.
// Uses KEYEVENTF_UNICODE for Vietnamese characters (UTF-16 code units).
// Marks all injected events with INJECTED_MARKER in dwExtraInfo so the hook
// can identify and pass them through without re-processing.

using System.Runtime.InteropServices;

namespace GoxViet.Input;

public sealed class TextInjector
{
    public static readonly TextInjector Instance = new();

    private const uint INPUT_KEYBOARD    = 1;
    private const uint KEYEVENTF_KEYUP   = 0x0002;
    private const uint KEYEVENTF_UNICODE = 0x0004;
    private const uint VK_BACK           = 0x08;

    // Sentinel value written to dwExtraInfo on every injected event.
    // The hook callback checks this to avoid processing its own output.
    // "VNIE" = 0x564E4945
    internal const uint INJECTED_MARKER = 0x564E4945;

    private TextInjector() { }

    /// Send backspaceCount backspaces then insert the replacement text.
    public void Inject(byte backspaceCount, string text)
    {
        var inputs = new List<INPUT>(backspaceCount * 2 + text.Length * 2);

        // Phase 1: backspaces to erase the original in-progress text
        for (int i = 0; i < backspaceCount; i++)
        {
            inputs.Add(KeyboardInput(VK_BACK, 0, 0));
            inputs.Add(KeyboardInput(VK_BACK, 0, KEYEVENTF_KEYUP));
        }

        // Phase 2: inject replacement as Unicode code units
        // KEYEVENTF_UNICODE requires UTF-16 wScan — surrogate pairs need two events each
        foreach (char c in text)
        {
            inputs.Add(UnicodeInput(c, 0));
            inputs.Add(UnicodeInput(c, KEYEVENTF_KEYUP));
        }

        if (inputs.Count == 0) return;

        var arr = inputs.ToArray();
        uint sent = Win32.SendInput((uint)arr.Length, arr, Marshal.SizeOf<INPUT>());
        if (sent != arr.Length)
        {
            System.Diagnostics.Debug.WriteLine(
                $"[GoxViet] SendInput: requested {arr.Length}, sent {sent}");
        }
    }

    private static INPUT KeyboardInput(uint vk, ushort scan, uint flags) => new()
    {
        type = INPUT_KEYBOARD,
        u    = new INPUTUNION
        {
            ki = new KEYBDINPUT
            {
                wVk         = (ushort)vk,
                wScan       = scan,
                dwFlags     = flags,
                time        = 0,
                dwExtraInfo = new IntPtr(INJECTED_MARKER),
            }
        }
    };

    private static INPUT UnicodeInput(char c, uint flags) => new()
    {
        type = INPUT_KEYBOARD,
        u    = new INPUTUNION
        {
            ki = new KEYBDINPUT
            {
                wVk         = 0,
                wScan       = c,              // UTF-16 code unit goes in wScan for KEYEVENTF_UNICODE
                dwFlags     = KEYEVENTF_UNICODE | flags,
                time        = 0,
                dwExtraInfo = new IntPtr(INJECTED_MARKER),
            }
        }
    };

    [StructLayout(LayoutKind.Sequential)]
    internal struct INPUT
    {
        public uint      type;
        public INPUTUNION u;
    }

    [StructLayout(LayoutKind.Explicit)]
    internal struct INPUTUNION
    {
        [FieldOffset(0)] public KEYBDINPUT ki;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct KEYBDINPUT
    {
        public ushort wVk;
        public ushort wScan;
        public uint   dwFlags;
        public uint   time;
        public IntPtr dwExtraInfo;
    }
}
