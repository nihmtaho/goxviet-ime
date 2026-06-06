# 📝 Release Notes - Phiên bản 2.0.16

**Ngày phát hành:** 2026-06-06
**Phiên bản:** 2.0.16
**PRs:**
- [#86 – feat: integrate features into GoxViet engine and macOS platform](https://github.com/nihmtaho/goxviet-ime/pull/86)

---

## ✨ Tính năng mới

### Free Diacritic Toggle Mode (PR #86)

Nhấn **Ctrl** để tạm thời tắt transformation tiếng Việt — gõ ký tự nào sẽ ra ký tự đó mà không cần tắt hẳn IME. Nhấn Ctrl lần nữa để trở về chế độ Vietnamese bình thường.

**Ví dụ:** Đang gõ tiếng Việt → nhấn Ctrl → gõ `iOS`, `API`, `HTTP` nguyên bản → nhấn Ctrl → tiếp tục gõ tiếng Việt.

---

### Selection Injection Strategy (commit 222b651)

Chiến lược inject text mới sử dụng **Shift+Left** để select N ký tự ngược, sau đó dán qua **Cmd+V** — tránh hiện tượng jitter (nhảy cursor) trong các trường autocomplete và combo box của một số ứng dụng.

---

### English Dictionary Integration — vi.dic / keep.dic (PR #86)

Tích hợp từ điển tiếng Việt mở rộng (`vi.dic`) và danh sách từ tiếng Anh cần giữ nguyên (`keep.dic`). Engine nhận diện chính xác hơn khi nào nên giữ nguyên từ tiếng Anh thay vì chuyển sang tiếng Việt.

**Ví dụ:** `below`, `window`, `download`, `elbow` + SPACE → giữ nguyên tiếng Anh đúng cách.

---

### Shortcuts hoạt động khi IME tắt (PR #86)

Immediate shortcuts (ví dụ: `->` → `→`, `=>` → `⇒`) giờ vẫn hoạt động ngay cả khi IME đang ở trạng thái tắt.

---

### Cải tiến Per-App Mode (PR #86)

- Per-app row UI được nâng cấp với thêm thông tin và điều khiển rõ ràng hơn.
- Advanced tab được wire đầy đủ với các cài đặt engine mới.

---

## 🐛 Sửa lỗi

### Từ tiếng Anh kết thúc bằng 'w' + SPACE (commit 6afb1b9)

Sửa lỗi các từ như `below`, `window`, `elbow`, `narrow` bị chuyển thành tiếng Việt khi nhấn SPACE. Nguyên nhân: 'w' trong Telex vừa là horn modifier vừa là phụ âm cuối tiếng Anh — engine giờ phân biệt đúng.

### 'download' stroke bug (commit 2b8a1ee)

Sửa lỗi ký tự 'd' đầu trong `download` bị stroke khi tiếp tục gõ các ký tự sau. Cải thiện phonotactic detection cho các từ tiếng Anh bắt đầu bằng 'd' + vowel.

### Buffer bị giữ khi tắt IME trong Free Diacritic Mode (commit 5156329)

Khi tắt IME trong khi đang ở Free Diacritic Mode, buffer nay được clear hoàn toàn — không còn hiện tượng ký tự từ buffer cũ bị "rò rỉ" ra ngoài.

---

## ⚠️ Known Issues

- Không có vấn đề đã biết trong phiên bản này.
