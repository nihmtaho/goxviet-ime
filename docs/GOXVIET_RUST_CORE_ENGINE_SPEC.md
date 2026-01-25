# GOXVIET RUST CORE – ENGINE ARCHITECTURE & IMPLEMENTATION SPEC

---

## 1. TỔNG QUAN KIẾN TRÚC & TRIẾT LÝ

Gõ Việt (GoxViet) sử dụng kiến trúc **Finite State Machine (FSM)** cho xử lý tiếng Việt, kết hợp **từ điển whitelist/blacklist** để nhận diện tiếng Anh, đảm bảo hiệu năng, độ chính xác và khả năng mở rộng vượt trội.

---

## 2. THÀNH PHẦN CHÍNH & LUỒNG XỬ LÝ

### 2.1. Thành phần chính
- **FSM Core:** Phân tích, xác thực và biến đổi âm tiết tiếng Việt theo quy tắc ngữ âm, mapping Telex/VNI, đặt dấu, xử lý edge case.
- **Dictionary Layer:** Từ điển whitelist (từ tiếng Anh, tên riêng, từ kỹ thuật) và blacklist (từ mượn, false positive) để override hoặc chặn nhận diện tiếng Anh.
- **Phonotactic Classifier:** Rule-based nhận diện tiếng Việt/ngoại lai dựa trên cấu trúc âm tiết, kết hợp với từ điển để tăng độ chính xác.
- **Buffer & History:** Quản lý raw input, buffer syllable, undo/restore, auto-restore cho từ tiếng Anh.

### 2.2. Luồng xử lý
1. **Capture:** Lưu keystroke vào Raw Buffer.
2. **Analyze:** FSM phân tích buffer thành [Phụ âm đầu] – [Nguyên âm] – [Phụ âm cuối] – [Dấu thanh].
3. **Validate:** Kiểm tra hợp lệ tiếng Việt bằng FSM + phonotactic + tra từ điển blacklist.
4. **Detect English:** Nếu không hợp lệ, tra whitelist; nếu trùng, đánh dấu là tiếng Anh (bật auto-restore).
5. **Transform:** Áp dụng quy tắc Telex/VNI, đặt dấu, xử lý edge case.
6. **Render:** Trả về chuỗi hiển thị.

---

## 3. YÊU CẦU KỸ THUẬT RUST CORE

### 3.1. No Panic Policy
- Core tuyệt đối **KHÔNG ĐƯỢC PANIC** trong bất kỳ tình huống nào.
- Sử dụng `Result<T, E>` cho mọi hàm có khả năng lỗi.
- Tại biên giới FFI (`extern "C"`), sử dụng `std::panic::catch_unwind` để bắt panic nếu có, tránh làm crash ứng dụng chủ (Host Application).

### 3.2. State Management
- Sử dụng Pattern **State Machine** để quản lý việc bỏ dấu (Telex/VNI).
- Trạng thái không chỉ là chuỗi ký tự, mà phải bao gồm lịch sử các phím đã gõ (`Raw Buffer`) để hỗ trợ Undo/Backspace chính xác.
- Cấu trúc dữ liệu phải tách biệt rõ ràng giữa `Input State` (đang gõ) và `Committed State` (đã hoàn thành).

### 3.3. String Handling & Encoding
- **Internal:** Sử dụng `String` (UTF-8) của Rust cho mọi xử lý nội bộ.
- **Windows Interop:** Chuyển đổi sang UTF-16 (`Vec<u16>`) khi giao tiếp với Windows API. Lưu ý xử lý `Surrogate Pairs` nếu cần.
- **macOS Interop:** Chuyển đổi sang C-String (UTF-8, null-terminated) khi giao tiếp với Swift/Objective-C.

---

## 4. CƠ CHẾ TỪ ĐIỂN NHẬN DIỆN TIẾNG ANH

### 4.1. Whitelist
- Lưu các từ tiếng Anh, tên riêng, từ kỹ thuật, viết tắt phổ biến ("Windows", "GitHub", "API", "HTTP", ...).
- Ưu tiên override logic rule-based: Nếu buffer trùng whitelist, engine xử lý như tiếng Anh (không biến đổi, bật auto-restore).
- Cho phép user/dev bổ sung động.

