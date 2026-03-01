# Clean Architecture – Core Engine (v3.0.0)

The Rust core (`core/src`) implements Clean Architecture with four layers. Dependencies only flow **inward** — outer layers depend on inner layers, never the reverse.

```
┌──────────────────────────────────────────────────────────────────┐
│  4. presentation/   C FFI api.rs · types.rs · di/Container       │
├──────────────────────────────────────────────────────────────────┤
│  3. infrastructure/ adapters/ · engine/ · repositories/          │
├──────────────────────────────────────────────────────────────────┤
│  2. application/    use_cases/ · services/ · dto/                 │
├──────────────────────────────────────────────────────────────────┤
│  1. domain/         entities/ · value_objects/ · ports/           │
└──────────────────────────────────────────────────────────────────┘
```

---

## Layer 1 – Domain

**Path:** `core/src/domain/`  
**Rule:** Zero external dependencies. No I/O, no FFI, no `std::io`.

### Entities

| File | Description |
|---|---|
| `entities/syllable.rs` | `Syllable` – Vietnamese syllable with initial, vowel, final consonant, tone |
| `entities/tone.rs` | `ToneType` enum – Ngang, Sắc, Huyền, Hỏi, Ngã, Nặng |
| `entities/key_event.rs` | `KeyEvent` – a keystroke (char + modifier flags) |
| `entities/buffer.rs` | Logical word buffer |
| `entities/engine_buffer.rs` | Engine-level buffer with history support |

### Value Objects

| File | Description |
|---|---|
| `value_objects/char_sequence.rs` | `CharSequence` – immutable character string |
| `value_objects/transformation.rs` | `TransformationResult` – outcome of a transform |
| `value_objects/validation_result.rs` | `ValidationResult` – syllable validation result |
| `value_objects/engine_types.rs` | Shared type aliases |

### Ports (Traits / Interfaces)

Ports are **the contracts** that the domain defines and infrastructure implements.

| Port | Trait | Description |
|---|---|---|
| `ports/input/input_method.rs` | `InputMethod` | Classify keys: tone mark? modifier? stroke? |
| `ports/state/buffer_manager.rs` | `BufferManager` | Read/write the composing buffer |
| `ports/state/history_tracker.rs` | `HistoryTracker` | Track word history for backspace-over-space |
| `ports/transformation/mark_transformer.rs` | `MarkTransformer` | Apply vowel diacritics (â, ă, ơ, ư …) |
| `ports/transformation/tone_transformer.rs` | `ToneTransformer` | Apply / reposition tone marks |
| `ports/validation/syllable_validator.rs` | `SyllableValidator` | Validate Vietnamese phonotactics |
| `ports/validation/language_detector.rs` | `LanguageDetector` | Detect if text is English |

---

## Layer 2 – Application

**Path:** `core/src/application/`  
**Rule:** Depends only on domain. Contains business logic, no I/O.

### Use Cases

| File | Description |
|---|---|
| `use_cases/process_keystroke.rs` | `ProcessKeystrokeUseCase` – main entry, orchestrates all services |
| `use_cases/transform_text.rs` | `TransformTextUseCase` – applies mark/tone transformations |
| `use_cases/validate_input.rs` | `ValidateInputUseCase` – runs syllable validation |
| `use_cases/manage_shortcuts.rs` | `ManageShortcutsUseCase` – CRUD for text expansion shortcuts |

### Services

| File | Description |
|---|---|
| `services/processor_service.rs` | `ProcessorService` – orchestrates adapters for a single keystroke |
| `services/config_service.rs` | `ConfigService` – read/write engine configuration |

### DTOs

| File | Description |
|---|---|
| `dto/engine_config.rs` | `EngineConfig` – input method, tone style, feature flags |
| `dto/processing_context.rs` | `ProcessingContext` – keystroke + current buffer state |

---

## Layer 3 – Infrastructure

**Path:** `core/src/infrastructure/`  
**Rule:** Implements domain ports. May use I/O, third-party crates, static data.

### Input Adapters (implements `InputMethod`)

| File | Description |
|---|---|
| `adapters/input/telex_adapter.rs` | `TelexAdapter` – `aa→â`, `aw→ă`, `s→sắc`, `dd→đ`, etc. |
| `adapters/input/vni_adapter.rs` | `VniAdapter` – `6→circumflex`, `1→sắc`, `9→đ`, etc. |

### State Adapters (implements `BufferManager`, `HistoryTracker`)

| File | Description |
|---|---|
| `adapters/state/memory_buffer_adapter.rs` | In-memory composing buffer |
| `adapters/state/simple_history_adapter.rs` | Ring-buffer of last N committed words |

