# Settings UI & Configuration

The configuration interface is built purely in **SwiftUI**, leveraging `AppStorage` for persistence and `NotificationCenter` for state synchronization.

## Settings Architecture (`SettingsRootView.swift`)

The settings window uses a Tab-based layout (`TabView`) with four main sections:

1.  **General**: Core typing behavior (Method, Tone Style).
2.  **Per-App**: Smart Mode configuration.
3.  **Advanced**: Metrics and Debugging.
4.  **About**: Version info and updates.

### State Synchronization
To keep the native Menu Bar in sync with the SwiftUI Settings window:
-   **`AppState` Singleton**: The source of truth. Wraps `UserDefaults` access.
-   **`NotificationCenter`**:
    -   When a setting changes in SwiftUI, `AppState` posts a notification (e.g., `.inputMethodChanged`).
    -   `AppDelegate` observes these notifications to update the Menu Bar checkmarks immediately.

## Per-App Configuration (`PerAppSettingsView`)

GoxViet supports a "Smart Mode" that remembers the toggle state for each application.

-   **Data Structure**: Stored as a dictionary `[BundleID: Bool]` in `UserDefaults`.
-   **Logic**:
    -   When switching to an app, `PerAppModeManager` checks the dictionary.
    -   If an entry exists, it automatically enables/disables the IME.
    -   If no entry exists, it uses the global default.
-   **UI**:
    -   Displays a list of "Known Apps" (apps where the user has explicitly toggled the IME).
    -   Allows users to clear individual app settings or reset all.

## Shortcut Recording (`ShortcutRecorder.swift`)

A custom implementation for capturing keyboard shortcuts within SwiftUI.
-   Captures modifiers (`NSEvent.ModifierFlags`).
-   Prevents capturing system-critical shortcuts (e.g., Cmd+Tab).
-   Stores shortcuts as a custom `KeyboardShortcut` struct (optionally serialized to JSON/Data for persistence).
