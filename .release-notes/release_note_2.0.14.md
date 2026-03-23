# 📝 Release Notes - Phiên bản 2.0.14

**Ngày phát hành:** 2026-03-23
**Phiên bản:** 2.0.14
**PRs:**
- [#78 – feat(core): core engine improvements — English prefix patterns, CTRL-commit, mic/rayc fixes](https://github.com/nihmtaho/goxviet-ime/pull/78)
- [#66 – ci: Add Claude Code GitHub Workflow](https://github.com/nihmtaho/goxviet-ime/pull/66)

---

## ✨ Tính năng mới

### CTRL-commit (PR #78)

Giải quyết tình huống phổ biến: đang gõ tiếng Việt rồi cần dùng phím tắt CTRL trong ứng dụng.

#### Cách hoạt động

Trước đây, khi nhấn CTRL trong lúc đang gõ, buffer bị xóa và CTRL+key có thể bị drop hoặc xử lý sai. Với CTRL-commit:

1. Người dùng nhấn **Control** (đơn thuần, không kèm Cmd/Option)
2. Engine nhận tín hiệu "one-shot CTRL pending"
3. Phím tiếp theo được xử lý với `ctrl=true`:
   - Buffer Vietnamese hiện tại **commit ngay lập tức** như-là
   - CTRL+phím được **truyền thẳng** đến ứng dụng như ký tự thô
4. Không có Vietnamese transform nào được áp dụng

**Ví dụ:** Đang gõ `xin chào` → nhấn Ctrl+A để select-all → `xin chào` được commit đúng, Ctrl+A hoạt động bình thường.

**Không ảnh hưởng:** Cmd/Opt shortcuts (Cmd+C, Opt+Space…) vẫn hoạt động như cũ.

---

### English Prefix Patterns: `mic-` và `rayc-` (PR #78)

Thêm hai prefix patterns mới vào bảng English detection:

#### `mic-` (microphone, microscope, microchip, mic stand…)

- Vấn đề trước: `micr`→`mỉc`, `micf`→`mìc`, `micx`→`mĩc` (sai)
- Chỉ `mics`→`míc` và `micj`→`mịc` là Vietnamese hợp lệ — engine vẫn giữ hai trường hợp này
- **Priority 1c fast-path:** `micr`/`micf`/`micx`/`micw` được intercept trước khi Vietnamese transform xảy ra → không bao giờ hiện intermediate `mỉc`/`mìc`/`mĩc`

#### `rayc-` (raycast, raycasting — computer graphics)

- Prefix `rayc` không có từ tiếng Việt nào bắt đầu bằng chuỗi này
- Detect ngay từ ký tự thứ 4 (`c`), restore toàn bộ về English

---

## 🐛 Sửa lỗi

### `core` → `core` (không bị giữ làm `coẻ`) (PR #78)

Sửa một edge case tinh tế trong Vietnamese detection:

- **Vấn đề:** Khi gõ `c-o-r-e`, chữ `r` (Telex hỏi) bị absorbed vào `o` → tạo `coẻ`
  - `oe`-compound structurally hợp lệ theo FSM → Priority 1b bỏ qua English detection
  - `coẻ` KHÔNG có trong TuDien → không phải từ tiếng Việt thật
- **Nguyên nhân sâu:** Mid-word tone absorption tạo ra intermediate state trông valid nhưng thực ra là English
- **Giải pháp:** Fallback mới phát hiện pattern này:
  - Tone modifier key bị consumed ở giữa từ (không phải ký tự cuối)
  - Output là structurally valid nhưng không có trong TuDien
  - → Restore về raw English keystrokes

### `uu` + `w` → `ưu` (horn lên u đầu) (PR #78)

Sửa lỗi thứ tự đặt dấu khi gõ cluster `u+u`:

- **Vấn đề:** Gõ `u-u-w` tạo `uư` thay vì `ưu`
- **Nguyên nhân:** `find_horn_positions` không có special case cho `u+u` cluster
- **Giải pháp:** Thêm special case: khi `k1 == U && k2 == U`, horn luôn đặt lên vị trí đầu tiên (`pos1`) — phù hợp với `HORN_PATTERNS[0]` quy định `HornPlacement::First` cho `U+U`

---

## 🔧 CI/Chores

### Claude Code GitHub Workflow (PR #66)

Thêm workflow CI chạy Claude Code agent tự động trong GitHub Actions.

### Cleanup old planning docs

Xóa tài liệu planning cũ không còn dùng và cập nhật issue templates cho phù hợp với workflow hiện tại.

---

## 📊 Tóm tắt

| Category | Count |
|---|---|
| ✨ Features | 2 |
| 🐛 Bug Fixes | 3 |
| 🔧 CI/Chores | 2 |

**Tổng số file thay đổi (PR #78):** 7 files, +578 / -6 lines
