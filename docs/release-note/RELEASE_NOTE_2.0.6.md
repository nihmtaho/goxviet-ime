# GoxViet v2.0.6 – Release Note

**Ngày phát hành:** 2026-02-04  
**Phiên bản:** 2.0.6

---

## 🚩 Tổng quan

Phiên bản 2.0.6 tập trung vào **cải tiến hệ thống cập nhật** với cấu trúc trạng thái tinh gọn hơn. Cải thiện kiến trúc code bằng cách centralize update state management, tách logic kiểm tra cập nhật, và cập nhật UI để phản ánh trạng thái cập nhật chi tiết hơn.

---

## ✨ Tính năng mới

### 1. Update State với Associated Values

- **Mô tả:** Cấu trúc lại `UpdateState` enum sử dụng Swift associated values, thêm trạng thái `installing` để theo dõi quá trình cài đặt.
- **Cách sử dụng:** UpdateManager sẽ tự động chuyển đổi giữa các trạng thái: `idle` → `checking` → `downloading` → `installing` → `idle`.
- **Ảnh hưởng:** UI có thể hiển thị tiến trình cập nhật chi tiết hơn, người dùng biết được IME đang ở giai đoạn nào của quá trình cập nhật.

---

## 🔧 Cải tiến

### 1. UpdateManager Centralization

- **Chi tiết:** Tập trung toàn bộ logic quản lý trạng thái cập nhật vào `UpdateManager`.
- **Lợi ích:** Code dễ bảo trì, ít duplicate logic, dễ thêm tính năng mới sau này.

### 2. UpdateChecker Extraction

- **Chi tiết:** Tách logic kiểm tra cập nhật vào module `UpdateChecker` riêng biệt.
- **Lợi ích:** Separation of concerns, dễ unit test, dễ bảo trì.

### 3. UI Components Update

- **Chi tiết:** Cập nhật các SwiftUI component (progress indicator, status text) để phản ánh trạng thái mới.
- **Lợi ích:** UX được cải thiện, người dùng có visual feedback rõ ràng về quá trình cập nhật.

---

## ⚠️ Breaking Changes

- Không có breaking changes trong phiên bản này.

---

## ✅ Ảnh hưởng & kiểm thử

- **Hiệu suất:** Không ảnh hưởng, latency vẫn < 16ms (đạt chuẩn 60fps)
- **Bộ nhớ:** Không memory leak, update state management được optimize
- **Tương thích:** macOS 12.0+

---

## 📋 Tổng kết thay đổi

| Loại | Số lượng |
|------|----------|
| Tính năng mới | 1 |
| Cải tiến | 3 |
| Sửa lỗi | 0 |

---

## 📥 Cài đặt

### Tải DMG trực tiếp

1. Tải file `GoxViet-2.0.6-unsigned.dmg` từ phần Assets bên dưới
2. Mở DMG và kéo GoxViet vào thư mục Applications
3. Cấp quyền Accessibility khi được yêu cầu
4. Khởi động lại IME nếu cần

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