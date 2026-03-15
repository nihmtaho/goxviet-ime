// Singleton settings manager.
// Persists settings to %APPDATA%\GoxViet\settings.json.
// Exposes Changed event so TrayIcon and KeyboardHook can react to updates.

using System.Text.Json;
using GoxViet.FFI;

namespace GoxViet.Settings;

public sealed class SettingsManager
{
    public static readonly SettingsManager Instance = new();

    private static readonly string SettingsDir = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData), "GoxViet");
    private static readonly string SettingsPath = Path.Combine(SettingsDir, "settings.json");

    private static readonly JsonSerializerOptions JsonOptions = new() { WriteIndented = true };

    public event EventHandler<AppSettings>? Changed;

    public AppSettings Current { get; private set; } = new();
    public bool IsEnabled => Current.IsEnabled;

    private SettingsManager() { }

    public void Load()
    {
        try
        {
            if (File.Exists(SettingsPath))
            {
                string json = File.ReadAllText(SettingsPath);
                Current = JsonSerializer.Deserialize<AppSettings>(json) ?? new();
            }
        }
        catch
        {
            Current = new();
        }
    }

    public void Save()
    {
        try
        {
            Directory.CreateDirectory(SettingsDir);
            string json = JsonSerializer.Serialize(Current, JsonOptions);
            File.WriteAllText(SettingsPath, json);
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine($"[GoxViet] Settings save failed: {ex.Message}");
        }
    }

    /// Apply updated settings: persist, sync to engine, fire event.
    public void Apply(AppSettings updated)
    {
        Current = updated;
        Save();
        SyncToEngine();
        Changed?.Invoke(this, Current);
    }

    /// Toggle IME on/off.
    public void ToggleEnabled()
    {
        Current.IsEnabled = !Current.IsEnabled;
        Save();
        SyncToEngine();
        Changed?.Invoke(this, Current);
    }

    private void SyncToEngine()
    {
        var cfg = new FfiConfig_v2
        {
            InputMethod           = Current.InputMethod,
            ToneStyle             = Current.ToneStyle,
            SmartMode             = Current.SmartMode,
            InstantRestoreEnabled = Current.InstantRestore,
            EscRestoreEnabled     = Current.EscRestore,
            EnableShortcuts       = true,
        };
        try
        {
            RustBridge.Instance.SetConfig(cfg);
        }
        catch
        {
            // Engine may not be initialized yet during startup — that's fine,
            // App.xaml.cs will initialize it with these settings.
        }
    }
}
