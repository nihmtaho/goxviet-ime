# 📝 Release Notes - Phiên bản 2.0.11

**Ngày phát hành:** 2026-03-01
**Phiên bản:** 2.0.11

---

## ✨ Tính năng mới

### Vietnamese Dictionary Embedding (Sprint A)
- **Mô tả:** Nhúng toàn bộ từ điển tiếng Việt vào binary tại build time bằng `phf` (perfect hash function). TuDien (~7K âm tiết đơn) → `phf::Set` O(1). TuDienTuGhep (~68K từ ghép) → sorted binary O(log n). Không còn file I/O ở runtime.
- **API mới:** `is_valid_vietnamese_syllable(s)` và `is_vietnamese_compound(s)` expose qua `core/src/data/mod.rs`
- **Commit:** `87e6ee1`

### SyllableStructureValidator — Thay thế FSM (Sprint B)
- **Mô tả:** PAD (Phụ Âm Đầu), NA (Nguyên Âm), PAC (Phụ Âm Cuối) lookup tables từ `GhepVan.ini` được compile thành `const` arrays. `SyllableStructureValidator` implement port `SyllableValidator` và được wire vào DI container, xóa hoàn toàn `FsmValidatorAdapter` khỏi production path.
- **Tests:** 29 unit tests + 27 integration tests trong `syllable_pad_na_pac.rs`
- **Commit:** `100ef59`

### Vietnamese-first Auto-restore Pipeline (Sprint C)
- **Mô tả:** Rewrite `language_decision.rs` với thứ tự ưu tiên: (1) TuDien lookup — nếu là âm tiết hợp lệ → giữ tiếng Việt, (2) phonotactics — nếu có chỉ dấu tiếng Anh → restore, (3) diacritics — nếu có dấu tiếng Việt → giữ. Xóa `dictionary.rs` + `dictionary_data.rs` (English dictionary cũ).
- **Behavioral change:** `"cost"` → `"cốt"` được giữ làm tiếng Việt (intentional Vietnamese-first)
- **Commit:** `8f1d731`

### Data-driven InputMethodConfig + FFI (Sprint D)
- **Mô tả:** `InputMethodConfig` domain type trong `domain/entities/input_method_config.rs` với `InputAction` enum (14 variants: Append, AppendTone, AppendHorn, v.v.), JSON serialization qua `serde`. Built-in factories `telex()` và `vni()`.
- **FFI:** `ime_load_input_config_v2(engine_ptr, json_bytes, len) → FfiStatusCode` — thay đổi input method config ở runtime mà không cần restart engine. `ErrorParseError = -12` cho JSON parse failures.
- **Swift:** `RustBridgeV2.loadInputConfig(_:)` + `InputMethodDefinition.swift` (pre-built JSON configs) + `InputManager` load config tại init và mỗi lần `setInputMethod` thay đổi.
- **Tests:** 15 integration tests (`sprint_d_integration_test.rs`), benchmarks (`sprint_d_bench.rs`)
- **Commit:** `63ec5b3`

### Phonotactic Pattern Expansion
- **Mô tả:** Thêm các pattern để nhận diện từ tiếng Anh phổ biến: `-core`/`core-` prefix/suffix (hardcore, multicore), `-yc` (psych, sync), `-ycast` (dynasty, gymnast), `SH` L2 onset cluster, `-ly` suffix, `-al` suffix.
- **Commits:** `37101db`, `e53fb54`, `d604a32`

---

## 🐛 Sửa lỗi

### Accessibility Permission Reset sau Auto-update
- **Vấn đề:** Sau khi auto-update, macOS thu hồi accessibility permission, người dùng phải cấp lại mỗi lần cập nhật.
- **Nguyên nhân gốc (3 root causes):**
  1. `goxviet.entitlements` dùng `<dict/>` self-closing → app build với empty entitlements
  2. `UpdateManager` dùng `rsync -a --delete` thay file in-place → phá code signature → TCC thu hồi
  3. Không có TCC-settle delay → alert hiện ngay khi permission chưa kịp recover
- **Giải pháp:** Entitlements fix (`<dict>…</dict>`), atomic bundle swap (`rm -rf + mv`), `--post-update` launch arg + 0.8s delay + 3× retry `AXIsProcessTrusted`
- **Commit:** `6fb7288`

