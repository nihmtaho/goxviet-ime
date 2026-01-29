# DỰ ÁN GÕ VIỆT (GOXVIET) – PROJECT OVERVIEW

## 1. TỔNG QUAN
Gõ Việt (GoxViet) là bộ gõ tiếng Việt đa nền tảng, hiệu suất cao, hướng tới trải nghiệm native trên macOS và Windows. Dự án sử dụng kiến trúc hybrid: core engine viết bằng Rust, platform layer native (Swift cho macOS, C++/C# cho Windows).

## 2. MỤC TIÊU DỰ ÁN
- Độ trễ xử lý < 3ms (core), < 16ms (end-to-end)
- An toàn bộ nhớ tuyệt đối, không panic qua FFI
- Trải nghiệm người dùng native, đồng nhất trên mọi nền tảng
- Hỗ trợ logic tiếng Việt chuẩn, undo/backspace thông minh, smart English/Việt detection
- Dễ mở rộng, bảo trì, tuân thủ chuẩn mã nguồn mở

## 3. PHẠM VI DỰ ÁN
- Core engine: Xử lý logic tiếng Việt, buffer, transform, validation, FFI
- Platform macOS: Tích hợp hệ điều hành, UI SwiftUI, quản lý lifecycle, settings
- Platform Windows: Tích hợp TSF, UI, quản lý lifecycle
- Tài liệu: Đầy đủ, rõ ràng, versioned, tuân thủ cấu trúc docs/

## 4. KIẾN TRÚC HỆ THỐNG
- Hybrid: Core (Rust) + Platform (Native)
- Dual engine: engine (legacy, ổn định), engine_v2 (modular, hiện đại)
- Data flow: Keystroke → Platform → FFI → Core → Buffer/Transform → Result
- Strict buffer/state management, không patch trực tiếp chuỗi hiển thị

## 5. QUY TẮC PHÁT TRIỂN & CHẤT LƯỢNG
- Không dùng tên/từ ngữ từ project mẫu, không chỉnh sửa project mẫu
- Không tạo file ngoài cấu trúc, không panic trong FFI
- Sử dụng đúng thương hiệu, bundle, log path
- Viết lại thuật toán với style riêng, ghi credit nếu tham khảo
- Viết test trước khi sửa bug, mọi thao tác gõ < 16ms
- Tuân thủ quy trình Git: branch, commit, PR, review, squash & merge
- Coding standards: Không heap alloc hot path, không unwrap panic, luôn kiểm tra pointer FFI
- Đảm bảo 70% unit test, 20% integration, 10% E2E, benchmark < 1ms/keystroke

## 6. ĐỊNH DANH & THƯƠNG HIỆU
- Brand: Gõ Việt
- App: GoxViet
- Bundle ID: com.goxviet.ime
- Log path: ~/Library/Logs/GoxViet/

## 7. TÀI LIỆU THAM KHẢO
- .github/instructions/ (master rules, workflow, logic tiếng Việt, git workflow)
- .docs/features/core-engine/ (chi tiết engine)
- .docs/features/platform/macos/ (chi tiết macOS)

---
Mọi thay đổi, phát triển, tối ưu đều phải tuân thủ các quy tắc, kiến trúc và mục tiêu trên để đảm bảo chất lượng, hiệu năng và thương hiệu cho Gõ Việt (GoxViet).
