# Kế hoạch triển khai cho Per-App Tracking Opt-in
## Mô tả
Chỉ bắt đầu tracking ứng dụng trong "Saved Applications" sau khi ứng dụng đó đã từng bật chế độ gõ tiếng Việt (Enabled) ít nhất một lần. Ứng dụng mới chưa từng bật gõ tiếng Việt sẽ không bị lưu/tracking.

## Problem Issue
### Current Issues
- Danh sách "Saved Applications" có thể phình to do lưu cả trạng thái Disabled khi người dùng chưa từng bật gõ tiếng Việt.
- Mặc định tiếng Việt tắt (English), nhưng vẫn có khả năng tạo entry không cần thiết.

### Root Causes
- `setPerAppMode(bundleId, enabled)` đang lưu cả `enabled=false` cho app mới và luôn gọi `recordKnownApp`.

## Các bước triển khai
1. Sửa `AppState.setPerAppMode`:
   - Nếu `enabled == false` và app chưa có entry trước đó → KHÔNG lưu, KHÔNG record known app.
   - Nếu `enabled == true` → lưu entry và `recordKnownApp`.
   - Nếu app đã có entry → cho phép cập nhật true/false bình thường.
2. Giữ nguyên `PerAppModeManager` để gọi `setPerAppMode` khi chuyển app/toggle; logic mới sẽ tự áp dụng.
3. Cập nhật tài liệu trong `docs/features/platform/macos/` (settings.md, settings_features.md, settings_usecases.md).

## Proposed Changes
- Thay đổi điều kiện trong `AppState.setPerAppMode` để thực thi opt-in tracking như trên.
- Không thay đổi API công khai, không thay đổi UI.

## Thời gian dự kiến
- Code & test thủ công: 30 phút
- Cập nhật tài liệu: 15 phút

## Tài nguyên cần thiết
- Không cần thêm dependency; dùng sẵn UserDefaults và các lớp hiện tại.

## Implementation Order
1. Sửa logic trong `AppState.setPerAppMode`.
2. Rà soát `PerAppModeManager` bảo toàn hành vi.
3. Cập nhật tài liệu và rà soát nội dung UI/Settings.
