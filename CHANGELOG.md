# CHANGELOG

## [2.0.9] - 2026-02-10

> 📝 **Release Note**: [.release-notes/release_note_2.0.9.md](.release-notes/release_note_2.0.9.md)

### ✨ Features
- **Project Structure Refactoring**: Reorganized directory structure, consolidated agent skills into `.agent/skills/`
- **STRUCTURE.md Documentation**: Added detailed project structure guide

### 🐛 Bug Fixes
- **Smart 'w' Double-Apply**: Fixed `khuow` → `khươ` issue, now correctly produces `khuơ` (phonotactic validation)
- **Compound Vowel Over-Aggressive**: Fixed `khoeo` auto-conversion, now preserves original input
- **Foreign Word Auto-Restore**: Fixed `tareh` false trigger, now correctly preserves foreign words
- **VNI Compound Mark**: Fixed `thuo73` → `thưở` issue, now correctly produces `thuở`
- **"uyu" Triphthong**: Added support for `khuyur` → `khuỷu` pattern
- **Per-App Mode Race Condition**: Fixed race condition in per-app mode manager causing inconsistent behavior

### ⚡ Improvements
- **Test Coverage**: Improved Telex rate to 99.44% (6,540/6,577) and VNI to 99.59% (6,550/6,577)
- **Project Cleanup**: Removed 16 unnecessary files, updated .gitignore
- **Test Analysis**: Generated comprehensive failure analysis reports
- **Memory Optimization**: Optimized memory usage and improved app lifecycle management in macOS app
- **Settings UI**: Removed diagnostic features (metrics, memory profiling) from Advanced settings tab

### 📊 Test Results
- 5 critical engine bugs identified and fixed
- 10 new test cases added for regression prevention
- Dictionary test optimized with foreign word filtering

---

## [2.0.8] - 2026-02-06

