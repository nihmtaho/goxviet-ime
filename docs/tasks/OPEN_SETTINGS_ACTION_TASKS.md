# Nhiệm vụ cho OpenSettingsAction Integration

- [ ] Tạo bridge `SettingsActionBridge` lưu `openSettingsAction` và API `open()`.
- [ ] Tạo view ẩn `SettingsActionInstaller` để capture `openSettingsAction` và đăng vào bridge khi app launch.
- [ ] Khởi tạo installer từ App (hoặc AppDelegate) thông qua hosting controller ẩn, tránh mở cửa sổ hiển thị.
- [ ] Cập nhật `openSettings()` trong AppDelegate ưu tiên dùng bridge, fallback selector/WindowManager nếu bridge chưa sẵn sàng.
- [ ] Kiểm thử thủ công menu Settings…, phím tắt Cmd+,, và hành vi Dock/focus.
