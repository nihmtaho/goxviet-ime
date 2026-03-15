// Application entry point.
// Startup order: Load settings → Init Rust engine → Install keyboard hook → Show tray icon.
// Runs as a tray-only application (ShutdownMode.OnExplicitShutdown, no main window).

using System.Windows;
using GoxViet.FFI;
using GoxViet.Input;
using GoxViet.Settings;
using GoxViet.UI;
using Application = System.Windows.Application;

namespace GoxViet;

public partial class App : Application
{
    protected override void OnStartup(StartupEventArgs e)
    {
        base.OnStartup(e);

        // No main window — app lives in the system tray
        ShutdownMode = ShutdownMode.OnExplicitShutdown;

        // 1. Load persisted settings
        SettingsManager.Instance.Load();

        // 2. Initialize Rust engine with persisted config
        var settings = SettingsManager.Instance.Current;
        var engineConfig = new FfiConfig_v2
        {
            InputMethod           = settings.InputMethod,
            ToneStyle             = settings.ToneStyle,
            SmartMode             = settings.SmartMode,
            InstantRestoreEnabled = settings.InstantRestore,
            EscRestoreEnabled     = settings.EscRestore,
            EnableShortcuts       = true,
        };
        RustBridge.Instance.Initialize(engineConfig);

        // 3. Install global keyboard hook (must run on UI thread — it already is)
        KeyboardHook.Instance.Install();

        // 4. Show system tray icon (accessing Instance triggers the constructor)
        _ = TrayIcon.Instance;
    }

    protected override void OnExit(ExitEventArgs e)
    {
        KeyboardHook.Instance.Dispose();
        TrayIcon.Instance.Dispose();
        RustBridge.Instance.Dispose();
        base.OnExit(e);
    }
}
