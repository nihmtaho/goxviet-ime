# Bảo Toàn Từ Tiếng Việt - Hướng Dẫn Người Dùng

**Ngày cập nhật:** 22/12/2025  
**Phiên bản:** 1.3.1  
**Dành cho:** Người dùng GoxViet IME  

---

## Tổng Quan

GoxViet IME giờ đây thông minh hơn trong việc phân biệt giữa từ tiếng Việt và từ tiếng Anh. Khi bạn gõ một từ có dấu thanh, hệ thống sẽ tự động giữ nguyên từ tiếng Việt của bạn thay vì "sửa" thành tiếng Anh.

---

## Hành Vi Mới

### ✅ Từ Tiếng Việt Được Giữ Nguyên

Khi bạn gõ các từ tiếng Việt có dấu, hệ thống sẽ **GIỮ NGUYÊN** kết quả tiếng Việt:

**Ví dụ 1: Từ "tét"**
```
Bạn gõ: t-e-s-t (trong đó 's' tạo dấu sắc)
Màn hình hiển thị: tét
Bạn nhấn: SPACE
Kết quả: tét (GIỮ NGUYÊN - không đổi thành "test")
```

**Ví dụ 2: Từ "tẽt"**
```
Bạn gõ: t-e-x-t (trong đó 'x' tạo dấu ngã)
Màn hình hiển thị: tẽt
Bạn nhấn: SPACE
Kết quả: tẽt (GIỮ NGUYÊN - không đổi thành "text")
```

**Ví dụ 3: Từ "mĩ"**
```
Bạn gõ: m-i-x (trong đó 'x' tạo dấu ngã)
Màn hình hiển thị: mĩ
Bạn nhấn: SPACE
Kết quả: mĩ (GIỮ NGUYÊN - không đổi thành "mix")
```

---

### ✅ Từ Tiếng Anh Vẫn Được Tự Động Sửa

Nếu bạn gõ từ tiếng Anh **KHÔNG có dấu thanh**, hệ thống vẫn tự động sửa lại:

**Ví dụ: Từ "fix"**
```
Bạn gõ: f-i-x (không có dấu nào được tạo)
Màn hình hiển thị: fix
Bạn nhấn: SPACE
Kết quả: fix (với khoảng trắng tự động)
```

---

## Nguyên Tắc Hoạt Động

### 🎯 Quy Tắc Chính

> **"Nếu có dấu thanh, hệ thống giữ nguyên kết quả tiếng Việt"**

Hệ thống sử dụng logic thông minh để quyết định:

1. **Kiểm tra xem có dấu thanh không:**
   - Có dấu sắc (´), huyền (`), hỏi (?), ngã (~), nặng (.)
   - Có dấu mũ (^) hoặc dấu trăng (˘)
   - Có chữ đ gạch ngang

2. **Nếu CÓ dấu:**
   - → Giữ nguyên kết quả tiếng Việt
   - → KHÔNG tự động sửa thành tiếng Anh

3. **Nếu KHÔNG CÓ dấu:**
   - → Kiểm tra xem có phải từ tiếng Anh phổ biến không
   - → Nếu có, tự động sửa lại

---

## So Sánh Trước và Sau

| Tình huống | Bạn gõ | Trước đây | Bây giờ | Giải thích |
|-----------|--------|-----------|---------|-----------|
| Từ Việt có dấu | `t-e-s-t` | ❌ "test" | ✅ "tét" | Giữ nguyên từ Việt |
| Từ Việt có dấu | `t-e-x-t` | ❌ "text" | ✅ "tẽt" | Giữ nguyên từ Việt |
| Từ Anh không dấu | `f-i-x` | ✅ "fix" | ✅ "fix" | Vẫn tự động sửa |
| Từ Việt có dấu | `m-i-x` | ❌ "mix" | ✅ "mĩ" | Giữ nguyên từ Việt |

---

## Tính Năng Backspace Thông Minh

### Xóa và Khôi Phục

Khi bạn gõ sai và muốn sửa, tính năng backspace hoạt động chính xác:

**Kịch bản:**
```
1. Gõ "test" → Hiện "tét"
2. Nhấn SPACE → Buffer xóa, lưu "tét" vào lịch sử
3. Nhấn BACKSPACE → Khôi phục "tét"
4. Nhấn BACKSPACE lần nữa → Xóa chữ 't', hiện "té"
5. Tiếp tục gõ → Hoạt động bình thường
```

**Đã sửa lỗi:**
- ❌ Trước: Sau bước 4, nếu gõ tiếp có thể ra "ttẽ" (lỗi)
- ✅ Bây giờ: Sau bước 4, gõ tiếp hoạt động chính xác

---

## Câu Hỏi Thường Gặp

### ❓ Tại sao "test" không tự động sửa thành "test" nữa?

**Trả lời:** Vì khi bạn gõ `t-e-s-t` trong Telex, chữ 's' là phím tạo dấu sắc, nên kết quả là "tét" (từ tiếng Việt). Hệ thống tôn trọng ý định gõ tiếng Việt của bạn.

**Nếu bạn thực sự muốn gõ "test" (tiếng Anh):**
- Cách 1: Gõ "test" trong ứng dụng tiếng Anh (tắt bộ gõ)
- Cách 2: Gõ "tesst" rồi xóa một chữ 's'
- Cách 3: Sử dụng tính năng Raw Mode (nếu có)

---

### ❓ Tại sao "fix" vẫn được tự động sửa?

**Trả lời:** Vì khi gõ `f-i-x`, chữ 'x' đứng sau 'i' không tạo dấu thanh (ngã tone chỉ áp dụng cho một số nguyên âm). Do đó kết quả vẫn là "fix" (không có dấu), và hệ thống nhận biết đây là từ tiếng Anh phổ biến.

---

### ❓ Làm sao biết từ nào có dấu, từ nào không?

**Trả lời:** Rất đơn giản - nhìn vào màn hình:
- Nếu bạn thấy dấu thanh (´ ` ? ~ .) → Từ có dấu
- Nếu bạn thấy dấu mũ (^) hoặc trăng (˘) → Từ có dấu
- Nếu bạn thấy chữ đ → Từ có dấu
- Nếu chỉ thấy chữ cái thường → Từ không dấu

