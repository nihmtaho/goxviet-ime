# Todos: Core Vietnamese Engine Refactor

**Feature:** core-refactor-vietnamese-engine  
**Milestone:** v2.0.11 (Q1 2026)

---

## Track 1: Regression Tests First (Pre-condition)

### T1.1 — Viết regression tests trước khi xoá English detection
**As a developer, I want** regression tests covering current auto-restore behavior  
**so that** I can safely remove English detection without breaking known good cases.

**Acceptance Criteria:**
- [ ] Test file `core/tests/auto_restore_regression.rs` tồn tại
- [ ] Các từ tiếng Anh đã biết test: "array", "aroma", "windows", "enter", "stop"
- [ ] Các từ tiếng Việt đã biết test: "ăn", "uống", "người", "đường", "bình"
- [ ] Compound words test: "ánh sáng", "ai đó", "ăn cơm"
- [ ] Tất cả tests pass trước khi bắt đầu refactor

**Story points:** 3

---

## Track 2: Vietnamese Dictionary (phf Build-Time)

### T2.1 — Setup phf build infrastructure
**As a developer, I want** phf code generation setup  
**so that** dictionary data được embed tại build time.

**Acceptance Criteria:**
- [ ] `phf` và `phf_codegen` thêm vào `Cargo.toml`
- [ ] `build.rs` script tạo phf sets từ TuDien.txt và TuDienTuGhep.txt
- [ ] Build time tăng không quá 30s (benchmark)
- [ ] Generated file ở `core/src/data/generated/`

**Story points:** 3

### T2.2 — Embed TuDien.txt (single-syllable, ~7K entries)
**As a developer, I want** O(1) lookup cho ~7,000 âm tiết tiếng Việt hợp lệ  
**so that** engine có thể validate syllable chính xác và nhanh.

**Acceptance Criteria:**
- [ ] `phf::Set<&'static str>` cho TuDien.txt tạo thành công
- [ ] `is_valid_vietnamese_syllable(s: &str) -> bool` function hoạt động
- [ ] Lookup time < 1μs (benchmark với criterion)
- [ ] Test: tất cả entries trong TuDien.txt trả về `true`

**Story points:** 2

### T2.3 — Convert TuDienTuGhep.txt to `.bin` binary format
**As a developer, I want** TuDienTuGhep compressed vào binary format  
**so that** 1MB .txt → ~250KB binary, reuse infrastructure sẵn có.

**Acceptance Criteria:**
- [ ] Build script tạo `viet_compound.bin` từ TuDienTuGhep.txt
- [ ] Format: sorted UTF-8 strings, mỗi entry có length prefix (hoặc null-terminated)
- [ ] `include_bytes!` nhúng vào crate
- [ ] `is_vietnamese_compound(text: &str) -> bool` dùng binary search, O(log n)
- [ ] Lookup time < 50μs (benchmark)
- [ ] Binary size ≤ 300KB

**Story points:** 3

---

## Track 3: Syllable Structure Refactor (GhepVan.ini → PAD/NA/PAC)

### T3.1 — Thiết kế và implement PAD/NA/PAC lookup tables
**As a developer, I want** Vietnamese syllable structure validation theo PAD/NA/PAC groups  
**so that** in-progress syllables được validate đúng với Vietnamese phonology.

**Acceptance Criteria:**
- [ ] 3 PAD groups (PAD.0, PAD.1, PAD.2) defined as const arrays
- [ ] 6 NA groups (NA.0–NA.5) defined với đầy đủ entries
- [ ] 3 PAC groups (PAC.0, PAC.1, PAC.2) defined
- [ ] PAD_NA combination table: 3 rows
- [ ] NA_PAC combination table: 6 rows
- [ ] Unit tests cho tất cả valid combinations từ GhepVan.ini
- [ ] Test file: `core/tests/syllable_pad_na_pac.rs`

**Story points:** 5

### T3.2 — Implement PAD/NA/PAC validator trong infrastructure layer
**As a developer, I want** `SyllableStructureValidator` implement `SyllableValidationPort`  
**so that** validator mới drop-in thay FSM.

