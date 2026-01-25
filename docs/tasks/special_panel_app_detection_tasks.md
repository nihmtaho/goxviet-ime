# Nhiệm vụ cho Special Panel App Detection

- [x] Nhiệm vụ 1: Tạo SpecialPanelAppDetector.swift
  - [x] Whitelist special panel app bundle IDs
  - [x] Cache mechanism với TTL 300ms
  - [x] Fast-path: check focused element (single AX query)
  - [x] Slow-path: full window scan (CGWindowListCopyWindowInfo)
  - [x] App change tracking methods

- [x] Nhiệm vụ 2: Tích hợp vào PerAppModeManager
  - [x] Add polling timer property (200ms interval)
  - [x] Start/stop polling timer trong lifecycle
  - [x] Invalidate cache on normal app switch
  - [x] Update lastFrontMostApp tracking

- [x] Nhiệm vụ 3: Special panel detection methods
  - [x] checkForSpecialPanelApp() polling method
  - [x] handleAppSwitch() cho cả normal và special panel apps
  - [x] Integration với smart mode

- [ ] Nhiệm vụ 4: Testing và verification
  - [ ] Test với Spotlight (Cmd+Space)
  - [ ] Test với Raycast
  - [ ] Test với Emoji picker
  - [ ] Verify không ảnh hưởng latency (<16ms)
  - [ ] Verify cache hoạt động đúng (300ms TTL)
  - [ ] Test app switching từ special panel về normal app

- [ ] Nhiệm vụ 5: Documentation
  - [x] Implementation plan
  - [x] Task list
  - [ ] Workflow review
  - [ ] Update GETTING_STARTED.md nếu cần
