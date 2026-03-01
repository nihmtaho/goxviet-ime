# SOLID Processors – Input Method Adapters

Tài liệu này mô tả cách các input method (Telex, VNI) được implement theo kiến trúc SOLID trong Clean Architecture v3.0.0.

> **Xem thêm:** [clean-architecture.md](./clean-architecture.md) để hiểu toàn bộ cấu trúc layer.

---

## Ports (Domain Interfaces)

Defined in `core/src/domain/ports/input/input_method.rs`:

```rust
pub enum InputMethodId { Telex, Vni, Plain }

pub trait InputMethod: Send + Sync {
    fn id(&self) -> InputMethodId;
    fn classify_key(&self, key: char) -> KeyClassification;
}

pub enum KeyClassification {
    ToneMark(ToneType),     // s→Sắc, f→Huyền, r→Hỏi, x→Ngã, j→Nặng
    VowelModifier(char),    // aa→â, aw→ă, ee→ê, oo→ô, ow→ơ, uw→ư
    StrokeModifier,         // dd→đ
    RemoveDiacritic,        // z (Telex), 0 (VNI)
    Regular(char),          // Normal letter, pass to buffer
    Consumed,               // Modifier absorbed (double-tap escape)
}
```

---

## Implementations (Infrastructure Adapters)

### TelexAdapter

**Path:** `core/src/infrastructure/adapters/input/telex_adapter.rs`

```rust
use crate::infrastructure::adapters::input::TelexAdapter;
// TelexAdapter implements domain::ports::input::InputMethod

let adapter = TelexAdapter::new();
let classification = adapter.classify_key('s');
// → KeyClassification::ToneMark(ToneType::Sắc)
```

**Key mappings:**

| Input | Output | Type |
|---|---|---|
| `s` | Sắc (´) | ToneMark |
| `f` | Huyền (`) | ToneMark |
| `r` | Hỏi (?) | ToneMark |
| `x` | Ngã (~) | ToneMark |
| `j` | Nặng (.) | ToneMark |
| `z` | Remove | RemoveDiacritic |
| `aa` | â | VowelModifier |
| `aw` | ă | VowelModifier |
| `ee` | ê | VowelModifier |
| `oo` | ô | VowelModifier |
| `ow` | ơ | VowelModifier |
| `uw` / `w` | ư | VowelModifier |
| `dd` | đ | StrokeModifier |

**Double-tap escape:** Gõ lại phím modifier để hủy (ví dụ `ss` → xóa dấu sắc và ra `s`).  
**Smart `w`:** `ow` → `ơ`, `uw` → `ư`, `uow` → `ươ` (xử lý trong `TransformTextUseCase`).

### VniAdapter

**Path:** `core/src/infrastructure/adapters/input/vni_adapter.rs`

```rust
use crate::infrastructure::adapters::input::VniAdapter;

let adapter = VniAdapter::new();
let classification = adapter.classify_key('1');
// → KeyClassification::ToneMark(ToneType::Sắc)
```

**Key mappings:**

| Input | Output | Type |
|---|---|---|
| `1` | Sắc | ToneMark |
| `2` | Huyền | ToneMark |
| `3` | Hỏi | ToneMark |
| `4` | Ngã | ToneMark |
| `5` | Nặng | ToneMark |
| `0` | Remove | RemoveDiacritic |
| `6` | â/ê/ô (circumflex) | VowelModifier |
| `7` | ơ/ư (horn) | VowelModifier |
| `8` | ă (breve) | VowelModifier |
| `9` | đ | StrokeModifier |

---

## Dependency Injection

Adapters are wired by the IoC container — `ProcessorService` never imports concrete adapters directly.

**Path:** `core/src/presentation/di/container.rs`

```rust
// Container creates adapters and injects via Box<dyn InputMethod>
let input_method: Box<dyn InputMethod> = match config.input_method {
    InputMethodId::Telex => Box::new(TelexAdapter::new()),
    InputMethodId::Vni   => Box::new(VniAdapter::new()),
    InputMethodId::Plain => Box::new(PlainAdapter::new()),
};

// ProcessorService receives the trait object — OCP: add new method without changing service
let service = ProcessorService::new(input_method, mark_transformer, tone_transformer, validator, detector);
```

---

## Transformation Adapters

### VietnameseMarkAdapter
**Path:** `core/src/infrastructure/adapters/transformation/vietnamese_mark_adapter.rs`  
Implements `MarkTransformer`. Applies diacritic marks (circumflex `^`, horn `ʼ`, breve `˘`) to the vowel cluster.

### VietnameseToneAdapter
**Path:** `core/src/infrastructure/adapters/transformation/vietnamese_tone_adapter.rs`  
Implements `ToneTransformer`. Applies tone marks and repositions them when the vowel cluster changes.

**Tone placement rules** (New Style):
- Vowel with diacritic (â, ê, ô, ơ, ư) → tone on that vowel
- No diacritic → tone on second vowel in compound
- With final consonant → tone on the nucleus vowel

---

## Running Tests

```bash
# All adapter tests
cd core && cargo test adapters

# Single adapter
cd core && cargo test telex_adapter

# Integration: Telex typing simulation
cd core && cargo test --test trans_test

# Validation tests
cd core && cargo test --test validator_integration_test
```

---

## Adding a New Input Method

1. Create `core/src/infrastructure/adapters/input/my_method_adapter.rs`
2. Implement `trait InputMethod` from `domain::ports::input`
3. Add variant to `InputMethodId` enum in domain
4. Register in `presentation/di/container.rs`
5. No changes needed to use cases or services (OCP)
