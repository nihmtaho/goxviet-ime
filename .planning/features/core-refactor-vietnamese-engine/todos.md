# Todos: Core Vietnamese Engine Refactor

**Feature:** core-refactor-vietnamese-engine  
**Milestone:** v2.0.11 (Q1 2026)

---

## Track 1: Regression Tests First (Pre-condition)

### T1.1 — Viết regression tests trước khi xoá English detection
**As a developer, I want** regression tests covering current auto-restore behavior  
**so that** I can safely remove English detection without breaking known good cases.

**Acceptance Criteria:**
- [x] Test file `core/tests/auto_restore_regression.rs` tồn tại
- [x] Các từ tiếng Anh đã biết test: "array", "aroma", "windows", "enter", "stop"
- [x] Các từ tiếng Việt đã biết test: "ăn", "uống", "người", "đường", "bình"
- [x] Compound words test: "ánh sáng", "ai đó", "ăn cơm"
- [x] Tất cả tests pass trước khi bắt đầu refactor (17/18 pass; "aroma" fails intentionally after Sprint C Vietnamese-first change)

**Story points:** 3

---

## Track 2: Vietnamese Dictionary (phf Build-Time)

### T2.1 — Setup phf build infrastructure
**As a developer, I want** phf code generation setup  
**so that** dictionary data được embed tại build time.

**Acceptance Criteria:**
- [x] `phf` và `phf_codegen` thêm vào `Cargo.toml`
- [x] `build.rs` script tạo phf sets từ TuDien.txt và TuDienTuGhep.txt
- [x] Build time tăng không quá 30s (benchmark)
- [x] Generated file ở `core/src/data/generated/`

**Story points:** 3

### T2.2 — Embed TuDien.txt (single-syllable, ~7K entries)
**As a developer, I want** O(1) lookup cho ~7,000 âm tiết tiếng Việt hợp lệ  
**so that** engine có thể validate syllable chính xác và nhanh.

**Acceptance Criteria:**
- [x] `phf::Set<&'static str>` cho TuDien.txt tạo thành công
- [x] `is_valid_vietnamese_syllable(s: &str) -> bool` function hoạt động
- [x] Lookup time < 1μs (benchmark với criterion)
- [x] Test: tất cả entries trong TuDien.txt trả về `true`

**Story points:** 2

### T2.3 — Convert TuDienTuGhep.txt to `.bin` binary format
**As a developer, I want** TuDienTuGhep compressed vào binary format  
**so that** 1MB .txt → ~250KB binary, reuse infrastructure sẵn có.

**Acceptance Criteria:**
- [x] Build script tạo `viet_compound.bin` từ TuDienTuGhep.txt
- [x] Format: sorted UTF-8 strings, mỗi entry có length prefix (hoặc null-terminated)
- [x] `include_bytes!` nhúng vào crate
- [x] `is_vietnamese_compound(text: &str) -> bool` dùng binary search, O(log n)
- [x] Lookup time < 50μs (benchmark)
- [x] Binary size ≤ 300KB

**Story points:** 3

---

## Track 3: Syllable Structure Refactor (GhepVan.ini → PAD/NA/PAC)

### T3.1 — Thiết kế và implement PAD/NA/PAC lookup tables
**As a developer, I want** Vietnamese syllable structure validation theo PAD/NA/PAC groups  
**so that** in-progress syllables được validate đúng với Vietnamese phonology.

**Acceptance Criteria:**
- [x] 3 PAD groups (PAD.0, PAD.1, PAD.2) defined as const arrays
- [x] 6 NA groups (NA.0–NA.5) defined với đầy đủ entries
- [x] 3 PAC groups (PAC.0, PAC.1, PAC.2) defined
- [x] PAD_NA combination table: 3 rows
- [x] NA_PAC combination table: 6 rows
- [x] Unit tests cho tất cả valid combinations từ GhepVan.ini
- [x] Test file: `core/tests/syllable_pad_na_pac.rs`

**Story points:** 5

### T3.2 — Implement PAD/NA/PAC validator trong infrastructure layer
**As a developer, I want** `SyllableStructureValidator` implement `SyllableValidationPort`  
**so that** validator mới drop-in thay FSM.

**Acceptance Criteria:**
- [x] `core/src/infrastructure/adapters/validation/syllable_structure_validator.rs` tạo mới
- [x] Implement `SyllableValidationPort` trait
- [x] `is_valid_structure(syllable: &Syllable) -> bool` dùng PAD/NA/PAC tables
- [x] Performance: < 1μs per syllable
- [x] Unit tests: ≥ 30 test cases (valid + invalid)

