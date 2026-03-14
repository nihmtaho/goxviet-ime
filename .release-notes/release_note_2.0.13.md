# 📝 Release Notes - Phiên bản 2.0.13

**Ngày phát hành:** 2026-03-14
**Phiên bản:** 2.0.13
**PRs:**
- [#67 – fix(core): triple-tone Telex auto-correction with display sync and triphthong support](https://github.com/nihmtaho/goxviet-ime/pull/67)
- [#65 – fix(core): handle invalid Vietnamese combos with stroke restore & NA-PAC validation](https://github.com/nihmtaho/goxviet-ime/pull/65)
- [#64 – fix(macos): resolve Swift 6 concurrency warnings and compact settings UI](https://github.com/nihmtaho/goxviet-ime/pull/64)

---

## ✨ Tính năng mới

### Triple-tone Telex Auto-Correction (PR #67)

Giải quyết một vấn đề phổ biến khi gõ nhanh: nhấn nhầm ba lần cùng một nguyên âm rồi SPACE.

#### `aaa` + SPACE → `à` | `eee` + SPACE → `è` | `ooo` + SPACE → `ò`

- **Vấn đề:** Khi gõ nhanh, ngón tay dễ bấm lặp phím nguyên âm 3 lần (`aaa`, `eee`, `ooo`, `uuu`, `iii`) thay vì 1 lần. Trước đây engine không xử lý pattern này, ra ký tự sai hoặc chuỗi raw.
- **Giải pháp:** Phát hiện chuỗi triple-tone tại SPACE boundary — khi người dùng nhấn SPACE sau `aaa`/`eee`/`ooo`/`uuu`/`iii`, engine tự động sửa thành nguyên âm có dấu huyền tương ứng (`à`/`è`/`ò`/`ù`/`ì`).
- **Display sync:** Trong khi đang gõ (`aa`, `aaa`), suppression flag ngăn engine hiển thị trạng thái trung gian sai — chỉ áp dụng correction khi SPACE xác nhận.
- **Idempotency:** Nếu display đã hiển thị đúng rồi, SPACE không xóa và gõ lại (tránh double-apply).
- **Boundary safety:** Khi triple-tone flag active, toàn bộ boundary checks bị bỏ qua để tránh English false restore trên `aaa`, `eee`.
- **Không ảnh hưởng:** Gõ đúng `a`+SPACE, `aa`+SPACE (không phải triple) vẫn hoạt động bình thường.

---

## 🐛 Sửa lỗi

### Core Engine: Xử lý Stroke Invalid Combo và NA-PAC Validation

Sửa lỗi đơn giản nhưng quan trọng trong xử lý phụ âm với stroke (đặc biệt `đ`):

#### `ddd` → `ddd` (không phải `dd` toggle)

- **Vấn đề:** Khi gõ ba lần `d`, engine sẽ:
  - Lần 1: `d` → `d`
  - Lần 2: `dd` → `đ` (stroke applied)
  - Lần 3: `đd` → `dd` (toggle back sai, vì `đd` không hợp lệ tiếng Việt)
- **Nguyên nhân:** Đ là phụ âm bắt đầu âm tiết (initial consonant), **không bao giờ** có thể đứng trước một phụ âm khác. Trạng thái `đd` là tổ hợp không hợp lệ.
- **Giải pháp:** Thêm phương thức `restore_stroke_to_raw()` — khi phát hiện stroke không hợp lệ (buffer chỉ chứa stroked-d), lập tức khôi phục thành raw keystrokes: `đd` → `ddd`
- **Không ảnh hưởng:** Multi-syllable contexts như `xôđa`, `đumđum` vẫn hoạt động bình thường (buffer có ký tự khác sau `đ`)

#### NA-PAC Validity: Kiểm tra coda digraph hợp lệ

- **Vấn đề:** Engine không xác thực tính hợp lệ NA (nucleus-aperture / cụm nguyên âm) + PAC (phonetic coda / phụ âm cuối) khi hoàn thành digraph coda
- **Ví dụ:** `ơi` (NA.5, open-only — không cho phép phụ âm cuối) + `c` → không nên tạo `ơic`
- **Giải pháp:** Thêm hai hàm kiểm tra trước khi thêm phụ âm vào buffer:
  1. **`check_coda_extension_validity()`** — Khi mở rộng phụ âm cuối thành digraph (`ch`, `ng`, `nh`), kiểm tra xem NA hiện tại có cho phép PAC đó không
  2. **`check_first_coda_validity()`** — Khi thêm phụ âm đầu tiên vào buffer kết thúc bằng nguyên âm, kiểm tra xem NA có open-only không
- **Hành động:** Nếu tổ hợp NA-PAC không hợp lệ, lập tức khôi phục buffer thành raw input
- **Không ảnh hưởng:** Các tổ hợp NA-PAC hợp lệ vẫn hoạt động (ví dụ: `ương` = `ươ` NA.2 + `ng` PAC.1 ✓)

#### Digraph Coda Guard: Vietnamese Transforms Prevent English Restoration

- **Vấn đề:** Khi hoàn thành digraph coda (`ch`, `ng`, `nh`), nếu buffer có Vietnamese transforms (tone marks, diacritical marks), engine vẫn có thể sai nhầm là English và restore sai
- **Ví dụ:** `ích` (từ Việt có dấu) + 'c' → `ích` → 'h' hoàn thành `ch`, nhưng intermediate state trông phonotactically invalid
- **Giải pháp:** Thêm guard `just_completed_digraph` — nếu phím mới sẽ hoàn thành digraph hợp lệ và buffer đã có Vietnamese transforms, bỏ qua English restoration
- **Không ảnh hưởng:** English words thực sự (`rich` không có dấu) vẫn khôi phục bình thường

#### Dictionary Cleanup: Xóa Obsolete English Dictionary Files

- **Vấn đề:** Sau refactor sang phonotactic-only English detection (v2.0.11), dictionary files (`common_2chars.txt` đến `common_16chars.txt`) không còn được dùng
- **Giải pháp:** Xóa 15 text dictionary files (~93,666 words, ~85k lines), 15 binary `.bin` files, và source file `EnglishWords.txt`
- **Tác động:** Giảm repository size ~106k lines, tăng clarity (tránh confusion về engine internals)
- **Không ảnh hưởng:** English detection logic (phonotactic patterns) vẫn hoạt động 100%, không có thay đổi behavior

### Triphthong `ươu` và English Context Restore (PR #67)

#### Triphthong `ươu` — `ơ+u` whitelist

- **Vấn đề:** Engine từ chối tổ hợp `ươu` vì `ơ+u` không nằm trong danh sách vowel cluster cho phép — gây lỗi khi gõ từ như `rượu`, `mượu`.
- **Giải pháp:** Thêm `ơ+u` vào whitelist vowel cluster check — cho phép triphthong `ươu` hợp lệ trong tiếng Việt.
- **Không ảnh hưởng:** Các vowel cluster không hợp lệ khác vẫn bị từ chối đúng.

#### English context: Restore on invalid vowel cluster / non-digraph coda / dd-stroke

- **Vấn đề:** Trong một số ngữ cảnh English, engine giữ lại transform không hợp lệ thay vì khôi phục raw keystrokes:
  - Cụm nguyên âm không hợp lệ trong English sequence
  - Coda không phải digraph sau vowel
  - `dd` stroke khi người dùng đang gõ English (tạo `đ` sai)
- **Giải pháp:** Phát hiện ba pattern trên trong English detection path → khôi phục raw keystrokes thay vì output transform sai.
- **Không ảnh hưởng:** Vietnamese sequences với `đ` hợp lệ vẫn hoạt động bình thường.

### Telex/VNI: Xử lý tổ hợp phím không hợp lệ trong tiếng Việt

Năm lỗi liên quan đến cách engine áp dụng dấu thanh / dấu phụ và quyết định English detection khi cụm ký tự không tạo thành âm tiết tiếng Việt hợp lệ:

#### `yas` → `yas` (không phải `yá`)

- **Vấn đề:** Khi gõ `yas`, engine áp dụng dấu sắc lên `y`, tạo ra `yá` — nhưng `ya` không phải cụm nguyên âm hợp lệ trong tiếng Việt
- **Giải pháp:** Từ chối áp dụng dấu thanh khi cụm nguyên âm bắt đầu bằng `y` và theo sau là nguyên âm không phải `ê` (ví dụ: `ya`, `yo`, `yu`). Chỉ `y` đơn và `yê` là hợp lệ.
- **Không ảnh hưởng:** `ys` → `ý`, `hoas` → `hoá`, `ans` → `án` vẫn hoạt động bình thường

#### `hoacwj` → `hoặc` (không phải `hơacj`)

- **Vấn đề:** Trong buffer `hoac`, khi gõ `w` để thêm dấu móc, engine tìm `o` hoặc `u` cuối cùng — nhưng `o` trong `hoac` là âm đệm (glide) trước `a`, không phải nguyên âm chính. Kết quả engine áp dụng sai vào `o` → `hơac`
- **Nguyên nhân sâu xa:** `find_horn_positions` dùng `unwrap_or(0)` nhưng `keys::A == 0`, khiến vị trí `o` không có ký tự tiếp theo bị nhầm là có `a` đứng sau
- **Giải pháp:** Đổi sang `map_or(false, |&nk| nk == keys::A || nk == keys::E)` để kiểm tra chính xác; bỏ qua `o` khi nó là glide (theo sau bởi `a`/`e`), áp dụng dấu móc vào nguyên âm chính `a` → `hoặc`
- **Không ảnh hưởng:** `duocwj` → `dược` (o trong `duoc` không là glide vì không có `a`/`e` tiếp theo)

#### `hoawjch` → `hoawjch` (không phải `hoặch`)

- **Vấn đề:** Sau khi gõ `hoawj` → `hoặc`, nếu gõ thêm `h` sẽ tạo coda digraph `ch`. Nhưng `oă` (NA.3) chỉ hợp lệ với PAC.1 (`c`, `ng`) và PAC.2 (`m`, `n`, `p`, `t`) — **không hợp lệ** với PAC.0 (`ch`, `nh`). Engine không kiểm tra điều này và xuất ra `hoặch` sai.
- **Giải pháp:** Thêm kiểm tra tương thích NA-PAC trước khi mở rộng phụ âm cuối thành digraph. Nếu tổ hợp NA + coda đề xuất vi phạm bảng `NA_PAC_COMPAT`, engine lập tức khôi phục về chuỗi raw.
- **Không ảnh hưởng:** `hoach` → `hoach` (coda `ch` với `oa` NA.1 là hợp lệ)

#### `rích`, `huỵch`, `cõng` gõ đúng (không bị restore sai)

- **Vấn đề:** Khi gõ `rich`, `huyjch`, `coxng`, engine phân tích buffer ở trạng thái *trung gian* kết thúc bằng `c` hoặc `n` — trông phonotactically invalid nên bị nhận nhầm là English và restore về raw trước khi `h`/`g` hoàn thành digraph coda `ch`/`ng`/`nh`.
- **Nguyên nhân:** English detection chạy *trước* `handle_normal_letter`, không biết phím tiếp theo sẽ hoàn thành digraph hợp lệ.
- **Giải pháp:** Thêm hai guard trong `on_key`:
  1. **Early guard** (trước English dict lookup): nếu phím mới sẽ tạo digraph `ch`/`ng`/`nh` và buffer đã có Vietnamese transform → chuyển thẳng vào `handle_normal_letter`.
  2. **Late guard** (trước `instant_restore_english`): bỏ qua restore khi phím mới sẽ hoàn thành digraph coda.
- **Không ảnh hưởng:** English words thực sự (`rich` thuần Anh không có dấu) vẫn restore bình thường vì không có Vietnamese transform trong buffer.

#### English detection: Diacritical modifier guard mở rộng

- **Vấn đề:** Guard không-restore chỉ kiểm tra circumflex/horn/breve (diacritical marks từ `aa`/`aw`/etc.) nhưng bỏ sót tone marks (`s`/`f`/`r`/`x`/`j`) — có thể restore sai từ như `tốc` → raw khi người dùng đã chủ ý gõ dấu sắc.
- **Giải pháp:** Mở rộng điều kiện `has_diacritical_modifier` sang `c.mark != 0` (tone marks), không chỉ `c.tone != tone::NONE`.

### Swift 6 Concurrency Warnings (130 → 1)

- **Vấn đề:** Build với Swift 6 strict concurrency báo 130 warnings trên 17 files — chủ yếu do notification/timer/DispatchSource observer closures capture `self` không qua `MainActor`, và `nonisolated(unsafe)` được dùng sai
- **Giải pháp:**
  - Bọc tất cả `NotificationCenter`, `Timer`, `DispatchSource` observer closures trong `Task { @MainActor in }` hoặc `MainActor.assumeIsolated {}` trên: `AppDelegate`, `InputManager`, `PerAppModeManagerEnhanced`, `ResourceManager`, `InputSourceMonitor`, `UpdateManager`, `UpdateSimulator`
  - Xóa `nonisolated(unsafe)` thừa khỏi singleton `static let`
  - Đổi FFI constants (`telex`/`vni`/`traditional`/`modern`) sang `nonisolated static var` (computed property) để có thể truy cập từ `nonisolated init`
  - Thêm `@Sendable` vào `TypedNotifications` handler parameters; extract `Notification.Name` trước `queue.async` closures để tránh MainActor isolation issue
  - Mark `UpdateManager.runShell`, `findAppBundle`, `relaunchWithNewApp` là `nonisolated`; dispatch `InputManager.stop()` và `NSApp.terminate()` về main thread
  - Thêm `@discardableResult` cho `SettingsManager.addShortcut` / `updateShortcut`
  - Xóa `[weak self]` capture thừa trong `SettingsManager` `DispatchWorkItem`
- **Tác động:** 1 warning còn lại là region-isolation false positive trong `PerAppModeManagerEnhanced` workspace observer (`MainActor.assumeIsolated` là synchronous nhưng Swift 6 type system vẫn flag)

### Accessibility Retry cải thiện

- **Vấn đề:** macOS đôi khi thu hồi Accessibility permission chậm (không chỉ sau update) — sau khi re-grant, app cần vài giây để TCC nhận ra. Với 3 retry × 0.5s = 1.5s tổng, không đủ trên một số máy chậm.
- **Giải pháp:**
  - Tăng max retry: 3 → 8 lần
  - Tăng interval: 0.5s → 0.75s (tổng 6s budget)
  - Mở rộng điều kiện retry: ngoài `isPostUpdateLaunch`, cũng retry patiently khi `hadPermissionBefore` (`SettingsKey.permissionGranted`) — bao gồm manual revocation/re-grant và TCC reset không liên quan update
  - Tăng initial delay lên 1.5s khi `hadPermissionBefore`
  - Lưu `permissionGranted = true` vào UserDefaults khi quyền được cấp lần đầu

### UpdateSimulator Timer Race Condition

- **Vấn đề:** Data race khi `progressTimer` được invalidate trong async context
- **Giải pháp:** Kiểm tra `self != nil` ngoài `Task`; dùng `self.progressTimer` (captured strong reference) để invalidate đúng instance

### Deprecated `.onChange` API

- **Vấn đề:** `.onChange(of:perform:)` deprecated trong macOS 14+
- **Giải pháp:** Chuyển sang two-parameter form `{ oldValue, newValue in }` tại `AdvancedSettingsView` và `ShortcutEditorSheet`

---

## ✨ Tính năng mới / Cải thiện UI

### Compact Settings UI

Giảm mật độ visual trong Settings window để tận dụng không gian màn hình tốt hơn:

| Thành phần | Trước | Sau |
|---|---|---|
| Row vertical padding | 8pt | 5pt |
| Row horizontal padding | 12pt | 10pt |
| Icon size (SettingRow) | 24×24pt | 20×20pt |
| Section spacing | 20pt | 14pt |
| GroupBox content padding | 12pt | 6pt |
| Container padding | 24pt | 20pt |
| App icon (Per-App) | 32pt | 28pt |
| List row spacing | 8pt | 4pt |
| Toolbar padding | 24/12pt | 16/8pt |

- Thêm `.controlSize(.small)` cho tất cả Toggle/Switch trong General và Per-App tabs

### General Settings Reorder

Thứ tự section mới theo mức độ sử dụng thường xuyên:

1. **Keyboard Shortcut** (mới lên đầu)
2. Input Method
3. Tone Settings
4. Smart Features *(kết hợp Instant Auto-Restore vào đây)*
5. Editing *(đổi tên từ "Auto-Restore")*

---

## 📦 Chores

### SettingsKey Enum (SettingsKeys.swift)

Tập trung tất cả `UserDefaults` key strings vào file mới `SettingsKeys.swift` dưới `enum SettingsKey`:
- 25 keys được nhóm theo MARK: Core Input, IME Features, Per-App Mode, Keyboard Shortcuts, Update Manager, App Lifecycle, Services
- Namespace thống nhất `com.goxviet.ime.*`
- Xóa `Keys` inner struct trong `SettingsManager` và tất cả string literals rải rác trong `AppDelegate`, `UpdateManager`, `Log`, `TextInjectionHelper`, `KeyboardShortcut`, `RestoreShortcut`

### Test Data Cleanup

- Xóa 162 dòng symbol/garbage khỏi `english_words.txt` (dòng bắt đầu bằng `!`, `$`, `%`, `'`, ký tự đặc biệt)
- Sửa `vietnamese_69k_pure.txt`: xóa `lôgic`, `sâmbanh`; đổi `mô đéc`/`môbilet` → `mô đé`

### Artifact Cleanup

- Xóa binary artifacts (`.a`, `.bin` thừa), temp docs, và redundant test data khỏi repo

---

## 👥 Người đóng góp

- Thao Truong Minh

---

*Generated by Release Note Generator Skill*
