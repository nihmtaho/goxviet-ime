# SOLID Processors

## Overview

Các processors mới được thiết kế theo chuẩn SOLID với các traits rõ ràng và tách biệt.

## Modules

### 1. Telex Processor

```rust
use goxviet_core::processors::TelexProcessor;
use goxviet_core::state::buffer::CharBuffer;
use goxviet_core::traits::processor::{InputProcessor, KeyEvent};

let processor = TelexProcessor::new();
let mut buffer = CharBuffer::new();

// Process a key
let key_event = KeyEvent::simple('a' as u16, false);
let result = processor.process_key(&key_event, &mut buffer);
```

**Key mappings:**
- Marks: `s`=sắc, `f`=huyền, `r`=hỏi, `x`=ngã, `j`=nặng
- Tones: `aa`=â, `ee`=ê, `oo`=ô, `w`=ư/ơ/ă
- Stroke: `dd`=đ
- Remove: `z`

### 2. VNI Processor

```rust
use goxviet_core::processors::VniProcessor;
use goxviet_core::state::buffer::CharBuffer;
use goxviet_core::traits::processor::{InputProcessor, KeyEvent};

let processor = VniProcessor::new();
let mut buffer = CharBuffer::new();

// Process a key
let key_event = KeyEvent::simple('a' as u16, false);
let result = processor.process_key(&key_event, &mut buffer);
```

**Key mappings:**
- Marks: `1`=sắc, `2`=huyền, `3`=hỏi, `4`=ngã, `5`=nặng
- Tones: `6`=circumflex, `7`=horn, `8`=breve
- Stroke: `9`
- Remove: `0`

### 3. Processor Registry

```rust
use goxviet_core::processors::ProcessorRegistryImpl;
use goxviet_core::traits::input_method::InputMethodId;
use goxviet_core::traits::processor::ProcessorRegistry;

// Create registry with defaults
let registry = ProcessorRegistryImpl::with_defaults();

// Get a processor
if let Some(processor) = registry.get_processor(InputMethodId::Telex) {
    // Use processor...
}

// Get default processor
let default = registry.default_processor();
```

## Architecture

### Traits

1. **InputMethod** (`traits/input_method.rs`)
   - `method_id()`: Get method identifier
   - `mark()`: Check if key is a tone mark
   - `tone_modifier()`: Check if key is a tone modifier
   - `tone_targets()`: Get valid target vowels
   - `is_stroke()`: Check if key is stroke modifier
   - `is_remove()`: Check if key removes diacritics

2. **InputProcessor** (`traits/processor.rs`)
   - `input_method()`: Get processor's method ID
   - `process_key()`: Process a key event
   - `is_modifier_key()`: Check if key is a modifier
   - `priority()`: Get processor priority

### Design Principles

1. **Single Responsibility**: Mỗi processor chỉ xử lý một input method
2. **Open/Closed**: Dễ dàng thêm processor mới bằng cách implement traits
3. **Interface Segregation**: Các traits nhỏ, tập trung
4. **Dependency Inversion**: Processors phụ thuộc vào abstractions (traits)

## Testing

Chạy tests:

```bash
cargo test --lib processors::
```

## Migration từ Legacy

### Legacy (engine/mod.rs)

```rust
use goxviet_core::engine::Engine;

let mut engine = Engine::new();
engine.set_method(0); // Telex
let result = engine.on_key(key, caps, ctrl);
```

### SOLID mới

```rust
use goxviet_core::processors::TelexProcessor;
use goxviet_core::state::buffer::CharBuffer;
use goxviet_core::traits::processor::{InputProcessor, KeyEvent};

let processor = TelexProcessor::new();
let mut buffer = CharBuffer::new();
let key_event = KeyEvent::new(key, caps, shift, ctrl);
let result = processor.process_key(&key_event, &mut buffer);
```

## Future Work

- [ ] Tích hợp với validators để kiểm tra chính tả
- [ ] Tích hợp với transformers cho logic phức tạp hơn
- [ ] Thêm processors cho các input method khác (VIQR, etc.)
