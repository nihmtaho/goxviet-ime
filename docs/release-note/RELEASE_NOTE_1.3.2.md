# GoxViet v1.3.2 – Release Note

**Ngày phát hành:** 2025-12-24  
**Phiên bản:** 1.3.2

---

## 🚩 Tổng quan

Phiên bản 1.3.2 tập trung vào việc khắc phục lỗi nghiêm trọng liên quan đến tính năng **Telex Auto Restore English** – đảm bảo trải nghiệm gõ tiếng Việt và tiếng Anh mượt mà, không bị biến đổi sai hoặc mất đồng bộ buffer khi chuyển đổi giữa hai ngôn ngữ.

---

## 🐞 Sửa lỗi nổi bật

### 1. Fix: Telex Auto Restore English Bug

- **Mô tả lỗi:**  
  Khi gõ các từ tiếng Anh có cụm phụ âm không hợp lệ trong tiếng Việt (ví dụ: `improve`, `improvement`, `import`, `express`, `please`, ...), bộ gõ đã nhận diện sai, áp dụng quy tắc Telex lên các ký tự tiếng Anh (ví dụ: `r` bị coi là dấu hỏi), dẫn đến kết quả sai như `ỉmpove ` thay vì `improve `, hoặc buffer và màn hình bị lệch nhau.
- **Nguyên nhân:**  
  Hàm phát hiện tiếng Anh (`has_english_word_pattern()`) chưa nhận diện đủ các cụm phụ âm bất khả thi trong tiếng Việt (như `mp`, `pr`, `pl`, `ps`, `pt`, `wr`, `f`+phụ âm, `w`+phụ âm, `j`+phụ âm, `z`+phụ âm). Điều này khiến engine áp dụng nhầm quy tắc tiếng Việt cho từ tiếng Anh.
- **Giải pháp:**  
  - Bổ sung logic nhận diện các cụm phụ âm bất khả thi vào hàm kiểm tra tiếng Anh.
  - Khi phát hiện từ tiếng Anh, engine sẽ:
    - Không áp dụng quy tắc Telex (giữ nguyên từ gốc).
    - Nếu đã áp dụng nhầm, tự động khôi phục lại từ tiếng Anh đúng khi nhấn Space (auto-restore).
  - Đảm bảo không ảnh hưởng đến logic gõ tiếng Việt, không làm giảm hiệu suất.
- **Kết quả:**  
  - Các từ tiếng Anh như `improve`, `import`, `express`, `please`... được gõ và khôi phục chính xác, không còn lỗi biến đổi dấu hoặc lệch buffer.
  - Các từ tiếng Việt và edge case vẫn hoạt động đúng như kỳ vọng.
  - Đã bổ sung và mở rộng test coverage cho cả tiếng Anh và tiếng Việt.

---

## ✅ Ảnh hưởng & kiểm thử

- **Không ảnh hưởng đến hiệu suất:** Độ trễ vẫn < 3ms cho mọi thao tác.
- **Không phát sinh lỗi mới:** Đã kiểm thử toàn diện với bộ test tự động và kiểm thử thủ công.
- **Tài liệu:** Đã cập nhật hướng dẫn phát hiện tiếng Anh và logic auto-restore trong tài liệu phát triển.

---

## 📋 Tổng kết thay đổi

- Sửa lỗi nhận diện tiếng Anh trong Telex, đảm bảo auto-restore hoạt động chính xác.
- Mở rộng test cho các trường hợp tiếng Anh và edge case.
- Không thay đổi API công khai, không ảnh hưởng FFI.

---

## 🔗 Tham khảo

- [Chi tiết kỹ thuật & test case](../fixes/TELEX_AUTO_RESTORE_ENGLISH.md) *(nếu có)*
- [Hướng dẫn sử dụng & cấu hình](../getting-started/QUICK_START.md)
- [Lịch sử phát hành](./)

---

**Gõ Việt (GoxViet) – Đúng chuẩn, đúng ngữ cảnh, không lỗi dấu!**