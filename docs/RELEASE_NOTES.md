# RELEASE_NOTES.md
# Gõ Việt (GoxViet) – TỔNG HỢP GHI CHÚ PHÁT HÀNH (RELEASE NOTES)

**Cập nhật lần cuối:** 2025-12-23  
**Phiên bản tài liệu:** 1.0  
**Phạm vi:** Lịch sử phát hành, thay đổi chính, hướng dẫn release, changelog  
**Dành cho:** Người dùng, lập trình viên, quản lý dự án

---

## 📑 MỤC LỤC

1. [Giới thiệu & Quy tắc phát hành](#giới-thiệu--quy-tắc-phát-hành)
2. [Lịch sử phát hành (Release History)](#lịch-sử-phát-hành-release-history)
    - Các bản phát hành chính (Major/Minor)
    - Các bản vá lỗi (Patch/Hotfix)
3. [Changelog tổng hợp](#changelog-tổng-hợp)
4. [Hướng dẫn quy trình release](#hướng-dẫn-quy-trình-release)
5. [Automation & Checklist](#automation--checklist)
6. [Tài liệu liên quan](#tài-liệu-liên-quan)

---

## Giới thiệu & Quy tắc phát hành

Gõ Việt (GoxViet) sử dụng quy trình phát hành chuẩn, đảm bảo:
- Mỗi bản phát hành đều có ghi chú rõ ràng, liệt kê thay đổi chính, lỗi đã sửa, cải tiến hiệu năng.
- Quy trình release tuân thủ Git Flow, có checklist kiểm thử, automation CI/CD, và hướng dẫn rollback.
- Changelog được cập nhật liên tục, giúp người dùng và dev dễ dàng theo dõi lịch sử thay đổi.

---

## Lịch sử phát hành (Release History)

### Phiên bản 1.3.1 – 2025-12-23
- **Loại:** Patch
- **Nội dung:**
  - Sửa lỗi nhỏ liên quan đến toggle shortcut trên macOS 14.3.
  - Cải thiện hiệu năng khi chuyển đổi nhanh giữa các ứng dụng.
  - Bổ sung kiểm thử tự động cho Safari address bar.

### Phiên bản 1.3.0 – 2025-12-20
- **Loại:** Minor
- **Nội dung:**
  - Thêm hỗ trợ macOS Sonoma.
  - Tối ưu hóa backspace cho VSCode, Zed, Terminal.
  - Nâng cấp UI menu bar, trạng thái toggle luôn hiển thị màu sắc chính xác.
  - Sửa lỗi memory leak nhỏ trong Rust core.
  - Cập nhật tài liệu hướng dẫn sử dụng nhanh.

### Phiên bản 1.2.3 – 2025-10-15
- **Loại:** Patch
- **Nội dung:**
  - Sửa lỗi không nhận diện phím tắt khi dùng nhiều bàn phím.
  - Cải thiện độ ổn định khi sleep/wake máy.

### Phiên bản 1.2.2 – 2025-09-01
- **Loại:** Patch
- **Nội dung:**
  - Sửa lỗi không gõ được dấu trong Safari address bar.
  - Bổ sung kiểm thử tự động cho Chrome, Safari.

### Phiên bản 1.2.0 – 2025-07-10
- **Loại:** Minor
- **Nội dung:**
  - Thêm tính năng cài đặt qua Homebrew (brew install --cask goxviet).
  - Tối ưu hóa hiệu năng backspace (<3ms).
  - Sửa lỗi không lưu trạng thái toggle khi chuyển app.
  - Cập nhật tài liệu hướng dẫn Homebrew.

### Phiên bản 1.0.1 – 2025-01-20
- **Loại:** Patch
- **Nội dung:**
  - Sửa lỗi không phản hồi phím trên một số máy.
  - Cải thiện kiểm thử tự động, bổ sung test case cho VSCode, Terminal.

---

## Changelog tổng hợp

### Tính năng nổi bật qua các phiên bản

- **Shortcut Toggle:** Control+Space ưu tiên cao nhất, không bị override bởi app khác.
- **Backspace thông minh:** O(1) cho ký tự thường, O(s) cho rebuild âm tiết, tối ưu cho VSCode, Safari, Terminal.
- **Cài đặt Homebrew:** Hỗ trợ cài đặt nhanh, không cần tài khoản Apple Developer.
- **UI Menu Bar:** Hiển thị trạng thái rõ ràng, toggle luôn giữ màu sắc chính xác, hỗ trợ dark mode.
- **Kiểm thử tự động:** Đầy đủ test cho các app phổ biến, stress test, memory leak test.
- **Tối ưu hiệu năng:** Độ trễ < 16ms, backspace < 3ms, không memory leak, không crash khi gõ nhanh.
- **Hỗ trợ đa nền tảng:** macOS 11+, tối ưu cho cả Intel và Apple Silicon.

### Lỗi đã sửa (Hotfix/Patch)

- Sửa lỗi không nhận phím tắt khi dùng nhiều bàn phím.
- Sửa lỗi không gõ được dấu trong Safari/Chrome address bar.
- Sửa lỗi không lưu trạng thái toggle khi chuyển app.
- Sửa lỗi memory leak nhỏ trong Rust core.
- Sửa lỗi không phản hồi phím trên một số máy.

---

## Hướng dẫn quy trình release

### 1. Quy trình chuẩn (theo Git Flow)

- Tạo nhánh `release/<version>` từ `develop`.
- Đảm bảo đã cập nhật changelog, tài liệu hướng dẫn, checklist kiểm thử.
- Merge vào `main` và `develop` sau khi kiểm thử xong.
- Tag version, tạo release note trên GitHub.
- Đảm bảo CI/CD build thành công, kiểm thử tự động pass 100%.

### 2. Checklist trước khi release

- [x] Đã cập nhật CHANGELOG.md, RELEASE_NOTES.md.
- [x] Đã kiểm thử trên các app phổ biến (TextEdit, VSCode, Terminal, Safari, Chrome).
- [x] Đã kiểm tra memory leak, stress test.
- [x] Đã cập nhật tài liệu hướng dẫn sử dụng.
- [x] Đã kiểm tra lại shortcut toggle, backspace, ESC restore.
- [x] Đã cập nhật Homebrew tap/cask nếu có thay đổi.

### 3. Automation

- Sử dụng script build DMG, tạo Homebrew cask tự động.
- CI/CD kiểm thử tự động trên macOS (GitHub Actions).
- Tự động gửi thông báo release qua Slack/Email nếu cấu hình.

### 4. Rollback (nếu cần)

- Nếu phát hiện lỗi nghiêm trọng sau khi release:
  - Tạo hotfix branch từ `main`.
  - Sửa lỗi, kiểm thử lại, merge vào `main` và `develop`.
  - Tag bản vá mới, cập nhật release note.

---

## Automation & Checklist

### Công cụ hỗ trợ release

- **Script build DMG:** `./scripts/build-dmg.sh <version>`
- **Script tạo Homebrew cask:** `./scripts/create-cask.sh <version> <dmg-url>`
- **Kiểm thử tự động:** `./test-performance.sh`
- **CI/CD:** GitHub Actions, kiểm thử trên macOS runner

### Checklist phát hành nhanh

- [x] Build thành công trên cả Intel & Apple Silicon
- [x] Test shortcut toggle, backspace, ESC restore
- [x] Test trên VSCode, Terminal, Safari, Chrome
- [x] Không memory leak, không crash khi gõ nhanh
- [x] Đã cập nhật changelog, release notes, tài liệu hướng dẫn
- [x] Đã cập nhật Homebrew tap/cask (nếu có)
- [x] Đã thông báo release cho user/dev team

---

## Tài liệu liên quan

- `project/CHANGELOG.md` – Lịch sử thay đổi chi tiết
- `project/RELEASE_AUTOMATION_SETUP.md` – Hướng dẫn tự động hóa release
- `project/RELEASE_WORKFLOW.md` – Quy trình phát hành chuẩn
- `project/CHANGELOG_ACCESSIBILITY_API.md` – Lịch sử thay đổi liên quan Accessibility
- `GETTING_STARTED.md` – Hướng dẫn cài đặt, build, test nhanh
- `FIXES.md` – Tổng hợp các lỗi đã sửa, troubleshooting
- `PROJECT.md` – Tổng quan dự án, roadmap

---

**Mọi thắc mắc về phát hành, vui lòng liên hệ team phát triển Gõ Việt (GoxViet).**  
**Chúc bạn trải nghiệm phiên bản mới mượt mà và ổn định! 🇻🇳**

---