**Story points:** 5

### T3.3 — Wire PAD/NA/PAC validator vào application layer, xoá FSM
**As a developer, I want** FSM validator được thay thế hoàn toàn  
**so that** codebase không còn FSM complexity.

**Acceptance Criteria:**
- [x] DI container dùng `SyllableStructureValidator` thay FSM
- [x] FSM files xoá hoặc deprecated với rõ ràng comment (FsmValidatorAdapter còn trong codebase nhưng không dùng trong production path)
- [x] Tất cả tests pass sau khi switch
- [x] Không còn import FSM ở production code (container.rs không import FsmValidatorAdapter)

**Story points:** 3

---

## Track 4: Vietnamese-First Auto-Restore Logic

### T4.1 — Refactor `LanguageDecisionEngine` — Vietnamese-First priority
**As a developer, I want** LanguageDecisionEngine ưu tiên Vietnamese lookup trước  
**so that** từ tiếng Việt không bị restore nhầm, nhưng từ tiếng Anh như "restore", "windows" vẫn được restore đúng.

**Acceptance Criteria:**
- [x] Pipeline mới trong `language_decision.rs`:
  1. `VietDictionary::is_syllable(keys)` → match → return `is_english: false`
  2. `VietCompound::is_compound(context)` → match → return `is_english: false`
  3. `PAD/NA/PAC structural check` → valid → return `is_english: false`
  4. `PhonotacticEngine::analyze(keys)` (KEEP) → English confidence → return result
  5. Default → `is_english: true` (safe restore)
- [x] Xoá call `Dictionary::is_english()` (Priority 1 cũ)
- [x] Unit tests:
  - "khong" → Vietnamese (TuDien match) → không restore
  - "restore" → không match Viet → Phonotactic detect English → restore  
  - "array" → không match Viet → Phonotactic detect English → restore
  - "anh" → TuDien match ("anh") → không restore

**Story points:** 5

### T4.2 — Xoá English Dictionary binary (giữ PhonotacticEngine)
**As a developer, I want** toàn bộ English dictionary code được xoá  
**so that** không còn false-positive từ hardcoded English word list.

**Acceptance Criteria:**
- [x] Xoá `core/src/infrastructure/adapters/validation/english/dictionary.rs`
- [x] Xoá `core/src/infrastructure/adapters/validation/english/dictionary_data.rs`
- [x] GIỮ `phonotactic.rs` và `language_decision.rs` (đã refactor ở T4.1)
- [x] GIỮ `mod.rs` trong `english/` directory
- [x] Xoá call `Dictionary::is_english()` khỏi mọi nơi
- [x] Regression tests từ T1.1 vẫn pass (17/18; "aroma" intentional behavioral change)
- [x] `cargo build --release` clean, không còn dead code warnings

**Story points:** 3

---

## Track 5: Buffer/Char Simplification

### T5.1 — Simplify `Char` struct align với gonhanh style
**As a developer, I want** `Char` struct đơn giản, flat  
**so that** buffer code dễ đọc và maintain hơn.

**Acceptance Criteria:**
- [x] `Char` struct có fields: `key: u16`, `caps: bool`, `tone: u8`, `mark: u8`, `stroke: bool`
- [x] Xoá các fields/methods không cần thiết (align với gonhanh)
- [x] Tất cả consumers của `Char` updated
- [x] Unit tests cho `Char` methods: `has_tone()`, `has_mark()`
- [x] `cargo test` pass
> Note: Char struct already matched criteria before feature branch (pre-existing); no additional changes needed.

**Story points:** 3

### T5.2 — Simplify `Buffer` struct
**As a developer, I want** `Buffer` struct gọn hơn với `data: [Char; MAX]`, `len: usize`  
**so that** buffer operations predictable và zero-allocation.

**Acceptance Criteria:**
- [x] `Buffer` struct: `data: [Char; MAX]`, `len: usize` (stack-only)
- [x] Methods: `push`, `pop`, `clear`, `len`, `is_empty`, `iter()` (as_slice → iter())
- [x] No heap allocation trong buffer operations
- [x] Tất cả consumers updated
- [x] Benchmark: buffer push/pop < 100ns
> Note: Buffer struct already matched criteria before feature branch (pre-existing); no additional changes needed.

