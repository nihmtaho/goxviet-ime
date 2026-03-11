# 📝 Release Notes - Phiên bản 2.0.13 [DRAFT]

**Ngày phát hành:** TBD
**Phiên bản:** 2.0.13
**PR:** [#64 – fix(macos): resolve Swift 6 concurrency warnings and compact settings UI](https://github.com/nihmtaho/goxviet-ime/pull/64)

---

## 🐛 Sửa lỗi

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

*Generated by Release Note Generator Skill — DRAFT, chưa publish*
