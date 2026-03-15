// Code-behind for SettingsWindow.xaml.
// Binds UI controls to SettingsManager and saves on confirm.

using System.Windows;
using GoxViet.FFI;
using GoxViet.Settings;

namespace GoxViet.UI;

public partial class SettingsWindow : Window
{
    public SettingsWindow()
    {
        InitializeComponent();
        LoadFromSettings();
        ShowVersion();
    }

    private void LoadFromSettings()
    {
        var s = SettingsManager.Instance.Current;
        ChkEnabled.IsChecked        = s.IsEnabled;
        CboInputMethod.SelectedIndex = (int)s.InputMethod;
        CboToneStyle.SelectedIndex   = (int)s.ToneStyle;
        ChkSmartMode.IsChecked      = s.SmartMode;
        ChkInstantRestore.IsChecked = s.InstantRestore;
    }

    private void ShowVersion()
    {
        try
        {
            var (major, minor, patch) = GoxViet.FFI.RustBridge.Instance.GetVersion();
            TxtVersion.Text = $"Engine v{major}.{minor}.{patch}";
        }
        catch
        {
            TxtVersion.Text = string.Empty;
        }
    }

    private void Save_Click(object sender, RoutedEventArgs e)
    {
        var updated = new AppSettings
        {
            IsEnabled      = ChkEnabled.IsChecked      == true,
            InputMethod    = (FfiInputMethod)(CboInputMethod.SelectedIndex >= 0 ? CboInputMethod.SelectedIndex : 0),
            ToneStyle      = (FfiToneStyle)(CboToneStyle.SelectedIndex >= 0 ? CboToneStyle.SelectedIndex : 0),
            SmartMode      = ChkSmartMode.IsChecked      == true,
            InstantRestore = ChkInstantRestore.IsChecked == true,
            EscRestore     = SettingsManager.Instance.Current.EscRestore,
        };
        SettingsManager.Instance.Apply(updated);
        Close();
    }

    private void Cancel_Click(object sender, RoutedEventArgs e) => Close();
}
