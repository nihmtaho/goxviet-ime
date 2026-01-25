# Đánh giá Quy trình làm việc cho Per-App Tracking Opt-in
## Mô tả
Triển khai cơ chế opt-in: chỉ lưu/tracking ứng dụng vào "Saved Applications" sau khi app bật gõ tiếng Việt ít nhất một lần. App mới chưa từng bật sẽ không bị lưu.

## Những gì đã làm tốt
- Điều chỉnh logic lưu trong `AppState.setPerAppMode` rõ ràng, không đổi API.
- Tương thích với `PerAppModeManager` hiện tại, không cần sửa call-sites.
- Cập nhật tài liệu Settings (features, use cases, kiến trúc) để phản ánh hành vi mới.

## Những gì cần cải thiện
- Có thể thêm unit test UI-level (nếu có harness) để xác nhận danh sách Saved chỉ xuất hiện sau lần bật đầu tiên.

## Bài học rút ra
- Mặc định tiếng Việt tắt (English), nên tránh lưu trạng thái không cần thiết để giữ danh sách gọn.
- Ghi nhận lần bật đầu tiên là ngưỡng hợp lý cho tracking per-app.

## Notes/Important
- Hành vi mới: `enabled == false` cho app chưa có entry → không lưu, không record known app. App đã có entry vẫn cập nhật bình thường.
