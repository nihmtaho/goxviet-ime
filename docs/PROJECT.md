# PROJECT.md
# Gõ Việt (GoxViet) – TỔNG HỢP TÀI LIỆU QUẢN LÝ DỰ ÁN

**Cập nhật lần cuối:** 2025-12-23  
**Phiên bản tài liệu:** 1.0  
**Phạm vi:** Tổng quan dự án, roadmap, changelog, quy tắc commit, branding, checklist phát triển  
**Dành cho:** Quản lý dự án, lập trình viên, tester, contributor

---

## 📑 MỤC LỤC

1. [Giới thiệu & Tổng quan dự án](#giới-thiệu--tổng-quan-dự-án)
2. [Roadmap phát triển](#roadmap-phát-triển)
3. [Lịch sử thay đổi (Changelog)](#lịch-sử-thay-đổi-changelog)
4. [Quy tắc commit & workflow Git](#quy-tắc-commit--workflow-git)
5. [Branding & Rebranding](#branding--rebranding)
6. [Checklist phát triển & release](#checklist-phát-triển--release)
7. [Tài liệu liên quan](#tài-liệu-liên-quan)

---

## Giới thiệu & Tổng quan dự án

**Gõ Việt (GoxViet)** là bộ gõ tiếng Việt đa nền tảng, core bằng Rust, hướng tới:
- Độ trễ < 16ms (60fps)
- An toàn bộ nhớ tuyệt đối
- Trải nghiệm gõ native, tối ưu cho macOS & Windows
- Cấu trúc monorepo, dễ mở rộng, dễ bảo trì

**Kiến trúc chính:**
- `core/`: Engine xử lý tiếng Việt (Rust)
- `platforms/`: Triển khai cho từng hệ điều hành (macOS, Windows)
- `docs/`: Toàn bộ tài liệu kỹ thuật, hướng dẫn, tối ưu hóa, kiểm thử
- `test-performance.sh`: Script benchmark hiệu năng

---

## Roadmap phát triển

### 2024-2025

- [x] Xây dựng core Rust IME engine
- [x] Tích hợp macOS (Swift + Rust FFI)
- [x] Tối ưu backspace (latency < 3ms)
- [x] Hỗ trợ Telex, VNI, ESC restore, toggle shortcut
- [x] Tối ưu memory (zero leak, zero panic)
- [x] Tài liệu hóa toàn bộ quy trình, checklist kiểm thử
- [ ] Settings UI cho phép đổi shortcut, cấu hình input
- [ ] Tối ưu injection cho Safari/Chrome/Terminal
- [ ] Tích hợp Windows TSF (đang phát triển)
- [ ] Tự động hóa release (CI/CD, Homebrew, installer)
- [ ] Đa cấu hình profile (per-app, per-user)
- [ ] Tối ưu hóa cho accessibility & VoiceOver
- [ ] Đa dạng hóa test case, benchmark cross-platform

**Xem chi tiết:**  
- `RUST_CORE_ROADMAP.md` – Lộ trình phát triển core Rust  
- `PROJECT_STATUS.md` – Trạng thái hiện tại, các milestone

---

## Lịch sử thay đổi (Changelog)

### 2025-12-23
- Tổng hợp tài liệu, chuẩn hóa cấu trúc docs/
- Gom các nhóm tài liệu lớn: Getting Started, Shortcuts, Performance, Fixes, Project, Release Notes, Archive

### 2025-12-21
- Tối ưu hiệu năng backspace, latency < 3ms
- Fix Safari/Chrome address bar injection
- Cập nhật Homebrew deployment, bổ sung checklist kiểm thử

### 2025-11-30
- Tích hợp toggle shortcut (Control+Space), ưu tiên kernel-level
- Cập nhật UI menu bar, trạng thái IME

### 2025-10-15
- Refactor core engine, chuẩn hóa FFI, bổ sung unit test
- Đổi branding sang Gõ Việt (GoxViet), cập nhật toàn bộ log path, bundle id

### 2025-09-01
- Ra mắt bản thử nghiệm đầu tiên trên macOS

**Xem chi tiết:**  
- `CHANGELOG.md` – Lịch sử thay đổi chi tiết  
- `CHANGELOG_ACCESSIBILITY_API.md` – Lịch sử thay đổi liên quan accessibility

---

## Quy tắc commit & workflow Git

### Quy trình nhánh

- **main**: Mã nguồn ổn định, đã phát hành
- **develop**: Nhánh phát triển chính
- **feature/***: Nhánh tính năng mới (tách từ develop)
- **release/***: Nhánh chuẩn bị phát hành (tách từ develop)
- **hotfix/***: Nhánh sửa lỗi khẩn cấp (tách từ main)

### Quy tắc commit (Conventional Commits)

- Cú pháp: `<type>(scope): <subject>`
- type: `feat`, `fix`, `chore`, `refactor`, `perf`, `docs`, `test`, `build`, `ci`
- scope: core, macos, winui, ffi, v.v.
- subject: ngắn gọn, không viết hoa đầu câu, không dấu chấm cuối

**Ví dụ:**
- `feat(core): add smart uow transformation`
- `fix(macos): handle null text_ptr in bridge`
- `perf(core): reduce allocs in process_key`
- `docs(instructions): add interop strategy`

### Checklist trước khi mở Pull Request

- [ ] Đã chạy test liên quan (`cargo test`, unit test platform nếu có)
- [ ] Đã kiểm tra lint/format (`cargo fmt`, `cargo clippy`)
- [ ] Đã cập nhật doc/hướng dẫn nếu thay đổi public API/FFI
- [ ] Đã mô tả rõ behavior trước/sau, ảnh hưởng hiệu năng (nếu có)
- [ ] Tag reviewer phù hợp (core Rust, macOS Swift, Windows TSF/WinUI)
- [ ] Với thay đổi FFI, yêu cầu ít nhất một review từ người phụ trách core

**Xem chi tiết:**  
- `COMMIT_MESSAGE_TEMPLATE.md` – Mẫu commit  
- `.github/instructions/` – Quy tắc chi tiết

---

## Branding & Rebranding

### Quy tắc đặt tên & nhận diện

- Brand: **Gõ Việt**
- Display/App: **GoxViet**
- Code/Repo: **goxviet**
- Bundle ID: `com.goxviet.ime`
- Log path: `~/Library/Logs/GoxViet/`

**Không sử dụng:**  
- "GoNhanh", "gonhanh", "go-nhanh" hoặc bất kỳ tên liên quan đến project mẫu

### Checklist rebranding

- [x] Đổi toàn bộ tên file, biến, bundle id, log path
- [x] Cập nhật README, doc, hướng dẫn
- [x] Kiểm tra lại các file cấu hình, CI/CD, Homebrew tap
- [x] Đảm bảo không còn sót tên cũ trong mã nguồn, tài liệu

**Xem chi tiết:**  
- `BRANDING_UPDATE_SUMMARY.md` – Tổng hợp quá trình rebranding  
- `LOG_PATH_MIGRATION.md` – Lộ trình chuyển đổi log path

---

## Checklist phát triển & release

### Checklist phát triển

- [x] Đọc kỹ quy tắc dự án trong `.github/instructions/` và `copilot-instructions.md`
- [x] Đảm bảo tuân thủ cấu trúc monorepo, không tạo file ngoài quy định
- [x] Viết tài liệu, cập nhật mục lục khi thêm/chỉnh sửa file
- [x] Tham khảo code mẫu đúng cách, không copy tên/branding
- [x] Đảm bảo code không có memory leak, panic, hoặc lỗi FFI

### Checklist release

- [x] Đã chạy toàn bộ test (unit, integration, performance)
- [x] Đã kiểm tra hiệu năng (latency, memory, stress test)
- [x] Đã cập nhật changelog, release notes
- [x] Đã cập nhật doc, hướng dẫn sử dụng
- [x] Đã kiểm thử trên nhiều ứng dụng (VSCode, Terminal, Safari, Chrome...)
- [x] Đã kiểm tra lại branding, log path, bundle id
- [x] Đã tạo tag release, build installer, cập nhật Homebrew tap (nếu có)

**Xem chi tiết:**  
- `PROJECT_RESTRUCTURE_SUMMARY.md` – Tổng hợp các đợt refactor lớn  
- `RELEASE_AUTOMATION_SETUP.md` – Hướng dẫn tự động hóa release  
- `RELEASE_WORKFLOW.md` – Quy trình release chuẩn

---

## Tài liệu liên quan

- `PROJECT_STATUS.md` – Trạng thái dự án, milestone
- `RUST_CORE_ROADMAP.md` – Lộ trình phát triển core Rust
- `DOCUMENTATION_CATEGORY_SUMMARY.md` – Phân loại tài liệu
- `CHANGELOG.md` – Lịch sử thay đổi chi tiết
- `BRANDING_UPDATE_SUMMARY.md` – Tổng hợp rebranding
- `LOG_PATH_MIGRATION.md` – Lộ trình chuyển đổi log path
- `RELEASE_AUTOMATION_SETUP.md` – Tự động hóa release
- `RELEASE_WORKFLOW.md` – Quy trình release
- `.github/instructions/` – Quy tắc đóng góp, code style, review

---

**Gõ Việt – Dự án bộ gõ tiếng Việt hiện đại, tối ưu, mở rộng dễ dàng, hướng tới trải nghiệm native và hiệu năng vượt trội.**

---