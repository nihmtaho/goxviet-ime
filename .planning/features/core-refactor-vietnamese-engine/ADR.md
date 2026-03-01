# ADR: Vietnamese-First Engine + KieuGo Pattern

**ADR Number:** 004  
**Date:** 2026-02-27  
**Status:** Accepted  
**Deciders:** @nihmtaho  
**Milestone:** v2.0.11

---

## Context

GoxViet IME dùng **English-based language detection** (dictionary + phonotactic) để quyết định khi nào restore về raw keystrokes. Cơ chế này:
- Phức tạp, nhiều false-positive
- Không tận dụng nguồn dữ liệu tiếng Việt phong phú (TuDien.txt ~7K, TuDienTuGhep.txt ~68K)
- FSM validator khó maintain, không align với Vietnamese phonology model
- InputMethod trait hardcoded, không extensible

---

## Decision

### ADR-004-A: Vietnamese-First + Phonotactic Hybrid Restore Decision

**Quyết định:** Giữ PhonotacticEngine, xoá English dictionary binary, thêm Vietnamese dict checks làm Priority đầu tiên.

**Revised Pipeline:** 
```
Priority 1: TuDien (phf) exact match       → Vietnamese → commit
Priority 2: TuDienTuGhep (.bin) compound   → Vietnamese → commit
Priority 3: PAD/NA/PAC structural check    → valid Viet structure → continue
Priority 4: PhonotacticEngine (KEEP)        → English pattern → restore
Priority 5: Default                        → restore (safe)
```

**Alternatives considered:**

| Option | Pros | Cons | Decision |
|--------|------|------|----------|
| Giữ English dict như cũ | Đã có code | False-positive với từ Việt trùng pattern | ❌ Rejected |
| Xoá toàn bộ English detection | Đơn giản nhất | "restore", "windows" → bị gõ thành "rétore" | ❌ Rejected |
| Vietnamese-first + Phonotactic (our choice) | Chính xác cả hai chiều | Cần thêm Viet dicts | ✅ Accepted |

---

### ADR-004-B: FSM → PAD/NA/PAC Replacement

**Quyết định:** Xoá FSM validator, thay bằng group-based PAD/NA/PAC lookup tables từ GhepVan.ini.

**Rationale:**
- PAD/NA/PAC model là chuẩn ngôn ngữ học tiếng Việt, được validate bởi nhiều reference implementations
- O(1) lookup với const arrays, không state machine overhead
- Dễ đọc, dễ test, dễ extend (thêm nhóm mới chỉ cần thêm entry)
- gonhanh's `syllable.rs` + `validation.rs` đã prove approach này hoạt động

**Breaking change:** NO — internal validator thay thế, FFI API không thay đổi

---

### ADR-004-C: Char/Buffer Struct Simplification

**Quyết định:** Align `Char` struct với gonhanh style:
```rust
// Before: scattered fields qua nhiều types
// After: flat struct, giống gonhanh
pub struct Char {
    pub key: u16,
    pub caps: bool,
    pub tone: u8,    // vowel diacritics
    pub mark: u8,    // tone marks (sắc/huyền/hỏi/ngã/nặng)
    pub stroke: bool // d→đ
}
```

**Rationale:** Đơn giản hóa mà không phá vỡ SOLID layers — struct này vẫn nằm trong `domain` hoặc `shared/buffer`, tầng trên không biết implementation.

---

### ADR-004-D: KieuGo.ini Pattern — Config in Rust Core via FFI

**Quyết định:** Rust core define và own `InputMethodConfig`. Swift truyền config xuống qua FFI khi init.

```c
// Swift tạo config theo định nghĩa KieuGo.ini, truyền xuống Rust
void ime_load_input_config_v2(const uint8_t* config_json, size_t len);
```

**Rationale:**
- Config logic ở Rust → single source of truth, dễ test unit
- Swift chỉ là data container (không implement transformation)
- Future: support VIQR, Microsoft layout, custom user layouts
- JSON format dễ parse, extensible, human-readable

**Alternatives:**

| Option | Decision |
|--------|----------|
| Config ở Swift tầng, không pass Rust | ❌ Duplicates logic |
| Hardcode cả hai nơi | ❌ Sync nightmare |
| Config file trên disk | ❌ Runtime overhead |
| Config trong Rust, pass via FFI (our choice) | ✅ Accepted |

---

### ADR-004-E: Hybrid Dictionary Embedding (phf + .bin)

**Quyết định:** Hai format khác nhau tối ưu cho từng use case:
- **TuDien (~7K syllables)**: `phf::Set<&'static str>` — O(1), ~15KB embedded
- **TuDienTuGhep (~68K phrases)**: `.bin` binary format (sorted + include_bytes! + binary search) — ~250KB, O(log n)

**Rationale:**
- TuDien: ít entries, `phf` compile nhanh, O(1) là best choice
- TuDienTuGhep: 68K entries — `phf` sẽ tăng compile time > 60s và binary size lớn (~3MB). Binary search approach reuse infrastructure sẵn có từ English dict, không cần dependency mới
- Cả hai đều `include_bytes!` / build-time — zero runtime IO, zero startup cost

**Size comparison:**

| File | Raw .txt | Embedded format | Savings |
|------|----------|-----------------|---------|
| TuDien.txt | 42KB | ~15KB (phf) | ~64% |
| TuDienTuGhep.txt | 1.0MB | ~250KB (.bin) | ~75% |

**Breaking change:** NO — chỉ thêm dữ liệu mới, không thay đổi API

---

## Consequences

**Positive:**
- Codebase đơn giản hơn đáng kể (xoá ~500 LOC English detection)
- Auto-restore chính xác hơn, ít false-positive
- Vietnamese phonology model rõ ràng, documented trong code
- InputMethod extensible mà không cần rebuild Swift UI

**Negative:**
- Build time tăng nhẹ (phf generation)
- Migration effort cho Char/Buffer struct consumers

**Neutral:**
- Mất English dictionary binary data (không còn cần thiết)
- FSM code bị xoá (đã được replace hoàn toàn)
