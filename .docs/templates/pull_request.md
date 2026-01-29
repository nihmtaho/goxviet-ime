## 1. Mục tiêu của Pull Request
- [ ] Tính năng mới
- [ ] Sửa lỗi
- [ ] Cải tiến hiệu năng
- [ ] Cập nhật tài liệu
- [ ] Khác (ghi rõ):

## 2. Mô tả ngắn gọn
<!-- Tóm tắt thay đổi chính, lý do thực hiện, phạm vi ảnh hưởng. -->

## 3. Checklist trước khi mở PR
- [ ] Đã test đầy đủ (unit/integration/regression)
- [ ] Đã kiểm tra lint/format (`cargo fmt`, `cargo clippy`)
- [ ] Đã cập nhật tài liệu nếu thay đổi public API/FFI
- [ ] Đã mô tả rõ behavior trước/sau, ảnh hưởng hiệu năng
- [ ] Đã rebase/squash commit hợp lệ
- [ ] Đã kiểm tra không vi phạm branding/kiến trúc

## 4. Ảnh hưởng & rủi ro
- Ảnh hưởng đến module nào?
- Có thay đổi FFI/platform không?
- Có thể gây regression ở đâu?
- Đã benchmark hiệu năng chưa?

## 5. Hướng dẫn kiểm thử
- Các bước test thủ công (nếu cần)
- Dataset/Case đặc biệt cần chú ý

## 6. Liên kết issue/tài liệu liên quan
- Issue/Task liên quan: #
- Tài liệu tham khảo: (nếu có)

## 7. Ghi chú khác
<!-- Thêm thông tin bổ sung, ảnh, log, benchmark nếu cần -->

---
*PR không tuân thủ checklist sẽ bị từ chối. Đảm bảo không vi phạm branding, hiệu năng, kiến trúc.*
