// System tray icon and context menu.
// Reacts to SettingsManager.Changed to update icon state and tooltip.

using System.Drawing;
using System.Reflection;
using System.Windows;
using System.Windows.Forms;
using GoxViet.Settings;

namespace GoxViet.UI;

public sealed class TrayIcon : IDisposable
{
    public static readonly TrayIcon Instance = new();

    private readonly NotifyIcon _notifyIcon;
    private SettingsWindow?     _settingsWindow;
    private bool                _disposed;

    private TrayIcon()
    {
        _notifyIcon = new NotifyIcon
        {
            Visible = true,
            Text    = "GoxViet",
            Icon    = LoadIcon(SettingsManager.Instance.IsEnabled),
        };

        var menu = new ContextMenuStrip();
        menu.Items.Add("Bật/Tắt (Ctrl+Space)", null, (_, _) =>
            SettingsManager.Instance.ToggleEnabled());
        menu.Items.Add("Cài đặt...", null, (_, _) => OpenSettings());
        menu.Items.Add(new ToolStripSeparator());
        menu.Items.Add("Thoát", null, (_, _) => Shutdown());

        _notifyIcon.ContextMenuStrip = menu;
        _notifyIcon.DoubleClick     += (_, _) => OpenSettings();

        SettingsManager.Instance.Changed += OnSettingsChanged;
    }

    private void OnSettingsChanged(object? sender, AppSettings settings)
    {
        _notifyIcon.Icon = LoadIcon(settings.IsEnabled);
        _notifyIcon.Text = settings.IsEnabled ? "GoxViet (Bật)" : "GoxViet (Tắt)";
    }

    private void OpenSettings()
    {
        if (_settingsWindow == null || !_settingsWindow.IsLoaded)
            _settingsWindow = new SettingsWindow();
        _settingsWindow.Show();
        _settingsWindow.Activate();
    }

    private static void Shutdown()
    {
        System.Windows.Application.Current.Shutdown();
    }

    private static Icon LoadIcon(bool enabled)
    {
        string name   = enabled ? "tray_on" : "tray_off";
        var    stream = Assembly.GetExecutingAssembly()
            .GetManifestResourceStream($"GoxViet.Resources.{name}.ico");
        return stream != null
            ? new Icon(stream)
            : SystemIcons.Application;
    }

    public void Dispose()
    {
        if (_disposed) return;
        _disposed = true;
        _notifyIcon.Visible = false;
        _notifyIcon.Dispose();
    }
}
