# Release Notes – Gõ Việt (GoxViet) v1.2.3

**Release Date:** December 22, 2025  
**Version:** 1.2.3  
**Type:** Critical Stability & Memory Optimization Release

---

## 🛡️ TỔNG QUAN PHIÊN BẢN

Phiên bản 1.2.3 tập trung giải quyết triệt để các vấn đề về **memory leak** (rò rỉ bộ nhớ) và **memory bloat** (phình bộ nhớ) trên cả hai lớp Rust core và Swift/macOS, đảm bảo Gõ Việt (GoxViet) hoạt động ổn định, tiết kiệm tài nguyên, không tăng bộ nhớ theo thời gian sử dụng.

---

## 🚀 NỘI DUNG CHÍNH

### 1. Khắc phục Memory Leak (Swift/macOS Layer)

- **Nguyên nhân:** Các observer của NotificationCenter trong `InputManager.swift` và `AppDelegate.swift` không được remove đúng cách, dẫn đến closure bị giữ lại trong bộ nhớ.
- **Giải pháp:**  
  - Lưu lại token của observer, remove toàn bộ khi `deinit` hoặc `stop()`.
  - Đảm bảo không tạo observer trùng lặp.
- **Kết quả:**  
  - Không còn hiện tượng tăng bộ nhớ dần (~50-200KB/giờ).
  - Đã xác nhận qua kiểm thử dài hạn.

### 2. Ngăn chặn Memory Bloat (Rust Core & Swift Layer)

- **Rust Core:**  
  - **ShortcutTable:** Giới hạn cứng `MAX_SHORTCUTS = 200` cho số shortcut người dùng có thể lưu.
  - **Buffer, RawInputBuffer, WordHistory:** Đều đã có giới hạn kích thước (Buffer: 64, WordHistory: 10).
- **Swift/macOS Layer:**  
  - **Per-App Settings:** Giới hạn tối đa `MAX_PER_APP_ENTRIES = 100` cho số app lưu trạng thái input mode.
  - **UI:** Cảnh báo người dùng khi gần đạt giới hạn, cho phép xóa dữ liệu cũ.
- **Kết quả:**  
  - Không còn bất kỳ cấu trúc dữ liệu nào có thể tăng không giới hạn.
  - Bộ nhớ duy trì ổn định ở mức ~25-30MB, không tăng dù sử dụng nhiều ngày liên tục.

### 3. Cập nhật & Chuẩn hóa Tài liệu

- Đã bổ sung, chuẩn hóa các tài liệu:
  - `MEMORY_LEAK_FIX.md`
  - `MEMORY_BLOAT_PREVENTION.md`
  - Cập nhật mục lục và hướng dẫn tại `docs/README.md`, `DOCUMENTATION_STRUCTURE.md`, `STRUCTURE_VISUAL.md`
- Đảm bảo mọi thay đổi đều có test (unit test Rust, kiểm thử UI Swift).

---

## 📈 KẾT QUẢ & ẢNH HƯỞNG

- **Ổn định bộ nhớ tuyệt đối:** Không còn tăng bộ nhớ bất thường, kể cả khi sử dụng lâu dài.
- **Bảo vệ trải nghiệm người dùng:** Không còn crash, lag, hoặc giảm hiệu năng do bộ nhớ.
- **Đảm bảo an toàn bộ nhớ:** Đáp ứng tiêu chí memory safety, không rò rỉ, không phình bộ nhớ.
- **Sẵn sàng cho production:** Đã kiểm thử thực tế, phù hợp triển khai diện rộng.

---

## 🔗 TÀI LIỆU THAM KHẢO

- [MEMORY_LEAK_FIX.md](../MEMORY_LEAK_FIX.md)
- [MEMORY_BLOAT_PREVENTION.md](../MEMORY_BLOAT_PREVENTION.md)
- [docs/README.md](../README.md)
- [docs/DOCUMENTATION_STRUCTURE.md](../DOCUMENTATION_STRUCTURE.md)
- [docs/STRUCTURE_VISUAL.md](../STRUCTURE_VISUAL.md)

---

**Kết luận:**  
Gõ Việt (GoxViet) v1.2.3 là bản phát hành trọng yếu, đảm bảo bộ gõ hoạt động ổn định, tiết kiệm tài nguyên, không còn bất kỳ nguy cơ rò rỉ hay phình bộ nhớ. Khuyến nghị tất cả người dùng cập nhật ngay để có trải nghiệm tốt nhất.

---