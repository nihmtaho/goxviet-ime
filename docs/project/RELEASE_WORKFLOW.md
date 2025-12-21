# Gõ Việt (GoxViet) - Release Workflow Documentation

**Created:** 2025-12-21  
**Last Updated:** 2025-12-21  
**Version:** 1.0.0

---

## 📋 Table of Contents

1. [Overview](#overview)
2. [Workflow Files](#workflow-files)
3. [Release Process](#release-process)
4. [Manual Release](#manual-release)
5. [Automatic Release](#automatic-release)
6. [Troubleshooting](#troubleshooting)

---

## 1. Overview

Project **Gõ Việt (GoxViet)** sử dụng GitHub Actions để tự động hóa quy trình release, bao gồm:

- ✅ Tự động tạo version tags
- ✅ Build Rust core library
- ✅ Build macOS app (Xcode)
- ✅ Tạo file DMG installer
- ✅ Tạo GitHub Release
- ✅ Upload assets (DMG) lên release

### Architecture

```
VERSION file update → Auto Tag Workflow → Release Workflow → GitHub Release
     (main branch)         (create tag)      (build & upload)    (with DMG)
```

---

## 2. Workflow Files

### 2.1. Release Workflow (`.github/workflows/release.yml`)

**Trigger:** Khi push tag có format `v*.*.*` (ví dụ: `v1.0.0`, `v1.2.3`)

**Chức năng:**
- Build Rust core library (`core/`)
- Build macOS app từ Xcode project
- Tạo DMG installer
- Tạo GitHub Release với release notes
- Upload DMG lên release assets

**Environment Variables:**
```yaml
APP_NAME: GoxViet
BUNDLE_ID: com.goxviet.ime
RUST_BACKTRACE: 1
```

### 2.2. Auto Tag Workflow (`.github/workflows/auto-tag.yml`)

**Trigger:** Khi push lên `main` branch và có thay đổi trong:
- `VERSION` file
- `core/Cargo.toml`
- `platforms/macos/goxviet/goxviet/Info.plist`

**Chức năng:**
- Đọc version từ file `VERSION`
- Kiểm tra xem tag đã tồn tại chưa
- Tạo annotated tag mới nếu chưa tồn tại
- Push tag lên GitHub (trigger release workflow)

---

## 3. Release Process

### 3.1. Prerequisites

Đảm bảo các yêu cầu sau đã được thỏa mãn:

1. **Rust toolchain** đã được cài đặt và cấu hình
2. **Xcode** với Command Line Tools
3. **DMG creation script** tại `scripts/create_dmg.sh`
4. **GitHub Token** có quyền tạo release (mặc định `GITHUB_TOKEN` được cung cấp bởi Actions)

### 3.2. Version Management

Version được quản lý bằng file `VERSION` ở root directory:

```
goxviet/VERSION
```

Format: `x.y.z` (Semantic Versioning)
- `x` = Major version (breaking changes)
- `y` = Minor version (new features)
- `z` = Patch version (bug fixes)

**Ví dụ:**
```
0.1.0  # Initial release
0.2.0  # New feature added
0.2.1  # Bug fix
1.0.0  # First stable release
```

---

## 4. Manual Release

### Step 1: Update Version

Cập nhật version trong file `VERSION`:

```bash
echo "1.0.0" > VERSION
```

### Step 2: Update Changelog

Cập nhật file `CHANGELOG.md` với các thay đổi mới:

```markdown
## [1.0.0] - 2025-12-21

### Added
- New feature X
- New feature Y

### Fixed
- Bug fix Z

### Changed
- Improvement A
```

### Step 3: Commit and Push

```bash
git add VERSION CHANGELOG.md
git commit -m "chore: bump version to 1.0.0"
git push origin main
```

### Step 4: Create Tag Manually (Optional)

Nếu không muốn dùng auto-tag workflow:

```bash
git tag -a v1.0.0 -m "GoxViet 1.0.0"
git push origin v1.0.0
```

### Step 5: Wait for Workflow

GitHub Actions sẽ tự động:
1. Build app
2. Tạo DMG
3. Tạo release
4. Upload DMG

Kiểm tra tiến trình tại: `https://github.com/YOUR_USERNAME/goxviet/actions`

---

## 5. Automatic Release

### Workflow tự động hoàn toàn:

1. **Developer:** Update file `VERSION` và push lên `main`
   ```bash
   echo "1.0.0" > VERSION
   git add VERSION
   git commit -m "chore: bump version to 1.0.0"
   git push origin main
   ```

2. **Auto Tag Workflow:** Tự động tạo tag `v1.0.0` và push

3. **Release Workflow:** Tự động trigger khi phát hiện tag mới
   - Build Rust core
   - Build macOS app
   - Create DMG
   - Create GitHub Release
   - Upload assets

4. **Result:** Release sẵn sàng tại GitHub Releases page

### Kiểm tra kết quả:

```bash
# Check GitHub Release
https://github.com/YOUR_USERNAME/goxviet/releases/latest

# Download DMG
https://github.com/YOUR_USERNAME/goxviet/releases/download/v1.0.0/GoxViet.dmg
```

---

## 6. Troubleshooting

### 6.1. Workflow Fails on Rust Build

**Error:** `cargo build failed`

**Solution:**
```bash
# Test locally first
cd core
cargo build --release
cargo test --release
```

Đảm bảo tất cả tests pass trước khi push tag.

### 6.2. Workflow Fails on Xcode Build

**Error:** `xcodebuild failed`

**Solution:**
```bash
# Test locally
cd platforms/macos/goxviet
xcodebuild -project goxviet.xcodeproj -scheme goxviet -configuration Release clean build
```

Kiểm tra:
- Bridging header đúng path
- Library search paths
- Code signing settings

### 6.3. DMG Creation Fails

**Error:** `DMG creation failed`

**Solution:**
```bash
# Make sure script is executable
chmod +x scripts/create_dmg.sh

# Test locally
./scripts/create_dmg.sh
```

Đảm bảo:
- App được build thành công
- Script có quyền executable
- Có đủ disk space

### 6.4. Tag Already Exists

**Error:** `tag already exists`

**Solution:**

Nếu cần recreate tag:
```bash
# Delete local tag
git tag -d v1.0.0

# Delete remote tag
git push origin :refs/tags/v1.0.0

# Create new tag
git tag -a v1.0.0 -m "GoxViet 1.0.0"
git push origin v1.0.0
```

### 6.5. Release Not Created

**Error:** `failed to create release`

**Solution:**

Kiểm tra:
- `GITHUB_TOKEN` có quyền tạo release
- Repository settings → Actions → Workflow permissions
- Đặt thành "Read and write permissions"

### 6.6. Upload Asset Failed

**Error:** `failed to upload asset`

**Solution:**

Kiểm tra:
- DMG file tồn tại tại `dist/GoxViet.dmg`
- File size không vượt quá giới hạn GitHub (2GB)
- Network connection stable

---

## 7. Best Practices

### 7.1. Version Numbering

- **Patch (0.0.x):** Bug fixes, minor improvements
- **Minor (0.x.0):** New features, backwards compatible
- **Major (x.0.0):** Breaking changes, major rewrites

### 7.2. Release Frequency

- **Patch releases:** Weekly or as needed for critical bugs
- **Minor releases:** Monthly or when significant features are ready
- **Major releases:** Quarterly or when breaking changes are necessary

### 7.3. Pre-release Testing

Trước khi release:
1. ✅ Run full test suite: `cargo test --release`
2. ✅ Build locally: `xcodebuild clean build`
3. ✅ Test DMG creation: `./scripts/create_dmg.sh`
4. ✅ Manual testing on clean macOS install
5. ✅ Update CHANGELOG.md
6. ✅ Update documentation if needed

### 7.4. Release Notes Quality

Release notes nên bao gồm:
- 📦 Installation instructions
- ✨ What's new (features)
- 🐛 Bug fixes
- 🔧 Changes/improvements
- ⚠️ Breaking changes (nếu có)
- 🔗 Links to full changelog

---

## 8. Manual Intervention Points

Workflow có thể cần can thiệp thủ công trong các trường hợp:

### 8.1. Draft Release

Để tạo draft release (review trước khi publish):

Edit `.github/workflows/release.yml`:
```yaml
- name: Create GitHub Release
  uses: softprops/action-gh-release@v1
  with:
    draft: true  # Change to true
    prerelease: false
```

### 8.2. Pre-release

Để đánh dấu là pre-release (alpha, beta, rc):

```yaml
- name: Create GitHub Release
  uses: softprops/action-gh-release@v1
  with:
    draft: false
    prerelease: true  # Change to true
```

Tag format: `v1.0.0-beta.1`, `v1.0.0-rc.1`

### 8.3. Custom Release Notes

Để tùy chỉnh release notes, edit phần generate release notes trong workflow.

---

## 9. Monitoring & Notifications

### 9.1. GitHub Actions Dashboard

Monitor workflow tại:
```
https://github.com/YOUR_USERNAME/goxviet/actions
```

### 9.2. Email Notifications

GitHub tự động gửi email nếu workflow fails.

### 9.3. Workflow Status Badge

Thêm badge vào README.md:
```markdown
[![Release](https://github.com/YOUR_USERNAME/goxviet/actions/workflows/release.yml/badge.svg)](https://github.com/YOUR_USERNAME/goxviet/actions/workflows/release.yml)
```

---

## 10. Security Considerations

### 10.1. Code Signing (Future)

Hiện tại app chưa được code sign. Để add code signing:

1. Thêm Apple Developer certificate vào GitHub Secrets
2. Update Xcode build settings
3. Modify workflow để sign app trước khi tạo DMG

### 10.2. Notarization (Future)

Để app pass Gatekeeper trên macOS:

1. Sign app với Developer ID
2. Submit app cho Apple notarization
3. Staple notarization ticket vào DMG

### 10.3. Secrets Management

- ❌ KHÔNG commit secrets vào repo
- ✅ Sử dụng GitHub Secrets cho sensitive data
- ✅ Rotate secrets định kỳ

---

## 11. Related Documentation

- [Build Instructions](../getting-started/BUILD.md)
- [Distribution Guide](./DISTRIBUTION.md)
- [Changelog](../../CHANGELOG.md)
- [Contributing Guidelines](../../CONTRIBUTING.md)

---

## 12. Support

Nếu gặp vấn đề với release workflow:

1. Kiểm tra [GitHub Actions logs](https://github.com/YOUR_USERNAME/goxviet/actions)
2. Review [Troubleshooting section](#troubleshooting)
3. Open issue tại: https://github.com/YOUR_USERNAME/goxviet/issues

---

**Maintained by:** GoxViet Development Team  
**License:** See LICENSE file in root directory