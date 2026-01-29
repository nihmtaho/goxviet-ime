# GoxViet v2.0.3 – Release Note

**Ngày phát hành:** 2026-01-29  
**Phiên bản:** 2.0.3

---

## 🚩 Tổng quan

Phiên bản 2.0.3 là bản patch focusing vào **tối ưu hóa English detection**, **cải thiện Telex double-key logic**, và **hoàn thiện documentation**. Release này đánh dấu sự hoàn thành của Phase 2.1 (Engine Enhancement) với cải thiện đáng kể về độ chính xác khi phát hiện từ tiếng Anh và xử lý Telex tone trong điều kiện gõ nhanh.

---

## ✨ Tính năng mới

### 1. Architecture & Coding Standards Documentation
- **Mô tả:** Hoàn thiện tài liệu kiến trúc, quy tắc coding, và quy trình phát triển chính thức.
- **Cách sử dụng:** Các contributor mới có thể tham khảo `.github/instructions/` để hiểu rõ quy trình phát triển.
- **Ảnh hưởng:** Giảm onboarding time, tăng code consistency, chuẩn hóa quy trình review.

### 2. GitHub Workflow Templates
- **Mô tả:** Thêm template cho Issue (bug report, feature request) và Pull Request.
- **Cách sử dụng:** Người dùng tự động nhận template khi tạo issue/PR mới.
- **Ảnh hưởng:** Giúp các contributor cung cấp đủ thông tin, tăng tốc độ review và fix.

---

## 🐞 Sửa lỗi

### 1. Fix: Key Skipping Logic (Critical)
- **Mô tả lỗi:** Logic bỏ qua phím (key skipping) đôi khi không nhận diện đúng từ tiếng Anh, dẫn đến sai transform.
- **Nguyên nhân:** Logic skipping quá aggressive, bỏ qua các phím cần thiết cho Telex tone handling.
- **Giải pháp:** Vô hiệu hóa key skipping logic và dùng confidence threshold trong English detection thay vào đó.
- **Kết quả:** Phát hiện tiếng Anh chính xác hơn, giảm false transform, tone handling Telex tốt hơn.
- **Commit:** #46

### 2. Fix: English Auto-Restore with Confidence Thresholds
- **Mô tả lỗi:** Auto-restore tiếng Anh đôi khi restore nhầm từ khi gõ nhanh, ví dụ: "off" → "òf" không restore đúng.
- **Nguyên nhân:** Không có threshold để đánh giá độ tin cậy của English word detection.
- **Giải pháp:** Thêm confidence threshold (0.0-1.0) dựa trên phonotactic rules và dictionary lookup.
- **Kết quả:** Restore chỉ khi độ tin cậy cao, giảm false positive khi gõ nhanh hoặc typo.

### 3. Fix: Dictionary Integrity
- **Mô tả lỗi:** Các tập từ điển (2-7 ký tự) có dữ liệu không đồng nhất hoặc bị hỏng.
- **Nguyên nhân:** Quá trình cập nhật từ điển thủ công không kiểm tra đầy đủ.
- **Giải pháp:** Chạy validation script trên tất cả tập từ điển, sửa duplicates và invalid entries.
- **Kết quả:** Từ điển đồng nhất, không có entry trùng lặp hoặc bị hỏng.

### 4. Fix: Telex Double-Key Logic
- **Mô tả lỗi:** Double-key trong Telex (ví dụ: `ss`, `ff`, `rr`) đôi khi không toggle đúng hoặc làm transform nhầm.
- **Nguyên nhân:** Logic xử lý double-key không đủ robust, nhầm lẫn với các pattern khác.
- **Giải pháp:** Nâng cấp double-key detection logic để phân biệt rõ ràng giữa "undo tone" vs "transform".
- **Kết quả:** Double-key hoạt động chính xác, người dùng có thể undo tone một cách đáng tin cậy.

---

## 🔧 Cải tiến

- **English Detection Performance:** Tăng tốc độ English word lookup bằng binary search thay vì linear scan.
- **Dictionary Loading:** Optimize dictionary binary format loading, giảm startup time.
- **GitHub Actions CI/CD:** Cải thiện release workflow, version tracking bằng build number.
- **Memory Footprint:** Giảm bộ nhớ sử dụng trong buffer management nhờ optimization dictionary structure.

---

## ⚠️ Breaking Changes

- Không có breaking changes trong phiên bản này.
- Tất cả API FFI từ 2.0.2 vẫn compatible.

---

## ✅ Ảnh hưởng & Kiểm thử

- **Hiệu suất:** Độ trễ < 16ms (đạt chuẩn 60fps), một số case giảm từ 12-14ms xuống 8-11ms.
- **Bộ nhớ:** Không memory leak, footprint giảm ~5% so với 2.0.2 nhờ dictionary optimization.
- **Tương thích:** macOS 12.0+, Windows 10 21H2+.
- **Kiểm thử:** Đã test với ~72K từ tiếng Việt + ~100K từ tiếng Anh, pass tất cả regression test.

---

## 📋 Tổng kết thay đổi

| Loại | Số lượng | Chi tiết |
|------|----------|----------|
| Tính năng mới | 2 | Architecture docs, GitHub templates |
| Sửa lỗi | 4 | Key skipping, auto-restore, dictionary, double-key |
| Cải tiến | 4 | Performance, CI/CD, memory, lookup speed |
| File thay đổi | ~50+ | Core engine, docs, workflows, dictionaries |

---

## 📥 Cài đặt

### Tải DMG trực tiếp

1. Tải file `GoxViet-2.0.3-unsigned.dmg` từ phần Assets bên dưới
2. Mở DMG và kéo GoxViet vào thư mục Applications
3. Cấp quyền Accessibility khi được yêu cầu
4. Khởi động lại ứng dụng (nếu đang chạy)

### Homebrew

```bash
brew install --cask goxviet
# hoặc update từ 2.0.2
brew upgrade goxviet
```

---

## 🔗 Tham khảo

- [Hướng dẫn nhanh](../getting-started/QUICK_START.md)
- [Báo cáo lỗi](https://github.com/nihmtaho/goxviet/issues)
- [Lịch sử phát hành](./)
- [Changelog](../../CHANGELOG.md)

---

## 💬 Feedback & Support

Nếu gặp vấn đề hoặc có góp ý, vui lòng:
- Tạo issue trên GitHub: https://github.com/nihmtaho/goxviet/issues
- Hoặc liên hệ qua email (nếu có)

---

**Gõ Việt (GoxViet) – Bộ gõ tiếng Việt hiệu suất cao!** 🚀
