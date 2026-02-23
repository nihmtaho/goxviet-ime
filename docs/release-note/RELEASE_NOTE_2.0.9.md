# 📝 Release Notes - Phiên bản 2.0.9

**Ngày phát hành:** 10/02/2026
**Phiên bản:** 2.0.9

---

## ✨ Tính năng mới

- **Cải tiến cấu trúc dự án:** Tái cấu trúc toàn bộ cấu trúc thư mục, gộp các skills từ nhiều nguồn vào `.agent/skills/`
- **Tạo STRUCTURE.md:** Thêm tài liệu hướng dẫn cấu trúc dự án chi tiết
- **Release Note Generator Skill:** Thêm skill tự động tạo release notes với workflow toàn diện để tạo changelog và release notes chi tiết
  - Tích hợp với skill-git và pr-manager
  - Phân loại thay đổi thành Features, Bug Fixes, Improvements, Documentation
  - Tạo cả CHANGELOG.md entry và detailed release note
  - Hỗ trợ script extract-changelog.sh cho GitHub Actions workflow

---

## 🐛 Sửa lỗi

### Text Injection & English Detection Improvements (PR #61)

- **English Detection for Horn/Breve**: Skip English detection for Horn (ʼ) and Breve (˘) diacritical marks to prevent false negatives in Vietnamese text processing
- **Text Injection Methods**: Improved text injection handlers with better bundleId-based application detection
- **Memory Leaks**: Fixed memory leaks in MemoryProfiler, InputManager, and Log services
- **Redundant Checks**: Removed redundant English auto-restore checks for cleaner logic flow

### Engine Core - 5 lỗi logic quan trọng đã được sửa

