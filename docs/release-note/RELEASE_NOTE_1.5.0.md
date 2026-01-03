# GoxViet v1.5.0 – Release Note

**Ngày phát hành:** 2025-12-31  
**Phiên bản:** 1.5.0

---

## 🚩 Tổng quan

Phiên bản 1.5.0 tập trung vào nâng cao trải nghiệm người dùng và độ ổn định của bộ gõ trên macOS. Các cải tiến chính:
- **Trình quản lý cập nhật tự động**: GoxViet sẽ tự động kiểm tra phiên bản mới và cung cấp tùy chọn cập nhật qua DMG không cần chạy Homebrew thủ công.
- **Hỗ trợ đa ngôn ngữ**: Tự động vô hiệu hóa bộ gõ tiếng Việt khi chuyển sang bàn phím không phải Latin (Nhật, Hàn, Trung, v.v.), tự động khôi phục khi quay lại Latin.
- **Quyền Accessibility tự động**: Đảm bảo quyền được giữ lại sau khi cài đặt lại hoặc rebuild, tự động phát hiện cấp quyền mà không cần click lặp lại.

---

## ✨ Tính năng mới

### 1. Tự động tắt bộ gõ tiếng Việt khi chuyển sang bàn phím không Latin

- **Mô tả:** Khi người dùng chuyển sang bàn phím nhập liệu không phải Latin (Nhật, Hàn, Trung, Thái, Ả Rập, v.v.), Gõ Việt sẽ tự động tạm thời vô hiệu hóa chế độ gõ tiếng Việt để tránh chuyển đổi ký tự ngoài ý muốn. Khi quay lại bàn phím Latin, chế độ gõ tiếng Việt sẽ tự động được khôi phục.
- **Cách sử dụng:** Tính năng này được bật mặc định. Có thể bật/tắt trong phần Settings → Per-App → Multi-Language Support.
- **Ảnh hưởng:** Trải nghiệm gõ đa ngôn ngữ mượt mà, không bị lỗi chuyển đổi khi dùng nhiều layout bàn phím.

### 2. Giao diện xin quyền Accessibility mới, tự động phát hiện

- **Mô tả:** Quy trình xin quyền Accessibility được đơn giản hóa, tự động phát hiện khi người dùng cấp quyền mà không cần nhấn "Check Again". Hướng dẫn rõ ràng, trạng thái chờ trực quan.
- **Cách sử dụng:** Khi chạy lần đầu hoặc chưa có quyền, popup sẽ hướng dẫn từng bước. Khi quyền được cấp, app tự động nhận diện và kích hoạt bộ gõ.
- **Ảnh hưởng:** Giảm thao tác thủ công, không còn phải cấp lại quyền sau mỗi lần cài lại/rebuild (nếu code signing không đổi).

### 3. Trình quản lý cập nhật tự động (DMG Auto-Installer)

- **Mô tả:** GoxViet tự kiểm tra phiên bản mới theo chu kỳ (mặc định 6 giờ) và thông báo ngay khi có bản cập nhật. Người dùng có thể tải và cài đặt tự động qua download DMG từ GitHub, hoặc mở trang release để tải thủ công.
- **Cách sử dụng:** Menu bar → "Check for Updates" hoặc Settings → About → Updates để kiểm tra thủ công. Khi có bản mới, nhấn "Download & Install" để GoxViet tự động tải DMG, mount, copy .app vào /Applications, và khởi động lại app.
- **Ảnh hưởng:** Đảm bảo người dùng luôn ở phiên bản mới nhất với quy trình cập nhật tối giản - chỉ cần một lần click.

---

## 🐞 Sửa lỗi

### 1. Fix: Mất quyền Accessibility sau khi rebuild/cài lại

- **Mô tả lỗi:** Mỗi lần build lại app ở chế độ Debug, người dùng phải cấp lại quyền Accessibility.
- **Nguyên nhân:** Debug build trước đây không ký code (unsigned), khiến macOS coi là app mới mỗi lần build.
- **Giải pháp:** Debug build giờ đây sử dụng code signing giống Release (`Apple Development`). Thêm entitlement cho phép load thư viện Rust chưa ký hoặc ký khác Team ID khi phát triển.
- **Kết quả:** Quyền Accessibility được giữ lại giữa các lần build/cài lại.

### 2. Fix: Crash SIGABRT khi chạy từ Xcode (EXC_CRASH)

- **Mô tả lỗi:** App bị crash với SIGABRT khi chạy từ Xcode do lỗi code signature mismatch giữa app và thư viện Rust.
- **Nguyên nhân:** Thư viện Rust (`libgoxviet_core.dylib`) chưa được ký hoặc ký khác Team ID so với app.
- **Giải pháp:** Thêm script tự động ký thư viện Rust, bổ sung entitlement `disable-library-validation` cho Debug, đảm bảo đồng bộ code signing.
- **Kết quả:** App chạy ổn định, không còn crash khi phát triển.

### 3. Fix: Gõ tiếng Việt sai trong thanh địa chỉ Chromium (Issue #26)

- **Mô tả lỗi:** Khi gõ tiếng Việt trong thanh địa chỉ của các trình duyệt Chromium-based (Chrome, Edge, Brave, Vivaldi, Opera, Arc...), kết quả hiển thị sai:
  - `[h,a,f,n,h]` → "haành" thay vì "hành"
  - `[d,d]` → "dđ" thay vì "đ"
  - `[a,a]` → "aâ" thay vì "â"
  - `[u,w]` → "uư" thay vì "ư"
  - `[v,e,e,f]` → "veề" thay vì "về"
