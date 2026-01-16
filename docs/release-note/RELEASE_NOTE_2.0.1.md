# GoxViet v2.0.1 – Release Note

**Ngày phát hành:** 2026-01-16  
**Phiên bản:** 2.0.1

---

## 🚩 Tổng quan

Phiên bản 2.0.1 là bản cập nhật sửa lỗi quan trọng và tối ưu hóa hiệu năng. Mục tiêu chính là khắc phục các lỗi logic cốt lõi ảnh hưởng đến trải nghiệm gõ hàng ngày (như lỗi gõ 3 ký tự, lỗi xóa ký tự đầu), đồng thời tối ưu hóa đáng kể tốc độ xử lý cho kiểu gõ VNI để đạt ngang bằng với Telex.

---

## ✨ Tính năng & Cải tiến

### 1. Tối ưu hóa hiệu năng VNI
- **Mô tả:** Thay thế thuật toán tìm kiếm tuyến tính bằng tìm kiếm nhị phân (binary search) cho việc đặt dấu.
- **Kết quả:** Giảm độ trễ gõ VNI từ 15-18ms xuống còn **8-11ms**.

### 2. Tối ưu hóa Core Engine
- **Mô tả:** Cải thiện hiệu quả thao tác bộ đệm (buffer), giảm thiểu cấp phát bộ nhớ không cần thiết trong các thao tác biến đổi.
- **Ảnh hưởng:** Giúp bộ gõ hoạt động nhẹ nhàng và ổn định hơn.

---

## 🐞 Sửa lỗi

### 1. Fix: Lỗi Logic Toggle & Revert (Nghiêm trọng)
- **Mô tả lỗi:**
  - Gõ 3 lần phím (vd: `d` + `d` + `d`) không quay lại được trạng thái `dd`.
  - Revert một biến đổi đôi khi xóa mất ký tự phía trước (vd: gõ "add", khi revert chữ `đ` cuối cùng lại xóa mất chữ `a` đầu tiên, thành "dd").
- **Giải pháp:** Sửa lại logic `revert_stroke` để sử dụng độ dài bộ đệm cũ chính xác.
- **Kết quả:** Các thao tác gõ lặp phím và xóa hoạt động đúng như mong đợi.

### 2. Fix: Validate Tiếng Việt & Từ bắt đầu bằng "TR"
- **Mô tả lỗi:**
  - Một số từ bắt đầu bằng "tr" (như 'truyền', 'triển') không bỏ dấu được do luật ngữ âm quá chặt.
  - Cho phép các tổ hợp sai (vd: "neư" từ "new" + w).
- **Giải pháp:** Điều chỉnh luật ngữ âm, cho phép "tr" và chặn các tổ hợp nguyên âm + w sai.

### 3. Fix: Ký tự đặc biệt & Nhận diện Tiếng Anh
- **Mô tả lỗi:** Gõ dấu câu (vd: `!`, `%`) ngay sau từ tiếng Việt khiến từ bị hoàn tác về dạng thô (vd: "đã!" → "d9a41"). Lỗi gõ từ tiếng Anh như "off" bị thành "òf".
- **Giải pháp:** Loại bỏ heuristic phát hiện tiếng Anh quá nhạy cảm với phím Shift+Số. Cải thiện logic revert tone cho từ tiếng Anh.

### 4. Fix: Đồng bộ UI (macOS)
- **Mô tả:** Trạng thái bật/tắt trên thanh Menu không khớp với cửa sổ Cài đặt.
- **Kết quả:** Trạng thái hiển thị nhất quán trên toàn hệ thống.

---

## 🔧 Thay đổi kỹ thuật
- **FFI Update:** Sửa lỗi con trỏ thô (`*mut u32`) và cập nhật cầu nối Swift để sử dụng `HeapAllocatedResult` an toàn hơn.
- **Tests:** Thêm bộ test toàn diện cho logic backspace thông minh và validate dấu.

---

## ✅ Ảnh hưởng & kiểm thử
- **Hiệu suất:** VNI latency < 11ms, Telex latency < 3ms.
- **Độ ổn định:** Đã kiểm tra thủ công các trường hợp biên (edge cases) như "add", "mmm", "ddd" và các từ bắt đầu bằng "tr".

---

## 📋 Tổng kết thay đổi
| Loại | Số lượng |
|------|----------|
| Tính năng mới | 0 |
| Sửa lỗi | 4 |
| Cải tiến | 2 |

---

## 🔗 Tham khảo
- [Báo cáo lỗi](https://github.com/nihmtaho/goxviet/issues)
- [Lịch sử phát hành](./)

---
**Gõ Việt (GoxViet) – Bộ gõ tiếng Việt hiệu suất cao!**