1. **Sửa lỗi Smart 'w' Double-Apply** (Issue #1)
   - **Vấn đề:** `khuow` → `khươ` (sai), đúng ra phải là `khuơ`
   - **Nguyên nhân:** Hàm `normalize_uo_compound()` tự động chuyển `u + ơ` → `ươ` nhưng "khươ" không hợp lệ sau cụm phụ âm "kh"
   - **Giải pháp:** Thêm kiểm tra ngữ âm học (phonotactic) cho các cụm KH, TH, PH trước khi chuẩn hóa
   - **File:** `core/src/engine/vietnamese/vowel_compound.rs`

2. **Sửa lỗi Compound Vowel Over-Aggressive** (Issue #2)
   - **Vấn đề:** `khoeo` → `khôe` (sai), đúng ra phải giữ nguyên `khoeo`
   - **Nguyên nhân:** Logic backward application của Telex không kiểm tra nguyên âm xen giữa khi áp dụng dấu mũ `oo → ô`
   - **Giải pháp:** Thêm kiểm tra nguyên âm trong khoảng `(pos+1..self.buf.len())` trước khi áp dụng dấu
   - **File:** `core/src/engine/mod.rs`

3. **Sửa lỗi Foreign Word Auto-Restore** (Issue #3)
   - **Vấn đề:** `tareh` → `Taẻh` (sai), đúng ra phải giữ nguyên `tareh`
   - **Nguyên nhân:** Cơ chế auto-restore kích hoạt sai trên từ có hậu tố ngoại lai '-eh'
   - **Giải pháp:** Tự động khắc phục sau khi sửa Issues #1 và #2
   - **Kết quả:** ✅ `tareh` → `tareh` (đúng)

4. **Sửa lỗi VNI Compound Mark** (Issue #4)
   - **Vấn đề:** `thuo73` → `thưở` (sai), đúng ra phải là `thuở`
   - **Nguyên nhân:** Cùng nguyên nhân Issue #1 - tự động chuẩn hóa `u + ơ` → `ươ` không hợp lệ sau "th"
   - **Giải pháp:** Sử dụng cùng fix với Issue #1, mở rộng kiểm tra cho TH và PH
   - **Kết quả:** ✅ `thuo73` → `thuở` (đúng)

5. **Sửa lỗi "uyu" Triphthong Not Recognized** (Issue #5)
   - **Vấn đề:** `khuyur` / `khuyu3` không chuyển được thành `khuỷu`
   - **Nguyên nhân:** Validator thiếu bigram "yu" (y+u) trong danh sách tổ hợp 2 nguyên âm hợp lệ
   - **Giải pháp:** Thêm `(keys::Y, keys::U)` vào danh sách bigram hợp lệ
   - **File:** `core/src/engine_v2/vietnamese_validator.rs`

6. **Sửa lỗi Per-App Mode Race Condition** (PR #59)
   - **Vấn đề:** Race condition khi switch app nhanh liên tiếp
   - **Nguyên nhân:** Mode bị lưu cho app mới thay vì app cũ, và restore mode sai
   - **Giải pháp:** 
     - Capture `previousId` trước khi update `currentBundleId`
     - Thêm check `previousId != bundleId` để tránh lưu khi switch cùng app
     - Truyền `bundleId` trực tiếp vào `restoreModeForCurrentApp`
   - **File:** `platforms/macos/goxviet/goxviet/Managers/PerAppModeManagerEnhanced.swift`

### Test Fixes

- **Sửa đường dẫn file test:** `vietnamese_22k_pure.txt` → `vietnamese_69k_pure.txt`
- **Loại bỏ từ ngoại lai:** Xóa "taxi" khỏi dictionary test (từ tiếng Anh)

---

## ⚡ Cải thiện

### Test Optimization

- **Tối ưu dictionary_vietnamese_test.rs:** Hỗ trợ đầy đủ Telex và VNI
- **Cải thiện vowel conversion functions:** Chuyển đổi nguyên âm chính xác hơn
- **Lọc từ ngoại lai:** Phát hiện và loại bỏ từ không phải tiếng Việt
- **Phân tích lỗi test:** Phân loại và phân tích chi tiết các lỗi còn lại
- **Cập nhật dictionary data:** Đã cập nhật common_4chars.bin và common_6chars.bin
- **Làm sạch dữ liệu từ điển:** Clean 69,401 từ tiếng Việt trong vietnamese_69k_pure.txt

### Kết quả Test (Sau cải tiến)

| Phương pháp | Tổng số từ | Đạt | Tỷ lệ |
|-------------|-----------|-----|-------|
| **Telex** | 6,577 | 6,540 | 99.44% |
| **VNI** | 6,577 | 6,550 | 99.59% |

**Phân loại lỗi còn lại:**
- Dictionary Issues: 60 lỗi (từ không hợp lệ, mẫu âm tiết sai)
- Engine Logic Issues: 4 lỗi (đã sửa trong release này)

### Structure Cleanup

- Dọn dẹp 16 files không cần thiết
- Gộp agent skills từ `.agent/`, `.claude/`, `scripts/skills/`
- Xóa các thư mục trống (bindings, examples)
- Cập nhật .gitignore
- Loại bỏ các test files deprecated
- Thêm các báo cáo phân tích lỗi test (failures_telex.txt, failures_vni.txt)

### macOS Platform SOLID Refactoring

- **Tái cấu trúc codebase theo SOLID principles:**
  - Phân tách file vào các module logic: `App/`, `Core/`, `Managers/`, `Models/`, `Services/`, `UI/`, `Utilities/`
  - Mỗi module có trách nhiệm duy nhất (Single Responsibility)
  - Dễ bảo trì và mở rộng hơn
  
- **Migrate RustBridge sang RustBridgeSafe:**
  - Xóa `RustBridge.swift` (legacy), migrate sang `Core/RustBridgeSafe.swift`
  - Thêm `setShortcutsEnabled()` method vào `RustBridgeSafe`
  - Update `SettingsManager` và `InputManager` sử dụng `RustBridgeSafe`
  - Thread-safe, better error handling
  
- **File Organization:**
  - `App/` - Application entry point (GoxVietApp, AppDelegate)
  - `Core/` - Business logic và FFI bridge
  - `Managers/` - Input, Injection, Update, Resource, Window
  - `Models/` - KeyboardShortcut, LRUCache
  - `Services/` - Log, MemoryProfiler, InputSourceMonitor
  - `UI/` - MenuBar, Settings, Components
  - `Utilities/` - Helper classes và protocols

**Breaking Change:** `RustBridge` class đã bị xóa. Code cũ sử dụng `RustBridge.shared` cần migrate sang `RustBridgeSafe.shared`.

### macOS Performance Optimization (PR #59)

- **Memory Usage**: Tối ưu memory usage và cải thiện app lifecycle management
- **Memory Cleanup**: Cải thiện memory cleanup procedures, giảm subscription leaks
- **App Lifecycle**: Tối ưu AppDelegate và window management
- **Settings UI**: Loại bỏ Engine Metrics, Memory Profiling, System Info khỏi Advanced tab
- **Files Removed**: 
  - `MemoryProfilingView.swift` (314 lines)
  - `MetricsChartView.swift` (156 lines)

---

## 📦 Thay đổi khác

- **Thêm test suite mới:** `core/tests/engine_bug_fixes_test.rs` - 10 test cases kiểm tra các bug đã sửa
- **Cải thiện test analysis:** Phân tích chi tiết 37 lỗi Telex và 27 lỗi VNI còn lại
- **Cập nhật Release Workflow:** GitHub Actions workflow đã cập nhật để extract changelog từ CHANGELOG.md
- **Cập nhật .gitignore:** Cải thiện cấu hình .gitignore cho .claude/ và .github/

---

## 👥 Ngườii đóng góp

- **Thao Truong Minh** (@nihmtaho) - Cleanup structure, test analysis, engine fixes

---

## 🔗 Tham khảo

- PR: #61 - "fix(core,macos): Improve English detection for Horn/Breve diacriticals and text injection"
- PR: #60 - "chore: cleanup structure + add dictionary test analysis"
- PR: #59 - "Memory optimization and app lifecycle improvements"
- Issue: #53 - Project structure cleanup
- Commit: `7c468d2` - "fix(core,macos): Improve English detection for Horn/Breve diacriticals and text injection"
  - Skip English detection for Horn/Breve diacritical marks
  - Improve text injection methods and bundleId-based detection
  - Fix memory leaks in MemoryProfiler, InputManager, and Log
  - Remove redundant English auto-restore checks
  - Add new injection methods and role resolution helpers
  - Improve text injection handling with TextInjectionHelper updates
  - Update core library and test expectations

---

## 📋 Test Results

```
running 10 tests
test test_issue_1_smart_w_double_apply_telex ... ok
test test_issue_1_smart_w_double_apply_vni ... ok
test test_issue_2_compound_vowel_oeo_telex ... ok
test test_issue_3_foreign_word_tareh ... ok
test test_issue_4_vni_compound_mark_thuow ... ok
test test_issue_5_uyu_triphthong_telex ... ok
test test_issue_5_uyu_triphthong_vni ... ok
test test_normal_uo_compound_still_works_telex ... ok
test test_normal_uo_compound_still_works_vni ... ok
test test_issue_2_compound_vowel_khoeo_with_tone_telex ... ignored

test result: ok. 9 passed; 0 failed; 1 ignored
```

---

*Generated by Release Note Generator Skill*
