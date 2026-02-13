# Unified Engine Documentation

## Overview

Unified engine là kết quả của việc gộp `engine/` (legacy) và `engine_v2/` (modern) thành một engine duy nhất theo chuẩn **SOLID**.

## Cấu trúc

```
core/src/unified_engine/
├── mod.rs              # Main module exports
├── core.rs             # Core types (Action, Result, Transform, Config)
├── buffer.rs           # Buffer và RawInputBuffer
├── validation.rs       # Vietnamese syllable validation (từ engine_v2)
├── english.rs          # English detection (từ engine_v2)
├── transform.rs        # Text transformations
├── state.rs            # Word history
└── features.rs         # Shortcuts và encoding
```

## Tính năng từ Engine Legacy

### 1. Buffer Management (`buffer.rs`)
- **Buffer**: Fixed-size array-based buffer (256 chars max)
- **Char**: Character với metadata (tone, mark, stroke, caps)
- **RawInputBuffer**: Lưu trữ raw keystrokes cho ESC restore

```rust
pub struct Char {
    pub key: u16,
    pub caps: bool,
    pub tone: u8,      // 0=none, 1=circumflex, 2=horn/breve
    pub mark: u8,      // 0=none, 1-5=tone marks
    pub stroke: bool,  // d → đ
}
```

### 2. State Management (`state.rs`)
- **WordHistory**: Ring buffer cho backspace-after-space feature
- **HistoryEntry**: Lưu buffer và raw_input

### 3. Transformations (`transform.rs`)
- **apply_mark()**: Áp dụng dấu thanh (sắc, huyền, hỏi, ngã, nặng)
- **apply_tone()**: Áp dụng dấu mũ/móc (â, ê, ô, ơ, ư, ă)
- **apply_stroke()**: Áp dụng gạch ngang (đ)
- **find_tone_position()**: Tìm vị trí đặt dấu thanh theo quy tắc tiếng Việt

### 4. Features (`features.rs`)
- **ShortcutTable**: Bảng shortcuts với max 100 entries
- **EncodingConverter**: Hỗ trợ Unicode, TCVN3, VNI, CP1258

## Tính năng từ Engine V2

### 1. Vietnamese Validation (`validation.rs`)

**FsmValidator**: O(1) validation sử dụng FSM tables

Các quy tắc kiểm tra:
1. **Initial consonants**: Kiểm tra phụ âm đầu hợp lệ
2. **Invalid clusters**: phát hiện các cụm phụ âm tiếng Anh (bl, br, cl, cr, etc.)
3. **Bigram validation**: Sử dụng `VIETNAMESE_BIGRAMS` table
4. **Coda validation**: Kiểm tra phụ âm cuối
5. **Vowel sequences**: Kiểm tra các kết hợp nguyên âm

```rust
pub trait SyllableValidator {
    fn validate(&self, keys: &[u16]) -> ValidationResult;
}

pub fn validate_syllable(keys: &[u16]) -> ValidationResult {
    // 7 rules validation
}
```

### 2. English Detection (`english.rs`)

**PhonotacticEngine**: 8-layer phonotactic analysis

| Layer | Check | Confidence |
|-------|-------|------------|
| L1 | Invalid initials (F, J, W, Z) | 100% |
| L2 | Onset clusters (bl, br, cl, etc.) | 95% |
| L3 | Double consonants (ll, ss, etc.) | 90% |
| L4 | Suffixes (-ing, -ed, -tion) | 95% |
| L5 | Coda clusters (st, nd, nt) | 85% |
| L6 | Prefixes (un-, re-, pre-) | 95% |
| L7 | Vowel patterns (ea, ou, oo) | 75% |
| L8 | Impossible bigrams (qb, zs) | 90% |

**Dictionary**: Common English words và programming terms

**LanguageDecisionEngine**: Unified decision making

```rust
pub struct LanguageDecision {
    pub is_english: bool,
    pub confidence: u8,
    pub should_restore: bool,
}
```

## SOLID Principles trong Unified Engine

### 1. Single Responsibility Principle

Mỗi module có một nhiệm vụ duy nhất:
- `validation.rs`: Chỉ làm validation
- `english.rs`: Chỉ làm English detection
- `buffer.rs`: Chỉ quản lý buffer
- `transform.rs`: Chỉ xử lý transformations

