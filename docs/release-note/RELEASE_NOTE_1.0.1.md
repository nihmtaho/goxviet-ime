# RELEASE NOTE v1.0.1
**Ngày phát hành:** 2025-12-21  
**Phiên bản:** 1.0.1

---

## 🎯 TÍNH NĂNG NỔI BẬT

### 1. Smart Per-App Mode (Ghi nhớ chế độ gõ theo từng ứng dụng)
- Tự động nhớ trạng thái bật/tắt bộ gõ tiếng Việt cho từng ứng dụng.
- Không cần chuyển thủ công khi chuyển app – hệ thống tự động bật/tắt đúng trạng thái đã dùng lần trước.
- Lưu trữ thông minh: chỉ lưu các app bị tắt, mặc định luôn bật cho app mới.
- Cài đặt và trạng thái được lưu vĩnh viễn (UserDefaults).

### 2. UI & Trải nghiệm người dùng
- Thêm nút bật/tắt Smart Per-App Mode ngay trên menu bar.
- Giao diện Settings hiển thị trạng thái từng app, số lượng app đã lưu, và cho phép xóa toàn bộ cấu hình per-app chỉ với 1 click.
- Icon menu bar cập nhật trạng thái tức thì (🇻🇳/EN).

### 3. Cải tiến & Sửa lỗi
- Refactor toàn bộ state sang AppState (single source of truth) – loại bỏ lỗi đồng bộ trạng thái giữa các thành phần.
- Sửa lỗi tên hàm Rust FFI (`ime_set_enabled` → `ime_enabled`, v.v.).
- Loại bỏ code cũ, duplicate, và warning biên dịch.
- Tối ưu hiệu suất lookup trạng thái app: O(1), không ảnh hưởng tốc độ gõ.

---

## 🧪 KIỂM THỬ & ỔN ĐỊNH

- Đã kiểm thử thủ công trên nhiều ứng dụng: Chrome, Notes, Terminal, VSCode, Slack...
- 10 kịch bản test thực tế, bao gồm edge case và chuyển app liên tục.
- Không phát hiện crash, lag, hay mất trạng thái sau khi khởi động lại.

---



## 🚀 HƯỚNG DẪN SỬ DỤNG NHANH

1. **Bật Smart Per-App Mode:** Click icon 🇻🇳 trên menu bar → bật "Smart Per-App Mode".
2. **Chuyển app:** Bộ gõ sẽ tự động nhớ trạng thái cho từng app.
3. **Xem/cài đặt nâng cao:** Vào Settings để xem danh sách app đã lưu, xóa cấu hình, hoặc kiểm tra trạng thái hiện tại.

---

**Cảm ơn bạn đã sử dụng Vietnamese IME!**