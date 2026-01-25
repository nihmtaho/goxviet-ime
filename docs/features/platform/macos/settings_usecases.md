# Settings Use Cases

This document details the user interactions and workflows available within the GoxViet Settings window.

## General Tab

The primary hub for configuring core typing behavior.

| Feature | Use Case | User Action | System Behavior |
| :--- | :--- | :--- | :--- |
| **Input Method** | User prefers Telex or VNI style. | Selects "Telex" or "VNI" from the segment picker. | Immediately updates `AppState`. `InputManager` switches the internal engine processing method. |
| **Tone Placement** | User prefers modern orthography (`hoà`) over traditional (`hòa`). | Toggles "Modern" / "Traditional" radio buttons. | Updates `AppState`. Engine revalidates tone positioning rules on subsequent keystrokes. |
| **Free Tone** | User wants to type tone marks at any time (e.g., `ngyx` -> `ngỹ`) rather than waiting for vowels. | Toggles "Free tone placement". | Disables strict vowel-tone validation in the engine. |
| **Instant Restore** | User frequently types mixed English/Vietnamese and wants instant correction for English words. | Toggles "Auto-restore English words". | Engine aggressively checks dictionary after every key; if an English word is detected, it reverts transformations immediately. |
| **Auto-Disable** | User uses CJK keyboards and wants GoxViet to step aside. | Toggles "Auto-disable for non-Latin keyboards". | `InputSourceMonitor` actively checks the system input source and effectively disables GoxViet if a non-Latin source is active. |
| **Dock Visibility** | User prefers a stealthy, menu-bar-only application. | Toggles "Hide from Dock". | Changes `NSApplication.ActivationPolicy` between `.regular` (dock icon) and `.accessory` (no dock icon). |

## Per-App Tab

Manages the "Smart Mode" which remembers the IME state for individual applications.

### Smart Mode Workflow

1.  **Activates**: User switches to an application (e.g., "Terminal").
2.  **Opt-in Tracking**:
	- If the user has never enabled Vietnamese for this app, no record is created; the global default (English) applies.
	- When the user turns GoxViet **ON** for this app the first time, GoxViet saves `{ "com.apple.Terminal": true }` and starts tracking this app.
3.  **Subsequent Changes**: After the app is tracked, turning **OFF** later updates the saved entry to `{ "com.apple.Terminal": false }`.
4.  **Return**: On future switches back to Terminal, the saved state is restored automatically.

### Management UI

| Feature | Use Case | User Action |
| :--- | :--- | :--- |
| **Enable/Disable** | User wants global consistent state vs. per-app memory. | Toggles "Enable Smart Per-App Mode". |
| **View Saved Apps** | User wants to see which apps have custom states. | Scrolls through the "Saved Applications" list. Only apps that have been enabled at least once appear here. Shows App Name, Bundle ID, and saved state (ON/OFF). |
| **Remove App** | User wants an app to revert to following the global state. | Clicks the "X" button next to an app in the list. |
| **Clear All** | User wants to reset learning history. | Clicks "Clear All Settings" -> Confirms in alert. |

## Advanced Tab

Tools for power users and debugging.

| Feature | Use Case | User Action |
| :--- | :--- | :--- |
| **Shortcut Recording** | User wants to change the toggle shortcut (e.g., to `Cmd+Shift+Space`). | Clicks actual shortcut button -> Press new key combination. **Supports**: modifiers (Cmd, Opt, Ctrl, Shift) and Fn keys. |
| **Performance Metrics** | User is curious about typing efficiency. | Views real-time stats: Total Keystrokes, Backspace Count (corrections), Avg Buffer Length. |
| **Debug Logs** | User encounters a bug and needs to send logs to developer. | Clicks "Open Log File". Opens the log text file in the default editor (Console/TextEdit). |

## About Tab

Information and software updates.

### Update Workflow

1.  **Check**: App automatically checks for updates when the About tab is viewed (if strict interval passed). User can manually click "Check for Updates".
2.  **Download**: If a new version exists, a "Download Update" button appears properly labeled with the new version number.
3.  **Progress**: A circular progress bar shows download percentage.
4.  **Install**: Upon completion, the status changes to "Ready to Install". The app will automatically relaunch to apply the update.

### Other Actions
-   **GitHub Link**: Opens source repository.
-   **Report Issue**: Opens GitHub Issues page for bug reporting.
