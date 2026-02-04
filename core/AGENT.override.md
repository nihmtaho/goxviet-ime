# AGENT OVERRIDE: CORE ENGINE (RUST)

## Context
You are working on the **Core Engine** of GoxViet. This is a high-performance Rust library that handles all linguistic processing for Vietnamese input. It is exposed to native platforms (macOS, Windows) via FFI.

## Rules & Standards

### 1. Performance is Paramount
- **Zero Allocations in Hot Path:** Avoid `String` heaps in the main loop (`process_key`). Use `SmallVec` or stack arrays.
- **Efficiency:** Latency must be < 3ms.

### 2. FFI Safety (CRITICAL)
- **NO PANICS:** You must NEVER panic across FFI boundaries.
    - Catch Unwind: Use `std::panic::catch_unwind` in all `extern "C"` functions.
- **Pointer Safety:** Always validate pointers (`is_null()`) before unsafe dereference.

### 3. Coding Style
- **Rust API Guidelines:** Follow official guidelines.
- **Error Handling:** Use `Result` propagation. Do not ignore errors.
- **String Handling:** Use UTF-8 internally. Convert at boundaries.

### 4. Testing
- **Unit Tests:** Crucial for logic verification.
- **Integration Tests:** Verify FFI interactions.

### 5. Documentation
- Update `.docs/features/core-engine/` when changing logic.
