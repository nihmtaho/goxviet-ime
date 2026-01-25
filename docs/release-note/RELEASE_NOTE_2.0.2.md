# GoxViet v2.0.2 – Release Note

**Ngày phát hành:** 2026-01-25  
**Phiên bản:** 2.0.2

---

> *“Release trong một buổi chiều với một hàng dài xe đang xếp hàng 🚗🚙🚕.”*

## 🚩 Tổng quan

Phiên bản 2.0.2 tập trung vào việc ổn định Engine thông qua hệ thống kiểm thử từ điển (dictionary-based testing), cải thiện độ chính xác của kiểu gõ Telex và tối ưu hóa trải nghiệm người dùng với tính năng quản lý thiết lập theo từng ứng dụng (Per-App Tracking).

---

## ✨ Tính năng mới

### 1. Opt-in Per-App Tracking

- **Mô tả:** Thay vì tự động lưu mọi ứng dụng đã từng "focus" vào danh sách "Saved Applications", hệ thống giờ đây chỉ lưu ứng dụng khi người dùng thực sự bật Gõ Việt cho ứng dụng đó lần đầu tiên.
- **Cách sử dụng:** Truy cập Settings > Per-App để quản lý danh sách các ứng dụng đã được cá nhân hóa.
- **Ảnh hưởng:** Giảm thiểu "rác" trong danh sách ứng dụng, chỉ giữ lại những app người dùng thực sự quan tâm.

### 2. Real-time Saved Applications Update

- **Mô tả:** Danh sách ứng dụng trong màn hình Settings giờ đây cập nhật ngay lập tức (real-time) khi người dùng bật/tắt Gõ Việt hoặc gỡ bỏ ứng dụng khỏi danh sách.

---

## 🐞 Sửa lỗi

### 1. Fix: Telex Typing Bug (d+i+s)

- **Mô tả lỗi:** Gõ `dis` không ra `dí` mà vẫn giữ nguyên ASCII.
- **Giải pháp:** Cải tiến logic nhận diện dấu khi đi kèm với phím chức năng trong Telex Core.

### 2. Fix: Breve Modifier Transform (aw -> ă)

- **Mô tả lỗi:** Lỗi không chuyển đổi hoặc đếm sai phím xóa khi thực hiện gõ `aw` để tạo chữ `ă`.
- **Giải pháp:** Cập nhật backspace count chính xác trong engine khi áp dụng các biến đổi dấu (breve transform).

---

## 🔧 Cải tiến

- **Infrastructure:** Triển khai Dictionary-based testing với tập dữ liệu khổng lồ (~172K từ), giúp phát hiện sớm các lỗi gõ sai chính tả hoặc nhận diện nhầm tiếng Anh.
- **English Dictionary:** Thêm từ "console" và các biến thể để tránh bị engine "Vietnamese- hóa" nhầm.
- **Documentation:** Cập nhật toàn bộ tài liệu về kiến trúc per-app và hướng dẫn sử dụng trên macOS.

---

## ⚠️ Breaking Changes (nếu có)

- Không có breaking changes trong phiên bản này.

---

## ✅ Ảnh hưởng & kiểm thử

- **Hiệu suất:** Duy trì độ trễ < 1ms cho các phím gõ thông thường, đảm bảo chuẩn 60fps.
- **Bộ nhớ:** Đã kiểm tra leak-free thông qua Xcode Instruments.
- **Tương thích:** macOS 12.0+

---

## 📋 Tổng kết thay đổi

| Loại | Số lượng |
|------|----------|
| Tính năng mới | 2 |
| Sửa lỗi | 3 |
| Cải tiến | 3 |

---

## 📥 Cài đặt

### Tải DMG trực tiếp

1. Tải file `GoxViet-2.0.2-unsigned.dmg` từ phần Assets bên dưới
2. Mở DMG và kéo GoxViet vào thư mục Applications
3. Cấp quyền Accessibility khi được yêu cầu

### Homebrew (Comming soon)

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