# Vietnamese IME Core Engine

The core engine (`core/src`) is a high-performance, portable Rust library for Vietnamese input method processing. Starting from **v3.0.0**, the engine follows **Clean Architecture** with strict SOLID principles.

## Architecture Overview

The engine is organized into four concentric layers — inner layers know nothing about outer layers.

```
┌──────────────────────────────────────────────────────┐
│  presentation/        C FFI + Dependency Injection   │
├──────────────────────────────────────────────────────┤
│  application/         Use Cases + Services           │
├──────────────────────────────────────────────────────┤
│  infrastructure/      Adapters + Implementations     │
├──────────────────────────────────────────────────────┤
│  domain/              Entities + Ports (pure Rust)   │
└──────────────────────────────────────────────────────┘
```

See [clean-architecture.md](./clean-architecture.md) for the full layer-by-layer breakdown.

## Key Features

- **Clean Architecture**: Strict layer separation, all cross-layer dependencies go inward via ports.
- **SOLID Compliance**: Traits as ports, adapters as implementations, DI container wires everything.
- **Validation-First**: Phonotactic FSM ensures the buffer always holds valid Vietnamese syllables.
- **Bilingual Detection**: English detection (dictionary + phonotactics) prevents accidental transforms.
- **Zero Panic FFI**: All public C functions use `catch_unwind`. No panics cross the ABI boundary.
- **Per-Engine Config**: No global state — each engine instance has its own `FfiConfig_v2`.
- **Shortcut Expansion**: User-defined text abbreviations, managed as a use case.

## Directory Structure

```
core/src/
├── lib.rs                    # Crate root & public re-exports
├── domain/                   # Inner ring: entities, value objects, ports (traits)
│   ├── entities/             # Syllable, ToneType, KeyEvent, Buffer, EngineBuffer
│   ├── value_objects/        # CharSequence, TransformationResult, ValidationResult
│   └── ports/                # Abstract interfaces (traits) for input, state, transformation, validation
├── application/              # Use cases + orchestration DTOs
│   ├── use_cases/            # ProcessKeystroke, TransformText, ValidateInput, ManageShortcuts
│   ├── services/             # ConfigService, ProcessorService
│   └── dto/                  # EngineConfig, ProcessingContext
├── infrastructure/           # Concrete implementations of domain ports
│   ├── adapters/
│   │   ├── input/            # TelexAdapter, VniAdapter  (impl InputMethod)
│   │   ├── state/            # MemoryBufferAdapter, SimpleHistoryAdapter
│   │   ├── transformation/   # VietnameseMarkAdapter, VietnameseToneAdapter
│   │   └── validation/       # FsmValidatorAdapter, LanguageDetectorAdapter, English/FSM
│   ├── engine/               # Migrated legacy engine internals (buffer, state, transform)
│   ├── external/             # Auto-update checker
│   └── repositories/         # DictionaryRepo, ShortcutRepo
├── presentation/             # Outermost ring: FFI + DI
│   ├── ffi/                  # api.rs (C exports), types.rs (repr(C) structs), conversions.rs
│   └── di/                   # Container (IoC wiring)
├── shared/                   # Cross-layer utilities (buffer types, config types)
├── features/                 # Standalone features: shortcut.rs, encoding.rs
├── data/                     # Static data: chars, vowels, keys, constants
├── unified_engine.rs         # Backward-compatible re-export facade
├── input/                    # Legacy re-exports for Telex/VNI (backward compat)
└── utils.rs                  # General helper functions
```

## Processing Pipeline

```
User keystroke (CGEventTap / Windows hook)
        │
        ▼
ime_process_key_v2(engine_ptr, key, &mut result)   ← FFI boundary
        │
        ▼
presentation::ffi::api    →  catch_unwind wrapper
        │
        ▼
application::use_cases::ProcessKeystroke::execute()
        │
        ├── ConfigService      (read current config)
        └── ProcessorService   (orchestrate)
                │
                ├── InputMethod adapter   (Telex/VNI key classification)
                ├── MarkTransformer       (vowel modifiers: aa→â, aw→ă, ow→ơ)
                ├── ToneTransformer       (tone placement: s→sắc, f→huyền …)
                ├── SyllableValidator     (FSM phonotactic check)
                └── LanguageDetector      (English auto-restore decision)
        │
        ▼
FfiProcessResult_v2 { text, backspace_count, consumed }
```

## FFI API v2

All platform code uses the **v2 API** (v1 was removed in v3.0.0).

See [lib.md](./lib.md) for the complete C API reference.

## Legacy Notes

`engine/` and `engine_v2/` source directories **no longer exist** in the Rust codebase.

- `engine/`    → migrated to `infrastructure/engine/`
- `engine_v2/` → migrated to `infrastructure/adapters/validation/` and `infrastructure/adapters/transformation/`

The docs under `engine/` and `engine_v2/` subdirectories are kept for historical reference only.
