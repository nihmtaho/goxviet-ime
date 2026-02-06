# GoxViet v2.0.8 – Release Note

**Ngày phát hành:** 2026-02-06  
**Phiên bản:** 2.0.8

---

## 🚩 Tổng quan

Phiên bản 2.0.8 tập trung vào sửa lỗi và tối ưu hiệu suất, đặc biệt là khắc phục các vấn đề liên quan đến Zen Browser, xử lý dấu thanh, và ổn định build.

---

## 🐞 Sửa lỗi

### 1. Fix: Zen Browser Duplication Bug (Issue #54)

- **Mô tả lỗi:** Gõ tiếng Việt trên Zen Browser gây ra trùng lặp ký tự.
- **Nguyên nhân:** `TextInjectionHelper` sử dụng phương thức không phù hợp với Zen Browser's override mechanism.
- **Giải pháp:** Chuyển sang sử dụng phương thức `AX API Direct` kết hợp với logic fallback tối ưu. Code fallback được viết lại để xử lý nhanh hơn: thoát ngay nếu phát hiện browser override (autocomplete) và chỉ retry khi gặp lỗi kết nối AX. Đặc biệt xử lý triệt để case gõ "đ" (dđ) bằng logic workaround thông minh (Type -> Left -> Backspace -> Right) với độ trễ tối ưu 1ms.
- **Kết quả:** Gõ tiếng Việt trên Zen Browser hoạt động bình thường, không trùng lặp ký tự.

### 2. Fix: Proxy Event Injection

- **Mô tả lỗi:** `TextInjectionHelper` không sử dụng proxy khi fallback, gây mất ổn định khi inject text.
- **Nguyên nhân:** Logic fallback không gọi proxy event injection.
- **Giải pháp:** Đảm bảo `TextInjectionHelper` sử dụng proxy event injection trong cả trường hợp AX API thất bại.
- **Kết quả:** Tính ổn định khi inject text được nâng cao, đặc biệt trong các trường hợp AX API không hoạt động.

### 3. Fix: UI Layout Recursion

- **Mô tả lỗi:** Warning `_NSDetectedLayoutRecursion` xuất hiện khi sử dụng ứng dụng.
- **Nguyên nhân:** `MenuToggleView` thay thế SwiftUI RootView liên tục, gây lặp layout.
- **Giải pháp:** Refactor `MenuToggleView` sử dụng `ObservableObject` thay vì thay thế RootView liên tục.
- **Kết quả:** Loại bỏ warning, UI ổn định hơn.

### 4. Fix: Build Stability

- **Mô tả lỗi:** Lỗi biên dịch do thiếu import `Combine` và thiếu định nghĩa `KeyCode`.
- **Giải pháp:** Bổ sung các import và định nghĩa cần thiết.
- **Kết quả:** Build ổn định, không lỗi biên dịch.

### 5. Fix: Tone Repositioning

- **Mô tả lỗi:** Lỗi transform khi nhấn SPACE khiến việc gõ không thoải mái.
- **Giải pháp:** Cải thiện logic xử lý SPACE trong tone repositioning.
- **Kết quả:** Gõ tiếng Việt mượt mà hơn, không bị gián đoạn bởi SPACE.

---

## 🔧 Cải tiến

- Tối ưu hóa fallback logic trong AX API, giảm latency khi Zen Browser xảy ra tình trạng override.
- Cải thiện xử lý event injection để đảm bảo tính ổn định.
- Refactor UI components để tránh layout recursion.
- Nâng cao độ ổn định của build system.

---

## ⚠️ Breaking Changes

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
| Tính năng mới | 0 |
| Sửa lỗi | 5 |
| Cải tiến | 4 |

---

## 📥 Cài đặt

### Tải DMG trực tiếp

1. Tải file `GoxViet-2.0.8-unsigned.dmg` từ phần Assets bên dưới
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