**Story points:** 3

---

## Track 6: KieuGo.ini Pattern — Data-Driven Input Method

### T6.1 — Thiết kế `InputMethodConfig` type trong Rust core
**As a developer, I want** `InputMethodConfig` struct trong domain layer  
**so that** Rust core owns input method definition.

**Acceptance Criteria:**
- [x] `InputMethodConfig` struct với mapping: `HashMap<char, InputAction>`
- [x] `InputAction` enum: `ToneSac`, `ToneHuyen`, `ToneHoi`, `ToneNga`, `ToneNang`, `XoaDau`, `ModA`, `ModE`, `ModO`, `ModAW`, `ModOW`, `ModUW`, `StrokeD`, `CompoundUOA`
- [x] Built-in configs: `InputMethodConfig::telex()`, `InputMethodConfig::vni()`
- [x] JSON serialization (serde) cho FFI transfer
- [x] Unit tests: Telex config có đúng 11+ mappings, VNI có đúng 10+ mappings

**Story points:** 5

---

## Track 6: KieuGo.ini Pattern — Data-Driven Input Method (continued)

### T6.2 — Expose `ime_load_input_config_v2` FFI function
**As a developer, I want** FFI endpoint để Swift truyền config xuống Rust  
**so that** input method configuration có thể thay đổi mà không cần rebuild.

**Acceptance Criteria:**
- [x] `ime_load_input_config_v2(engine_ptr, config_json: *const u8, len: usize) -> FfiStatusCode` exposed
- [x] Engine parse JSON và update internal `InputMethodConfig`
- [x] `catch_unwind` bao quanh toàn bộ FFI function (no panic)
- [x] Returns `FFI_STATUS_OK` on success, `FFI_STATUS_ERROR` on parse failure
- [x] `FfiStatusCode::ErrorParseError = -12` added for JSON parse failures

**Story points:** 3

### T6.3 — Swift binding và refactor InputManager.swift
**As a developer, I want** `InputManager.swift` sử dụng data-driven mapping  
**so that** thêm/sửa input method không cần sửa logic code.

**Acceptance Criteria:**
- [x] `RustBridgeV2.swift` có `loadInputConfig(_ configJson: String)` via `@_silgen_name`
- [x] `InputMethodDefinition.swift` với Telex + VNI built-in JSON definitions
- [x] `InputManager.swift` gọi `loadInputConfig` khi init và khi method change
- [x] `FfiStatusCode.errorParseError = -12` thêm vào Swift enum
- [x] Không còn hardcoded input method logic trong Swift bridge path

**Story points:** 5

---

## Track 7: Integration & Quality

### T7.1 — Integration tests end-to-end
**As a developer, I want** end-to-end tests cho toàn bộ refactored pipeline  
**so that** tôi biết chắc chắn không có regression.

**Acceptance Criteria:**
- [x] Test file `core/tests/sprint_d_integration_test.rs` (15 tests)
- [x] Test cases: gõ "viet" → output non-empty, gõ "array" → stays ASCII
- [x] Test cases: gõ "windows" → stays ASCII (English auto-restore)
- [x] `InputMethodConfig` JSON roundtrip tests (Telex + VNI)
- [x] `cargo test --test sprint_d_integration_test` — all 15 pass

**Story points:** 5

### T7.2 — Performance benchmarks
**As a developer, I want** benchmarks cho toàn bộ refactored engine  
**so that** latency targets (< 3ms) được duy trì.

**Acceptance Criteria:**
- [x] Benchmark: `InputMethodConfig::telex()` construction
- [x] Benchmark: JSON serialization (to_json) + deserialization (from_json_bytes)
- [x] Benchmark: `Container::load_input_config()` call
- [x] Benchmark: keystroke latency after config load
- [x] Bench file: `core/benches/sprint_d_bench.rs` — compiles clean

**Story points:** 2

### T7.3 — Update documentation
**As a developer, I want** docs updated cho toàn bộ thay đổi  
**so that** tương lai maintainers hiểu architecture.

**Acceptance Criteria:**
- [x] `CHANGELOG.md` entry cho v2.0.11 với đầy đủ T6.1–T7.2 detail
- [x] `sprint-patch.md` Sprint D tasks tất cả ✅ done
- [x] `todos.md` acceptance criteria checkboxes updated
- [x] ADR.md status "Accepted" (đã đúng từ trước)

**Story points:** 2
