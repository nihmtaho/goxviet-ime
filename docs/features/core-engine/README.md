# Vietnamese IME Core Engine

The core engine (`core/src`) is a high-performance, portable library for Vietnamese input method processing. It is written in Rust and designed to be embedded in various applications via FFI.

## Overview

The engine processes keystrokes and maintains the state of the current word being composed. It supports both Telex and VNI input methods.

### Key Features

- **Validation-First Architecture**: Ensures that the input buffer always contains valid Vietnamese syllables before applying transformations.
- **Pattern-Based Processing**: Scans the entire buffer for patterns rather than relying on complex state machines for every character.
- **FFI Support**: Exposes a C-compatible API for integration with macOS, Windows, Linux, and mobile platforms.
- **Configurable**: Supports options for specific behaviors like `w` shortcut, modern tone placement, and English auto-restore.
- **English Detection**: sophisticated English word detection to prevent accidental Vietnamese transformations on English words.
- **History Tracking**: Maintains word history for advanced editing features like "backspace-after-space" restoration.

## Directory Structure

- **`lib.rs`**: The main entry point for the library, defining the FFI interface.
- **`engine/`**: The core processing logic.
    - **`mod.rs`**: Main `Engine` struct and processing pipeline.
    - **`buffer/`**: Internal buffer representation.
    - **`english/`**: English detection and word lists.
    - **`vietnamese/`**: Vietnamese specific transformations and validation.
    - **`features/`**: Additional features like shortcuts.
- **`input/`**: Input method definitions (Telex, VNI).
- **`data/`**: Static data, including character maps and keys.
- **`utils.rs`**: Common utility functions.
- **`updater/`**: Update mechanism (separate from the core input logic).

## Usage

The engine is typically initialized once. For each keystroke, the application calls `ime_key` with the key code and modification flags. The engine returns an `ImeResult` containing the action to perform (e.g., replace text, restore text).

See [Library API](./lib.md) for detailed FFI documentation.
