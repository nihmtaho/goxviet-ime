# FEATURE SPEC: Core Vietnamese Engine Refactor

**Feature slug:** `core-refactor-vietnamese-engine`  
**Milestone:** v2.0.11 (Q1 2026, in_progress)  
**Status:** Planning  
**Created:** 2026-02-27  

---

## Problem Statement

Hệ thống auto-restore hiện tại của GoxViet phụ thuộc vào:
1. **English dictionary** (binary lookup) để phát hiện từ tiếng Anh — gây false-positive với từ tiếng Việt trùng key sequence
2. **Phonotactic analysis** (PhonotacticEngine 8-layer) để đoán ngôn ngữ — **GIỮ LẠI** (thuật toán tốt)
3. **LanguageDecisionEngine** — **GIỮ LẠI** nhưng cần cải thiện thứ tự priority

Vấn đề cốt lõi:
- **English dictionary** chiếm Priority 1, override cả Vietnamese signals → false-positive
- Không có Vietnamese dictionary check trước English check
- FSM validator khó mở rộng và maintain
- Vietnamese .txt files quá lớn (TuDien: 42KB, TuDienTuGhep: 1MB), chưa được nhúng vào engine

Đồng thời, **InputMethod trait** bị hardcode, không theo pattern data-driven của `KieuGo.ini`.

---

## Proposed Solution

### Track 1: Vietnamese-First + Phonotactic Hybrid Pipeline
Điều chỉnh thứ tự priority trong `LanguageDecisionEngine`:
```
Pipeline mới (ưu tiên từ trên xuống):
  1. TuDien (phf, O(1))       → exact syllable match → Vietnamese → KHÔNG restore
  2. TuDienTuGhep (.bin, O(log n)) → compound match → Vietnamese → KHÔNG restore
  3. PAD/NA/PAC structural check  → valid structure → in-progress Viet → KHÔNG restore
  4. PhonotacticEngine (giữ)   → English pattern detected → RESTORE
  5. Default                   → RESTORE (safe fallback)
```
**Giữ lại:** `PhonotacticEngine` + `LanguageDecisionEngine` skeleton  
**Xoá:** `Dictionary::is_english()` và toàn bộ English `.bin` data

### Track 2: Vietnamese Dictionaries — Compact Embedded Format
- **TuDien.txt (42KB → ~15KB)**: Dùng `phf::Set<&'static str>` — O(1) lookup, build-time embed
- **TuDienTuGhep.txt (1MB → ~250KB)**: Dùng binary `.bin` format giống English dict hiện tại — sorted UTF-8 codepoint arrays, `include_bytes!`, binary search O(log n)

### Track 2: Syllable Structure Refactor (GhepVan.ini → PAD/NA/PAC)
Thay FSM validator bằng group-based validation:
```
PAD.0 = b d đ g gh m n nh p ph r s t tr v    (trước mọi NA)
PAD.1 = c h k kh qu th                        (không trước NA.3, NA.4)
PAD.2 = ch gi l ng ngh x                      (chỉ trước NA.0-NA.2)

NA.0 = ê i ua uê uy y
NA.1 = a iê oa uyê yê
NA.2 = â ă e o oo ô ơ oe u ư uâ uô ươ
NA.3 = oă
NA.4 = uơ
NA.5 = ai ao au âu ay ây eo êu ia iêu iu...  (không PAC)

PAC.0 = ch nh           (sau NA.0, NA.2)
PAC.1 = c ng            (sau NA.0–NA.4)
PAC.2 = m n p t         (sau NA.0–NA.4)
```

### Track 3: Buffer/Char Simplification
Align `Char` struct với gonhanh style, giữ SOLID layer:
```rust
pub struct Char {
    pub key: u16,
    pub caps: bool,
    pub tone: u8,    // 0=none, 1=circumflex(^), 2=horn/breve
    pub mark: u8,    // 0=none, 1=sắc, 2=huyền, 3=hỏi, 4=ngã, 5=nặng
    pub stroke: bool // d→đ
}
pub struct Buffer { data: [Char; MAX], len: usize }
```

### Track 4: KieuGo.ini Pattern — Data-Driven Input Method
Rust core nhận `InputMethodConfig` qua FFI, Swift truyền xuống khi init:
```c
// New FFI
void ime_load_input_config_v2(const uint8_t* config_json, size_t len);
```
Swift định nghĩa mapping tables (Telex, VNI, và tương lai: VIQR, Microsoft layout) và truyền xuống Rust core thay vì hardcode.

