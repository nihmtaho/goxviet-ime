# ⚠️ DEPRECATED – Engine Module (`engine/`)

> **Lưu ý:** Module `engine/` đã được migrate sang Clean Architecture trong v3.0.0.  
> Tài liệu này được giữ lại cho mục đích lịch sử. **Không còn phản ánh codebase hiện tại.**
>
> **Xem thay thế:** [clean-architecture.md](../clean-architecture.md)  
> **Code mới tại:** `infrastructure/engine/` và `infrastructure/adapters/`

---

## Migration Map

| Legacy path | New path (v3.0.0) |
|---|---|
| `engine/mod.rs` (Engine struct) | `presentation/di/container.rs` (Container) + `application/use_cases/process_keystroke.rs` |
| `engine/buffer/` | `infrastructure/engine/buffer/` + `shared/buffer/` |
| `engine/english/` | `infrastructure/adapters/validation/english/` |
| `engine/vietnamese/` | `infrastructure/adapters/transformation/` + `infrastructure/adapters/validation/` |
| `engine/features/` | `infrastructure/engine/features/` + `features/` |
| `engine/state/history.rs` | `infrastructure/engine/state/history.rs` + `infrastructure/adapters/state/simple_history_adapter.rs` |

---

# [Historical] Engine Overview (`engine/mod.rs`)

The `Engine` struct was the central component of the library prior to v3.0.0. It orchestrated the input processing pipeline, managing state, validation, transformation, and output generation.

## Architecture

The engine used a **validation-first, pattern-based** approach:

1.  **Maintains a Buffer**: Holds the current word being composed.
2.  **Scans the Buffer**: On every keystroke, checking for patterns (English words, shortcuts, Vietnamese structures).
3.  **Transforms**: Applies changes to the buffer if valid (e.g., adding a tone, modifying a vowel).
4.  **Rebuilds Output**: Generates the final result for the application.

## Core `Engine` Struct (Historical)

- **State:** `buf`, `method`, `shortcuts`, `raw_input`, `word_history`
- **Configuration Flags:** `enabled`, `raw_mode`, `skip_w_shortcut`, `esc_restore_enabled`, `free_tone_enabled`, `modern_tone`, `instant_restore_enabled`

## Key Processing Pipeline (Historical)

1. English Detection (Layer 1)
2. Modifier Check (tone/vowel/stroke)
3. Transformation Attempts (stroke → tone → mark → remove → w-shortcut)
4. Normal Letter
5. Output Rebuild (`rebuild_output_from_entire_buffer`)

## Advanced Features (Historical)

- **Word History**: Ring buffer of previous committed words for backspace-over-space restore.
- **English Auto-Restore**: 95% confidence threshold to avoid false positives.
- **Speculative Modifiers**: Explicit Vietnamese modifier overrides English detection.
