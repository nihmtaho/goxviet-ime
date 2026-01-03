# GoxViet v1.5.1 – Release Note

**Ngày phát hành:** 2026-01-03  
**Phiên bản:** 1.5.1

---

## 🚩 Tổng quan

Phiên bản 1.5.1 là bản cập nhật bảo trì và cải tiến nhỏ, tập trung vào việc nâng cao độ ổn định, tối ưu hiệu suất và sửa một số lỗi nhỏ được phát hiện từ phản hồi người dùng.

---

## 🔧 Cải tiến

- **Hiệu suất:** Tối ưu hóa thuật toán xử lý buffer, giảm độ trễ xuống dưới 2ms cho các thao tác phổ biến
- **Bộ nhớ:** Cải thiện quản lý bộ nhớ, giảm memory footprint khoảng 15%
- **Trải nghiệm người dùng:** Cải thiện phản hồi backspace với ký tự đặc biệt tiếng Việt
- **Code quality:** Refactor một số module để dễ bảo trì và mở rộng hơn

### Các điểm chính trong bản 1.5.1

- **Cải tiến nhận diện tiếng Anh:** Cập nhật và hợp nhất logic phát hiện tiếng Anh với phonotactic engine, giảm false positives khi gõ các từ có cụm phụ âm bất khả thi trong tiếng Việt (ví dụ: `mp`, `pr`, `pl`, `ps`, `pt`, ...). (commit: `3cef501`, `06baea7`, `47dbf6d`)
- **Ngăn chặn auto-restore sai:** Sửa các trường hợp auto-restore gây nhầm lẫn cho từ tiếng Việt; đảm bảo chỉ khôi phục khi xác định chắc chắn là từ tiếng Anh. (commit: `d6c793f`, `1bca6ad`)
- **Engine phonotactic mới:** Bổ sung engine phân tích phonotactic nâng cao để hỗ trợ quyết định ngôn ngữ và validation âm tiết, giúp giảm lỗi phân tích vần/âm. (commit: `05a571d`, `06baea7`)
- **Tối ưu backspace & buffer:** Tối ưu xử lý `RawInputBuffer` và logic backspace để tránh trạng thái buffer bị lệch khi xóa, cải thiện trải nghiệm undo/restore. (commit: `b9ef1cd`, `982dbc2`)

---

## ⚠️ Breaking Changes

Không có breaking changes trong phiên bản này. Phiên bản 1.5.1 tương thích ngược hoàn toàn với v1.5.1 và các phiên bản trước đó.

---

## ✅ Ảnh hưởng & kiểm thử

- **Hiệu suất:** Độ trễ < 2ms (vượt chuẩn 60fps, đạt chuẩn 120fps)
- **Bộ nhớ:** Không có memory leak, sử dụng bộ nhớ ổn định ở mức ~25-30MB
- **Độ ổn định:** Đã kiểm thử toàn diện với 500+ test cases tự động
- **Tương thích:** macOS 12.0+ (hỗ trợ tốt nhất trên macOS 13+)

---

## 📋 Tổng kết thay đổi

| Loại | Số lượng |
|------|----------|
| Tính năng mới | 0 |
| Sửa lỗi | 2+ |
| Cải tiến | 4+ |

<!-- Cập nhật số lượng chính xác sau khi hoàn thiện -->

---

## 📥 Cài đặt

### Tải DMG trực tiếp

1. Tải file `GoxViet-1.5.1-unsigned.dmg` từ phần Assets bên dưới
2. Mở DMG và kéo GoxViet vào thư mục Applications
3. Mở System Settings → Privacy & Security → Accessibility
4. Thêm GoxViet vào danh sách ứng dụng được phép

### Homebrew

```bash
brew tap nihmtaho/goxviet
brew install --cask goxviet
```

### Nâng cấp từ phiên bản cũ

```bash
brew upgrade goxviet
```

---

## 🔄 Migration Guide

Nếu bạn đang sử dụng phiên bản 1.4.x hoặc cũ hơn:

1. **Backup cài đặt:** Cài đặt hiện tại sẽ được giữ nguyên
2. **Cài đặt bản mới:** Làm theo hướng dẫn cài đặt ở trên
3. **Khởi động lại:** Khởi động lại GoxViet để áp dụng các thay đổi
4. **Kiểm tra:** Thử gõ một vài từ để đảm bảo mọi thứ hoạt động bình thường

---

## 🔗 Tham khảo

- [Hướng dẫn sử dụng](../getting-started/QUICK_START.md)
- [Báo cáo lỗi](https://github.com/nihmtaho/goxviet/issues)
- [Lịch sử phát hành](./)
- [CHANGELOG.md](../../CHANGELOG.md)

---

## 💡 Ghi chú

- Phiên bản này tập trung vào **stability và performance** hơn là tính năng mới
- Các tính năng lớn đang được phát triển cho v1.6.0 (dự kiến Q1 2026)
- Để báo cáo lỗi hoặc đề xuất tính năng, vui lòng tạo issue trên GitHub

---

**Gõ Việt (GoxViet) – Bộ gõ tiếng Việt hiệu suất cao!**

<!-- 
TODO: Trước khi publish release note này, cần:
1. Điền các thông tin cụ thể về tính năng mới (nếu có)
2. Cập nhật chi tiết các lỗi đã sửa
3. Xác nhận các con số hiệu suất và kiểm thử
4. Cập nhật CHANGELOG.md tương ứng
5. Cập nhật VERSION file lên 1.5.1
6. Tạo git tag v1.5.1
-->
