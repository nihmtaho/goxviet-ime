# GoNhanh – Release Notes v1.0.100 → v1.0.143

> Nguồn: https://github.com/khaphanspace/gonhanh.org/releases
> Phạm vi: v1.0.100 (2026-01-05) → v1.0.143 (2026-03-27)
> Ghi chú: v1.0.135 không tồn tại trên repo.

---

## v1.0.143 — 2026-03-27

### ✨ New Features
- Thêm từ "fomo" vào từ điển tiếng Anh, hỗ trợ auto-restore khi gõ foreign consonant

### ⚡ Improvements
- Cập nhật danh sách contributors

---

## v1.0.142 — 2026-03-25

### ⚡ Improvements
- Tăng browser injection delays lên medium để tương thích Telegram Web

---

## v1.0.141 — 2026-03-24

### 🐛 Bug Fixes
- (macOS) Sửa lỗi `backgroundtaskmanagementd` polling mỗi 2 giây gây spam hệ thống, chuyển sang `didBecomeActiveNotification` (#351)

---

## v1.0.140 — 2026-03-22

### ⚡ Improvements
- Thêm "momo" vào từ điển tiếng Anh để tránh auto-restore khi gõ
- Cập nhật danh sách contributors

---

## v1.0.139 — 2026-03-20

### 🐛 Bug Fixes
- Sửa auto-restore: giữ nguyên buffer khi revert double vowel (ee) tạo ra từ tiếng Anh hợp lệ (ví dụ: "memee" → "meme" thay vì restore về "memee")

---

## v1.0.138 — 2026-03-19

### 🐛 Bug Fixes
- Sửa lỗi auto-restore khi gõ ss/ff: giữ buffer nếu kết quả revert là từ tiếng Anh hợp lệ (ví dụ: "buss" → "bus", "loss" → "los") (#337)

---

## v1.0.137 — 2026-03-19

### 🐛 Bug Fixes
- (macOS) Hoàn tác tính năng Space bypass gây lỗi gõ nhanh dd→đ không nhận (#346)

---

## v1.0.136 — 2026-03-19

### 🐛 Bug Fixes
- (macOS) Tự động tắt Gõ Nhanh khi dùng Unicode Hex Input để tránh xung đột nhập ký tự hex (#342)

---

## v1.0.134 — 2026-03-18

### ✨ New Features
- Hỗ trợ xuống dòng trong nội dung gõ tắt — shortcut replacement có thể chứa nhiều dòng, hiển thị ↵ trong bảng, import/export escape `\n` (#343)
- (macOS) Bypass Telex/VNI khi giữ phím modifier (Cmd/Ctrl/Option) — tránh biến đổi tiếng Việt ngoài ý muốn khi dùng shortcut hệ thống hay leader key trong Neovim (#307)

### ⚡ Improvements
- (macOS) Ô nhập nội dung gõ tắt chuyển sang TextEditor hỗ trợ nhiều dòng, hiển thị gợi ý `⌥↩ xuống dòng` (#343)
- (windows) TextSender gửi VK_RETURN cho ký tự newline trong shortcut replacement, xử lý đúng CRLF (#343)
- Giới hạn `Result.send()` count tại u8::MAX (255) tránh overflow

---

## v1.0.133 — 2026-03-18

### ✨ New Features
- (macOS) Hỗ trợ gõ tiếng Việt trong Foxit PDF Reader bằng phương pháp char-by-char injection

---

## v1.0.132 — 2026-03-15

### ⚡ Improvements
- (macOS) Sử dụng emptyCharPrefix cho tất cả browser contexts, không chỉ address bar — giúp gõ tiếng Việt ổn định hơn trên mọi trang web

### 🐛 Bug Fixes
- Xóa các từ tiếng Anh ngắn (buf, les, mas, pas, sas) khỏi dictionary để giảm auto-restore sai

---

## v1.0.131 — 2026-03-07

### 🐛 Bug Fixes
- Sửa lỗi chained restore nhiều từ: xóa liên tục bằng backspace giờ khôi phục đúng từ trước đó trong lịch sử (ví dụ: "dươc vẫn " → xóa hết → gõ "j" → "được")
- Sửa lỗi telex double restore khi buffer là từ tiếng Anh hợp lệ (ví dụ: "business", "lowpass" không còn bị restore sai)

---

## v1.0.130 — 2026-03-03

### 🐛 Bug Fixes
- Sửa lỗi buffer không được khôi phục sau khi nhấn phím ngắt (dấu chấm, chấm phẩy...) rồi backspace — ví dụ "ddu." + backspace + "f" giờ cho đúng "đù" thay vì "đuf" (#334)

---

## v1.0.129 — 2026-03-03

### ⚡ Improvements
- (macOS) Tách menu bar toggle thành SwiftUI view riêng, dễ bảo trì hơn

### 🐛 Bug Fixes
- Sửa lỗi mất dấu móc trên 'ư' khi gõ "dươ" + space + backspace rồi tiếp tục gõ (#332)

---

## v1.0.128 — 2026-03-02

### 🐛 Bug Fixes
- (macOS) Sửa lỗi Cmd+Q không thoát hẳn ứng dụng — app tự khởi động lại thay vì tắt (#331)

---

## v1.0.127 — 2026-03-02

### ✨ New Features
- (macOS) Hỗ trợ giao diện Liquid Glass cho menu bar — dùng Auto Layout thay fixed frame, header co giãn theo nội dung (#330)

### ⚡ Improvements
- Thêm hướng dẫn xử lý lỗi nuốt chữ khi dùng Claude Code trong IDE (VS Code, Cursor, Windsurf...) vào README

---

## v1.0.126 — 2026-03-02

### ⚡ Improvements
- (macOS) Nâng cấp CI runners lên macos-26, hỗ trợ liquid glass
- (macOS) Mặc định ký Developer ID cho pre-release builds

### 🐛 Bug Fixes
- Sửa lỗi gõ sai dấu đ sau khi revert dấu mũ trên từ bắt đầu bằng "d" (vd: "dataad" không còn bị thành "đata") (#325)

---

## v1.0.125 — 2026-03-01

### 🐛 Bug Fixes
- (macOS) Sửa lỗi app tự khởi động lại khi thoát bằng Cmd+Q hoặc menu Thoát

---

## v1.0.124 — 2026-02-28

### ✨ New Features
- (macOS) Thêm tab Cài đặt nâng cao với tuỳ chỉnh theo từng ứng dụng, debug log viewer, và cài đặt hiệu năng (#320)

### ⚡ Improvements
- (macOS) Hỗ trợ macOS 26 (Tahoe), bỏ hỗ trợ macOS 12 (Monterey) trong bug report template

---

## v1.0.123 — 2026-02-26

### 🐛 Bug Fixes
- Chặn dấu mũ (circumflex) trên tổ hợp nguyên âm không hợp lệ kiểu V1+tone+V2+V2 (vd: tafoo → tàoo thay vì tafô)
- Cải thiện auto-restore cho pattern tone + nguyên âm kép, hỗ trợ cụm phụ âm đa ký tự như ch, tr, ng, ngh (vd: chaofo → chàoo, mufaa → muàa)
- (macOS) Dùng Developer ID cert cho dev build để giữ quyền Accessibility giữa các lần build, tự động cấp lại quyền qua TCC database khi cert thay đổi

### ⚡ Improvements
- Tái cấu trúc logic partial-restore và thu thập raw char cho dễ đọc, dễ bảo trì

---

## v1.0.122 — 2026-02-21

### ✨ New Features
- Tuỳ chọn "Gõ W thành Ư" giờ chặn W→Ư ở mọi vị trí, không chỉ đầu từ; horn modifier vẫn hoạt động bình thường (ow → ơ, uw → ư) (#317)

### ⚡ Improvements
- (macOS) Cập nhật link tải DMG luôn trỏ đến bản mới nhất thay vì version cố định

---

## v1.0.121 — 2026-02-15

### ✨ New Features
- (macOS) Tự động cập nhật ở background với đồng bộ trạng thái trên menu tray
- (macOS) Thêm tab System settings với tùy chọn tự động cập nhật (#242)
- (macOS) Hỗ trợ gõ tiếng Việt trong game apps qua syncProxy injection (#264, #309)
- Hiển thị release notes dạng HTML khi kiểm tra cập nhật
- Hỗ trợ pre-release version trong updater (semver với pre-release precedence)

### ⚡ Improvements
- (macOS) Thay thế Sparkle bằng cơ chế cập nhật tự xây dựng, gọn nhẹ hơn
- (macOS) Chuyển sang SwiftUI native cho launch badges, bỏ embedded images
- Cải thiện quy trình build/install: tự động dọn process cũ và quản lý login item
- Tách riêng format và lint thành các make target độc lập, thêm Swift lint vào CI
- (macOS) Áp dụng swiftformat cho toàn bộ codebase Swift
- Bật auto-restore mặc định cho người dùng mới
- Pre-release version format cải tiến, cao hơn release hiện tại để hỗ trợ update flow

### 🐛 Bug Fixes
- (engine) Sửa lỗi stroke modifier (đ) không nhận sau khi backspace-after-space restore (#314)
- (macOS) Sửa lỗi Cmd shortcuts (Cmd+A, Cmd+V...) gây ra từ bị lặp, dùng clearBufferAll thay vì clear thường (#312)
- (macOS) Kiểm tra DMG tồn tại trước khi restart, ngăn chặn kiểm tra cập nhật đồng thời
- Restart để cập nhật thay vì hiện cửa sổ khi bản cập nhật sẵn sàng
- (macOS) Sửa Sparkle update detection cho pre-release và bypass cooldown 1 giờ khi user chủ động kiểm tra
- Sửa pre-release versioning và CI signing/notarization flow

---

## v1.0.120 — 2026-02-07

### ✨ New Features
- (macOS) Thêm cấu hình injection method cho ứng dụng Caudex

### ⚡ Improvements
- (macOS) Thay thế phương thức injection `selectAll` và `autocomplete` bằng `emptyCharPrefix` — dùng ký tự rỗng (U+202F) để phá autocomplete highlight trên thanh địa chỉ trình duyệt, đơn giản hóa code injection đáng kể
- (macOS) Sửa chuyển đổi character count sang UTF-16 offset trong axDirect injection (#304)

### 🐛 Bug Fixes
- (macOS) Sửa lỗi con trỏ nhảy về đầu dòng khi gõ tiếng Việt trong Arc browser (#306)
- Sửa lỗi raw_input mất đồng bộ sau auto-restore + backspace, gây sai ký tự khi gõ tiếp (#305)

---

## v1.0.119 — 2026-02-06

### ⚡ Improvements
- Gộp từ điển tiếng Việt và đơn giản hóa spellcheck API, giảm ~6500 dòng code thừa (#300)

### 🐛 Bug Fixes
- (macOS) Sửa lỗi shortcut khôi phục không hoạt động khi dùng kèm phím modifier (#301)
- (macOS) Xóa buffer engine khi xóa dòng bằng Command+Delete/Backspace (#299)
- Loại trừ pre-release tag khỏi version detection trong Makefile

---

## v1.0.118 — 2026-02-04

### 🐛 Bug Fixes
- (macOS) Xóa buffer engine khi nhấn Option+Backspace xóa cả từ, tránh trạng thái engine bị lệch (#295)
- (macOS) Dùng selection method cho thanh địa chỉ Firefox (giữ axDirect cho Zen), khắc phục lỗi xóa ký tự khi gõ giữa văn bản

---

## v1.0.117 — 2026-02-03

### ⚡ Improvements
- Cải thiện phát hiện tác giả PR và định dạng trong release notes

### 🐛 Bug Fixes
- (macOS) Sử dụng selection method cho AXWindow trên trình duyệt Firefox-based (#290)
- (engine) Sửa lỗi nguyên âm đôi bị trùng khi backspace + gõ lại trong chế độ auto-restore (#289)

---

## v1.0.116 — 2026-02-02

### ⚡ Improvements
- Cải tiến quy trình release: sử dụng tag message cho release notes, tự động resolve tác giả PR
- Cập nhật README: đơn giản hóa bảng so sánh, hiển thị contributors trực tiếp, cập nhật thông số kỹ thuật
- Cập nhật danh sách contributors và sponsors

### 🐛 Bug Fixes
- (macOS) Hỗ trợ ký tự đặc biệt từ phím Option trong gõ tắt (Issue #275)
- Sửa lỗi auto-restore: xử lý nguyên âm đôi trùng lặp bằng ưu tiên từ điển, cải thiện độ chính xác khi chuyển đổi Anh-Việt
- Sửa tương thích bash 3.x khi tra tác giả PR trong CI/CD

---

## v1.0.115 — 2026-01-31

### ⚡ Improvements
- Tự động tạo release notes với tag message và bổ sung mention vào contributors

### 🐛 Bug Fixes
- (macOS) Hiển thị đúng phím F1-F20 và các phím đặc biệt (Home, End, Page Up/Down, NumPad Enter...) trong cài đặt phím tắt (#284)
- Auto-restore "perrmission" trả về đúng "permission" — nhận diện prefix "per-" cho các từ tiếng Anh (#281)
- Auto-restore "hiss" trả về đúng "his" — xử lý ngoại lệ cho từ phổ biến hơn (#280)

---

## v1.0.114 — 2026-01-28

### ✨ New Features
- Tự động khôi phục từ tiếng Anh thông minh hơn với từ điển Hunspell tiếng Việt (#270)
- (macOS) Hỗ trợ TeXstudio với phương thức nhập charByChar

### ⚡ Improvements
- (macOS) Loại bỏ overhead logging khi tắt chế độ debug

### 🐛 Bug Fixes
- Sửa lỗi viết hoa tự động không reset khi paste hoặc click chuột (#279)
- Sửa lỗi tràn bộ nhớ trong hàm rebuild buffer (#277)
- Sửa lỗi dấu mũ bị hoàn tác sai khi gõ phím thanh điệu (hojpow → họjpow thay vì hợp) (#276)
- Sửa lỗi gõ ba chữ O liên tiếp với dấu trước phụ âm cuối không hoạt động (booofng → boofng thay vì boòng) (#269)
- Sửa lỗi đếm backspace khi hoàn tác w thành nguyên âm cho từ nước ngoài
- (macOS) Sửa lỗi thiếu role trong cache key khi phát hiện ứng dụng

---

## v1.0.113 — 2026-01-26

### ⚡ Improvements
- Tổ chức lại scripts theo thư mục: `build/`, `setup/`, `release/`, `test/`
- Bổ sung hỗ trợ các case như: Sơn Đoòng, goòng, boòng
- Thêm test tracking failures cho Vietnamese dictionary tests (22k từ)
- Cải thiện test coverage với typing permutation tests và circumflex cancel variants
- Giảm test failures từ 573 xuống 269 và loại bỏ dữ liệu test trùng lặp

### 🐛 Bug Fixes
- Sửa lỗi auto-restore khôi phục sai các từ tiếng Việt hợp lệ có âm đệm W (vd: `banwfg` → `bằng`)
- Vô hiệu hóa circumflex vowel + phụ âm cuối `k` để auto-restore hoạt động đúng

---

## v1.0.112 — 2026-01-23

### 🐛 Bug Fixes
- Sửa lỗi gõ dấu mũ cho từ có 3 nguyên âm như "xuất", "buồm", "muốn" (#260)
- (macOS) Tăng độ trễ cho code editor và terminal để gõ ổn định hơn (VSCode, Cursor, Warp, Ghostty, JetBrains...)

---

## v1.0.111 — 2026-01-22

### ✨ New Features
- (macOS) Thêm cài đặt loại trừ ứng dụng khỏi tính năng tự động viết hoa đầu câu

### ⚡ Improvements
- Tái cấu trúc Makefile với các section rõ ràng, loại bỏ trùng lặp trong release targets
- (macOS) Thêm performance logging (RAM, keystroke count) khi debug mode bật

### 🐛 Bug Fixes
- (Engine) Sửa lỗi phím 'z' không được nhận diện là modifier sau khi restore bằng Space+Delete
- (macOS) Sửa lỗi buffer không được xóa khi chọn văn bản bằng Shift+Home/End rồi nhấn Backspace
- (macOS) Sửa memory leak trong NotificationCenter observer cho restore shortcut

---

## v1.0.110 — 2026-01-19

### ✨ New Features
- Thêm tùy chọn cho phép phụ âm ngoại lai (z, w, j, f) làm phụ âm đầu hợp lệ
- (macOS) Thêm logging sự kiện phím và tăng focus debounce lên 150ms

### 🐛 Bug Fixes
- Sửa lỗi chữ Đ đứng riêng bị auto-restore thành DD khi nhấn phím ngắt
- Sửa lỗi dấu mũ bị áp dụng lại sai sau khi revert khi gõ phím dấu
- Sửa lỗi dấu móc bị áp dụng sai cho 'u' trong pattern "Qu-"

---

## v1.0.109 — 2026-01-18

### ⚡ Improvements
- (macOS) Tối ưu per-app mode: chỉ lưu trạng thái riêng từng app, không ghi đè global state
- (macOS) Giảm log verbose trong RustBridge — code sạch hơn, ít noise trong debug output

### 🐛 Bug Fixes
- (macOS) Sửa lỗi phát hiện Spotlight bằng AXObserver event-driven thay vì polling mỗi keystroke — cải thiện độ chính xác và hiệu năng (#241)

---

## v1.0.108 — 2026-01-17

### 🐛 Bug Fixes
- (macOS) Sửa lỗi mất ký tự khi gõ trong Safari Google Docs với nhiều phím backspace liên tiếp (#237)

---

## v1.0.107 — 2026-01-16

### 🐛 Bug Fixes
- (macOS) Tự động lưu trạng thái chế độ gõ cho từng ứng dụng mới (#234)
- (macOS) Phím tắt Shift restore không còn chặn gõ chữ hoa
- Sửa lỗi auto-restore với pattern xen kẽ v-m-v-m (herere → here, therere → there)

---

## v1.0.106 — 2026-01-15

### 🐛 Bug Fixes
- (macOS) Sửa lỗi chế độ per-app không khôi phục đúng trạng thái cho ứng dụng được bật thủ công (#228)

---

## v1.0.105 — 2026-01-15

### ✨ New Features
- (macOS) Cho phép tuỳ chỉnh phím tắt khôi phục dấu (thay vì chỉ dùng ESC)
- (macOS) App mới mở lần đầu sẽ kế thừa trạng thái E/V từ app trước đó

### 🐛 Bug Fixes
- Sửa lỗi gõ tiếp sau khi khôi phục từ tiếng Anh (pure ASCII) không chuyển đổi đúng
- Sửa lỗi dấu nhảy sai vị trí khi gõ nguyên âm mở rộng với âm đầu "gi" (vd: "giri" → "gỉi")
- Sửa lỗi phím circumflex không hoạt động sau khi revert và xoá (vd: "eee" → xoá → "phee" giờ ra "phê")
- (macOS) Sửa lỗi debug log không ghi được khi file tạo sau khi app khởi động

---

## v1.0.104 — 2026-01-14

### ✨ New Features
- (macOS) Sử dụng phương pháp chậm hơn nhưng ổn định cho vùng nhập liệu trong Firefox (#160, #192, #215)
- Hỗ trợ thêm nguyên âm mở rộng sau khi hoàn nguyên dấu mũ (ví dụ: `oow` → `ôư`) (#211, #213)

### ⚡ Improvements
- Tối ưu script contributors: hợp nhất logic fetch issues/comments với hỗ trợ pagination
- Cải thiện hiển thị CONTRIBUTORS.md: sắp xếp theo số lượng đóng góp, giảm perRow xuống 6
- Hợp nhất generator HTML avatar thành hàm thống nhất `userTableHtml`
- Loại bỏ trigger sponsorship event khỏi contributors workflow

### 🐛 Bug Fixes
- Shortcut hoạt động trở lại sau khi xóa toàn bộ và gõ lại (#212, #214)

---

## v1.0.103 — 2026-01-12

### ✨ New Features
- Tự động tạo trang Contributors hiển thị sponsors, code contributors, issue reporters

### 🐛 Bug Fixes
- Sửa lỗi auto-restore cho các từ tiếng Anh có nhiều modifier như "nurses", "horses", "verses"
- Sửa lỗi xoá dấu không hoạt động sau khi nhấn backspace (ví dụ: "sẻv" → backspace → "sẻ" → 'r' → "ser")
- ESC restore giờ sử dụng đúng raw input gốc cho các transform đã revert (ví dụ: "off" + ESC → "off", không còn bị mất ký tự)

---

## v1.0.102 — 2026-01-09

### 🐛 Bug Fixes
- Sửa lỗi auto-restore cho pattern `lwu → lưu` và `moef → moè` — nhận diện đúng nguyên âm đôi tiếng Việt "ưu" (lưu, mưu, cưu) và dấu huyền trên "oe" (moè, boè)

---

## v1.0.101 — 2026-01-09

### ⚡ Improvements
- Thêm từ điển tiếng Anh 17k+ từ để cải thiện độ chính xác auto-restore
- Tối ưu thuật toán nhận diện từ tiếng Việt/tiếng Anh
- Mở rộng bộ test lên 100k từ tiếng Anh và 22k từ tiếng Việt

### 🐛 Bug Fixes
- Cải thiện auto-restore với độ chính xác 100% tiếng Việt, 97.6% tiếng Anh — giảm đáng kể lỗi chuyển đổi sai
- Sửa lỗi định vị dấu thanh cho nguyên âm đôi hợp lệ (diphthongs)
- Ưu tiên giữ nguyên dạng gốc khi buffer là từ tiếng Anh hợp lệ
- Xử lý đúng pattern w+o+final trong auto-restore (ương, ươn, etc.)
- Yêu cầu dấu cách sau dấu câu mới tự động viết hoa
- Sửa lỗi revert shortcut dấu ngoặc trả về ký tự sai khi dùng Shift/CapsLock

---

## v1.0.100 — 2026-01-05

### 🐛 Bug Fixes
- (macOS) Sửa lỗi chọn text bằng chuột trong thanh địa chỉ Firefox bị lỗi
- (macOS) Sửa lỗi tính toán vị trí con trỏ với ký tự Unicode (dùng UTF-16 offsets)
- Cho phép gõ dấu mũ linh hoạt với các nguyên âm đôi: `dauas` → `dấu`, `neues` → `nếu`, `toios` → `tối` (Issue #183)
