# Kế hoạch triển khai Special Panel App Detection cho GoxViet

## Mô tả
Implement tính năng detect special panel apps (Spotlight, Raycast) không trigger NSWorkspaceDidActivateApplicationNotification, sử dụng caching và fast-path detection để tránh expensive CGWindowListCopyWindowInfo và AX queries trên mỗi keystroke.

## Problem Issue

### Current Issues
1. **Thiếu detection cho special panel apps:** PerAppModeManager chỉ dựa vào NSWorkspaceDidActivateApplicationNotification, không detect được Spotlight, Raycast, Emoji picker.
2. **App switching không chính xác:** Khi user mở Spotlight/Raycast, GoxViet không biết để adjust mode hoặc clear buffer.
3. **Trải nghiệm người dùng không mượt:** User phải manually toggle mode khi switch giữa special panel apps và normal apps.

### Root Causes
- NSWorkspaceDidActivateApplicationNotification không fire cho overlay/panel apps (macOS system limitation)
- Không có polling mechanism để detect special panel app activation
- Không có cache mechanism để tránh expensive AX/Window queries

## Các bước triển khai

1. **Tạo SpecialPanelAppDetector class:**
   - Whitelist special panel app bundle IDs
   - Cache mechanism với TTL 300ms
   - Fast-path: check focused element (single AX query)
   - Slow-path: full window scan (CGWindowListCopyWindowInfo)

2. **Tích hợp vào PerAppModeManager:**
   - Thêm polling timer (200ms interval)
   - Invalidate cache khi app switch
   - Update lastFrontMostApp tracking
   - Handle app switch cho cả normal và special panel apps

3. **Performance optimization:**
   - Use CFAbsoluteTimeGetCurrent() thay vì Date()
   - Cache result với double optional pattern
   - Fast-path trước, slow-path chỉ khi cần
   - Timer runs in .common mode để không bị block

## Proposed Changes

### New Files
- `SpecialPanelAppDetector.swift`: Core detection logic với caching và fast/slow path

### Modified Files
- `PerAppModeManager.swift`:
  - Add polling timer property
  - Add special panel detection methods
  - Integrate detector vào lifecycle (start/stop)
  - Invalidate cache on app switch

## Thời gian dự kiến
- Tạo SpecialPanelAppDetector: ✅ Done
- Tích hợp vào PerAppModeManager: ✅ Done
- Testing và verification: 30 phút
- Documentation: 15 phút

## Tài nguyên cần thiết
- Reference implementation từ example-project
- macOS Accessibility API documentation
- CGWindowListCopyWindowInfo documentation

## Implementation Order
1. ✅ Tạo SpecialPanelAppDetector.swift với đầy đủ tính năng
2. ✅ Tích hợp polling timer vào PerAppModeManager
3. ✅ Update notification handler để invalidate cache
4. ✅ Add special panel detection methods
5. ⏳ Test với Spotlight, Raycast, và normal apps
6. ⏳ Verify performance (không ảnh hưởng latency)
