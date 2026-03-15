// Global low-level keyboard hook (WH_KEYBOARD_LL).
// Captures all keystrokes system-wide, routes them through the Rust engine,
// and injects replacement text via TextInjector.
//
// CRITICAL RULES:
// 1. The delegate (_proc) MUST be kept alive for the hook lifetime — stored as a field.
// 2. NEVER let an exception escape HookCallback — it silently kills the hook.
// 3. Injected events (dwExtraInfo == INJECTED_MARKER) must pass through unchanged.
// 4. Ctrl/Alt combos bypass the engine and reset state.
// 5. Install() must be called from a thread that pumps a Windows message loop (WPF UI thread).

using System.Diagnostics;
using System.Runtime.InteropServices;
using GoxViet.FFI;
using GoxViet.Settings;

namespace GoxViet.Input;

public sealed class KeyboardHook : IDisposable
{
    public static readonly KeyboardHook Instance = new();

    private const int WH_KEYBOARD_LL = 13;
    private const int WM_KEYDOWN     = 0x0100;
    private const int WM_SYSKEYDOWN  = 0x0104;

    private const int VK_SHIFT   = 0x10;
    private const int VK_CAPITAL = 0x14;
    private const int VK_CONTROL = 0x11;
    private const int VK_MENU    = 0x12; // Alt

    private const uint TOGGLE_HOTKEY_ID = 1;
    private const uint MOD_CONTROL      = 0x0002;
    private const int  VK_SPACE         = 0x20;

    private IntPtr _hookHandle  = IntPtr.Zero;
    private IntPtr _hwndMessage = IntPtr.Zero;

    // Store delegate in a field — GC will collect it if it's only in a local variable,
    // which silently uninstalls the hook.
    private readonly LowLevelKeyboardProc _proc;
    private bool _disposed;

    private KeyboardHook()
    {
        _proc = HookCallback;
    }

    /// Install the global keyboard hook. Must be called from the WPF UI thread.
    public void Install()
    {
        if (_hookHandle != IntPtr.Zero) return;

        using var module = Process.GetCurrentProcess().MainModule!;
        _hookHandle = Win32.SetWindowsHookEx(
            WH_KEYBOARD_LL,
            _proc,
            Win32.GetModuleHandle(module.ModuleName!),
            0);

        if (_hookHandle == IntPtr.Zero)
            throw new InvalidOperationException(
                $"SetWindowsHookEx failed with error {Marshal.GetLastWin32Error()}");

        // Register Ctrl+Space toggle hotkey
        Win32.RegisterHotKey(IntPtr.Zero, (int)TOGGLE_HOTKEY_ID, MOD_CONTROL, (uint)VK_SPACE);
    }

    /// Uninstall the hook and unregister the toggle hotkey.
    public void Uninstall()
    {
        if (_hookHandle != IntPtr.Zero)
        {
            Win32.UnhookWindowsHookEx(_hookHandle);
            _hookHandle = IntPtr.Zero;
        }
        Win32.UnregisterHotKey(IntPtr.Zero, (int)TOGGLE_HOTKEY_ID);
    }

    // The actual hook callback. Must never throw.
    private IntPtr HookCallback(int nCode, IntPtr wParam, IntPtr lParam)
    {
        if (nCode < 0)
            return Win32.CallNextHookEx(_hookHandle, nCode, wParam, lParam);

        try
        {
            int msg = (int)wParam;
            if (msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN)
            {
                var kbd = Marshal.PtrToStructure<KBDLLHOOKSTRUCT>(lParam);

                // Skip events injected by us to prevent re-processing
                if (kbd.dwExtraInfo.ToInt64() == TextInjector.INJECTED_MARKER)
                    return Win32.CallNextHookEx(_hookHandle, nCode, wParam, lParam);

                bool suppress = ProcessKey(in kbd);
                if (suppress)
                    return (IntPtr)1; // Non-zero return value suppresses the event
            }
        }
        catch (Exception ex)
        {
            // Log but never propagate — an unhandled exception here kills the hook permanently
            Debug.WriteLine($"[GoxViet] Hook callback exception: {ex}");
        }

        return Win32.CallNextHookEx(_hookHandle, nCode, wParam, lParam);
    }