### Transformation Adapters (implements `MarkTransformer`, `ToneTransformer`)

| File | Description |
|---|---|
| `adapters/transformation/vietnamese_mark_adapter.rs` | Applies circumflex/horn/breve |
| `adapters/transformation/vietnamese_tone_adapter.rs` | Applies & repositions tone marks |
| `adapters/transformation/tone_positioning.rs` | Tone placement rules (New/Old style) |
| `adapters/transformation/transform.rs` | Core transform logic |
| `adapters/transformation/syllable.rs` | Syllable decomposition |
| `adapters/transformation/vowel_compound.rs` | Compound vowel handling (iê, uô, ươ …) |

### Validation Adapters (implements `SyllableValidator`, `LanguageDetector`)

| File | Description |
|---|---|
| `adapters/validation/fsm_validator_adapter.rs` | `FsmValidatorAdapter` wrapping the FSM |
| `adapters/validation/vietnamese_validator.rs` | Vietnamese syllable rule validation |
| `adapters/validation/diacritical_validator.rs` | Diacritical mark validation |
| `adapters/validation/language_detector_adapter.rs` | Wires phonotactic + dictionary engines |
| `adapters/validation/english/phonotactic.rs` | `PhonotacticEngine` – pattern-based English detection |
| `adapters/validation/english/dictionary.rs` | Binary dictionary lookup |
| `adapters/validation/english/language_decision.rs` | `LanguageDecisionEngine` – combines signals, 95% threshold |
| `adapters/validation/fsm/` | FSM transition tables for Vietnamese phonotactics |

### Engine (Migrated Legacy Internals)

| Path | Description |
|---|---|
| `infrastructure/engine/buffer/` | Internal char buffer |
| `infrastructure/engine/state/` | History, restore state |
| `infrastructure/engine/transform/` | Core transform pipeline |
| `infrastructure/engine/english/` | English detection internals |

### Repositories

| File | Description |
|---|---|
| `repositories/dictionary_repo.rs` | English dictionary data access |
| `repositories/shortcut_repo.rs` | Shortcut table persistence |

---

## Layer 4 – Presentation

**Path:** `core/src/presentation/`  
**Rule:** Outermost layer. Owns the FFI surface and the DI container.

### FFI (`presentation/ffi/`)

| File | Description |
|---|---|
| `api.rs` | `#[no_mangle] extern "C"` functions — all wrapped with `catch_unwind` |
| `types.rs` | `#[repr(C)]` structs: `FfiConfig_v2`, `FfiProcessResult_v2`, `FfiStatusCode`, `FfiVersionInfo` |
| `conversions.rs` | Type conversions between FFI types and domain types |

### DI Container (`presentation/di/`)

| File | Description |
|---|---|
| `container.rs` | `Container` – IoC wiring. Creates all adapters, injects into services/use-cases |

The container wires the full dependency graph at startup:

```rust
Container::with_config(config)
    // input adapters
    .telex(TelexAdapter::new())
    .vni(VniAdapter::new())
    // state adapters
    .buffer(MemoryBufferAdapter::new())
    .history(SimpleHistoryAdapter::new())
    // transform adapters
    .mark_transformer(VietnameseMarkAdapter::new())
    .tone_transformer(VietnameseToneAdapter::new())
    // validation adapters
    .validator(FsmValidatorAdapter::new())
    .language_detector(LanguageDetectorAdapter::new())
```

---

## Supporting Modules

| Path | Description |
|---|---|
| `shared/buffer/` | `RawInputBuffer` (keystroke history for ESC restore), `Buffer`, rebuild logic |
| `shared/types/` | `Config` struct, type aliases |
| `features/shortcut.rs` | `ShortcutTable` – text expansion |
| `features/encoding.rs` | `OutputEncoding` – VIQR/Unicode output |
| `data/` | Static char maps, vowel tables, key constants |
| `unified_engine.rs` | Facade re-exporting legacy engine symbols for backward compatibility |

---

## SOLID in Practice

| Principle | How it's applied |
|---|---|
| **S** Single Responsibility | `ProcessKeystrokeUseCase` only orchestrates; `FsmValidatorAdapter` only validates |
| **O** Open/Closed | Add a new input method by implementing `InputMethod` trait, zero changes to core |
| **L** Liskov Substitution | Any `InputMethod` impl can replace another in the container |
| **I** Interface Segregation | `InputMethod`, `MarkTransformer`, `ToneTransformer`, `SyllableValidator` are separate traits |
| **D** Dependency Inversion | `ProcessorService` depends on `dyn InputMethod`, not `TelexAdapter` directly |