### EXC_BAD_ACCESS khi Toggle Tiếng Việt
- **Vấn đề:** Crash `EXC_BAD_ACCESS (code=2)` mỗi khi toggle Vietnamese input.
- **Nguyên nhân:** `@MainActor` annotation trên `DispatchWorkItem` closure khiến Swift 6 runtime ghi vào read-only memory page khi tạo closure thunk. Redundant vì work item đã dispatch trên `DispatchQueue.main`.
- **Commit:** `13590e4`

### False English Restore cho Telex Tone Keys
- **Vấn đề:** Khi gõ `casc` (cas+c), engine thấy SC cluster trong raw input → confidence 98 → restore sai thành English.
- **Nguyên nhân:** Tone modifier keys (s=sắc, f=huyền, r=hỏi, x=ngã, j=nặng) bị absorbed vào diacritics nhưng vẫn nằm trong `raw_input`, khiến phonotactic engine thấy chúng là consonants.
- **Fix:** Filter `TELEX_TONE_MODIFIERS {R,S,F,X,J}` ở positions > 0 trước phonotactic analysis khi `raw.len() > buf.len()`. Áp dụng cả `check_and_restore_english_at_boundary()` và `is_english_dictionary_word()`.
- **Commits:** `364a286`, `f6a2574`

### Vietnamese 100% Pass Rate (Telex + VNI)
- **Telex fixes:** Y→P bigram cho `khuýp`/`tuýp`; qu-initial vowel sequence cho `quáu`/`quều`; phonotactic bypass dùng full raw keys khi VN output invalid (`yard`, `parabol`); relax K+U rule cho loan words
- **VNI fixes:** `can_apply_diacritical` — consonant chỉ là "final" khi trực tiếp sau vowel; skip validation trong `try_tone` cho Horn/Breve types (cross-syllable signals)
- **Kết quả:** Telex 100.00% (6,538/6,538), VNI 100.00% (6,538/6,538)
- **Commits:** `1390143`, `934a15e`

### English Restore Edge Cases
- **`gièm/giệp`:** gi-onset guard trong `normalize_ie_compound` ngăn circumflex trên 'i' sau 'gi'
- **`vên`:** Reorder priority trong `is_english_dictionary_word` — Vietnamese syllable check TRƯỚC dict lookup
- **`core`:** Thêm 'core' vào `EnglishWords.txt`; `-store` raw-suffix trong `check_and_restore_english` cho instant mid-word restore
- **`safari`/`raycast`:** Instant restore qua heavy-absorption bypass (raw_len - buf_len ≥ 2); fix 78 Telex + 5 VNI regressions từ dict-before-guard approach trước đó
- **Commits:** `92bd2f4`, `87a60c8`, `ae02eb3`

### Swift 6 + macOS 11 Migration
- **Mô tả:** `SWIFT_DEFAULT_ACTOR_ISOLATION = MainActor` — migrate singletons sang `nonisolated(unsafe) static let` + `nonisolated private init()`. Fix `EXC_BREAKPOINT` trong `UpdateChecker`. API compat: `Color(NSColor.)`, `.onChange(of:perform:)`, `Alert` struct, `@preconcurrency import Foundation`.
- **Commit:** `e51625f`

---

## ⚡ Cải thiện

### English Detection Accuracy +4.78%
- **Trước:** 59.23% Telex (trên 100K English word test set)
- **Bây giờ:** 64.01% Telex (+475 words detected correctly)
- **Kỹ thuật:** SH onset cluster (không có trong Vietnamese), `-ly`/`-al` suffix (l không phải final consonant tiếng Việt), xóa 3 dead SUFFIXES_3 rules (require SPACE key, never fired)
- **Commit:** `d604a32`

### Vietnamese Baseline Improvement
- **Trước:** 99.46% (35 failures)
- **Bây giờ:** 99.59% sau tone modifier filter
- **Final:** 100.00% sau full syllable validation fixes

---

## 📦 Thay đổi khác

- Thêm `serde = { version = "1", features = ["derive"] }` và `serde_json = "1"` vào `core/Cargo.toml`
- Rename test data: `vietnamese_69k.txt` → `vietnamese_dictionary.txt`; tách `english_100k_failures.txt` → `english_100k_failures_words.txt`
- Thêm `trace_test.rs` (syllable validator tracing) và `failures_tone_early_telex.txt`

---

## 👥 Người đóng góp

Cảm ơn các đóng góp từ:
- @nihmtaho
- Claude Sonnet 4.6
- GitHub Copilot

---

*Generated by Release Note Generator Skill*
