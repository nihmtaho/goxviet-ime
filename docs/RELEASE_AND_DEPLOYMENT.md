# 🚀 Hướng dấn Release & Deployment (GoxViet)

Tài liệu này hướng dẫn quy trình đóng gói, ký số (signing), công chứng (notarization) và triển khai GoxViet lên Homebrew.

---

## 🏗️ 1. Quy trình Release nhanh

Để thực hiện một bản release đầy đủ, sử dụng script điều phối chính:

```bash
# Cập nhật version (ví dụ: 1.5.2)
./scripts/bump_version.sh 1.5.2

# Chạy script release toàn diện (Build + DMG + Sign + Notarize + Tag)
./scripts/release.sh 1.5.2
```

---

## 📦 2. Các bước chi tiết (Manual Steps)

### A. Build Release Bundle
Script `build-release.sh` sẽ dọn dẹp cache, build Rust core ở chế độ release và tạo app bundle.
```bash
./scripts/build-release.sh 1.5.2
```

### B. Tạo file DMG
Script `create-dmg.sh` tạo file cài đặt DMG có tích hợp link Applications.
```bash
./scripts/create-dmg.sh 1.5.2
```
*Output:* `platforms/macos/goxviet/dist/GoxViet-1.5.2.dmg`

### C. Ký số & Công chứng (Signing & Notarization)
Đây là bước bắt buộc để tránh cảnh báo "App is damaged" hoặc "unidentified developer" trên macOS.

1.  **Ký số (Codesign):**
    ```bash
    codesign --sign "Developer ID Application: Your Name (TEAM_ID)" \
             --timestamp \
             --options runtime \
             platforms/macos/goxviet/dist/GoxViet-1.5.2.dmg
    ```
2.  **Công chứng (Notarize):**
    ```bash
    ./scripts/notarize.sh platforms/macos/goxviet/dist/GoxViet-1.5.2.dmg
    ```
    *Lưu ý:* Bạn cần cấu hình `notary-profile` trước đó bằng `xcrun notarytool store-credentials`.

---

## 🍺 3. Triển khai lên Homebrew

GoxViet hỗ trợ cài đặt qua Homebrew Cask thông qua một Custom Tap.

### Cập nhật Cask mới
Khi có bản release mới trên GitHub, hãy cập nhật Cask:

1.  **Tạo Cask file:**
    ```bash
    ./scripts/create-homebrew-cask.sh 1.5.2 https://github.com/nihmtaho/goxviet/releases/download/v1.5.2/GoxViet-1.5.2.dmg
    ```
2.  **Đẩy lên Tap repository:**
    ```bash
    cd ../homebrew-goxviet
    cp ../goxviet/homebrew/goxviet.rb Casks/
    git add Casks/goxviet.rb
    git commit -m "Update GoxViet to v1.5.2"
    git push
    ```

### Hướng dẫn cài đặt cho người dùng
```bash
brew tap nihmtaho/goxviet
brew install --cask goxviet

# Vượt rào Gatekeeper nếu app chưa được sign
xattr -cr /Applications/GoxViet.app
```

---

## 🛠️ 4. Xử lý lỗi thường gặp (Troubleshooting)

| Lỗi | Giải pháp |
| :--- | :--- |
| **App is damaged** | Chạy `xattr -cr /Applications/GoxViet.app` |
| **Codesign failed** | Kiểm tra certificate trong Keychain Access và Team ID. |
| **Notarization rejected** | Xem log chi tiết: `xcrun notarytool log SUBMISSION_ID`. |
| **Rust build fail** | `cd core && cargo clean` sau đó build lại. |

---

## 📜 5. Danh sách Scripts bổ trợ

*   `bump_version.sh`: Tự động cập nhật version trong `Cargo.toml` và `Info.plist`.
*   `rust_build_lib_universal_for_macos.sh`: Build thư viện universal (x86_64 + arm64).
*   `notarize.sh`: Tự động submit, đợi kết quả và staple vào DMG.

---

**Thông tin chi tiết hơn xem tại:** [scripts/README.md](../scripts/README.md)