### 2. Open/Closed Principle

Mở rộng mà không sửa đổi:

```rust
// Thêm validator mới mà không cần sửa engine
pub trait SyllableValidator {
    fn validate(&self, keys: &[u16]) -> ValidationResult;
}

impl SyllableValidator for CustomValidator {
    fn validate(&self, keys: &[u16]) -> ValidationResult {
        // Custom logic
    }
}
```

### 3. Liskov Substitution Principle

Các implementation có thể thay thế cho trait:

```rust
fn validate(validator: &dyn SyllableValidator, keys: &[u16]) {
    validator.validate(keys); // Works with any validator
}
```

### 4. Interface Segregation Principle

Các trait nhỏ, tập trung:

```rust
pub trait SyllableValidator {
    fn validate(&self, keys: &[u16]) -> ValidationResult;
}

// Không bắt implement những method không cần
```

### 5. Dependency Inversion Principle

Phụ thuộc vào abstraction:

```rust
// Application layer phụ thuộc vào trait
pub fn process(
    &self,
    validator: &dyn SyllableValidator,  // Abstraction
    detector: &LanguageDecisionEngine,    // Concrete but injectable
) -> Result {
    // ...
}
```

## So sánh với Legacy

### Before (Legacy)
```
engine/
├── mod.rs          # ~3400 lines, monolithic
├── buffer/
├── vietnamese/
├── state/
└── features/

engine_v2/
├── mod.rs
├── vietnamese_validator.rs
├── diacritical_validator.rs
├── english/
└── fsm/
```

### After (Unified)
```
unified_engine/
├── mod.rs              # Clean exports only
├── core.rs             # Types (~250 lines)
├── buffer.rs           # Buffer management (~250 lines)
├── validation.rs       # Validation (~300 lines)
├── english.rs          # English detection (~350 lines)
├── transform.rs        # Transformations (~120 lines)
├── state.rs            # State management (~80 lines)
└── features.rs         # Features (~250 lines)
```

**Lợi ích**:
- Code rõ ràng, dễ đọc hơn
- Mỗi file nhỏ, tập trung
- Dễ test từng phần riêng biệt
- Dễ mở rộng

## Usage

```rust
use goxviet_core::unified_engine::{
    Buffer, Char, WordHistory,
    validate_syllable, FsmValidator,
    PhonotacticEngine, LanguageDecisionEngine,
    ShortcutTable, OutputEncoding
};

// Buffer
let mut buffer = Buffer::new();
buffer.push(Char::new(keys::A, false));

// Validation
let result = validate_syllable(&[keys::T, keys::R, keys::O, keys::N, keys::G]);
assert!(result.is_valid);

// English detection
let keys = vec![(keys::C, false), (keys::O, false), (keys::N, false), (keys::S, false)];
let result = PhonotacticEngine::analyze(&keys);
if result.is_english() {
    println!("English detected with {}% confidence", result.english_confidence);
}

// Shortcuts
let mut shortcuts = ShortcutTable::with_defaults();
if let Some((shortcut, backspace)) = shortcuts.try_match("vn") {
    println!("Expand '{}' to '{}'", shortcut.trigger, shortcut.replacement);
}
```

## Migration Plan

### Phase 1: Parallel (HIỆN TẠI)
✅ Unified engine được tạo song song với legacy
✅ Legacy engine vẫn hoạt động bình thường
✅ FFI API không thay đổi

### Phase 2: Gradual Refactor
- Dần dần chuyển logic từ legacy sang unified
- Test từng phần một
- Đảm bảo backward compatibility

### Phase 3: Full Migration
- Legacy engine được deprecate
- Unified engine trở thành default
- Xóa engine/ và engine_v2/

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

## FFI Compatibility

Unified engine không thay đổi FFI API. Tất cả các hàm FFI vẫn giữ nguyên:

```c
void ime_init();
ImeResult* ime_key(uint16_t key, bool caps, bool ctrl);
void ime_method(uint8_t method);
bool ime_add_shortcut(const char* trigger, const char* replacement);
// ... etc
```

## Notes

- Unified engine đang ở dạng **foundation/stub**
- Các functions chính đã được implement nhưng chưa tích hợp vào main processing flow
- Legacy engine vẫn là engine được sử dụng trong lib.rs
- Trong tương lai sẽ refactor để unified engine trở thành engine chính