- **Nguyên nhân:** Thanh địa chỉ Chromium có tính năng autocomplete tích cực, khiến backspace không hoạt động đúng - text cũ được giữ lại và text mới bị thêm vào.
- **Giải pháp:** Thêm phương thức injection mới `axDirect` sử dụng Accessibility API để trực tiếp thay đổi giá trị text field, bỏ qua hoàn toàn hành vi autocomplete của Chromium.
- **Kết quả:** Gõ tiếng Việt trong thanh địa chỉ của tất cả trình duyệt Chromium-based hoạt động chính xác như trong các ứng dụng khác.
- **Ảnh hưởng:** Hỗ trợ Chrome, Edge, Brave, Vivaldi, Opera, Arc, DuckDuckGo, SigmaOS, và các trình duyệt Firefox-based (Firefox, Waterfox, LibreWolf, Zen Browser, Tor Browser...).

---

## 🔧 Cải tiến

- Tối ưu hóa luồng kiểm tra quyền Accessibility: auto-polling, không cần thao tác thủ công.
- Đảm bảo thread safety và tránh race condition khi hiển thị alert xin quyền.
- Loại bỏ API cũ/deprecated (NSUserNotification), chuyển sang giải pháp an toàn, hiện đại.
- Bổ sung script `sign-rust-core.sh` để hỗ trợ ký thư viện Rust tự động.
- Cập nhật hướng dẫn, tài liệu kỹ thuật liên quan đến đa ngôn ngữ và quyền hệ thống.

---

### 3. English Auto-Restore & English Detection Logic

#### **Tự động phục hồi từ tiếng Anh khi nhấn SPACE (Auto-Restore English)**
- **Mô tả:** Khi người dùng gõ từ tiếng Anh (không dấu tiếng Việt) và nhấn SPACE, bộ gõ sẽ tự động phục hồi lại từ gốc tiếng Anh, không còn bị biến đổi thành âm tiết tiếng Việt sai.
- **Ví dụ:**
  - Gõ `with` + SPACE → **Trước:** "ưith " → **Sau:** "with "
  - Gõ `terms` + SPACE → **Trước:** "tém " → **Sau:** "terms "
  - Gõ `result` + SPACE → **Trước:** "reúlt " → **Sau:** "result "
  - Gõ `work` + SPACE → **Trước:** "ưởk " → **Sau:** "work "
- **Lưu ý:** Nếu từ có dấu tiếng Việt (ví dụ: "kêp", "dêp"), bộ gõ sẽ giữ nguyên, không auto-restore.

#### **Nâng cấp nhận diện tiếng Anh (English Detection)**
- **Mô tả:** Bộ gõ nhận diện tốt hơn các từ tiếng Anh phổ biến, không còn tự động chuyển thành âm tiết tiếng Việt sai.
- **Ví dụ:**
  - Gõ `view` → **Trước:** "vieư" → **Sau:** "view"
  - Gõ `add` → **Trước:** "ađd" → **Sau:** "add"
  - Gõ `browser` → **Trước:** "brởe" → **Sau:** "browser"
- **Ảnh hưởng:** Trải nghiệm gõ song ngữ Anh-Việt mượt mà, không còn lỗi chuyển đổi ngoài ý muốn với từ tiếng Anh thông dụng.

#### **Chi tiết kỹ thuật:**
- Cải tiến thuật toán nhận diện tiếng Anh đa lớp (multi-layer), bổ sung các pattern mới cho các từ phổ biến và các trường hợp đặc biệt.
- Tối ưu logic auto-restore: chỉ phục hồi về tiếng Anh khi không có dấu tiếng Việt, đảm bảo không ảnh hưởng đến trải nghiệm gõ tiếng Việt.
- Bổ sung nhiều test case cho các từ tiếng Anh phổ biến, đảm bảo không còn lỗi chuyển đổi sai.

---

## ⚠️ Breaking Changes (nếu có)

- Không có breaking changes trong phiên bản này.

---

## ✅ Ảnh hưởng & kiểm thử

- **Hiệu suất:** Độ trễ < 16ms (đạt chuẩn 60fps)
- **Bộ nhớ:** Không memory leak
- **Tương thích:** macOS 12.0+

---

## 📋 Tổng kết thay đổi

| Loại          | Số lượng |
|---------------|----------|
| Tính năng mới | 2        |
| Sửa lỗi       | 3        |
| Cải tiến      | 5        |

---

## 📥 Cài đặt

### Tải DMG trực tiếp

1. Tải file `GoxViet-1.5.0-unsigned.dmg` từ phần Assets bên dưới
2. Mở DMG và kéo GoxViet vào thư mục Applications
3. Cấp quyền Accessibility khi được yêu cầu
4. Ứng dụng sẽ tự động kiểm tra cập nhật mới trong Settings → Updates

---

## 🔗 Tham khảo

- [Hướng dẫn sử dụng](../getting-started/QUICK_START.md)
- [Báo cáo lỗi](https://github.com/nihmtaho/goxviet/issues)
- [Lịch sử phát hành](./)

---

**Gõ Việt (GoxViet) – Bộ gõ tiếng Việt hiệu suất cao!**