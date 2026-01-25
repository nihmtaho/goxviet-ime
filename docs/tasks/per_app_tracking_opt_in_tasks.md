# Nhiệm vụ cho Per-App Tracking Opt-in
- [ ] Sửa `AppState.setPerAppMode` để chỉ lưu app mới khi `enabled == true`
  - [ ] Bỏ ghi `recordKnownApp` khi `enabled == false` cho app mới
  - [ ] Cho phép cập nhật `false` cho app đã từng lưu
- [ ] Rà soát `PerAppModeManager` đảm bảo call-site không cần thay đổi
- [ ] Cập nhật tài liệu: `settings.md`, `settings_features.md`, `settings_usecases.md`
- [ ] Viết ghi chú review quy trình trong `docs/reviews/workflow_review.md`