**Acceptance Criteria:**
- [ ] `core/src/infrastructure/adapters/validation/syllable_structure_validator.rs` tạo mới
- [ ] Implement `SyllableValidationPort` trait
- [ ] `is_valid_structure(syllable: &Syllable) -> bool` dùng PAD/NA/PAC tables
- [ ] Performance: < 1μs per syllable
- [ ] Unit tests: ≥ 30 test cases (valid + invalid)

**Story points:** 5

### T3.3 — Wire PAD/NA/PAC validator vào application layer, xoá FSM
**As a developer, I want** FSM validator được thay thế hoàn toàn  
**so that** codebase không còn FSM complexity.

**Acceptance Criteria:**
- [ ] DI container dùng `SyllableStructureValidator` thay FSM
- [ ] FSM files xoá hoặc deprecated với rõ ràng comment
- [ ] Tất cả tests pass sau khi switch
- [ ] Không còn import FSM ở production code

**Story points:** 3

---

## Track 4: Vietnamese-First Auto-Restore Logic

### T4.1 — Refactor `LanguageDecisionEngine` — Vietnamese-First priority
**As a developer, I want** LanguageDecisionEngine ưu tiên Vietnamese lookup trước  
**so that** từ tiếng Việt không bị restore nhầm, nhưng từ tiếng Anh như "restore", "windows" vẫn được restore đúng.

**Acceptance Criteria:**
- [ ] Pipeline mới trong `language_decision.rs`:
  1. `VietDictionary::is_syllable(keys)` → match → return `is_english: false`
  2. `VietCompound::is_compound(context)` → match → return `is_english: false`
  3. `PAD/NA/PAC structural check` → valid → return `is_english: false`
  4. `PhonotacticEngine::analyze(keys)` (KEEP) → English confidence → return result
  5. Default → `is_english: true` (safe restore)
- [ ] Xoá call `Dictionary::is_english()` (Priority 1 cũ)
- [ ] Unit tests:
  - "khong" → Vietnamese (TuDien match) → không restore
  - "restore" → không match Viet → Phonotactic detect English → restore  
  - "array" → không match Viet → Phonotactic detect English → restore
  - "anh" → TuDien match ("anh") → không restore

**Story points:** 5

### T4.2 — Xoá English Dictionary binary (giữ PhonotacticEngine)
**As a developer, I want** toàn bộ English dictionary code được xoá  
**so that** không còn false-positive từ hardcoded English word list.

**Acceptance Criteria:**
- [ ] Xoá `core/src/infrastructure/adapters/validation/english/dictionary.rs`
- [ ] Xoá `core/src/infrastructure/adapters/validation/english/dictionary_data.rs`
- [ ] GIỮ `phonotactic.rs` và `language_decision.rs` (đã refactor ở T4.1)
- [ ] GIỮ `mod.rs` trong `english/` directory
- [ ] Xoá call `Dictionary::is_english()` khỏi mọi nơi
- [ ] Regression tests từ T1.1 vẫn pass
- [ ] `cargo build --release` clean, không còn dead code warnings

**Story points:** 3

---

## Track 5: Buffer/Char Simplification

### T5.1 — Simplify `Char` struct align với gonhanh style
**As a developer, I want** `Char` struct đơn giản, flat  
**so that** buffer code dễ đọc và maintain hơn.

**Acceptance Criteria:**
- [ ] `Char` struct có fields: `key: u16`, `caps: bool`, `tone: u8`, `mark: u8`, `stroke: bool`
- [ ] Xoá các fields/methods không cần thiết (align với gonhanh)
- [ ] Tất cả consumers của `Char` updated
- [ ] Unit tests cho `Char` methods: `has_tone()`, `has_mark()`
- [ ] `cargo test` pass

**Story points:** 3

### T5.2 — Simplify `Buffer` struct
**As a developer, I want** `Buffer` struct gọn hơn với `data: [Char; MAX]`, `len: usize`  
**so that** buffer operations predictable và zero-allocation.

**Acceptance Criteria:**
- [ ] `Buffer` struct: `data: [Char; MAX]`, `len: usize` (stack-only)
- [ ] Methods: `push`, `pop`, `clear`, `len`, `is_empty`, `as_slice`
- [ ] No heap allocation trong buffer operations
- [ ] Tất cả consumers updated
- [ ] Benchmark: buffer push/pop < 100ns

