// Settings POCO — serialized to %APPDATA%\GoxViet\settings.json

using GoxViet.FFI;

namespace GoxViet.Settings;

public sealed class AppSettings
{
    public bool           IsEnabled      { get; set; } = true;
    public FfiInputMethod InputMethod    { get; set; } = FfiInputMethod.Telex;
    public FfiToneStyle   ToneStyle      { get; set; } = FfiToneStyle.New;
    public bool           SmartMode      { get; set; } = true;
    public bool           InstantRestore { get; set; } = true;
    public bool           EscRestore     { get; set; } = false;
}
