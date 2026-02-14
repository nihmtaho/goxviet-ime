# GoxViet Core - Sequence Diagrams

This document contains detailed sequence diagrams showing the flow of data and control through the GoxViet core engine for key operations.

---

## Table of Contents

1. [Keystroke Processing Flow](#keystroke-processing-flow)
2. [Configuration Update Flow](#configuration-update-flow)
3. [Validation Pipeline Flow](#validation-pipeline-flow)
4. [Transformation Pipeline Flow](#transformation-pipeline-flow)
5. [Shortcut Expansion Flow](#shortcut-expansion-flow)
6. [Buffer Management Flow](#buffer-management-flow)
7. [Error Handling Flow](#error-handling-flow)
8. [Engine Lifecycle](#engine-lifecycle)

---

## Keystroke Processing Flow

### Complete End-to-End Flow

```mermaid
sequenceDiagram
    autonumber
    participant Client as Platform Client
    participant FFI as FFI API Layer
    participant Container as DI Container
    participant Processor as ProcessorService
    participant UseCase as ProcessKeystroke
    participant Input as InputMethod
    participant Validator as SyllableValidator
    participant Transformer as ToneTransformer
    participant Buffer as BufferManager
    participant Detector as LanguageDetector
    
    Client->>+FFI: ime_process_key(handle, "s", TEXT)
    
    Note over FFI: Catch panic boundary
    FFI->>FFI: Validate handle != NULL
    FFI->>FFI: Validate UTF-8 string
    FFI->>FFI: Convert C types to Rust
    
    FFI->>+Container: get_processor_service()
    Container-->>-FFI: Arc<Mutex<ProcessorService>>
    
    FFI->>+Processor: process_key(context)
    
    Note over Processor: Load current buffer
    Processor->>+Buffer: get_buffer()
    Buffer-->>-Processor: Buffer state
    
    Note over Processor: Detect language context
    Processor->>+Detector: is_vietnamese_context(buffer)
    Detector-->>-Processor: true
    
    Note over Processor: Create use case
    Processor->>Processor: create ProcessKeystroke use case
    
    Processor->>+UseCase: execute(context)
    
    Note over UseCase: Step 1: Parse input
    UseCase->>+Input: parse_input("s")
    Input-->>-UseCase: KeyAction::ToneMark(Sac)
    
    Note over UseCase: Step 2: Validate syllable
    UseCase->>UseCase: update buffer with action
    UseCase->>+Validator: validate(syllable)
    Validator-->>-UseCase: ValidationResult::Valid
    
    Note over UseCase: Step 3: Transform text
    UseCase->>+Transformer: apply_tone(syllable, Sac)
    Transformer-->>-UseCase: "viết"
    
    Note over UseCase: Build result
    UseCase->>UseCase: TransformResult::builder()
    UseCase-->>-Processor: TransformResult
    
    Note over Processor: Update state
    Processor->>+Buffer: update_buffer(new_state)
    Buffer-->>-Processor: OK
    
    Processor-->>-FFI: TransformResult
    
    Note over FFI: Convert Rust to C
    FFI->>FFI: to_ffi_string("viết")
    FFI->>FFI: FfiProcessResult::new()
    
    FFI-->>-Client: FfiProcessResult{text: "viết", backspace: 4}
    
    Note over Client: Platform integration
    Client->>Client: Send 4 backspaces
    Client->>Client: Insert "viết"
    Client->>FFI: ime_free_string(result.text)
```

**Key Points:**
1. **Panic safety**: All FFI calls wrapped in catch_unwind
2. **Validation**: Input validated before processing
3. **State management**: Buffer updated after successful transformation
4. **Memory safety**: Client responsible for freeing result strings

---

## Configuration Update Flow

### Updating Engine Configuration

```mermaid
sequenceDiagram
    autonumber
    participant Client as Platform Client
    participant FFI as FFI API Layer
    participant Container as DI Container
    participant Config as ConfigService
    participant Processor as ProcessorService
    participant Input as InputMethod
    
    Client->>+FFI: ime_set_config(handle, config)
    
    FFI->>FFI: Validate handle
    FFI->>FFI: Convert FfiConfig to EngineConfig
    
    FFI->>+Container: get_config_service()
    Container-->>-FFI: ConfigService
    
    FFI->>+Config: update_config(engine_config)
    Config->>Config: Validate config values
    Config->>Config: Store new config
    Config-->>-FFI: Result::Ok
    
    Note over FFI: Recreate input method with new config
    FFI->>+Container: get_processor_service()
    Container-->>-FFI: ProcessorService
    
    FFI->>+Container: create_input_method(config.input_method)
    
    alt InputMethod = Telex
        Container->>Container: TelexAdapter::new()
    else InputMethod = VNI
        Container->>Container: VniAdapter::new()
    end
    
    Container-->>-FFI: Box<dyn InputMethod>
    
    FFI->>+Processor: update_input_method(input_method)
    Processor->>Processor: Store new input method
    Processor-->>-FFI: OK
    
    FFI-->>-Client: FfiResult{success: true}
    
    Note over Client: Next keystroke uses new config
```

**Key Points:**
1. **Hot reload**: Configuration can be changed at runtime
2. **No restart**: Engine continues with new settings
3. **Atomic update**: All components updated consistently

---

## Validation Pipeline Flow

### Multi-stage Validation

```mermaid
sequenceDiagram
    autonumber
    participant UseCase as ValidateInput Use Case
    participant FSM as FSM Validator
    participant Phono as Phonotactic Validator
    participant Lang as Language Detector
    participant Dict as Dictionary Repo
    
    UseCase->>UseCase: Parse buffer into syllable
    
    Note over UseCase: Stage 1: FSM Validation
    UseCase->>+FSM: validate(syllable)
    FSM->>FSM: Check vowel combinations
    FSM->>FSM: Check consonant rules
    FSM->>FSM: Check tone placement
    
    alt Valid Vietnamese structure
        FSM-->>UseCase: ValidationResult::Valid
    else Invalid structure
        FSM-->>-UseCase: ValidationResult::Invalid(reason)
        UseCase-->>UseCase: Return early with error
    end
    
    Note over UseCase: Stage 2: Phonotactic Rules
    UseCase->>+Phono: validate(syllable)
    Phono->>Phono: Check initial consonant rules
    Phono->>Phono: Check final consonant rules
    Phono->>Phono: Check tone + final consonant
    
    alt Phonotactically valid
        Phono-->>UseCase: ValidationResult::Valid
    else Phonotactically invalid
        Phono-->>-UseCase: ValidationResult::Invalid(reason)
        UseCase-->>UseCase: Return early with error
    end
    
    Note over UseCase: Stage 3: Language Detection
    UseCase->>+Lang: is_vietnamese_word(text)
    Lang->>+Dict: lookup_english(text)
    
    alt Found in English dictionary
        Dict-->>Lang: true
        Lang-->>UseCase: false (English, not Vietnamese)
    else Not in English dictionary
        Dict-->>-Lang: false
        Lang-->>-UseCase: true (Vietnamese)
    end
    
    Note over UseCase: All stages passed
    UseCase-->>UseCase: Return ValidationResult::Valid
```

**Key Points:**
1. **Multi-stage**: Three independent validators
2. **Short-circuit**: Fails fast on first error
3. **Independent**: Each validator has single responsibility

---

## Transformation Pipeline Flow

### Tone and Mark Application

```mermaid
sequenceDiagram
    autonumber
    participant UseCase as TransformText Use Case
    participant Tone as ToneTransformer
    participant Mark as MarkTransformer
    participant Positioning as TonePositioning
    
    Note over UseCase: Input: "viet" + tone Sac
    UseCase->>UseCase: Parse syllable structure
    
    Note over UseCase: Step 1: Apply tone mark
    UseCase->>+Tone: apply_tone(syllable, Sac)
    Tone->>Tone: Identify vowel cluster: "ie"
    Tone->>+Positioning: find_tone_position("ie")
    
    Note over Positioning: Use modern/old style
    Positioning->>Positioning: Check rules for "ie"
    Positioning-->>-Tone: Position = 'e'
    
    Tone->>Tone: Add sắc to 'e': "é"
    Tone-->>-UseCase: "viét"
    
    Note over UseCase: Step 2: Check for diacritic marks
    UseCase->>+Mark: has_marks("viét")
    Mark-->>-UseCase: false (no circumflex/horn/breve)
    
    alt If marks needed (e.g., "vieetj" → "việt")
        UseCase->>+Mark: apply_mark(syllable, Circumflex)
        Mark->>Mark: Find target vowel: "e"
        Mark->>Mark: Apply circumflex: "ê"
        Mark-->>-UseCase: "viêt"
        
        Note over UseCase: Re-apply tone after mark
        UseCase->>+Tone: apply_tone("viêt", Sac)
        Tone->>+Positioning: find_tone_position("iê")
        Positioning-->>-Tone: Position = 'ê'
        Tone-->>-UseCase: "việt"
    end
    
    Note over UseCase: Build final result
    UseCase->>UseCase: TransformResult::builder()
    UseCase->>UseCase: set_new_text("việt")
    UseCase->>UseCase: set_backspace_count(4)
    UseCase-->>UseCase: TransformResult
```

**Key Points:**
1. **Order matters**: Marks first, then tones
2. **Re-application**: Tone position recalculated after marks
3. **Strategy pattern**: Modern vs old tone placement

---

## Shortcut Expansion Flow

### Text Expansion

```mermaid
sequenceDiagram
    autonumber
    participant Client as Platform Client
    participant UseCase as ManageShortcuts Use Case
    participant Repo as ShortcutRepository
    participant Processor as ProcessorService
    
    Note over Client: User types "brb" + space
    Client->>+Processor: process_key(context)
    
    Processor->>Processor: Detect space/trigger
    Processor->>+UseCase: expand_shortcut("brb")
    
    UseCase->>+Repo: find_by_trigger("brb")
    
    alt Shortcut found
        Repo-->>UseCase: Shortcut{trigger: "brb", expansion: "be right back"}
        
        UseCase->>UseCase: Build TransformResult
        UseCase->>UseCase: set_new_text("be right back")
        UseCase->>UseCase: set_backspace_count(3) // delete "brb"
        UseCase-->>Processor: TransformResult
        
        Processor-->>-Client: ProcessResult{text: "be right back", backspace: 3}
        
        Client->>Client: Delete "brb"
        Client->>Client: Insert "be right back"
        
    else Shortcut not found
        Repo-->>-UseCase: None
        UseCase-->>Processor: TransformResult{no change}
        Processor-->>Client: ProcessResult{consumed: false}
        Client->>Client: Insert space normally
    end
```

**Key Points:**
1. **Trigger detection**: Space, Enter, or punctuation
2. **Repository pattern**: Shortcuts stored independently
3. **Optional feature**: Can be disabled via config

---

## Buffer Management Flow

### State Tracking

```mermaid
sequenceDiagram
    autonumber
    participant Processor as ProcessorService
    participant Buffer as BufferManager
    participant History as HistoryTracker
    
    Note over Processor: User types "v"
    Processor->>+Buffer: append_char('v')
    Buffer->>Buffer: Update internal buffer: "v"
    Buffer->>+History: save_state(buffer_snapshot)
    History->>History: Push to history stack
    History-->>-Buffer: OK
    Buffer-->>-Processor: Buffer{content: "v"}
    
    Note over Processor: User types "i"
    Processor->>+Buffer: append_char('i')
    Buffer->>Buffer: Update internal buffer: "vi"
    Buffer->>+History: save_state(buffer_snapshot)
    History-->>-Buffer: OK
    Buffer-->>-Processor: Buffer{content: "vi"}
    
    Note over Processor: User types "e"
    Processor->>+Buffer: append_char('e')
    Buffer->>Buffer: Update internal buffer: "vie"
    Buffer->>+History: save_state(buffer_snapshot)
    History-->>-Buffer: OK
    Buffer-->>-Processor: Buffer{content: "vie"}
    
    Note over Processor: User presses backspace
    Processor->>+Buffer: undo()
    Buffer->>+History: pop_state()
    History->>History: Pop from stack
    History-->>-Buffer: Previous buffer: "vi"
    Buffer->>Buffer: Restore buffer: "vi"
    Buffer-->>-Processor: Buffer{content: "vi"}
    
    Note over Processor: User commits (space/enter)
    Processor->>+Buffer: clear()
    Buffer->>Buffer: Reset internal buffer
    Buffer->>+History: clear_history()
    History->>History: Clear stack
    History-->>-Buffer: OK
    Buffer-->>-Processor: Buffer{content: ""}
```

**Key Points:**
1. **Immutable snapshots**: Each state saved separately
2. **Undo support**: History stack enables backspace undo
3. **Memory management**: History limited to N entries

---

## Error Handling Flow

### Panic Recovery at FFI Boundary

```mermaid
sequenceDiagram
    autonumber
    participant Client as Platform Client
    participant FFI as FFI API Layer
    participant Panic as catch_unwind
    participant Processor as ProcessorService
    
    Client->>+FFI: ime_process_key(handle, key, action)
    
    FFI->>+Panic: catch_unwind(|| { ... })
    
    Note over Panic: Execute processing
    Panic->>+Processor: process_key(context)
    
    alt Normal execution
        Processor-->>Panic: TransformResult
        Panic-->>FFI: Ok(TransformResult)
        FFI->>FFI: Convert to FfiProcessResult
        FFI-->>Client: FfiProcessResult{success: true}
        
    else Panic occurs
        Processor->>Processor: panic!("unexpected error")
        Processor--xPanic: Panic caught
        Panic-->>-FFI: Err(PanicInfo)
        
        Note over FFI: Log panic details
        FFI->>FFI: eprintln!("Panic: {}", info)
        
        Note over FFI: Return safe error result
        FFI->>FFI: Create default FfiProcessResult
        FFI-->>-Client: FfiProcessResult{success: false, error_code: 5}
        
        Note over Client: Client handles error gracefully
        Client->>Client: Log error
        Client->>Client: Continue operation
    end
```

**Key Points:**
1. **No panic propagation**: All panics caught at FFI boundary
2. **Safe fallback**: Default error result returned
3. **Client resilience**: Platform code continues operating

---

### Validation Error Handling

```mermaid
sequenceDiagram
    autonumber
    participant UseCase as ValidateInput Use Case
    participant Validator as SyllableValidator
    participant Processor as ProcessorService
    participant Client as Platform Client
    
    UseCase->>+Validator: validate(syllable)
    
    alt Invalid syllable
        Validator->>Validator: Check rules
        Validator-->>-UseCase: ValidationResult::Invalid(reason)
        
        UseCase->>UseCase: Log validation error
        UseCase-->>Processor: TransformResult{consumed: false}
        
        Processor-->>Client: ProcessResult{consumed: false}
        
        Note over Client: Pass-through to OS
        Client->>Client: Insert character normally
        
    else Valid syllable
        Validator-->>UseCase: ValidationResult::Valid
        UseCase-->>Processor: TransformResult{...}
        Processor-->>Client: ProcessResult{consumed: true}
    end
```

**Key Points:**
1. **Non-blocking**: Invalid input passed through
2. **Graceful degradation**: Engine doesn't crash, just doesn't transform
3. **User experience**: Natural typing continues

---

## Engine Lifecycle

### Initialization and Cleanup

```mermaid
sequenceDiagram
    autonumber
    participant Client as Platform Client
    participant FFI as FFI API Layer
    participant Container as DI Container
    participant Config as ConfigService
    participant Processor as ProcessorService
    participant Adapters as Adapters
    
    Note over Client: Application startup
    Client->>+FFI: ime_engine_new_with_config(config)
    
    FFI->>FFI: Convert FfiConfig to EngineConfig
    FFI->>+Container: new(engine_config)
    
    Note over Container: Wire dependencies
    Container->>+Adapters: create_input_method()
    Adapters-->>-Container: Box<dyn InputMethod>
    
    Container->>+Adapters: create_validator()
    Adapters-->>-Container: Box<dyn SyllableValidator>
    
    Container->>+Adapters: create_tone_transformer()
    Adapters-->>-Container: Box<dyn ToneTransformer>
    
    Container->>+Adapters: create_buffer_manager()
    Adapters-->>-Container: Box<dyn BufferManager>
    
    Container->>+Config: new()
    Config-->>-Container: ConfigService
    
    Container->>+Processor: new(all dependencies)
    Processor->>Processor: Store trait objects
    Processor-->>-Container: ProcessorService
    
    Container->>Container: Wrap in Arc<Mutex<>>
    Container-->>-FFI: Container
    
    FFI->>FFI: Box::into_raw(container)
    FFI-->>-Client: FfiEngineHandle (opaque pointer)
    
    Note over Client: Use engine for processing...
    Client->>Client: Process keystrokes
    
    Note over Client: Application shutdown
    Client->>+FFI: ime_engine_free(handle)
    
    FFI->>FFI: Validate handle
    FFI->>FFI: Box::from_raw(handle)
    
    Note over FFI: Drop cascades through all components
    FFI->>Container: Drop
    Container->>Processor: Drop
    Processor->>Adapters: Drop trait objects
    Adapters->>Adapters: Clean up resources
    
    FFI-->>-Client: void
    
    Note over Client: Engine fully cleaned up
```

**Key Points:**
1. **RAII**: Rust ownership ensures cleanup
2. **Cascading drops**: All resources freed automatically
3. **No memory leaks**: Smart pointers handle everything

---

## Summary

### Flow Patterns

| Flow | Complexity | Key Pattern | Actors |
|------|-----------|-------------|---------|
| **Keystroke Processing** | High | Orchestration | 9 components |
| **Configuration Update** | Medium | Hot reload | 5 components |
| **Validation Pipeline** | Medium | Chain of responsibility | 4 validators |
| **Transformation Pipeline** | Medium | Strategy + Chain | 3 transformers |
| **Shortcut Expansion** | Low | Repository | 3 components |
| **Buffer Management** | Medium | Memento | 2 state managers |
| **Error Handling** | Low | Fail-safe | 2 layers |
| **Engine Lifecycle** | High | RAII | 6 components |

---

### Common Patterns Observed

1. **Dependency Injection**: Services receive dependencies via constructor
2. **Result/Option**: No exceptions, explicit error handling
3. **Trait Objects**: Box<dyn Trait> for runtime polymorphism
4. **Smart Pointers**: Arc<Mutex<>> for thread-safe sharing
5. **RAII**: Automatic cleanup via Drop trait
6. **Panic Safety**: catch_unwind at FFI boundaries

---

## Tools Used

- **Mermaid**: All diagrams use Mermaid sequence syntax
- **Rendering**: GitHub, VS Code with Mermaid extension, or https://mermaid.live

---

**Last Updated:** 2026-02-11  
**Version:** 1.0.0
