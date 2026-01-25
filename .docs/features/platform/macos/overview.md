# macOS Platform Architecture

The macOS platform implementation of GoxViet is a **hybrid application** combining a high-performance Rust core with a native Swift/Cocoa frontend.

## High-Level Architecture

The application follows a layered architecture designed for low latency and native system integration.

```mermaid
graph TD
    User[User Keyboard Input] --> EventTap[CGEventTap (Swift)]
    EventTap --> InputManager[InputManager (Swift)]
    
    subgraph Swift Layer
        InputManager --> AppState[AppState (Settings)]
        InputManager --> RustBridge[RustBridge (FFI Wrapper)]
        InputManager --> TextInjector[TextInjector (Output)]
    end
    
    subgraph Rust Core
        RustBridge --> FFIBoundary[C FFI Boundary]
        FFIBoundary --> Engine[Core Engine (Rust)]
        Engine --> Buffer[Input Buffer]
        Engine --> English[English Detection]
    end
    
    TextInjector --> TargetApp[Target Application]
```

## Key Components

### 1. Application Entry Point (`AppDelegate.swift`)
-   **Status Bar App**: Runs primarily as a `LSUIElement` (agent application) in the menu bar.
-   **Lifecycle Management**: Handles app launch, termination, and window activation.
-   **Permission Handling**: Checks for Accessibility API permissions (`AXIsProcessTrusted`) required for key interception.

### 2. Input Management (`InputManager.swift`)
The heart of the macOS platform layer.
-   **Event Tapping**: Uses `CGEvent.tapCreate` to intercept system-wide keyboard events.
-   **Filtering**: Selectively captures relevant keystrokes while passing through command shortcuts and navigation keys.
-   **Dispatch**: Forwards valid keys to the Rust core for processing.

### 3. State Management (`AppState.swift`, `SettingsRootView.swift`)
-   **Persistent Settings**: Stores user preferences (Input Method, Tone Style) using `UserDefaults`.
-   **Per-App State**: Tracks enabled/disabled state for individual applications (Smart Mode).
-   **Reactive UI**: Uses SwiftUI (`ObservableObject`) to update the Settings window and Menu Bar instantly.

### 4. Rust Bridge (`RustBridge.swift`)
-   **FFI Wrapper**: Provides a safe Swift API over the unsafe C bindings exposed by `goxviet-Bridging-Header.h`.
-   **Memory Safety**: Manages the allocation and freeing of pointers returned by the Rust core.

## Directory Structure

```text
platforms/macos/goxviet/goxviet/
├── App/
│   ├── AppDelegate.swift       # App lifecycle & Menu Bar setup
│   └── GoxVietApp.swift        # SwiftUI App definition
├── Input/
│   ├── InputManager.swift      # Core event loop & logic
│   ├── TextInjectionHelper.swift # Text insertion logic
│   └── RustBridge.swift        # FFI bridge to Rust
├── UI/
│   ├── SettingsRootView.swift  # Main settings window
│   ├── MenuToggleView.swift    # Custom menu items
│   └── ...
├── Core/
│   └── AppState.swift          # Central state management
└── Resources/
    └── goxviet-Bridging-Header.h # C header for Rust FFI
```
