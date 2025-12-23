# BRANDING & MIGRATION HISTORY - GÕ VIỆT (GOXVIET)

**File này hợp nhất toàn bộ lịch sử đổi tên, chuyển branding, và migration log path của dự án từ các tài liệu:**
- `BRANDING_UPDATE_SUMMARY.md`
- `REBRANDING_TO_GOXVIET.md`
- `LOG_PATH_MIGRATION.md`

> **Ghi chú:** Mỗi phần đều ghi rõ nguồn gốc, thời gian và trạng thái để tiện tra cứu lịch sử phát triển thương hiệu và cấu trúc dự án.

---

## 📋 MỤC LỤC

1. [Tóm tắt hợp nhất & chú thích](#tóm-tắt-hợp-nhất--chú-thích)
2. [Lịch sử cập nhật branding (BRANDING_UPDATE_SUMMARY.md)](#lịch-sử-cập-nhật-branding-branding_update_summarymd)
3. [Hướng dẫn đổi tên toàn diện (REBRANDING_TO_GOXVIET.md)](#hướng-dẫn-đổi-tên-toàn-diện-rebranding_to_goxvietmd)
4. [Lịch sử migration log path & cleanup (LOG_PATH_MIGRATION.md)](#lịch-sử-migration-log-path--cleanup-log_path_migrationmd)
5. [Tổng kết & liên kết tài liệu liên quan](#tổng-kết--liên-kết-tài-liệu-liên-quan)

---

## Tóm tắt hợp nhất & chú thích

- **Mục đích:** Lưu trữ tập trung toàn bộ quá trình đổi tên, chuyển branding, migration log path, và cleanup các tham chiếu cũ trong dự án Gõ Việt (GoxViet).
- **Nguồn gốc:** Nội dung từng phần được giữ nguyên bản, có bổ sung chú thích nguồn và ngày tháng.
- **Lợi ích:** Giúp tra cứu nhanh lịch sử phát triển thương hiệu, kiểm tra lại các bước migration, và đảm bảo tính nhất quán khi phát triển hoặc audit dự án.

---

## Lịch sử cập nhật branding (BRANDING_UPDATE_SUMMARY.md)

**Date:** December 21, 2025  
**Status:** ✅ COMPLETED  
**Migration:** Complete project rebranding

### Summary

Successfully updated all project documentation and configuration files to reflect the new **Gõ Việt (GoxViet)** branding. The project now has consistent naming across all files and documentation.

### Branding Convention

#### Official Names

| Context            | Name         | Usage                                  |
|--------------------|--------------|----------------------------------------|
| **Brand Name**     | Gõ Việt      | Marketing, user-facing materials       |
| **Display/App Name** | GoxViet    | Application name, menu bar, about dialog |
| **Code/Repository** | goxviet     | Directory names, repository, code identifiers |
| **Library**        | libgoxviet_core.a | Rust static library               |
| **Bundle ID**      | com.goxviet.ime | macOS bundle identifier             |
| **Log Directory**  | ~/Library/Logs/GoxViet/ | Runtime logs                  |

#### Naming Examples

```
✅ Brand:        "Gõ Việt - Vietnamese IME for macOS"
✅ App Display:  "GoxViet" (menu bar, dock)
✅ Directory:    goxviet/platforms/macos/goxviet/
✅ Xcode Target: goxviet
✅ Git Repo:     github.com/user/goxviet
```

### Files Updated

- `.github/copilot-instructions.md`: Đổi tên, cập nhật cấu trúc, bundle ID, log path, ví dụ.
- `README.md`: Đổi tiêu đề, cập nhật đường dẫn, lệnh build, branding.
- `CHANGELOG.md` (root): Đổi tiêu đề.
- `docs/project/CHANGELOG.md`: Cập nhật mô tả, link GitHub.
- `.github/instructions/00_master_rules.md`, `03_macos_swift.md`, `07_interop_strategy.md`: Cập nhật header, log path, tên file, ví dụ.

### Verification Checklist

- [x] Tất cả đường dẫn, tên hiển thị, bundle ID, log path đã đồng bộ "goxviet"/"GoxViet"/"Gõ Việt".
- [x] Không còn tham chiếu tên cũ trong code và tài liệu.

### Branding Guidelines

- **"Gõ Việt"**: Dùng cho marketing, tài liệu hướng người dùng, dialog, social media.
- **"GoxViet"**: Dùng cho tên app, menu bar, DMG, App Store.
- **"goxviet"**: Dùng cho tên thư mục, repo, biến code, file.

### Impact Assessment

- ✅ Không phá vỡ tương thích, không ảnh hưởng build/process.
- ✅ Tăng tính nhất quán, chuyên nghiệp, dễ nhận diện.
- ✅ Thân thiện hơn với người dùng Việt và quốc tế.

### Future Considerations

- [ ] Thiết kế logo, icon mới.
- [ ] Cập nhật website, tài liệu hướng dẫn, screenshot, video.
- [ ] Đổi tên DMG, metadata App Store, release notes.

---

## Hướng dẫn đổi tên toàn diện (REBRANDING_TO_GOXVIET.md)

**Date:** 2025-12-21  
**Status:** ✅ Partially Complete - Xcode Project Update Required  
**New Brand:** Gõ Việt (GoxViet)

### Summary of Changes

#### New Branding

- **Brand Name:** Gõ Việt
- **Display Name:** GoxViet
- **Repository:** goxviet
- **Xcode Project:** goxviet
- **Bundle ID:** com.goxviet.ime
- **Log Path:** ~/Library/Logs/GoxViet/

### Completed Changes

#### 1. File System & Directories

- Đã đổi tên root directory, Xcode project, entitlements, target, scheme, Info.plist, Rust core package.

#### 2. Swift Source Code

- Đã cập nhật toàn bộ header, log message, UserDefaults key, tooltip, alert, version, log path, description, v.v. sang branding mới.

#### 3. Rust Core

- Đã cập nhật tên package, library, authors, description.

### Remaining Tasks - CRITICAL

- **Xcode Project Configuration:**  
  - Mở project, đổi tên display, bundle ID, version, build, entitlements, target, scheme, Info.plist.
- **Rebuild Rust Core:**  
  - `cargo clean && cargo build --release`
- **Update Xcode Library Reference:**  
  - Thay thế library cũ bằng `libgoxviet_core.a` hoặc `.dylib`.
- **Update Build Settings:**  
  - Library search path, linker flags.
- **Clean & Rebuild:**  
  - Xcode hoặc command line.

### Documentation Updates Needed

- Tìm và thay thế toàn bộ tham chiếu tên cũ trong tài liệu, README, CHANGELOG, .github, scripts, CI/CD.

### Testing Checklist

- Build thành công, app chạy đúng, menu bar icon, about dialog, version, bundle ID, log path, UserDefaults, shortcut, per-app mode, v.v.

### Git Commit & Push

```bash
git add .
git commit -m "Rebrand to Gõ Việt (GoxViet)
- Rename project from VietnameseIMEFast to goxviet
- Update bundle ID to com.goxviet.ime
- Update all branding: Gõ Việt / GoxViet
- Update Rust core package name to goxviet-core
- Update log path to ~/Library/Logs/GoxViet/
- Version bump to 1.0.2
- All Swift files updated with new branding
- Xcode project and targets renamed"
git push origin main
```

### Summary of Naming Conventions

| Context         | Name              | Example                        |
|-----------------|-------------------|--------------------------------|
| Brand Name      | Gõ Việt           | "Gõ Việt - Vietnamese IME"     |
| Display Name    | GoxViet           | App name, menu bar             |
| Code/Technical  | goxviet           | File, variable, function names |
| Bundle ID       | com.goxviet.ime   | Reverse domain                 |
| Rust Package    | goxviet-core      | Cargo package                  |
| Rust Library    | goxviet_core      | Rust crate                     |
| UserDefaults    | com.goxviet.ime.* | Settings keys                  |
| Log Path        | GoxViet           | ~/Library/Logs/GoxViet/        |
| Git Repo        | goxviet           | github.com/username/goxviet    |

### Priority Order

1. **CRITICAL:** Xcode config, rebuild Rust core, update library, test build.
2. **HIGH:** Update docs, README, CHANGELOG, project rules.
3. **MEDIUM:** Update scripts, automation, icon/assets.
4. **LOW:** Release notes, contribution guidelines.

### Common Issues & Solutions

- **Build fails:** Rebuild Rust core, update search path, linker flags.
- **App crash:** Kiểm tra bundle ID, Info.plist, entitlements, certificate.
- **UserDefaults migration:** Thêm code migrate key cũ sang mới.
- **Logs không xuất hiện:** Tạo thư mục log thủ công.

### Final Verification

- [ ] Tất cả file đã đổi tên, cập nhật
- [ ] Build thành công
- [ ] App chạy đúng, branding mới
- [ ] Không còn tham chiếu tên cũ
- [ ] Tài liệu cập nhật
- [ ] Đã commit, push, update README

---

## Lịch sử migration log path & cleanup (LOG_PATH_MIGRATION.md)

**Date:** December 21, 2025  
**Status:** ✅ COMPLETED  
**Migration:** `VietnameseIME` → `GoxViet`

### Summary

- Đã migrate toàn bộ log path, xóa tham chiếu cũ, đồng bộ branding **GoxViet** trong codebase.

### Changes Made

1. **Bridging Header Updated:**  
   - Đổi guard từ `VietnameseIME_Bridging_Header_h` → `GoxViet_Bridging_Header_h`
2. **Log Path Verified:**  
   - Đã dùng đúng path: `~/Library/Logs/GoxViet/keyboard.log`
3. **Legacy Files Archived:**  
   - Di chuyển toàn bộ tài liệu, test cũ vào `platforms/macos/goxviet/archive/`
4. **Old Log Directory Removed:**  
   - `rm -rf ~/Library/Logs/VietnameseIME`

### Verification Results

- **Code Audit:** Không còn tham chiếu "VietnameseIME" trong code active.
- **Log Directory Check:** Chỉ còn `GoxViet/` trong `~/Library/Logs/`.
- **Build Test:** Build thành công với branding mới.
- **Runtime Test:** App chạy đúng, log ra đúng path mới.

### File System Structure

**Trước migration:**
```
~/Library/Logs/
├── VietnameseIME/   ✗ OLD
│   └── keyboard.log
└── GoxViet/         ✓ NEW
    └── keyboard.log
```
**Sau migration:**
```
~/Library/Logs/
└── GoxViet/         ✓ ONLY
    └── keyboard.log
```

### References Updated

| Category        | Item             | Old Value                      | New Value                   | Status      |
|-----------------|------------------|-------------------------------|-----------------------------|-------------|
| Header Guard    | Bridging Header  | VietnameseIME_Bridging_Header_h| GoxViet_Bridging_Header_h   | ✓ Updated   |
| Log Directory   | Runtime logs     | ~/Library/Logs/VietnameseIME/  | ~/Library/Logs/GoxViet/     | ✓ Migrated  |
| Log Messages    | App output       | "VietnameseIME starting..."    | "GoxViet starting..."       | ✓ Updated   |
| Documentation   | Legacy docs      | In root                        | Archived                    | ✓ Moved     |
| Test Files      | Old tests        | In platforms/macos/            | Archived                    | ✓ Moved     |

### Migration Checklist

- [x] Update header guards
- [x] Verify log path
- [x] Remove old log directory
- [x] Archive legacy docs/tests
- [x] Audit code
- [x] Clean build test
- [x] Runtime verification

### Impact Assessment

- ✅ Không phá vỡ tương thích, backward compatible.
- ✅ Branding đồng nhất, codebase sạch sẽ, tách biệt rõ legacy.
- ✅ Không thay đổi chức năng, chỉ cập nhật tên/branding.

### Notes

- Legacy files vẫn lưu trong `archive/` để tham khảo.
- Không còn functional changes, chỉ đổi tên.
- Nếu user còn thư mục log cũ, sẽ bị orphaned (không ảnh hưởng app).

---

## Tổng kết & liên kết tài liệu liên quan

- **Tài liệu này là nguồn tham khảo duy nhất về lịch sử đổi tên, migration, cleanup branding của dự án Gõ Việt (GoxViet).**
- Khi cần kiểm tra lại quá trình migration, chỉ cần tra cứu file này.
- Các tài liệu liên quan:
  - `/docs/DOCUMENTATION_STRUCTURE.md` - Cấu trúc tài liệu hiện tại
  - `.github/copilot-instructions.md` - Quy tắc branding, cấu trúc, naming
  - `/platforms/macos/goxviet/archive/README.md` - Giải thích về legacy files

---

**Tổng hợp & hợp nhất bởi:**  
GoxViet Documentation Team  
**Ngày cập nhật:** 2025-12-21  
**Trạng thái:** ✅ Đã hoàn tất migration & branding