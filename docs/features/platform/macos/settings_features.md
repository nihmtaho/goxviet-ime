# Settings Features

This document provides a detailed technical breakdown of the features available in each tab of the GoxViet Settings window.

## General Tab

The General tab controls the global behavior of the Input Engine and Application integration.

### Core Typing Configuration
*   **Input Method**: Switches the syntax rule set used by the engine.
    *   **Telex**: Uses repeated keys and letter keys (s, f, r, x, j, etc.) for tone marks and diacritics.
    *   **VNI**: Uses number keys (1-9) for tone marks and diacritics.
    *   *Implementation*: Maps directly to `ime_method(u8)` in the Rust core.
*   **Tone Placement**: Defines the aesthetic rule for placing tone marks on dipthongs/tripthongs.
    *   **Traditional**: Places tone on the first applicable vowel (e.g., `hòa`, `thủy`).
    *   **Modern**: Places tone on the second/main vowel (e.g., `hoà`, `thuý`).
    *   *Implementation*: Controls the `modern` boolean flag in `EngineConfig`.

### Smart Features
*   **Free Tone Placement**:
    *   Allows users to type tone marks at arbitrary positions in a word (e.g., `c-s-a` -> `cá`).
    *   When enabled, the engine skips strict "valid vowel" checks during tone processing.
*   **Instant Auto-Restore**:
    *   A heuristic feature that attempts to detect when a user is typing an English word while in Vietnamese mode.
    *   *Logic*: If the engine detects a valid dictionary word that would otherwise be transformed (e.g., "root" -> "rốt"), it automatically restores the raw input "root" without waiting for a space or boundary.
*   **Auto-Disable for Non-Latin**:
    *   A background monitor (`InputSourceMonitor`) that watches for system input source changes.
    *   If the user switches to a Chinese, Japanese, or Korean keyboard, GoxViet automatically suspends itself to prevent conflict.

### Application Behavior
*   **Hide from Dock**:
    *   Toggles the application's activation policy.
    *   **On**: App runs as an "Accessory" (Menu bar only).
    *   **Off**: App runs as a "Regular" app (Dock icon + Menu bar).

## Per-App Tab

The "Smart Mode" feature allows for granular control over where GoxViet is active.

### Smart Per-App Mode
*   **Functionality**: Automatically saves the "Enabled/Disabled" state of the IME for each specific application bundle ID.
*   **Storage**: Maintains a persistent dictionary in `UserDefaults`:
    ```swift
    // Format: [BundleID: IsEnabled]
    {
        "com.apple.Terminal": false,
        "com.microsoft.VSCode": false,
        "com.apple.mail": true
    }
    ```
*   **Lifecycle**:
    *   Monitors `NSWorkspace.didActivateApplicationNotification`.
    *   On app switch, looks up the target app in the dictionary.
    *   If found, applies the saved state. If not found, uses the global default without persisting.
    *   Opt-in tracking: an app is only added to the dictionary after the user enables Vietnamese for that app at least once. Turning it off (English) for a new app does not create a saved entry. Once saved via enabling, later OFF/ON changes are persisted.

## Advanced Tab

### Keyboard Shortcuts
*   **Toggle Shortcut**:
    *   Customizable global hotkey to switch GoxViet on/off.
    *   **Defaults**: `Ctrl + Space`.
    *   **Supported Modifiers**: Command, Option, Control, Shift, Fn.
    *   **Implementation**: Uses `CGEventTap` to intercept the specific key combination before it reaches the active application.

### Engine Telemetry
Displays real-time statistics from the internal counters:
*   **Total Keystrokes**: Number of keys processed by the engine.
*   **Backspace Count**: Number of artificial backspaces injected by the engine to correct text.
*   **Avg Buffer**: Average length of the pre-edit buffer (performance metric).

## About Tab

### Update System
*   **UpdateManager**: A background service that periodically queries the GitHub Releases API.
*   **States**:
    *   `Idle`: No update check in progress.
    *   `Checking`: Querying API.
    *   `UpdateAvailable`: A new version tag is found (higher than `CFBundleShortVersionString`).
    *   `Downloading`: Fetching the `.zip` asset.
    *   `ReadyToInstall`: Asset verified and unpacked.
*   **Mechanism**: Uses `Sparkle` or a custom `UpdateManager` (in this case, custom Swift implementation) to handle the download, verification, and atomic swap of the application bundle.