### 🐛 Bug Fixes
- **Zen Browser Duplication Bug**: Sửa lỗi gõ tiếng Việt trên Zen Browser (issue #54). Chuyển sang sử dụng phương thức `AX API Direct` kết hợp với logic fallback tối ưu. Code fallback được viết lại để xử lý nhanh hơn: thoát ngay nếu phát hiện browser override (autocomplete) và chỉ retry khi gặp lỗi kết nối AX. Đặc biệt: Đã xử lý triệt để case gõ "đ" (dđ) bằng logic workaround thông minh (Type -> Left -> Backspace -> Right) với độ trễ tối ưu 1ms.
- **Proxy Event Injection**: Sửa lỗi `TextInjectionHelper` không sử dụng proxy khi fallback, đảm bảo tính ổn định khi inject text trong các trường hợp AX API thất bại.
- **UI Layout Recursion**: Sửa warning `_NSDetectedLayoutRecursion` bằng cách refactor `MenuToggleView` sử dụng `ObservableObject` thay vì thay thế SwiftUI RootView liên tục.
- **Build Stability**: Sửa các lỗi biên dịch thiếu import `Combine` và thiếu định nghĩa `KeyCode`.
- **Tone Repositioning**: Sửa lỗi transform khi SPACE khiến việc gõ không thoải mái.

---


## [2.0.7] - 2026-02-04

### 🔧 Chores
- **macOS Build Version**: Bump build version and marketing version to 2.0.7

---

## [2.0.6] - 2026-02-04

### ✨ Features
- **Update State Refactor**: Cấu trúc lại hệ thống trạng thái cập nhật (UpdateState) sử dụng associated values, thêm trạng thái `installing` để theo dõi quá trình cài đặt chi tiết.

### 🔧 Refactoring
- **UpdateManager Centralization**: Tập trung quản lý trạng thái cập nhật vào `UpdateManager`, tách riêng logic kiểm tra cập nhật vào `UpdateChecker` để code modularity tốt hơn.
- **UI Components Update**: Cập nhật các component UI để phản ánh các trạng thái cập nhật mới, cải thiện UX khi theo dõi tiến trình cập nhật.

---


## [2.0.5] - 2026-02-04

### 🐛 Bug Fixes
- **Auto-caps VNI Input**: Sửa lỗi VNI input tự động viết hoa ký tự dù Caps Lock tắt, cải thiện logic phát hiện shift key trong macOS InputManager.
- **Shift Key Detection**: Hoàn thiện logic phát hiện trạng thái phím Shift, tránh gọi Shift handler không cần thiết khi buffer trống.

### 🔧 Chores
- **Test Coverage**: Thêm comprehensive edge case tests cho auto-capitalization handling, đảm bảo fix ổn định trên các scenario khác nhau.
- **Code Signing**: Cải thiện code signing trong release workflow, bảo đảm macOS app đúng chuẩn phát hành.

---

## [2.0.4] - 2026-02-04

### Added
- **AGENT.override.md Guidelines**: Thêm hướng dẫn chi tiết cho platform (macOS, Windows) và core engine, giúp contributor hiểu rõ architecture và best practices.
- **macOS App Updates**: Triển khai hệ thống cập nhật macOS dựa trên rsync, cho phép incremental updates và giảm kích thước download.
- **Tone & Circumflex Transform**: Cải tiến hỗ trợ đặt dấu thanh và dấu ^ (circumflex) sau nguyên âm và phụ âm cuối, tăng độ chính xác xử lý các pattern phức tạp (#50).
- **Phase 2 macOS UI Redesign**: Thiết kế lại giao diện macOS với các tính năng beta mới, tối ưu UX trên macOS 12.0+ (#49).
- **Phase 1 Core Features**: Triển khai các tính năng Phase 1 bao gồm keyboard shortcuts, Shift+Backspace xóa từ, và hỗ trợ multi-encoding (UTF-8, TCVN3, VNI, CP1258) (#44).

### Changed
- **Buffer & Restore Optimization**: Tối ưu logic buffer và restore với inlining, `copy_within`, và pre-allocation để giảm latency và memory usage.
- **Core Engine u8 Handling**: Thêm clamping cho u8 overflow trong core engine, tăng độ ổn định khi xử lý dữ liệu cạnh.

### Performance
- **Latency**: Giảm độ trễ xử lý nhờ optimize buffer operations (< 3ms hot path).
- **Memory**: Giảm memory allocation trong restore logic nhờ pre-allocation strategy.

---

# Changelog - Gõ Việt (GoxViet)

## [Unreleased]

### Fixed
- **Per-App Mode Race Condition**: Sửa lỗi race condition trong `PerAppModeManagerEnhanced` khi switch app nhanh liên tiếp. Lỗi khiến mode bị lưu cho app mới thay vì app cũ, và restore mode sai cho app hiện tại. Fix bao gồm: capture `previousId` trước khi update `currentBundleId`, thêm check `previousId != bundleId` để tránh lưu khi switch cùng app, và truyền `bundleId` trực tiếp vào `restoreModeForCurrentApp` để tránh race condition.

### Added


### Changed


## [2.0.3] - 2026-01-29

### Added
- **Architecture Documentation**: Hoàn thiện các tài liệu kiến trúc và quy tắc coding chính thức cho dự án (#45).
- **GitHub Workflow Templates**: Thêm template cho issue và pull request để chuẩn hóa quy trình contribute.

### Changed
- **English Detection**: Tăng cường logic phát hiện từ tiếng Anh với confidence threshold, giảm false positive khi gõ nhanh.
- **Telex Tone Logic**: Cải thiện xử lý double-key trong Telex (ví dụ: `ss`, `ff`, `rr`) để tránh biến đổi nhầm.
- **Dictionary System**: Cập nhật số lượng từ trong các tập từ điển (2-7 ký tự), thêm entries mới vào từ điển 7 ký tự.
- **GitHub Actions**: Nâng cấp workflow phát hành, cải thiện Info.plist version tracking bằng build number.

### Fixed
- **Key Skipping Logic**: Vô hiệu hóa logic bỏ qua phím để cải thiện phát hiện từ tiếng Anh và xử lý Telex tone chính xác hơn (#46).
- **English Auto-Restore**: Sửa lỗi auto-restore tiếng Anh với confidence threshold, ngăn chặn restore sai từ trong điều kiện gõ nhanh.
- **Dictionary Integrity**: Kiểm tra và sửa tính toàn vẹn dữ liệu từ điển trong tất cả tập tin 2-7 ký tự.
- **Release Workflow**: Cải thiện quy trình phát hành tự động qua GitHub Actions.


## [2.0.2] - 2026-01-25

### Added
- **Testing Infrastructure**: Triển khai hệ thống kiểm thử dựa trên từ điển với ~72K từ tiếng Việt và ~100K từ tiếng Anh, giúp tăng độ chính xác của engine và tránh regression (#41).
- **Per-App Tracking**: Thêm tính năng "opt-in" cho per-app settings. Ứng dụng chỉ được lưu vào danh sách khi người dùng bật bộ gõ tiếng Việt cho ứng dụng đó ít nhất một lần (#43).
- **Real-time UI**: Danh sách "Saved Applications" trong Settings sẽ cập nhật realtime khi trạng thái app thay đổi (#43).

### Changed
- **Telex Logic**: Cải tiến logic xử lý stroke và mark modifiers, tăng độ chính xác khi gõ các cụm từ phức tạp (#42).
- **English Detection**: Bổ sung "console" vào từ điển tiếng Anh và tối ưu logic bỏ qua (skip logic) cho một số pattern tiếng Anh (#42).
- **Documentation**: Cập nhật tài liệu kỹ thuật về per-app settings và tracking trên macOS (#43).

### Fixed
- **Telex Typing Bug**: Sửa lỗi gõ `d`+`i`+`s` ra `dis` thay vì `dí` (#42).
- **Modifier Transform**: Sửa lỗi transform `aw` → `ă` và đảm bảo đếm đúng số phím backspace khi apply breve transform (#42).
- **Multi-vowel Tone**: Xử lý lỗi nhận diện phím dấu cho các từ tiếng Anh có nhiều nguyên âm, tránh việc áp dấu nhầm (#42).


## [2.0.1] - 2026-01-16

### Added
- **Test Coverage**: Added comprehensive test cases for smart backspace, tone validation, and TR- initial words.

### Changed
- **VNI Optimization**: Optimized tone marking logic (linear → binary search), reducing latency to 8-11ms.
- **Core Engine**: Enhanced buffer manipulation efficiency, reducing unnecessary allocations.

### Fixed
- **Triple Key Toggle**: Fixed `d`+`d`+`d` failing to toggle back to `dd` correctly.
- **Prefix Deletion**: Fixed critical "add" bug where reverting a transform deleted preceding characters.
- **Vietnamese Validation**: Fixed invalid tone placements (e.g., "neư") and enabled "tr" initial transforms (truyền, triển).
- **Special Characters**: Fixed issue where punctuation (e.g., `!`) caused Vietnamese words to revert (e.g., "đã!" → "d9a41").
- **UI Sync**: Fixed MenuBar toggle states not synchronizing with Settings UI.
- **English Double Tone**: Fixed "off" → "òf" issue by marking word as English after tone revert.


## [2.0.0] - 2026-01-15

### Added
- Tích hợp engine mới giúp tăng tốc độ xử lý và tối ưu hiệu năng toàn ứng dụng
- Cập nhật giao diện người dùng cho trải nghiệm mượt mà hơn

### Changed
- Cải thiện thuật toán nhận diện tiếng Anh, giảm false positive khi gõ Telex
- Sửa lỗi nhận diện prefix/suffix cho các từ tiếng Anh phổ biến
- Xóa các tài liệu thừa, tối ưu cấu trúc dự án

### Fixed
- Fixed [Bug] Lỗi gõ VNI #33
- Fixed bug(telex): Telex đôi khi nhận nhầm từ tiếng Anh
- Fixed bug(telex): nhập số bị chuyển thành dấu hoặc ký tự đặc biệt #30
- Fixed bug: backspace deletes autocomplete suggestion instead of typed text in browsers #36


## [1.5.2] - 2026-01-05

### Added
- Modifier shortcut to toggle enable/disable Vietnamese keyboard (macOS)
- Option to toggle auto-hide in Dock (macOS)
- 

### Changed
- Improved UI for better user experience
- 

### Fixed
- 


## [1.5.1] - 2026-01-03

### Added
- advanced phonotactic analysis engine to support language decision and syllable validation (commit: `05a571d`)

### Changed
- consolidate English detection logic to use the phonotactic engine and refactor related modules (commit: `06baea7`, `4def9db`)
- performance optimizations for the English-detection pipeline to reduce false positives and speed up validation (commit: `47dbf6d`)
- phase-2 folder restructuring and refactor of engine modules for maintainability (commit: `4def9db`)
- optimizations to `RawInputBuffer` and backspace handling to avoid buffer desync and improve undo/restore behaviour (commit: `b9ef1cd`, `982dbc2`)

### Fixed
- prevent auto-restore false positives for Vietnamese words; ensure auto-restore only triggers for confirmed English words (commit: `d6c793f`)
- improve English detection to avoid false positives on valid Vietnamese syllables (commit: `3cef501`)


## [1.5.0] - 2026-01-02

### Added
- **Trình quản lý cập nhật tự động**: GoxViet kiểm tra phiên bản mới mỗi 6 giờ, cho phép cập nhật với một click (mount DMG, copy app, restart tự động)
- **Hỗ trợ đa ngôn ngữ**: Tự động vô hiệu hóa bộ gõ tiếng Việt khi chuyển sang bàn phím không Latin (Nhật, Hàn, Trung, v.v.), tự động khôi phục khi quay lại

### Changed
- **Quy trình xin quyền Accessibility**: Tự động phát hiện khi quyền được cấp, không cần nhấn "Check Again"
- **Cài đặt DMG**: Loại bỏ Homebrew auto-install, chỉ hỗ trợ tải DMG trực tiếp từ GitHub

### Fixed
- Fix: Mất quyền Accessibility sau khi rebuild/cài lại (code signing không đổi)
- Fix: Crash SIGABRT khi chạy từ Xcode (code signature mismatch giữa app và Rust library)
- Fix: Gõ tiếng Việt sai trong thanh địa chỉ Chromium (Issue #26) - thêm phương thức injection `axDirect` qua Accessibility API


## [1.4.1] - 2026-01-02

### Added
- 

### Changed
- 

### Fixed
- 


## [1.3.2] - 2025-12-24

### Added
- Mở rộng test coverage cho các từ tiếng Anh có cụm phụ âm bất khả thi trong tiếng Việt (vd: improve, import, express, please...).
- Cập nhật tài liệu hướng dẫn phát hiện tiếng Anh và logic auto-restore.
- 

### Changed
- Cải tiến logic nhận diện tiếng Anh trong engine: bổ sung kiểm tra các cụm phụ âm không hợp lệ tiếng Việt (mp, pr, pl, ps, pt, wr, f/w/j/z + phụ âm).
- Tối ưu hiệu suất kiểm tra, không ảnh hưởng tốc độ gõ tiếng Việt.
- 

### Fixed
- Sửa lỗi nghiêm trọng: Telex Auto Restore English không khôi phục đúng từ tiếng Anh khi có cụm phụ âm bất khả thi (vd: "improve" bị thành "ỉmpove").
- Đảm bảo auto-restore hoạt động chính xác, không còn lỗi lệch buffer hoặc biến đổi dấu sai khi gõ tiếng Anh.
- 


## [1.3.1] - 2025-12-23

### Added
- Enhanced English detection patterns: nhận diện `oo`, `tex`, `nex`, `sex`, `-isk`, `-usk` để giảm false positive khi gõ tiếng Anh.
- Tự động clear buffer khi undo/restore và bắt đầu từ đầu từ mới.
- Xử lý Cmd/Shift + Arrow trên macOS: clear buffer khi di chuyển con trỏ bằng phím điều hướng có modifier.

### Changed
- Cải thiện logic xác định open/closed syllable cho quy tắc đặt dấu tiếng Việt (đặc biệt với pattern "ua").
- Cập nhật test suite: thêm test cho English flag reset, UTF-8 backspace, English detection.

### Fixed
- Sửa lỗi backspace với ký tự UTF-8 (đ, ă, ơ, ư, nguyên âm có dấu): đếm ký tự màn hình thay vì byte.
- Sửa lỗi đặt dấu sai vị trí với pattern "ua" (mùa, chuẩn, ...).
- Sửa lỗi English word flag không reset sau khi xóa hết buffer, khiến không gõ được dấu tiếng Việt sau khi xóa từ tiếng Anh.


## [1.3.0] - 2025-12-22

### Added
- Refactor toàn bộ giao diện cửa sổ Settings (macOS) sử dụng SwiftUI NavigationSplitView chuẩn native.
- Thêm hiệu ứng mượt mà khi đóng/mở sidebar, loại bỏ hiện tượng giật lag.

### Changed
- Giảm bán kính bo góc (radius) và padding panel chi tiết để đồng bộ với sidebar, giao diện gọn gàng hơn.
- Loại bỏ animation chuyển panel, đảm bảo chuyển đổi tức thời, không gây lag.
- Loại bỏ Divider thừa trong sidebar, đồng bộ màu sắc và spacing.

### Fixed
- Sửa lỗi icon không hiển thị cho từng mục sidebar (General, Per-App, Advanced, About).
- Đảm bảo mọi thao tác chuyển đổi panel, toggle sidebar đều đúng chuẩn macOS.
- Không còn custom sidebar toggle, không còn lỗi icon sidebar.


## [1.2.3] - 2025-12-22

### Added
- Hard limit for ShortcutTable (`MAX_SHORTCUTS = 200`) in Rust core to prevent unbounded memory growth
- Hard limit for per-app settings (`MAX_PER_APP_ENTRIES = 100`) in Swift/macOS layer
- UI warning when approaching per-app settings limit, with option to clear old entries
- Documentation: `MEMORY_LEAK_FIX.md`, `MEMORY_BLOAT_PREVENTION.md` (memory safety, prevention, and test results)
- Unit tests for shortcut and per-app settings limits
- 

### Changed
- Standardized and updated documentation structure and release notes for v1.2.3
- Updated documentation indexes: `README.md`, `DOCUMENTATION_STRUCTURE.md`, `STRUCTURE_VISUAL.md`
- 

### Fixed
- Memory leak in Swift/macOS layer: NotificationCenter observers are now tracked and properly removed in `InputManager.swift` and `AppDelegate.swift`
- Memory bloat: all core buffers (Buffer, RawInputBuffer, WordHistory) are now strictly bounded
- No more unbounded growth in any data structure (Rust or Swift)
- Memory usage now stable at ~25-30MB regardless of session length
- 


## [1.2.2] - 2025-12-21

### Added
- 

### Changed
- 

### Fixed
- 


All notable changes to this project will be documented in this file.

---

## [1.3.0] - 2025-12-22

### 🎨 UI Settings Refactor (macOS)
- Chuẩn hóa toàn bộ giao diện cửa sổ Settings sử dụng SwiftUI `NavigationSplitView` theo phong cách native macOS.
- Sidebar: Sửa lỗi icon không hiển thị cho từng mục (General, Per-App, Advanced, About). Loại bỏ Divider thừa, đồng bộ màu sắc và spacing.
- Panel chi tiết: Giảm bán kính bo góc (radius) và padding để đồng bộ với sidebar, giao diện gọn gàng hơn. Loại bỏ animation chuyển panel, đảm bảo chuyển đổi tức thời, không gây lag.
- Trải nghiệm người dùng: Thêm hiệu ứng mượt mà khi đóng/mở sidebar, loại bỏ hiện tượng giật lag. Đảm bảo mọi thao tác chuyển đổi panel, toggle sidebar đều đúng chuẩn macOS.
- Refactor toàn bộ cấu trúc Settings UI, không còn custom sidebar toggle, không còn lỗi icon sidebar.
- Đảm bảo code SwiftUI sạch, dễ bảo trì, không phụ thuộc mã nguồn tham khảo ngoài.
- Không thay đổi logic xử lý cốt lõi hoặc API, chỉ cải thiện UI/UX.

## [Unreleased] - 2025-XX-XX


### Planned

## [1.2.1] - 2025-12-21
### 🚨 Critical Bugfix & Stability Release

#### Fixed – Accessibility Permission Issues
- Duplicate permission dialogs removed (only one custom dialog shown)
- Accessibility permission now persists across app restarts
- Auto-detection of permission grant (no need to click “Check Again”)
- Removed duplicate permission checks (fixed thread priority inversion)
- Added missing `Log.warning()` and `Log.error()` methods (fixed compile errors)
- Improved UX: clearer dialog, troubleshooting tips, “Restart Now” button

#### Fixed – Backspace Corruption Bug
- Fixed character duplication/corruption when deleting Vietnamese text (e.g. “gõ” → “gg”, “được” → “đđư”)
- Removed batch/coalescing logic that caused engine state desync
- Each DELETE is now processed immediately, keeping engine and screen state in sync
- No flicker or lag, <5ms per operation, all test cases pass

#### Quality Assurance
- All accessibility and backspace scenarios tested and passed
- No performance regression: <5ms per DELETE, 60fps maintained
- Documentation updated for all changes

#### References
- Pull Requests: #15 (Accessibility & Backspace Fix), #16 (Release to main)
- Issue: #13 (Backspace corruption & permission bug)
- Full syllable parsing cache completion
- Settings UI panel improvements
- Auto-update mechanism
- WASM target support
- Windows platform support enhancements

---

## [1.2.0] - 2025-12-21

### 🎨 Complete Rebranding & Infrastructure Update

#### Brand Identity
- **Official Name:** Gõ Việt (GoxViet)
  - Brand: **Gõ Việt**
  - Display/App Name: **GoxViet**
  - Code/Repo: **goxviet**
  - Bundle ID: `com.goxviet.ime`
- **New Paths:**
  - Log Directory: `~/Library/Logs/GoxViet/`
  - Rust Library: `libgoxviet_core.a`
  - Xcode Project: `platforms/macos/goxviet/`
  - Homebrew Cask: `goxviet`

#### Changed
- **[BREAKING]** Complete rebranding from "Vietnamese IME" to "Gõ Việt (GoxViet)"
  - All source code, documentation, and scripts updated
  - New app bundle identifier: `com.goxviet.ime`
  - New log path: `~/Library/Logs/GoxViet/`
  - New Homebrew cask name: `goxviet`
- **[BREAKING]** Project structure reorganized
  - `platforms/macos/VietnameseIMEFast/` → `platforms/macos/goxviet/`
  - `libvietnamese_ime_core.a` → `libgoxviet_core.a`
  - All build scripts and deployment processes updated

#### Updated
- **Documentation Suite (50+ files):**
  - All markdown files updated with new branding
  - Created `docs/project/BRANDING_UPDATE_SUMMARY.md`
  - Created `docs/project/LOG_PATH_MIGRATION.md`
  - Created `scripts/BRANDING_UPDATE_SUMMARY.md`
  - Updated all documentation indexes
- **Build & Deployment:**
  - All build scripts updated for new paths
  - Xcode project configuration updated
  - Rust core FFI updated
  - Distribution scripts modernized
- **Consistency:**
  - Zero references to old names in active code
  - Legacy files archived with clear historical markers
  - All menu items, dialogs, and UI updated

#### Fixed
- Xcode bridging header path updated for new structure
- Linker settings corrected for `libgoxviet_core.a`
- All script paths and references validated

#### Verified
- ✅ Rust core builds successfully (92/93 unit tests passed)
- ✅ Xcode/macOS app builds and runs correctly
- ✅ App initializes Rust core properly
- ✅ All logs confirm successful operation with new branding
- ✅ No old references remain in active codebase

#### Migration Notes
**For Users:**
- Uninstall old version: `brew uninstall --cask vietnamese-ime-fast`
- Install new version: `brew install --cask goxviet`
- Settings and preferences will be preserved

**For Developers:**
- Update all import paths and references
- Use new bundle ID: `com.goxviet.ime`
- Use new log path: `~/Library/Logs/GoxViet/`
- See `docs/project/BRANDING_UPDATE_SUMMARY.md` for complete migration guide

---

## [1.0.2] - 2025-12-20

### 🚀 Core Performance Optimizations

#### Added - Stroke & Pattern Optimization
- **87-95% faster stroke processing**
  - Fast path for 78% of operations (< 1ms each)
  - 3-level validation strategy (fast/basic/full)
  - Early rejection for invalid patterns saves 2-3ms per rejection
- **Performance achievements:**
  - "dd" → "đ": 87% faster (1.5ms → 0.2ms)
  - "w" → "ư": 95% faster (1.8ms → 0.1ms)
  - "nw" → "nư": 90% faster (2.0ms → 0.2ms)
  - Simple backspace: 91% faster (3.2ms → 0.3ms)
  - Complex backspace: 53% faster (4.5ms → 2.1ms)

#### Added - Rapid Keystroke Handling
- **Sub-16ms latency at 10+ keys/second**
  - Syllable boundary caching with 92% hit rate
  - Smart backspace path selection (68% fast path coverage)
  - Rebuild optimization: only affected syllable, not entire buffer
  - DELETE with cache: 75% faster (3.2ms → 0.8ms)
- **Rapid typing performance:**
  - "thuongj" (6 keys): 8.2ms total ✅
  - "dduwowcj" (8 keys): 12.4ms total ✅
  - "muoiwf" (6 keys): 9.1ms total ✅
  - All sequences < 16ms target achieved

#### Added - Pattern Validation Strategy
- **3-level intelligent validation:**
  - Level 1 (Fast Path): No validation - 78% of operations
  - Level 2 (Basic): Structure check only - 15% of operations
  - Level 3 (Full): Complete validation - 7% of operations
- **Invalid pattern detection:**
  - Breve + vowel (ăi, ăo, ău, ăy) - rejected early
  - Missing circumflex (eu without ê)
  - Spelling rules (ce, ci, cy, ka, ko, ku)

### Performance Metrics Summary
- ✅ **Fast Path Coverage:** 78% (target: 70%)
- ✅ **Sub-millisecond Ops:** 93% < 1ms (target: 90%)
- ✅ **Cache Hit Rate:** 92% for boundary detection
- ✅ **Max Latency:** < 5ms (target: < 16ms)
- ✅ **100% operations:** < 5ms
- ✅ **Zero heap allocations** in hot path

### Documentation
- **New comprehensive guides (1,200+ lines):**
  - `docs/STROKE_OPTIMIZATION.md` (265 lines)
  - `docs/RAPID_KEYSTROKE_HANDLING.md` (343 lines)
  - `docs/PATTERN_OPTIMIZATION_SUMMARY.md` (600+ lines)
- **Updated project documentation:**
  - `docs/project/RUST_CORE_ROADMAP.md` - Updated with achievements
  - `docs/project/CHANGELOG.md` - Detailed technical changelog
  - `docs/DOCUMENTATION_STRUCTURE.md` - Added new performance docs

---

## [1.0.1] - 2025-12-20

### 🎉 Major Fixes

#### ✅ Keyboard Shortcut Toggle
**Date:** 2024-01-20  
**Feature:** Thêm tính năng shortcut (phím tắt) để chuyển đổi nhanh giữa chế độ gõ tiếng Việt và tiếng Anh

**Default Shortcut:** Control+Space (⌃Space)

**Key Features:**
- ✅ **High Priority Event Capture:** Sử dụng `.headInsertEventTap` - priority cao nhất
- ✅ **No System Conflicts:** Control+Space không xung đột với Spotlight hay system shortcuts
- ✅ **Persistent Configuration:** Lưu settings qua UserDefaults
- ✅ **Not Overrideable:** Luôn được xử lý TRƯỚC tất cả ứng dụng khác
- ✅ **Customizable:** Hỗ trợ nhiều preset shortcuts (Control+Shift+Space, Control+Option+Space, etc.)
- ✅ **Display in Menu:** Hiển thị shortcut hiện tại trong menu bar
- ✅ **Clean Architecture:** Struct-based, zero heap allocation trong hot path

**Implementation Details:**

1. **KeyboardShortcut Structure:**
   - Codable struct với keyCode + modifiers
   - Display string generation (⌃Space, ⌘⇧V, etc.)
   - System conflict detection
   - Preset shortcuts for easy selection

2. **Event Priority:**
   - CGEvent.tapCreate với `.headInsertEventTap`
   - Captured at kernel level, before app-level shortcuts
   - Event swallowed (returns nil) when matched

3. **Matching Logic:**
   - Strict keyCode + modifier matching
   - Prevents extra modifiers from matching
   - Support for modifier-only shortcuts (future)

**Files Changed:**
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/KeyboardShortcut.swift` (NEW, 240 lines)
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/InputManager.swift` (Line 20-40, 122-136, 197-204, 409-421)
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/RustBridge.swift` (Line 518-533)
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/AppDelegate.swift` (Line 50-62, 154-165, 258-278)
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/KeyboardShortcutTests.swift` (NEW, 354 lines)

**Documentation:**
- `docs/SHORTCUT_GUIDE.md` (NEW, 335 lines) - Comprehensive implementation guide
- `README.md` - Updated with shortcut feature description
- `docs/README.md` - Added SHORTCUT_GUIDE.md to navigation

**Performance Metrics:**
- ✅ Latency: ~2ms from keypress to toggle (target < 5ms)
- ✅ CPU: < 0.05% overhead per shortcut check (negligible)
- ✅ Memory: Zero allocation in hot path (struct-based)

**User Experience:**
- Press Control+Space → Status bar changes 🇻🇳 ↔️ EN instantly
- Works in ALL applications system-wide
- No configuration required (works out of box)
- Composition buffer cleared on toggle

**Preset Shortcuts Available:**
- ⌃Space - Control+Space (Default, no conflicts)
- ⌘Space - Command+Space (⚠️ Conflicts with Spotlight)
- ⌃⇧Space - Control+Shift+Space (No conflicts)
- ⌃⌥Space - Control+Option+Space (No conflicts)
- ⌃⇧V - Control+Shift+V (No conflicts)

**Testing Checklist:**
- ✅ Toggle behavior works correctly
- ✅ Status bar updates immediately
- ✅ State persists across focus changes
- ✅ No conflicts with VSCode, Terminal, Slack shortcuts
- ✅ Rapid toggling (10× quickly) - no crashes
- ✅ Extra modifiers do not match (Control+Shift+Space ≠ Control+Space)
- ✅ Composition buffer cleared on toggle

**Impact:** ✅ HIGH PRIORITY - Người dùng giờ có thể chuyển đổi gõ Việt/English nhanh chóng với Control+Space, không bị override bởi bất kỳ ứng dụng nào!

---

#### ✅ Editor Performance Optimization: VSCode & Zed
**Date:** 2024-01-20  
**Issue:** Xóa ký tự trong VSCode, Zed, Sublime Text bị chậm (~14ms per backspace) mặc dù Rust core đã được tối ưu xuống 1-3ms
- VSCode/Zed: 14ms × 10 chars = 140ms lag khi xóa
- User experience: Noticeable lag, not native-like
- Rust optimization bị waste bởi Swift layer delays

**Root Cause:**
- VSCode bị phân loại nhầm vào `electronApps` với `.slow` method
- Sử dụng delays cao: (3ms, 8ms, 3ms) = 14ms per backspace
- Editors hiện đại có fast text buffers, không cần delays
- Delays cao gây lag không cần thiết

**Solutions:**

1. **Instant Injection Method:**
   - Tạo `.instant` enum case với zero delays
   - `injectViaInstant()` - batch backspaces consecutively, no delays
   - Chỉ 2ms settle time thay vì 5ms

2. **Separate Modern Editors:**
   - Tạo `modernEditors` list riêng với instant method
   - VSCode, Zed, Sublime → `.instant` (0, 0, 0)
   - Remove VSCode từ `electronApps` list
   - Terminals vẫn dùng `.slow` (no regression)

3. **Batch Backspace Helper:**
   - `postBackspaces()` - gửi nhiều backspace cùng lúc
   - Giảm event loop overhead
   - Tối ưu `injectViaBackspace()` với fast path khi delays = 0

**Files Changed:**
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/RustBridge.swift` (Line 59) - Added `.instant` case
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/RustBridge.swift` (Line 82-96) - Updated injectSync() switch
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/RustBridge.swift` (Line 98-128) - Implemented injectViaInstant() & postBackspaces()
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/RustBridge.swift` (Line 130-145) - Optimized injectViaBackspace()
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/RustBridge.swift` (Line 538-558) - Created modernEditors list
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/RustBridge.swift` (Line 599-607) - Removed VSCode from electronApps

**Documentation:**
- `docs/PERFORMANCE_OPTIMIZATION_GUIDE.md` - Full implementation guide (430+ lines)
- `docs/PERFORMANCE_SUMMARY.md` - Quick summary
- `docs/PERFORMANCE_COMPARISON.md` - Visual benchmarks
- `docs/EDITOR_PERFORMANCE_OPTIMIZATION.md` - Editor-specific details
- `test-performance.sh` - Performance benchmark script

**Performance Gains:**
- ✅ 47× faster deletion in editors (140ms → < 3ms for 10 chars)
- ✅ 63× faster for full Vietnamese words (190ms → < 3ms)
- ✅ Zero delays between events (14ms → 0ms)
- ✅ Native-like experience (instant deletion)
- ✅ No regression in other apps (terminals, browsers)
- ✅ Latency: 14ms → < 1ms per backspace

**Supported Editors:**
- Visual Studio Code, Zed, Sublime Text 3/4, Nova, VSCodium, CotEditor, Atom

**Impact:** ✅ CRITICAL - VSCode, Zed, và Sublime Text giờ gõ tiếng Việt nhanh như native, xóa ký tự instant!

---

#### ✅ Performance Fix: Backspace Optimization
**Date:** 2024-01-XX  
**Issue:** Khi xóa nhiều ký tự liên tiếp, hiệu suất giảm dần rõ rệt (càng xóa càng chậm)
- Xóa 10 ký tự → 100+ CGEvents → 100-200ms latency
- User experience: Noticeable lag, sluggish feel

**Root Cause:**
- Mỗi backspace rebuild TOÀN BỘ buffer từ đầu → O(n)
- Inject toàn bộ text thay vì 1 backspace → O(n) events
- Complexity: O(n²) cho n lần backspace!

**Solutions:**

1. **Smart Backspace:**
   - Chỉ rebuild khi cần thiết (character có tone/mark/stroke)
   - Xóa ký tự thường: O(1) thay vì O(n)
   - Không rebuild → chỉ 1 backspace event

2. **Syllable-based Rebuild:**
   - Rebuild từ syllable boundary thay vì toàn bộ buffer
   - O(syllable_size) thay vì O(buffer_size)
   - Typically 2-8 chars thay vì 10-50 chars

**Files Changed:**
- `core/src/engine/mod.rs` (Line 362-402) - Smart backspace logic
- `core/src/engine/mod.rs` (Line 1384-1416) - find_last_syllable_boundary()

**Documentation:**
- `docs/PERFORMANCE_FIX.md` - Chi tiết đầy đủ về performance optimization
- `docs/PERFORMANCE_FIX_SUMMARY.md` - Quick summary

**Performance Gains:**
- ✅ 3-15× faster backspace
- ✅ 67-90% reduction in CGEvents
- ✅ Smooth, lag-free deletion
- ✅ Latency: 10-20ms → 1-3ms per backspace

**Impact:** ✅ CRITICAL - Backspace giờ mượt mà và nhanh như native!

---

#### ✅ Fix: Backspace không hoạt động trên VSCode và Zed
**Date:** 2024-01-XX  
**Issue:** Khi gõ Telex, có thể gõ được nhưng không thể xóa bằng phím Backspace
- Ví dụ 1: Gõ "gõ " (có space) → Backspace lần 1 xóa space ✅ → Backspace lần 2-3 KHÔNG xóa được "õ" và "g" ❌
- Ví dụ 2: Gõ "được không" → Xóa "g" → Kết quả sai: "được kkhôn" ❌ (thay vì "được khôn")

**Root Causes (4 vấn đề):**

1. **Swift layer không thông báo engine:**
   - Code cũ không gọi `ime_key()` khi user nhấn Backspace
   - Engine buffer không đồng bộ với màn hình

2. **Rust engine không rebuild buffer:**
   - Khi pop character, engine return `None` thay vì rebuild
   - Swift layer không biết phải hiển thị text gì
   - Gây ra "stuck" không xóa được trên VSCode/Zed

3. **Backspace count sai - đếm buffer thay vì screen:**
   - Hàm `rebuild_from()` đếm buffer.len() SAU khi pop
   - Thiếu 1 ký tự → Xóa 9/10 ký tự → "được không" thành "được kkhôn"
   - **VẤN ĐỀ CRITICAL:** Phải đếm old_length TRƯỚC khi pop!

4. **System backspace không hoạt động với manual injection:**
   - Dựa vào system backspace → Không hoạt động với manually injected text
   - Cần inject backspace manually qua CGEvent

**Solutions:**

1. **Swift fix (`InputManager.swift`):**
   - Gọi `ime_key(backspace)` để thông báo engine
   - Inject backspace manually thay vì dựa vào system
   - Handle cả restore và delete cases

2. **Rust fix (`engine/mod.rs`):**
   - Lưu `old_length` TRƯỚC khi pop character
   - Rebuild buffer với `rebuild_from_with_backspace(0, old_length)`
   - Return `Result::send(old_length, chars)` với backspace count chính xác
   - Swift layer nhận và inject đúng text

3. **Rust new function (`engine/mod.rs`):**
   - Thêm hàm `rebuild_from_with_backspace()` với explicit backspace count
   - Fix bug "được kkhôn" bằng cách xóa đúng số ký tự trên screen

**Files Changed:**
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/InputManager.swift` (Line 264-320)
- `core/src/engine/mod.rs` (Line 357-375) - Lưu old_length và gọi rebuild_from_with_backspace
- `core/src/engine/mod.rs` (Line 1334-1357) - Hàm mới rebuild_from_with_backspace()

**Documentation:**
- `docs/BACKSPACE_FIX.md` - Chi tiết về 4 bugs và giải pháp (500+ dòng)
- `docs/TEST_BACKSPACE.md` - Test checklist (14 test cases)
- `docs/BACKSPACE_FIX_SUMMARY.md` - Summary ngắn gọn
- `docs/BACKSPACE_QUICK_TEST.md` - Quick test guide
- `docs/README_FIX_BACKSPACE.md` - Overview

**Impact:** ✅ CRITICAL - Backspace giờ hoạt động hoàn hảo trên mọi ứng dụng!
- Fix bug "stuck" trên VSCode/Zed
- Fix bug "được kkhôn" (backspace count sai)

**Note:** Performance được tối ưu thêm trong "Performance Fix: Backspace Optimization"

---

#### ✅ Fix: Ứng dụng không phản hồi phím
**Date:** 2024-01-XX (Previous)  
**Issue:** App build thành công nhưng không có phản ứng khi gõ phím  
**Root Cause:** Event tap callback không được gọi do vấn đề với Accessibility permission

**Solution:**
- Thêm proper Accessibility permission handling
- Sửa event tap creation và callback

**Files Changed:**
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/InputManager.swift`

**Documentation:**
- `docs/FIX_SUMMARY.md`

---

#### ✅ Fix: Telex không chuyển đổi ký tự
**Date:** 2024-01-XX (Previous)  
**Issue:** Gõ `aa` không ra `â`, gõ `vieets` không ra `việt`  
**Root Cause:** 
- Bridging header định nghĩa sai kích thước array `chars[32]` thay vì `chars[64]`
- Character extraction logic bị lỗi do size mismatch

**Solution:**
- Sửa bridging header: `uint32_t chars[64];`
- Update character extraction logic trong `InputManager.swift`

**Files Changed:**
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/VietnameseIMEFast-Bridging-Header.h`
- `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/InputManager.swift`

**Documentation:**
- `docs/TELEX_FIX_SUMMARY.md`
- `docs/TELEX_FIX_FINAL.md`
- `docs/TELEX_VERIFICATION.md`

**Impact:** ✅ CRITICAL - Telex giờ hoạt động hoàn hảo!

---

## [0.1.0] - 2024-Q3

### ✨ Features

- ✅ Rust core engine với hiệu suất cao (< 16ms latency)
- ✅ Keyboard shortcut toggle (Control+Space) - High priority, không bị override
- ✅ Hỗ trợ Telex input method
- ✅ Hỗ trợ VNI input method  
- ✅ Modern tone style và Old tone style
- ✅ Backspace-after-space (restore word)
- ✅ ESC key để restore raw ASCII
- ✅ Word boundary shortcuts
- ✅ Per-app IME state (remember enabled/disabled per app)
- ✅ Smart text injection (detect app-specific method)
- ✅ Memory safe (100% Rust core)
- ✅ Performance optimized (< 3ms latency per keystroke)

### 🎯 Platform Support

- ✅ macOS (tested on macOS 14+)
- ⏳ Windows (planned)

### 📝 Documentation

- `README.md` - Project overview
- `docs/README.md` - Documentation index (25 files, 6,500+ lines)
- `docs/QUICK_START.md` - 5-minute getting started guide
- `docs/TESTING_GUIDE.md` - Comprehensive testing guide
- `docs/PERFORMANCE_OPTIMIZATION_GUIDE.md` - Full performance implementation guide
- `docs/PERFORMANCE_COMPARISON.md` - Visual benchmarks and metrics
- `docs/BACKSPACE_FIX.md` - Complete backspace fix documentation (500+ lines)
- `docs/TELEX_FIX_FINAL.md` - Telex conversion fix details
- `docs/SHORTCUT_GUIDE.md` - Keyboard shortcut configuration & priority (335 lines)
- `docs/IMPLEMENTATION_COMPLETE.md` - Integration details
- And 17 more documentation files in `docs/`

### 🧪 Testing

- ✅ Basic Telex transforms (aa→â, aw→ă, oo→ô, etc.)
- ✅ Tone marks (s→sắc, f→huyền, r→hỏi, x→ngã, j→nặng)
- ✅ Complex transforms (ươ, uô, etc.)
- ✅ Backspace handling (restore previous state)
- ✅ Backspace-after-space (restore word)
- ✅ ESC restore (raw ASCII)
- ✅ Multi-app support (TextEdit, VSCode, Zed, Terminal)
- ✅ Performance testing (backspace latency < 3ms)
- ✅ 14 comprehensive backspace test cases

---

### Known Issues (Historical - Fixed in v1.0.2)

### ⚠️ Compatibility

- **Electron apps:** May need `slow` injection method (higher delays)
- **Terminal apps:** May need `slow` injection method
- **Browser address bars:** May need `selection` injection method
- **Spotlight:** May need `autocomplete` injection method

**Note:** Smart injection detection tự động xử lý hầu hết trường hợp.

### 🔧 Workarounds

Nếu gặp vấn đề với app cụ thể:
1. Check log: `tail -f /tmp/vietnameseime.log`
2. Điều chỉnh injection delays trong `RustBridge.swift`
3. Thêm app vào detection logic trong `detectMethod()`

---

## Future Plans

### 🚀 Phase 1: Core Features ✅ COMPLETE

- [x] **Keyboard shortcut toggle (Control+Space)** - ✅ DONE!
- [x] High-priority event capture (never overridden)
- [x] Persistent shortcut configuration
- [x] System-wide operation
- [x] Comprehensive documentation (2,900+ lines)

### ✅ Phase 2: Core Performance ✅ COMPLETE (2025-12-20)
- [x] Stroke optimization (87-95% faster)
- [x] Rapid keystroke handling (sub-16ms latency)
- [x] Pattern validation strategy (93% ops < 1ms)
- [x] Syllable boundary caching (92% hit rate)
- [x] Memory optimization (zero heap allocations)
- [x] Comprehensive benchmarking and metrics
- [x] Full documentation suite (1,200+ lines)

### 🚀 Phase 3: Shortcut Customization 🎯 NEXT

**Settings UI Panel:**
- [ ] Settings window with tabbed interface
- [ ] Visual shortcut recorder (like macOS System Settings)
  - Click to record shortcut
  - Press any key combination
  - Visual feedback during recording
  - Save/Cancel buttons
- [ ] Live preview of shortcut conflicts
  - Show system shortcut conflicts (Spotlight, etc.)
  - Show app-specific conflicts (VSCode, Terminal, etc.)
  - Conflict resolution suggestions
- [ ] Preset shortcuts selector
  - Control+Space (default, no conflicts)
  - Control+Shift+Space
  - Control+Option+Space
  - Control+Shift+V
  - Custom shortcut input
- [ ] Test shortcut button
  - Verify shortcut works
  - Test in current app
  - Show success/failure feedback
- [ ] Reset to default option
- [ ] Shortcut display in menu bar

**Advanced Shortcut Features:**
- [ ] Multiple shortcut support
  - Primary shortcut (Control+Space)
  - Secondary shortcut (backup)
  - Both work independently
- [ ] Modifier-only shortcuts
  - Double-tap Shift detection
  - Double-tap Control detection
  - Configurable timing threshold
- [ ] Per-app shortcut overrides
  - Different shortcut for VSCode
  - Different shortcut for Terminal
  - Auto-detect frontmost app
  - Override list management
- [ ] Shortcut profiles
  - Profile 1: Developer (Control+Shift+Space)
  - Profile 2: Writer (Control+Space)
  - Profile 3: Custom
  - Quick profile switcher in menu
- [ ] Import/export configurations
  - Export to JSON file
  - Import from JSON file
  - Share configs between devices

**Conflict Detection & Resolution:**
- [ ] Real-time system shortcut conflict warnings
  - Detect Spotlight (Cmd+Space)
  - Detect App Switcher (Cmd+Tab)
  - Detect other system shortcuts
  - Visual warning in settings
- [ ] App-specific conflict detection
  - VSCode shortcuts database
  - Terminal shortcuts database
  - Popular apps support
  - Show which apps conflict
- [ ] Automatic conflict resolution
  - Suggest alternative shortcuts
  - Rank suggestions by popularity
  - One-click apply suggestion
- [ ] Disable conflicting app shortcuts
  - Option to disable VSCode's Control+Space
  - Automatic app preference modification
  - Backup original settings

### 🚀 Phase 4: Enhanced Features

- [ ] Dictionary/Autocomplete
- [ ] Emoji picker
- [ ] Multiple dictionaries (North/South/Formal)
- [ ] Statistics (words typed, speed, etc.)
- [ ] Cloud sync settings (including shortcuts)
- [ ] Windows platform support

### 🎨 UI Improvements

- [ ] Menu bar icon với quick toggle
- [ ] Visual feedback khi toggle IME
- [ ] Candidate window (for autocomplete)
- [ ] Settings window
- [ ] About window

### ⚡ Performance

- [x] **Optimize backspace latency** - ✅ DONE! (< 3ms achieved)
- [x] **Smart rebuild strategy** - ✅ DONE! (syllable-based)
- [x] **Editor instant injection** - ✅ DONE! (47× faster)
- [ ] Further optimize text injection
- [ ] Reduce memory footprint
- [ ] Battery impact optimization
- [ ] Startup time optimization

---

## Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Update documentation in `docs/`
6. Submit a pull request

**Documentation Guidelines:**
- All documentation files use UPPER_CASE names
- Place all docs in `docs/` directory
- Update `docs/README.md` when adding new docs
- Include code examples with file paths and line numbers
- Document performance changes with benchmarks

---

## Performance Achievements (v1.0.2 - 2025-12-20)

### Latency Improvements
| Operation | Before | After | Improvement |
|-----------|--------|-------|-------------|
| Stroke "dd"→"đ" | 1.5ms | 0.2ms | **87% faster** |
| W-as-vowel "w"→"ư" | 1.8ms | 0.1ms | **95% faster** |
| Simple backspace | 3.2ms | 0.3ms | **91% faster** |
| Complex backspace | 4.5ms | 2.1ms | **53% faster** |
| DELETE with cache | 3.2ms | 0.8ms | **75% faster** |

### Coverage Statistics
- **78%** operations use fast path (< 1ms)
- **93%** operations complete in < 1ms
- **100%** operations complete in < 5ms
- **92%** cache hit rate for syllable boundaries

### User Experience
- ✅ Native-like typing experience
- ✅ Instant response for 93% of operations
- ✅ No perceptible lag during rapid typing
- ✅ Smooth backspace operation
- ✅ Sub-16ms latency at 10+ keys/second

---

## License

MIT License - See LICENSE file for details

---

## Credits

- **Core Engine:** Developed with research from open-source Vietnamese IME projects
- **Maintainers:** Vietnamese IME Team
- **Contributors:** Community contributors

**Acknowledgments:** This project benefits from the collective knowledge and experience of the Vietnamese IME development community.

---

**Last Updated:** 2025-12-21  
**Version:** 1.2.0  
**Status:** Active Development

---

## Version Links

[Unreleased]: https://github.com/your-repo/goxviet/compare/v1.2.0...HEAD  
[1.2.0]: https://github.com/your-repo/goxviet/compare/v1.0.2...v1.2.0  
[1.0.2]: https://github.com/your-repo/goxviet/compare/v1.0.1...v1.0.2
