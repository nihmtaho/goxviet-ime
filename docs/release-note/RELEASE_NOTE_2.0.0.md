# GoxViet v2.0.0 – Release Note

**Ngày phát hành:** 2026-01-15  
**Phiên bản:** 2.0.0

---

## 🚩 Tổng quan

Phiên bản 2.0.0 đánh dấu bước tiến quan trọng trong việc nâng cao hiệu năng và trải nghiệm người dùng. Phiên bản này tích hợp engine mới giúp tăng tốc độ xử lý, cải thiện giao diện người dùng, và khắc phục các lỗi quan trọng liên quan đến nhận diện tiếng Anh và xử lý phím.

---

## ✨ Tính năng mới

### 1. Engine mới tối ưu hiệu năng

- **Mô tả:** Tích hợp engine mới giúp tăng tốc độ xử lý và tối ưu hiệu năng toàn ứng dụng.
- **Ảnh hưởng:** Cải thiện đáng kể tốc độ phản hồi khi gõ phím, giảm độ trễ và tăng trải nghiệm người dùng.

### 2. Giao diện người dùng được cập nhật

- **Mô tả:** Cập nhật giao diện người dùng cho trải nghiệm mượt mà hơn.
- **Ảnh hưởng:** Giao diện trực quan hơn, dễ sử dụng và hiện đại hơn.

---

## 🐞 Sửa lỗi

### 1. Fix: Lỗi gõ VNI (#33)

- **Mô tả lỗi:** Lỗi khi sử dụng phương thức gõ VNI.
- **Giải pháp:** Đã khắc phục lỗi xử lý trong engine VNI.
- **Kết quả:** VNI hoạt động ổn định và chính xác.

### 2. Fix: Telex nhận nhầm từ tiếng Anh

- **Mô tả lỗi:** Telex đôi khi nhận nhầm từ tiếng Anh, gây ra chuyển đổi sai.
- **Giải pháp:** Cải thiện thuật toán nhận diện tiếng Anh, giảm false positive khi gõ Telex. Sửa lỗi nhận diện prefix/suffix cho các từ tiếng Anh phổ biến.
- **Kết quả:** Nhận diện tiếng Anh chính xác hơn, giảm thiểu chuyển đổi nhầm.

### 3. Fix: Nhập số bị chuyển thành dấu hoặc ký tự đặc biệt (#30)

- **Mô tả lỗi:** Khi nhập số trong chế độ Telex, số bị chuyển thành dấu hoặc ký tự đặc biệt.
- **Giải pháp:** Cải thiện logic xử lý số trong engine Telex.
- **Kết quả:** Nhập số hoạt động bình thường, không bị chuyển đổi sai.

### 4. Fix: Backspace xóa autocomplete suggestion thay vì text đã gõ (#36)

- **Mô tả lỗi:** Trong trình duyệt, backspace xóa gợi ý autocomplete thay vì văn bản đã gõ.
- **Giải pháp:** Cải thiện xử lý backspace để phân biệt giữa text đã gõ và autocomplete suggestion.
- **Kết quả:** Backspace hoạt động chính xác trong mọi trường hợp.

---

## 🔧 Cải tiến

- Cải thiện thuật toán nhận diện tiếng Anh, giảm false positive khi gõ Telex
- Tối ưu hiệu năng toàn ứng dụng với engine mới
- Cập nhật giao diện người dùng cho trải nghiệm mượt mà hơn
- Xóa các tài liệu thừa, tối ưu cấu trúc dự án

---

## ⚠️ Breaking Changes (nếu có)

<!-- Liệt kê các thay đổi không tương thích ngược -->

- Không có breaking changes trong phiên bản này.

---

## ✅ Ảnh hưởng & kiểm thử

- **Hiệu suất:** Độ trễ < 16ms (đạt chuẩn 60fps)
- **Bộ nhớ:** Không memory leak
- **Tương thích:** macOS 12.0+

---

## 📋 Tổng kết thay đổi

| Loại | Số lượng |
|------|----------|
| Tính năng mới | 2 |
| Sửa lỗi | 4 |
| Cải tiến | 4 |

---

## 📥 Cài đặt

### Tải DMG trực tiếp

1. Tải file `GoxViet-2.0.0-unsigned.dmg` từ phần Assets bên dưới
2. Mở DMG và kéo GoxViet vào thư mục Applications
3. Cấp quyền Accessibility khi được yêu cầu

### Homebrew (coming soon)

```bash
brew install --cask goxviet
```

---

## 🔗 Tham khảo

- [Hướng dẫn sử dụng](../getting-started/QUICK_START.md)
- [Báo cáo lỗi](https://github.com/nihmtaho/goxviet/issues)
- [Lịch sử phát hành](./)

---

**Gõ Việt (GoxViet) – Bộ gõ tiếng Việt hiệu suất cao!**