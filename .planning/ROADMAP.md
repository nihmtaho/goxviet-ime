# GOXVIET ROADMAP

## 2026 ROADMAP – GÕ VIỆT (GOXVIET)

### 1. Core Engine (Rust)

- [x] **Text Expansion (Gõ tắt)** ✅ *Completed 2026-01-26*
	- Cho phép định nghĩa, import/export các cụm gõ tắt (vd: "tt" → "thân thiện")
	- Tích hợp vào core, đảm bảo hiệu suất <1ms, undo/redo chuẩn
	- Có thể bật/tắt theo từng ứng dụng (per-app)

- [x] **Shift+Backspace – Xóa nhanh từ** ✅ *Completed 2026-01-26*
	- Thêm phím tắt Shift+Backspace để xóa nhanh một từ
	- Đảm bảo hoạt động nhất quán, không gây lỗi buffer
	- Tối ưu hiệu suất thao tác xóa hàng loạt

- [x] **Multi-Encoding Output** ✅ *Completed 2026-01-26*
	- Hỗ trợ lựa chọn bảng mã đầu ra: Unicode, TCVN3, VNI Windows, CP1258
	- Cho phép chọn encoding trong settings, tương thích ngược Unicode
	- Tích hợp logic chuyển đổi ký tự theo từng bảng mã

- [x] **Unit Test & Benchmark** ✅ *Completed 2026-01-29*
	- 70% unit test coverage (NOTE: 8 tests failing)
	- Benchmark <1ms/keystroke

### 2. Platform Layer

- [ ] **macOS**
	- Cải tiến UI SwiftUI, tối ưu Settings, hỗ trợ Smart Mode per-app
	- Tối ưu RustBridge, đảm bảo memory safety, không leak
	- Đảm bảo đồng bộ trạng thái với core engine

- [ ] **Windows**
	- Tối ưu tích hợp TSF, hỗ trợ các bảng mã legacy
	- Đảm bảo tương thích với các phần mềm kế toán, văn phòng cũ

### 3. Chất lượng & Kiểm thử

- [ ] 70% Unit Test (core logic), 20% Integration Test (FFI), 10% E2E (UI)
- [ ] Benchmark <1ms/keystroke, reject nếu giảm hiệu suất >5%
- [ ] Đảm bảo không panic, không crash qua FFI

### 4. Tài liệu & Cộng đồng

- [ ] Cập nhật tài liệu chi tiết cho từng tính năng mới
- [ ] Hướng dẫn import/export gõ tắt, chuyển đổi encoding
- [ ] Đảm bảo mọi thay đổi đều có doc, checklist, review

---
**Lưu ý:**
Mọi milestone đều phải tuân thủ quy tắc kiến trúc, coding standards, và quy trình kiểm thử của dự án Gõ Việt (GoxViet).

**Progress: Phase 1 Core Engine 75% complete (3/4 milestones)**
