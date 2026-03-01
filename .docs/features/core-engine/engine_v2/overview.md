# ⚠️ DEPRECATED – Engine V2 Module (`engine_v2/`)

> **Lưu ý:** Module `engine_v2/` đã được migrate sang Clean Architecture trong v3.0.0.  
> Tài liệu này được giữ lại cho mục đích lịch sử. **Không còn phản ánh codebase hiện tại.**
>
> **Xem thay thế:** [clean-architecture.md](../clean-architecture.md)  
> **Code mới tại:** `infrastructure/adapters/validation/` và `infrastructure/adapters/transformation/`

---

## Migration Map

| Legacy `engine_v2/` path | New path (v3.0.0) |
|---|---|
| `engine_v2/english/` | `infrastructure/adapters/validation/english/` |
| `engine_v2/vietnamese_validator/` | `infrastructure/adapters/validation/vietnamese_validator.rs` + `fsm_validator_adapter.rs` |
| `engine_v2/fsm/` | `infrastructure/adapters/validation/fsm/` |

---

# [Historical] Engine V2 Overview

`engine_v2/` was the "Modern Engine" module prior to v3.0.0, designed as the successor to `engine/`. It introduced:

- **Modular English detection**: Phonotactic + dictionary combination.
- **Vietnamese FSM validator**: Strict syllable validation using a Finite State Machine.
- **Separate concerns**: Detection and validation decoupled from the main engine loop.

These capabilities are now fully integrated into the Clean Architecture via the `infrastructure/adapters/validation/` adapters.

## Components (Historical → New)

### English Detection
Previously in `engine_v2/english/`:
- `PhonotacticEngine` → now at `infrastructure/adapters/validation/english/phonotactic.rs`
- `Dictionary` → now at `infrastructure/adapters/validation/english/dictionary.rs`
- `LanguageDecisionEngine` (95% threshold) → now at `infrastructure/adapters/validation/english/language_decision.rs`

### Vietnamese Validator (FSM)
Previously in `engine_v2/vietnamese_validator/`:
- `VietnameseSyllableValidator` → now at `infrastructure/adapters/validation/vietnamese_validator.rs`
- FSM tables → now at `infrastructure/adapters/validation/fsm/tables/`

Both are now accessible as `Box<dyn SyllableValidator>` and `Box<dyn LanguageDetector>` via the DI container.