**Story points:** 3

---

## Track 6: KieuGo.ini Pattern — Data-Driven Input Method

### T6.1 — Thiết kế `InputMethodConfig` type trong Rust core
**As a developer, I want** `InputMethodConfig` struct trong domain layer  
**so that** Rust core owns input method definition.

**Acceptance Criteria:**
- [ ] `InputMethodConfig` struct với mapping: `HashMap<char, InputAction>`
- [ ] `InputAction` enum: `ToneSac`, `ToneHuyen`, `ToneHoi`, `ToneNga`, `ToneNang`, `XoaDau`, `ModA`, `ModE`, `ModO`, `ModAW`, `ModOW`, `ModUW`, `StrokeD`, `CompoundUOA`
- [ ] Built-in configs: `InputMethodConfig::telex()`, `InputMethodConfig::vni()`
- [ ] JSON serialization (serde) cho FFI transfer
- [ ] Unit tests: Telex config có đúng 11+ mappings, VNI có đúng 10+ mappings

**Story points:** 5

### T6.2 — Expose `ime_load_input_config_v2` FFI function
**As a developer, I want** FFI endpoint để Swift truyền config xuống Rust  
**so that** input method configuration có thể thay đổi mà không cần rebuild.

**Acceptance Criteria:**
- [ ] `ime_load_input_config_v2(config_json: *const u8, len: usize) -> FfiStatusCode` exposed
- [ ] Engine parse JSON và update internal `InputMethodConfig`
- [ ] `catch_unwind` bao quanh toàn bộ FFI function (no panic)
- [ ] Returns `FFI_STATUS_OK` on success, `FFI_STATUS_ERROR` on parse failure
- [ ] Header file `.h` updated với new function declaration

**Story points:** 3

### T6.3 — Swift binding và refactor InputManager.swift
**As a developer, I want** `InputManager.swift` sử dụng data-driven mapping  
**so that** thêm/sửa input method không cần sửa logic code.

**Acceptance Criteria:**
- [ ] `RustBridgeSafe.swift` có `loadInputConfig(_ config: InputMethodConfig)`
- [ ] `InputMethodDefinition.swift` struct với Telex + VNI built-in definitions
- [ ] `InputManager.swift` gọi `loadInputConfig` khi init và khi method change
- [ ] Không còn hardcoded input method logic trong Swift
- [ ] Unit test (mock): Telex config loaded thành công

**Story points:** 5

---

## Track 7: Integration & Quality

### T7.1 — Integration tests end-to-end
**As a developer, I want** end-to-end tests cho toàn bộ refactored pipeline  
**so that** tôi biết chắc chắn không có regression.

**Acceptance Criteria:**
- [ ] Test file `core/tests/integration_vietnamese_first.rs`
- [ ] Test cases: gõ "viet" → commit "việt", gõ "array" → restore "array"
- [ ] Test cases: gõ "anh sang" → commit "ánh sáng" (compound match)
- [ ] Test cases: gõ "windows" → restore "windows"
- [ ] All 4 tracks hoạt động together trong một pipeline test
- [ ] `cargo test` pass, không có regression

**Story points:** 5

### T7.2 — Performance benchmarks
**As a developer, I want** benchmarks cho toàn bộ refactored engine  
**so that** latency targets (< 3ms) được duy trì.

**Acceptance Criteria:**
- [ ] Benchmark: single keystroke processing < 3ms
- [ ] Benchmark: TuDien lookup < 1μs
- [ ] Benchmark: TuDienTuGhep lookup < 100μs
- [ ] Benchmark: PAD/NA/PAC validation < 1μs
- [ ] Benchmark results documented trong PR description
- [ ] Không có regression so với v2.0.10

**Story points:** 2

### T7.3 — Update documentation
**As a developer, I want** docs updated cho toàn bộ thay đổi  
**so that** tương lai maintainers hiểu architecture.

**Acceptance Criteria:**
- [ ] `.docs/features/core-engine/` updated: mô tả Vietnamese-first pipeline
- [ ] Comment trong code giải thích PAD/NA/PAC model
- [ ] `CHANGELOG.md` entry cho v2.0.11
- [ ] ADR.md (file này) finalized với "Accepted" status

**Story points:** 2
