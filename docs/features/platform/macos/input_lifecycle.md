# Input Handling & Lifecycle

## Lifecycle Management (`InputManager.swift`)

The `InputManager` is a singleton that manages the active session of the IME. It implements the `LifecycleManaged` protocol.

### Start Sequence
1.  **Permission Check**: `AppDelegate` verifies Accessibility permissions (`AXIsProcessTrusted`).
2.  **Initialization**: `InputManager.shared.start()` is called.
3.  **Tap Creation**: Creates a `CGEventTap` at `.cghidEventTap` location, formatted as `.headInsertEventTap`.
4.  **Run Loop**: Adds the tap source to the current `CFRunLoop` (`.commonModes`).

### Event Loop (`handleEvent`)
Every keystroke on the system passes through `handleEvent`. The pipeline is:

1.  **Self-Check**: Ignores events injected by GoxViet itself (marker `0x564E5F494D45`).
2.  **Modifier filtering**: Passes through purely modifier keys (Cmd, Ctrl, Opt), unless they trigger a specific shortcut.
3.  **Toggle Check**: Checks if the key matches the Global Toggle Shortcut (default `Ctrl+Space`).
4.  **State Check**: If IME is disabled (globally or for current app), pass event through.
5.  **Rust Processing**: Calls `ime_key_ext` via `RustBridge`.

## Text Injection (`TextInjectionHelper.swift`)

When the Rust core returns a transformation, the `InputManager` must update the target application's text field.

### Processing Results
The helper handles three main actions from the Rust core:

-   **Pass Through (`Action = 0`)**: The event is returned `Unmanaged.passUnretained(event)`, allowing the system to handle it normally.
-   **Send (`Action = 1`)**:
    1.  **Backspace**: Deletes `N` characters (the partial buffer) using `CGEvent` posts (Backspace key).
    2.  **Insert**: Inserts the new Vietnamese text.
    -   *Optimization*: The input manager often coalesces backspaces or diffs the current buffer to minimize flicker.
-   **Restore (`Action = 2`)**: Used for "Undo" functionality (e.g., pressing ESC).
    -   Deletes the current transformed word.
    -   Re-types the original raw keystrokes.

### Backspace Handling
To reduce flicker, GoxViet handles the Backspace key (`Keycode 51`) specially:
-   Instead of letting the system handle it, the engine calculates the new state.
-   If the engine is empty, it passes the backspace through.
-   If the engine has content, it instructs the `injector` to update the pre-edit text in place.

## App Lifecycle (`AppDelegate.swift`)

-   **Launch**:
    -   Registers defaults.
    -   checks Accessibility permissions continuously (polling timer).
    -   Sets up the Menu Bar item (`NSStatusItem`).
-   **Activation**:
    -   Observes `NSApplication.didBecomeActiveNotification`.
    -   Re-checks permissions if they were previously missing.
-   **Termination**:
    -   Ensures `InputManager` is stopped cleanly.
    -   Removes the Status Item.
