# GoxViet Core - SOLID Architecture

## Tổng quan

Core library đã được tái cấu trúc theo nguyên tắc **SOLID** với kiến trúc phân lớp rõ ràng:

```
core/src/
├── lib.rs                    # FFI facade (giữ nguyên API)
├── domain/                   # Domain layer - traits/interfaces
│   ├── mod.rs
│   ├── input_method.rs       # InputMethod trait
│   ├── validator.rs          # SyllableValidator trait
│   ├── transformer.rs        # TextTransformer trait
│   └── types.rs              # Core types
├── application/              # Application layer - orchestration
│   ├── mod.rs
│   ├── processor.rs          # KeyProcessor
│   ├── context.rs            # ProcessingContext
│   └── config.rs             # EngineConfig
├── infrastructure/           # Infrastructure layer - implementations
│   ├── mod.rs
│   ├── input/                # Input methods
│   │   ├── telex.rs
│   │   └── vni.rs
│   ├── validation/           # Validators
│   │   ├── fsm_validator.rs
│   │   └── phonotactic.rs
│   ├── transformation/       # Transformers
│   │   └── vietnamese.rs
│   ├── detection/            # Language detection
│   │   └── english.rs
│   ├── data/                 # Data structures
│   │   ├── buffer.rs
│   │   ├── state.rs
│   │   └── mod.rs
│   └── features/             # Features
│       ├── shortcut.rs
│       └── encoding.rs
├── data/                     # Legacy data modules (preserved)
├── engine/                   # Legacy engine (preserved)
├── engine_v2/                # Legacy engine v2 (preserved)
├── input/                    # Legacy input (preserved)
├── updater/                  # Legacy updater (preserved)
└── utils.rs                  # Utilities
```

## Nguyên tắc SOLID được áp dụng

### 1. Single Responsibility Principle (SRP)

Mỗi module/class có một lý do duy nhất để thay đổi:

- **`InputMethod`** (domain/input_method.rs): Chỉ định nghĩa contract cho input methods
- **`SyllableValidator`** (domain/validator.rs): Chỉ định nghĩa contract cho validation
- **`TextTransformer`** (domain/transformer.rs): Chỉ định nghĩa contract cho transformations
- **`FsmSyllableValidator`** (infrastructure/validation/fsm_validator.rs): Implement validation dựa trên FSM
- **`TelexInputMethod`** (infrastructure/input/telex.rs): Implement Telex input method
- **`CharBuffer`** (infrastructure/data/buffer.rs): Quản lý buffer characters

### 2. Open/Closed Principle (OCP)

Mở rộng được nhưng không sửa đổi:

```rust
// Có thể thêm input method mới mà không sửa đổi engine
pub trait InputMethod: Send + Sync {
    fn id(&self) -> u8;
    fn name(&self) -> &'static str;
    fn mark(&self, key: u16) -> Option<MarkValue>;
    // ...
}

// Có thể thêm validator mới
pub trait SyllableValidator: Send + Sync {
    fn validate(&self, context: &ValidationContext) -> ValidationResult;
    // ...
}
```

### 3. Liskov Substitution Principle (LSP)

Các implementation có thể thay thế cho interface:

```rust
// Telex hay VNI đều có thể dùng thay thế cho InputMethod
let method: &dyn InputMethod = match method_id {
    1 => &VniInputMethod,
    _ => &TelexInputMethod,
};
```

### 4. Interface Segregation Principle (ISP)

Các interface nhỏ, tập trung:

- `InputMethod` - chỉ 8 methods
- `SyllableValidator` - chỉ 3 methods  
- `TextTransformer` - chỉ 3 methods

Không bắt implement phải implement những gì không cần.

### 5. Dependency Inversion Principle (DIP)

Phụ thuộc vào abstraction, không phải concrete:

```rust
// Application layer phụ thuộc vào traits
pub struct KeyProcessor {
    // Không phụ thuộc vào concrete implementations
}

impl KeyProcessor {
    pub fn process(
        &self,
        event: &KeyEvent,
        context: &mut ProcessingContext,
        method: &dyn InputMethod,        // Dependency injection
        validator: &dyn SyllableValidator, // Dependency injection
    ) -> ProcessingResult {
        // ...
    }
}
```

## Các Layer

### Domain Layer (`domain/`)

**Nhiệm vụ**: Định nghĩa business logic, contracts, và core types.

**Không phụ thuộc**: Không phụ thuộc vào bất kỳ layer nào khác.

**Các thành phần**:
- `input_method.rs`: InputMethod trait, ToneType, MarkValue
- `validator.rs`: SyllableValidator trait, ValidationResult
- `transformer.rs`: TextTransformer trait, TransformResult
- `types.rs`: ProcessingResult, Action, KeyEvent, Char

### Application Layer (`application/`)

**Nhiệm vụ**: Orchestration và coordination giữa các domain objects.

**Phụ thuộc**: Chỉ phụ thuộc Domain layer.

**Các thành phần**:
- `processor.rs`: KeyProcessor - điều phối xử lý key events
- `context.rs`: ProcessingContext - lưu trữ state trong quá trình xử lý
- `config.rs`: EngineConfig - cấu hình engine

### Infrastructure Layer (`infrastructure/`)

**Nhiệm vụ**: Concrete implementations của domain interfaces.

**Phụ thuộc**: Phụ thuộc Domain và Application layers.

**Các thành phần**:
- `input/`: Telex, VNI implementations
- `validation/`: FSM validator, phonotactic validator
- `transformation/`: Vietnamese text transformations
- `detection/`: English language detection
- `data/`: Buffers, state management
- `features/`: Shortcuts, encoding

## Migration Path

### Phase 1: Parallel Structure (HIỆN TẠI)
✅ Tạo SOLID structure mới song song với legacy code
✅ Legacy engine vẫn hoạt động bình thường
✅ FFI API không thay đổi

### Phase 2: Gradual Adoption
- Từng phần của legacy engine sẽ được refactor sang SOLID structure
- Giữ backward compatibility trong quá trình migration

### Phase 3: Full Migration
- Legacy modules được thay thế hoàn toàn
- Chỉ giữ lại data/ (language data) và utils/

## Ưu điểm của SOLID Architecture

1. **Testability**: Dễ dàng mock dependencies để test
2. **Maintainability**: Mỗi phần có trách nhiệm rõ ràng
3. **Extensibility**: Thêm tính năng mới không cần sửa code cũ
4. **Reusability**: Domain contracts có thể dùng cho nhiều implementations
5. **Collaboration**: Nhiều developer có thể làm việc trên các layer khác nhau

## Build và Test

```bash
# Build debug
cargo build

# Build release
cargo build --release

# Run tests
cargo test

# Build universal library cho macOS
bash scripts/rust_build_lib_universal_for_macos.sh
```

## FFI API (Không thay đổi)

Tất cả các hàm FFI vẫn giữ nguyên:

```c
// Khởi tạo
void ime_init();

// Xử lý key
ImeResult* ime_key(uint16_t key, bool caps, bool ctrl);
ImeResult* ime_key_ext(uint16_t key, bool caps, bool ctrl, bool shift);

// Cấu hình
void ime_method(uint8_t method);
void ime_enabled(bool enabled);
void ime_skip_w_shortcut(bool skip);
void ime_esc_restore(bool enabled);
void ime_free_tone(bool enabled);
void ime_modern(bool modern);
void ime_instant_restore(bool enabled);

// Shortcuts
bool ime_add_shortcut(const char* trigger, const char* replacement);
void ime_remove_shortcut(const char* trigger);
void ime_clear_shortcuts();

// Memory management
void ime_free(ImeResult* r);
```

## Lưu ý

- Legacy engine (`engine/`, `engine_v2/`) vẫn được giữ nguyên
- Test failures hiện tại là của legacy code, không liên quan đến SOLID structure mới
- SOLID modules mới đang ở dạng stub và sẽ được implement đầy đủ trong Phase 2
