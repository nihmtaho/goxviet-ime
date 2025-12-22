# Release Note – Gõ Việt (GoxViet) v1.3.0
**Ngày phát hành:** 2025-12-22

---

## 🎨 UI Settings Refactor (macOS)

- **Chuẩn hóa toàn bộ giao diện cửa sổ Settings** sử dụng SwiftUI `NavigationSplitView` theo phong cách native macOS.
- **Sidebar**: 
  - Sửa lỗi icon không hiển thị cho từng mục (General, Per-App, Advanced, About).
  - Loại bỏ Divider thừa, đồng bộ màu sắc và spacing.
- **Panel chi tiết**:
  - Giảm bán kính bo góc (radius) và padding để đồng bộ với sidebar, giao diện gọn gàng hơn.
  - Loại bỏ animation chuyển panel, đảm bảo chuyển đổi tức thời, không gây lag.
- **Trải nghiệm người dùng**:
  - Thêm hiệu ứng mượt mà khi đóng/mở sidebar, loại bỏ hiện tượng giật lag.
  - Đảm bảo mọi thao tác chuyển đổi panel, toggle sidebar đều đúng chuẩn macOS.

---

## 🛠️ Tổng hợp thay đổi

- Refactor toàn bộ cấu trúc Settings UI, không còn custom sidebar toggle, không còn lỗi icon sidebar.
- Đảm bảo code SwiftUI sạch, dễ bảo trì, không phụ thuộc mã nguồn tham khảo ngoài.
- Không thay đổi logic xử lý cốt lõi hoặc API, chỉ cải thiện UI/UX.

---

## 🔗 Tham khảo

- Đã cập nhật roadmap và changelog cho phiên bản này.
- Đảm bảo tuân thủ đầy đủ quy tắc dự án về tài liệu, định dạng và kiểm thử.

---

**Gõ Việt Team**