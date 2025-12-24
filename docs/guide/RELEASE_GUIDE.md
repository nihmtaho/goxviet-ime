# Hướng dẫn Release GoxViet

Tài liệu này mô tả quy trình phát hành phiên bản mới của GoxViet.

---

## 📋 Tổng quan quy trình

1. **Chuẩn bị code** - Đảm bảo code đã sẵn sàng trên nhánh `main`
2. **Viết Release Note** - Tạo file release note cho phiên bản mới
3. **Tạo Tag** - Push tag để trigger workflow tự động
4. **Kiểm tra Release** - Xác nhận release đã được tạo thành công

---

## 🔧 Chuẩn bị trước khi Release

### 1. Đảm bảo code đã merge vào `main`

```bash
# Checkout nhánh main
git checkout main

# Pull code mới nhất
git pull origin main

# Kiểm tra trạng thái
git status
```

### 2. Chạy test và build local

```bash
# Build Rust core
cd core
cargo build --release
cargo test

# Build macOS app (nếu cần test local)
cd ../platforms/macos/goxviet
xcodebuild -scheme goxviet -configuration Release build
```

### 3. Cập nhật version trong các file (nếu cần)

- `core/Cargo.toml` - Rust package version
- `platforms/macos/goxviet/goxviet/Info.plist` - macOS app version

---

## 📝 Viết Release Note

### Tạo file Release Note

Tạo file mới trong `docs/release-note/` với format:

```
docs/release-note/RELEASE_NOTE_X.Y.Z.md
```

Ví dụ: `RELEASE_NOTE_1.4.0.md`

### Sử dụng Template

Copy template từ `docs/release-note/TEMPLATE.md` và điền thông tin:

```bash
cp docs/release-note/TEMPLATE.md docs/release-note/RELEASE_NOTE_1.4.0.md
```

### Nội dung cần có

1. **Tổng quan** - Mục tiêu chính của phiên bản
2. **Tính năng mới** - Liệt kê các feature mới
3. **Sửa lỗi** - Các bug đã fix
4. **Cải tiến** - Optimization, refactor
5. **Breaking Changes** - Thay đổi không tương thích (nếu có)

### Commit Release Note

```bash
git add docs/release-note/RELEASE_NOTE_1.4.0.md
git commit -m "docs(release): add release note for v1.4.0"
git push origin main
```

---

## 🏷️ Tạo Tag để Trigger Release

### Format Tag

Tag phải theo format: `vX.Y.Z`

- `X` - Major version (breaking changes)
- `Y` - Minor version (new features)
- `Z` - Patch version (bug fixes)

### Các loại tag đặc biệt

- `v1.4.0` - Release chính thức
- `v1.4.0-alpha` - Alpha release (prerelease)
- `v1.4.0-beta` - Beta release (prerelease)
- `v1.4.0-rc.1` - Release candidate (prerelease)

### Tạo và Push Tag

```bash
# Tạo annotated tag với message
git tag -a v1.4.0 -m "Release version 1.4.0"

# Push tag lên GitHub
git push origin v1.4.0
```

### Tạo Tag với message chi tiết

```bash
git tag -a v1.4.0 -m "Release version 1.4.0

- Add new feature X
- Fix bug Y
- Improve performance Z"
```

---

## 🤖 Workflow tự động

Khi push tag, GitHub Actions sẽ tự động:

1. **Checkout code** từ nhánh `main`
2. **Build Rust core** - Universal binary (arm64 + x86_64)
3. **Build macOS app** - Unsigned release build
4. **Tạo DMG** - File cài đặt cho macOS
5. **Đọc Release Note** - Từ `docs/release-note/RELEASE_NOTE_X.Y.Z.md`
6. **Tạo GitHub Release** - Upload DMG và release note

### Theo dõi workflow

1. Vào tab **Actions** trên GitHub repository
2. Chọn workflow **Release**
3. Xem log của từng step

---

## ✅ Kiểm tra sau Release

### 1. Kiểm tra GitHub Release

- Vào tab **Releases** trên GitHub
- Xác nhận release đã được tạo với đúng version
- Kiểm tra DMG file đã được upload
- Kiểm tra release note hiển thị đúng

### 2. Test DMG trên máy sạch

```bash
# Download DMG
curl -LO https://github.com/YOUR_REPO/releases/download/v1.4.0/GoxViet-1.4.0-unsigned.dmg

# Mount và kiểm tra
hdiutil attach GoxViet-1.4.0-unsigned.dmg

# Test cài đặt
cp -R "/Volumes/GoxViet/GoxViet.app" /Applications/

# Unmount
hdiutil detach "/Volumes/GoxViet"
```

### 3. Verify checksum (optional)

```bash
shasum -a 256 GoxViet-1.4.0-unsigned.dmg
```

---

## 🔄 Rollback Release

### Xóa tag nếu release bị lỗi

```bash
# Xóa tag local
git tag -d v1.4.0

# Xóa tag trên remote
git push origin --delete tag v1.4.0
```

### Xóa GitHub Release

1. Vào tab **Releases**
2. Click vào release cần xóa
3. Click **Delete release**

---

## 📊 Quy ước Versioning

GoxViet sử dụng [Semantic Versioning](https://semver.org/):

| Thay đổi | Version bump | Ví dụ |
|----------|--------------|-------|
| Breaking change | Major | 1.0.0 → 2.0.0 |
| New feature | Minor | 1.0.0 → 1.1.0 |
| Bug fix | Patch | 1.0.0 → 1.0.1 |

---

## 🛠️ Troubleshooting

### Workflow thất bại

1. Kiểm tra log trong GitHub Actions
2. Các lỗi phổ biến:
   - **Xcode version mismatch** - Workflow sử dụng `macos-13` + Xcode 15.2
   - **Rust build failed** - Kiểm tra `Cargo.toml` và dependencies
   - **DMG creation failed** - Kiểm tra app path và permissions

### Release Note không hiển thị

- Đảm bảo file đặt đúng vị trí: `docs/release-note/RELEASE_NOTE_X.Y.Z.md`
- Tên file phải khớp với version trong tag (không có `v` prefix)
- File phải được commit và push **trước khi** tạo tag

### DMG không được upload

- Kiểm tra step "Create unsigned DMG" trong workflow log
- Đảm bảo app build thành công trước đó
- Kiểm tra disk space trên runner

---

## 📚 Tham khảo

- [GitHub Actions Workflow](.github/workflows/release.yml)
- [Release Note Template](../release-note/TEMPLATE.md)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)

---

**Gõ Việt (GoxViet) – Bộ gõ tiếng Việt hiệu suất cao!**