    private bool ProcessKey(in KBDLLHOOKSTRUCT kbd)
    {
        int vk = (int)kbd.vkCode;

        bool ctrl  = (Win32.GetKeyState(VK_CONTROL) & 0x8000) != 0;
        bool alt   = (Win32.GetKeyState(VK_MENU)    & 0x8000) != 0;
        bool shift = (Win32.GetKeyState(VK_SHIFT)   & 0x8000) != 0;
        bool caps  = (Win32.GetKeyState(VK_CAPITAL) & 0x0001) != 0;

        // Ctrl/Alt combos (shortcuts, copy/paste, etc.) bypass IME
        if (ctrl || alt)
        {
            RustBridge.Instance.ResetAll();
            return false;
        }

        // IME disabled — pass through and reset buffer
        if (!SettingsManager.Instance.IsEnabled)
        {
            RustBridge.Instance.ResetAll();
            return false;
        }

        // Break keys flush the buffer but pass through to the application unchanged
        if (VkMapper.IsBreakKey(vk))
        {
            RustBridge.Instance.ResetBuffer();
            return false;
        }

        char? ascii = VkMapper.ToAscii(vk);
        if (ascii == null)
        {
            // Non-letter/digit keys (function keys, arrows, etc.) reset state
            if (vk != VK_SHIFT && vk != VK_CONTROL && vk != VK_CAPITAL && vk != VK_MENU)
                RustBridge.Instance.ResetAll();
            return false;
        }

        // CapsLock and Shift XOR — Shift inverts caps state (same as macOS logic)
        bool capsActive = caps ^ shift;
        sbyte keyChar   = (sbyte)ascii.Value;

        ProcessResult result;
        try
        {
            result = RustBridge.Instance.ProcessKey(keyChar, capsActive, shift, ctrl: false);
        }
        catch (Exception ex)
        {
            Debug.WriteLine($"[GoxViet] ProcessKey failed: {ex.Message}");
            return false;
        }

        if (!result.Consumed)
            return false;

        if (result.BackspaceCount > 0 || !string.IsNullOrEmpty(result.Text))
        {
            TextInjector.Instance.Inject(result.BackspaceCount, result.Text);
            return true; // suppress original keystroke
        }

        return false;
    }

    public void Dispose()
    {
        if (_disposed) return;
        _disposed = true;
        Uninstall();
    }

    private delegate IntPtr LowLevelKeyboardProc(int nCode, IntPtr wParam, IntPtr lParam);

    [StructLayout(LayoutKind.Sequential)]
    private struct KBDLLHOOKSTRUCT
    {
        public uint   vkCode;
        public uint   scanCode;
        public uint   flags;
        public uint   time;
        public IntPtr dwExtraInfo;
    }
}

/// Win32 APIs for hook and input management.
/// Kept separate from NativeMethods.cs (which owns only Rust FFI DllImports).
internal static class Win32
{
    [DllImport("user32.dll", SetLastError = true)]
    internal static extern IntPtr SetWindowsHookEx(
        int idHook, Delegate lpfn, IntPtr hMod, uint dwThreadId);

    [DllImport("user32.dll", SetLastError = true)]
    internal static extern bool UnhookWindowsHookEx(IntPtr hhk);

    [DllImport("user32.dll")]
    internal static extern IntPtr CallNextHookEx(
        IntPtr hhk, int nCode, IntPtr wParam, IntPtr lParam);

    [DllImport("kernel32.dll", CharSet = CharSet.Auto)]
    internal static extern IntPtr GetModuleHandle(string lpModuleName);

    [DllImport("user32.dll")]
    internal static extern short GetKeyState(int vkCode);

    [DllImport("user32.dll", SetLastError = true)]
    internal static extern uint SendInput(
        uint nInputs, TextInjector.INPUT[] pInputs, int cbSize);

    [DllImport("user32.dll", SetLastError = true)]
    internal static extern bool RegisterHotKey(
        IntPtr hWnd, int id, uint fsModifiers, uint vk);

    [DllImport("user32.dll", SetLastError = true)]
    internal static extern bool UnregisterHotKey(IntPtr hWnd, int id);
}
