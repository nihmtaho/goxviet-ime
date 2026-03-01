# Version 2.0.12

## 🐛 Bug Fixes

- **Sửa mất quyền Accessibility sau update**: Sau khi cài bản mới, GoxViet đôi khi không nhận phím vì macOS thu hồi quyền accessibility mà không báo. Giờ app tự phát hiện và nhắc bật lại với hướng dẫn rõ ràng (toggle OFF → ON trong System Settings).
- **Tự nhận biết bản update**: Trước đây chỉ nhận ra bản mới khi dùng script tự động (flag `--post-update`). Nay app so sánh version khi khởi động — cài qua Homebrew hay thay DMG thủ công cũng được phát hiện đúng, hiển thị đúng hộp thoại "Re-enable sau update".

## 📈 Improvements

- **CI/CD Release Workflow**: Dọn gọn pipeline — bỏ job thừa và artifact 1 ngày không dùng tới. Build nhanh hơn, log gọn hơn.

## ⚠️ Known Issues

- Accessibility permission vẫn cần bật lại thủ công (toggle OFF/ON) sau mỗi lần cài bản mới — đây là giới hạn của macOS TCC với app chưa ký Developer ID.