---

### ❓ Tôi có thể tắt tính năng này không?

**Trả lời:** Tính năng này là cải tiến để bảo vệ từ tiếng Việt của bạn, không nên tắt. Tuy nhiên, nếu bạn thực sự cần, có thể:
- Tắt bộ gõ khi gõ tiếng Anh thuần túy
- Sử dụng chế độ Raw Mode (nếu có trong cài đặt)

---

## Lợi Ích

### ✅ Cho Người Dùng Việt Nam

1. **Tôn trọng từ tiếng Việt:**
   - Không còn bị "sửa" sang tiếng Anh ngoài ý muốn
   - Từ như "tét", "bét", "rét" được giữ nguyên

2. **Gõ song ngữ tự nhiên hơn:**
   - Gõ tiếng Việt → Giữ nguyên
   - Gõ tiếng Anh → Tự động sửa (nếu không dấu)
   - Không cần suy nghĩ nhiều

3. **Ít lỗi hơn:**
   - Backspace và khôi phục hoạt động chính xác
   - Không bị lỗi ký tự bị nhân đôi

---

## Ví Dụ Thực Tế

### Tình huống 1: Viết status Facebook

```
Câu muốn viết: "Hôm nay đi test rồi, tét lắm!"

Gõ như sau:
1. "Hôm nay đi " → OK
2. "test" → Hiện "tét" (có dấu sắc)
3. Nhấn SPACE → Giữ "tét" (không đổi thành "test")
4. "rồi, " → OK
5. "test" → Hiện "tét" (có dấu sắc)
6. Nhấn SPACE → Giữ "tét"
7. "lắm!" → OK

Kết quả: "Hôm nay đi tét rồi, tét lắm!" ✅
```

---

### Tình huống 2: Viết email công việc

```
Câu muốn viết: "Please fix the bug before next release"

Gõ như sau:
1. "Please " → OK
2. "fix" → Hiện "fix" (không dấu)
3. Nhấn SPACE → Tự động thêm space, thành "fix " ✅
4. "the bug before " → OK
5. "next" → Hiện "nẽt" (có dấu ngã từ 'x')
6. Nhấn SPACE → Giữ "nẽt" (không đổi thành "next") ⚠️

Lưu ý: Nếu muốn gõ "next" (tiếng Anh), cần:
- Tắt bộ gõ tiếng Việt trước khi gõ email tiếng Anh
- Hoặc gõ "nexxt" rồi xóa một 'x'
```

**Khuyến nghị:** Tắt bộ gõ tiếng Việt khi viết văn bản tiếng Anh thuần túy.

---

## Các Từ Tiếng Việt Thường Gặp

Những từ này bây giờ sẽ được giữ nguyên (không bị sửa thành tiếng Anh):

| Bạn gõ | Kết quả | Từ tiếng Anh tương tự | Ghi chú |
|--------|---------|------------------------|---------|
| t-e-s-t | tét | test | Từ Việt hợp lệ |
| t-e-x-t | tẽt | text | Từ Việt hợp lệ |
| b-e-s-t | bét | best | Từ Việt hợp lệ |
| r-e-s-t | rét | rest | Từ Việt hợp lệ |
| n-e-x-t | nẽt | next | Từ Việt hợp lệ |
| m-i-x | mĩ | mix | Từ Việt hợp lệ |

---

## Kết Luận

Hệ thống GoxViet IME bây giờ **THÔNG MINH HƠN** trong việc phân biệt ý định gõ tiếng Việt và tiếng Anh:

- ✅ **Có dấu thanh** → Giữ nguyên tiếng Việt
- ✅ **Không dấu thanh** → Tự động sửa tiếng Anh (nếu cần)
- ✅ **Backspace** → Hoạt động chính xác

---

## Hỗ Trợ

Nếu bạn gặp vấn đề hoặc có câu hỏi:

1. Đọc tài liệu: `docs/AUTO_SPACE_FEATURE_VI.md`
2. Xem hướng dẫn kỹ thuật: `docs/FIX_VIETNAMESE_WORD_PRESERVATION_2025-12-22.md`
3. Báo lỗi trên GitHub: [Issues](https://github.com/yourusername/goxviet/issues)

---

**Cảm ơn bạn đã sử dụng GoxViet IME!**

---

**Phiên bản tài liệu:** 1.0  
**Ngày cập nhật:** 22/12/2025  
**Người viết:** GoxViet Development Team