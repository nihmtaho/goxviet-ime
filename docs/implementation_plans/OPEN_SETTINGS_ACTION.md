# Kế hoạch triển khai cho OpenSettingsAction Integration

## Mô tả

Áp dụng kiến trúc OpenSettingsAction của Apple cho menu Settings của GoxViet để mở Settings chuẩn, thay cho gọi trực tiếp WindowManager hoặc selector thủ công.

## Problem Issue (nếu có)

### Current Issues

- Menu Settings vẫn phụ thuộc vào triển khai thủ công (WindowManager/showSettingsWindow) và không sử dụng OpenSettingsAction chuẩn của SwiftUI.
- Hành vi mở Settings không ổn định trên app menubar (Dock ẩn/hiện, focus) và khó bảo trì.

### Root Causes

- Chưa có bridge lưu trữ OpenSettingsAction trong môi trường SwiftUI để AppDelegate có thể gọi.
- App menubar không có Scene hiển thị, nên chưa có nơi khởi tạo OpenSettingsAction.

## Các bước triển khai

1. Tạo bridge lấy `openSettingsAction` từ môi trường SwiftUI và lưu trữ toàn cục để AppDelegate gọi được.
2. Chèn một SwiftUI installer view ẩn (không UI) để capture `openSettingsAction` ngay khi app launch.
3. Sửa `openSettings()` trong AppDelegate để ưu tiên gọi `OpenSettingsAction` bridge, fallback về selector/WindowManager khi cần.
4. Đảm bảo policy Dock/activation vẫn hoạt động: sau khi mở Settings, giữ trạng thái hiện tại và không tạo cửa sổ rác.
5. Kiểm thử thủ công: click menu Settings…, phím tắt Cmd+,, và kiểm tra Settings hiển thị đúng.

## Proposed Changes

- Thêm `SettingsActionBridge` (singleton) lưu `OpenSettingsAction` và API `open()`.
- Thêm view `SettingsActionInstaller` để capture `@Environment(\.openSettingsAction)` và đăng vào bridge.
- Khởi tạo installer trong App (hoặc AppDelegate) bằng hosting controller ẩn, không mở window visible.
- Sửa `openSettings()` dùng bridge.open(); fallback selector hiện có nếu bridge chưa sẵn sàng.

## Thời gian dự kiến

- Phân tích & thiết kế: 0.5 giờ
- Coding & wiring: 1.0 giờ
- Kiểm thử thủ công: 0.5 giờ

## Tài nguyên cần thiết

- Tài liệu Apple: <https://developer.apple.com/documentation/swiftui/opensettingsaction>
- Mã nguồn hiện tại AppDelegate, GoxVietApp, WindowManager

## Implementation Order

1. Thêm bridge + installer view (ẩn) để lấy OpenSettingsAction.
2. Khởi tạo installer khi app launch.
3. Cập nhật AppDelegate.openSettings() dùng bridge trước, selector/WindowManager sau.
4. Kiểm thử thủ công và điều chỉnh Dock/activation nếu cần.