---

## Impact Analysis

```
IMPACT ANALYSIS for: core-refactor-vietnamese-engine
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📦 Affected Modules:
  - [MODIFY]  core/src/infrastructure/adapters/validation/english/language_decision.rs
      → Đổi Priority 1 từ English dict check → Vietnamese dict check
      → Xoá call Dictionary::is_english()
  - [REMOVE]  core/src/infrastructure/adapters/validation/english/dictionary.rs
  - [REMOVE]  core/src/infrastructure/adapters/validation/english/dictionary_data.rs
  - [KEEP]    core/src/infrastructure/adapters/validation/english/phonotactic.rs  ← GIỮ NGUYÊN
  - [KEEP]    core/src/infrastructure/adapters/validation/english/language_decision.rs ← CẬP NHẬT
  - [REMOVE]  core/src/infrastructure/adapters/validation/language_detector_adapter.rs (nếu chỉ wrap dict)
  - [UPDATE]  core/src/application/services/processor_service.rs
      → Loại bỏ call English dict; wire Vietnamese dicts vào pipeline
  - [UPDATE]  core/src/unified_engine.rs
      → Inject Vietnamese dicts vào LanguageDecisionEngine
  - [ADD]     core/src/data/viet_syllables.rs
      → phf::Set cho TuDien (~7K entries) — O(1), ~15KB
  - [ADD]     core/src/infrastructure/data/viet_compound.bin  (build artifact)
      → Binary sorted data cho TuDienTuGhep (~68K phrases) — O(log n), ~250KB
  - [ADD]     core/src/infrastructure/data/viet_compound_data.rs
      → include_bytes! + binary_search_in_bytes wrapper (reuse pattern từ dictionary_data.rs)
  - [REFACTOR] core/src/infrastructure/adapters/validation/syllable_validator.rs
      → Thay FSM bằng PAD/NA/PAC group lookup tables
  - [SIMPLIFY] core/src/shared/buffer/ (Char, Buffer structs)
  - [ADD]    core/src/infrastructure/engine/ (InputMethodConfig type)
  - [ADD]    core/src/presentation/ffi/api.rs
      → New: ime_load_input_config_v2()
  - [ADD]    platforms/macos/goxviet/goxviet/FFI/RustBridgeSafe.swift
      → Swift binding cho ime_load_input_config_v2
  - [REFACTOR] platforms/macos/goxviet/goxviet/Managers/Input/InputManager.swift
      → Data-driven input method, truyền config xuống Rust core

🗄️ DB Changes: None

🔌 API Changes (FFI):
  - [NEW]  ime_load_input_config_v2(config_json: *const u8, len: usize)
  - [REMOVE] (internal) LanguageDecision APIs — không exposed qua FFI
  - Breaking change: NO — thay đổi internal logic, không thay đổi process_key API

⚠️ Risks:
  - phf build-time cho 68K compound words → có thể tăng build time ~10–20s
    → Mitigation: tách thành feature flag, benchmark build trước
  - Xoá English detection → có thể có regression với một số từ tiếng Anh gõ bằng Telex
    → Mitigation: viết regression tests trước khi remove
  - Char/Buffer struct change → breaking internal API, cần update toàn bộ consumers
    → Mitigation: dùng type alias hoặc migration path
```

---

## Success Criteria

- [ ] Không còn English dictionary binary trong codebase (chỉ giữ PhonotacticEngine)
- [ ] TuDien + TuDienTuGhep được embed qua phf, lookup < 1μs
- [ ] PAD/NA/PAC validator pass toàn bộ test cases từ GhepVan.ini
- [ ] FSM validator đã được xoá hoặc deprecated
- [ ] `InputManager.swift` sử dụng data-driven mapping, không hardcode
- [ ] Rust engine nhận `InputMethodConfig` qua FFI
- [ ] All existing tests pass (+ regression tests mới)
- [ ] Latency core vẫn < 3ms

---

## References
- `.planning/goTiengViet-analysis.md` — phân tích reference implementation
- `.uvasx/goTiengViet-resource/Resources/GhepVan.ini`
- `.uvasx/goTiengViet-resource/Resources/TuDien.txt`
- `.uvasx/goTiengViet-resource/Resources/TuDienTuGhep.txt`
- `.uvasx/goTiengViet-resource/Resources/KieuGo.ini`
- `.uvasx/gonhanh.org-main/core/src/engine/` — reference buffer/syllable implementation