### 4.2. Blacklist
- Lưu các từ mượn, từ ngoại lai nhưng cần xử lý như tiếng Việt, hoặc các false positive ("pin", "pizza", ...).
- Nếu buffer trùng blacklist, engine xử lý như tiếng Việt (bắt buộc qua FSM, không auto-restore).
- Cho phép user/dev bổ sung động.

### 4.3. Cấu trúc dữ liệu
- Trie hoặc HashSet để lookup nhanh, không ảnh hưởng hiệu năng.
- Từ điển lưu ở memory, có thể nạp từ file cấu hình.

---

## 5. FFI PATTERN (FOREIGN FUNCTION INTERFACE)

Khi viết hàm export cho Native, tuân thủ mẫu sau:

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn ime_process_key(engine_ptr: *mut Engine, key_code: u32) -> *mut c_char {
    // 1. Kiểm tra null pointer
    if engine_ptr.is_null() {
        return std::ptr::null_mut();
    }

    // 2. Reconstruct reference an toàn
    let engine = unsafe { &mut *engine_ptr };

    // 3. Xử lý logic (bọc trong catch_unwind để an toàn tuyệt đối)
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        engine.process(key_code)
    }));

    match result {
        Ok(processed_string) => {
            // 4. Trả về CString (dùng into_raw để chuyển quyền sở hữu cho phía gọi)
            CString::new(processed_string).unwrap_or_default().into_raw()
        },
        Err(_) => {
            // Log lỗi panic và trả về null hoặc chuỗi rỗng
            std::ptr::null_mut()
        }
    }
}
```

---

## 6. MEMORY MANAGEMENT RULES

1.  **Ownership:** Phía nào cấp phát (allocate), phía đó phải giải phóng (free).
    - Rust cấp phát chuỗi kết quả -> Rust phải cung cấp hàm `ime_free_string`.
    - Native App gọi `ime_create` -> Native App phải gọi `ime_destroy` khi đóng.
2.  **Lifetimes:** Không bao giờ trả về tham chiếu (`&str`) qua FFI. Luôn copy dữ liệu hoặc chuyển quyền sở hữu (`into_raw`).

---

## 7. TESTING STRATEGY & BENCHMARK

- **Unit Test:** Viết test kỹ cho các trường hợp gõ tiếng Việt phức tạp (ví dụ: `t` + `r` + `u` + `o` + `w` + `n` + `g` -> `trường`).
- **Integration Test:** Mô phỏng việc gọi FFI từ C để đảm bảo không leak bộ nhớ.
- Sử dụng `cargo test` thường xuyên.
- Viết test case cho các edge case: tên riêng, từ kỹ thuật, từ mượn, false positive.
- Benchmark lookup từ điển với buffer lớn.

---

## 8. MODULE HÓA & MỞ RỘNG

- Tách riêng module FSM, module Dictionary, module Phonotactic Classifier.
- API cho phép nạp từ điển động, reload không cần restart.
- Cho phép user thêm/sửa/xóa từ whitelist/blacklist qua UI hoặc file config.

---

## 9. ƯU TIÊN XỬ LÝ & QUY TRÌNH NHẬN DIỆN

1. Blacklist > Whitelist > Rule-based
2. Nếu buffer trùng blacklist, luôn xử lý tiếng Việt.
3. Nếu buffer trùng whitelist, luôn xử lý tiếng Anh.
4. Nếu không trùng, dùng FSM + classifier.

---

## 10. KẾT LUẬN

Kết hợp FSM với từ điển whitelist/blacklist giúp engine Gõ Việt đạt hiệu năng cao, nhận diện tiếng Anh chính xác, xử lý edge case tốt, dễ mở rộng và cá nhân hóa. Đây là hướng đi tối ưu cho IME hiện đại, đáp ứng mọi nhu cầu gõ tiếng Việt và tiếng Anh trên đa nền tảng.
