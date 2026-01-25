# Nhiệm vụ: Fix Issue #36 - Backspace xóa nhầm autocomplete trong browsers

- [ ] **Task 1:** Phân tích và hiểu rõ logic hiện tại
  - [ ] Đọc code `injectViaAX` trong TextInjectionHelper.swift
  - [ ] Trace flow xử lý autocomplete suggestion
  - [ ] Identify root cause của bug

- [ ] **Task 2:** Fix suffix + selection logic (UTF-16 safe)
  - [ ] Sử dụng NSString/UTF-16 cho cursor/selection để tránh lệch index
  - [ ] Sửa `userText` và `suffix` để bỏ toàn bộ suggestion đang highlight
  - [ ] Handle edge case: không autocomplete, manual selection

- [ ] **Task 3:** Cursor + logging
  - [ ] Log cursor position, selection length
  - [ ] Log fullText, userText, suffix
  - [ ] Log calculation details (deleteStart, bs, newText)
  - [ ] Đặt cursor mới bằng `.utf16.count`

- [ ] **Task 4:** Testing
  - [ ] Test case 1: Arc Browser address bar với autocomplete
  - [ ] Test case 2: Gõ tiếng Việt với autocomplete (diện)
  - [ ] Test case 3: No autocomplete - text bình thường
  - [ ] Test case 4: Manual selection bằng tay
  - [ ] Test trên Chrome, Safari, Firefox

- [ ] **Task 5:** Code review và cleanup
  - [ ] Add comments giải thích logic mới
  - [ ] Remove debug logs không cần thiết
  - [ ] Ensure code style tuân thủ Swift conventions

- [ ] **Task 6:** Documentation
  - [ ] Update implementation plan với kết quả
  - [ ] Create workflow review
  - [ ] Update CHANGELOG if needed
