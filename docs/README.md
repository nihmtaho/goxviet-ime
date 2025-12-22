# 📚 Gõ Việt (GoxViet) Documentation

Comprehensive documentation for Gõ Việt (GoxViet) project, organized by topic for easy navigation.

**Total Documentation:** 64 files | **Lines:** 19,100+ | **Last Updated:** 2025-12-21

---

## 📁 Documentation Structure

---

### 📚 TỔNG QUAN PHÂN LOẠI TÀI LIỆU

#### 1. **Tài liệu TÍNH NĂNG** (Features, Shortcuts, Accessibility...)
- Mô tả các chức năng chính: Smart Per-App, phím tắt, Accessibility, hướng dẫn sử dụng nhanh, checklist kiểm thử.
- Mục tiêu: Giúp người dùng và lập trình viên hiểu rõ cách sử dụng, cấu hình, mở rộng các tính năng.

#### 2. **Tài liệu IMPROVEMENT** (Performance, Optimization, Auto-Restore...)
- Tập trung tối ưu hiệu năng, cải thiện thuật toán, nâng cấp trải nghiệm: Smart Backspace, English Auto-Restore, Memory Optimization, Benchmark.
- Mục tiêu: Đảm bảo bộ gõ luôn đạt hiệu năng cao nhất, trải nghiệm mượt mà, cập nhật cải tiến mới.

#### 3. **Tài liệu FIX BUGS** (Fixes, Patch, Solution...)
- Tổng hợp các giải pháp khắc phục lỗi thực tế: Backspace, Arrow Keys, Telex, Menu Bar, Safari/Browser, checklist xác nhận fix.
- Mục tiêu: Đảm bảo mọi lỗi đều có hướng dẫn fix, xác nhận lại bằng test, truy vết lịch sử sửa lỗi.

---

**Lưu ý:**  
- Tài liệu được phân loại rõ ràng theo thư mục: `shortcuts/`, `performance/`, `fixes/`, `project/`, `getting-started/`, v.v.
- Khi thêm tài liệu mới, luôn cập nhật vào các file mục lục (`README.md`, `DOCUMENTATION_STRUCTURE.md`, `STRUCTURE_VISUAL.md`) để đảm bảo đồng bộ và dễ tra cứu.

---

```
docs/
├── getting-started/     # 🚀 Start here for quick setup
├── shortcuts/           # ⌨️ Keyboard shortcut features
├── fixes/               # 🔧 Bug fixes and solutions
├── performance/         # ⚡ Performance optimization
├── project/             # 📋 Project management
├── release-note/        # 📝 Release notes for versions
└── archive/             # 📦 Historical documents
```

---

## 🚀 Getting Started

**New to the project? Start here!**

### Quick Start
- **[getting-started/QUICK_START.md](getting-started/QUICK_START.md)** - 5-minute setup guide
- **[getting-started/TESTING_GUIDE.md](getting-started/TESTING_GUIDE.md)** - Comprehensive testing guide

### Deployment Guides ⭐ NEW
- **[DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md)** - Full deployment with code signing & notarization
- **[HOMEBREW_DEPLOYMENT.md](HOMEBREW_DEPLOYMENT.md)** - Homebrew deployment (FREE, no Apple Developer needed)
- **[DEPLOYMENT_CHECKLIST.md](DEPLOYMENT_CHECKLIST.md)** - Quick deployment checklist
- **[DEPLOYMENT_QUICK_REFERENCE.md](DEPLOYMENT_QUICK_REFERENCE.md)** - One-page reference

### Essential Reading (15 minutes)
1. Read `QUICK_START.md` - Build and run the app
2. Read `shortcuts/SHORTCUT_QUICK_START.md` - Learn Control+Space toggle
3. Try typing Vietnamese - Test basic functionality

---

## ⌨️ Keyboard Shortcuts

**Complete keyboard shortcut system with customization roadmap**

### Main Documentation
- **[MENU_TOGGLE_IMPLEMENTATION.md](MENU_TOGGLE_IMPLEMENTATION.md)** - Menu toggle with SwiftUI (389 lines) ⭐
- **[shortcuts/SHORTCUT_GUIDE.md](shortcuts/SHORTCUT_GUIDE.md)** - Complete implementation guide (335 lines)
- **[shortcuts/SHORTCUT_QUICK_START.md](shortcuts/SHORTCUT_QUICK_START.md)** - Quick start (223 lines)

### Implementation Details
- **[shortcuts/implementation/SHORTCUT_IMPLEMENTATION_SUMMARY.md](shortcuts/implementation/SHORTCUT_IMPLEMENTATION_SUMMARY.md)** - Technical overview (460 lines)

### Testing & Verification
- **[shortcuts/testing/TEST_SHORTCUT.md](shortcuts/testing/TEST_SHORTCUT.md)** - 20 test cases (629 lines)
- **[shortcuts/testing/SHORTCUT_VERIFICATION_CHECKLIST.md](shortcuts/testing/SHORTCUT_VERIFICATION_CHECKLIST.md)** - Pre-deployment checklist (564 lines)

### Future Roadmap
- **[shortcuts/roadmap/SHORTCUT_CUSTOMIZATION_ROADMAP.md](shortcuts/roadmap/SHORTCUT_CUSTOMIZATION_ROADMAP.md)** - 4-phase roadmap (966 lines) ⭐
- **[shortcuts/roadmap/SHORTCUT_ROADMAP_SUMMARY.md](shortcuts/roadmap/SHORTCUT_ROADMAP_SUMMARY.md)** - Quick overview (276 lines)

**Key Features:**
- ✅ Default Control+Space toggle (Phase 1 - Complete)
- 🎯 Settings UI & Customization (Phase 2 - Next, 10 weeks)
- 🔮 Advanced Features: Multiple shortcuts, per-app overrides (Phase 3, 11 weeks)
- 🌟 Polish & Optimization (Phase 4, 6 weeks)

---

## 🎯 Features

**Advanced features for seamless workflow**

### Smart Per-App Mode
- **[SMART_PER_APP_MODE.md](SMART_PER_APP_MODE.md)** - Automatic per-app Vietnamese input memory (436 lines) ⭐ NEW

**Key Features:**
- ✅ Automatic mode switching per application
- ✅ Remembers Vietnamese input preference for each app
- ✅ Efficient storage (only stores disabled apps)
- ✅ NSWorkspace integration for app detection
- ✅ Settings UI with toggle control
- ✅ Clear per-app settings functionality

---

## 🌐 Accessibility & Browser Support

**Intelligent text input across all applications using macOS Accessibility API**

### Main Documentation
- **[ACCESSIBILITY_QUICK_REFERENCE.md](ACCESSIBILITY_QUICK_REFERENCE.md)** - Quick reference guide (265 lines) ⭐ START HERE
- **[ACCESSIBILITY_API_SUPPORT.md](ACCESSIBILITY_API_SUPPORT.md)** - Complete Accessibility API implementation guide (691 lines) ⭐ NEW
- **[BROWSER_SUPPORT.md](BROWSER_SUPPORT.md)** - Comprehensive browser support (38 browsers) (422 lines) ⭐ NEW
- **[BROWSER_AUTOCOMPLETE_FIX.md](BROWSER_AUTOCOMPLETE_FIX.md)** - Browser placeholder fix & performance optimization (713 lines) ⭐ NEW v1.1.0
- **[TEST_ACCESSIBILITY_API.md](TEST_ACCESSIBILITY_API.md)** - Testing guide for Accessibility API features (637 lines) ⭐ NEW
- **[CHANGELOG_ACCESSIBILITY_API.md](CHANGELOG_ACCESSIBILITY_API.md)** - Changelog for Accessibility API feature (v1.1.0, 550+ lines) ⭐ NEW

**Key Features:**
- ✅ **Spotlight Support** - Special autocomplete method with Forward Delete
- ✅ **Arc Browser** - Full support with selection method (< 8ms latency)
- ✅ **38 Browsers** - Chrome, Firefox, Safari, Edge, Brave, Opera, and more
- ✅ **Smart Detection** - Automatic app/context detection via Accessibility API
- ✅ **6 Injection Methods** - Instant, Fast, Slow, Selection, BrowserSelection, Autocomplete
- ✅ **Autocomplete Fix** - BrowserSelection method clears placeholders before text replacement (v1.1.0)
- ✅ **Address Bar Optimization** - 100% success rate on browser search bars (v1.1.0)
- ✅ **< 8ms Latency** - Browser address bars achieve 2x better than 16ms target

**v1.1.0 Performance Improvements:**
- 🚀 **50% faster** single-keystroke processing (1-3 chars: 8.2ms → 4.1ms)
- 🚀 **51% faster** backspace operations (5.7ms → 2.8ms)
- 🚀 **50-100% reduction** in heap allocations per keystroke
- 🚀 **100% success rate** on browser autocomplete fields (was 65-80%)

**Supported Browser Families:**
- 🌐 Chromium-based (13): Chrome, Brave, Edge, Vivaldi, Yandex
- 🦊 Firefox-based (8): Firefox, Waterfox, LibreWolf, Floorp, Tor
- 🧭 Safari/WebKit (3): Safari, Safari Tech Preview, Orion
- 🎭 Opera-based (5): Opera, Opera GX, Opera Air, Opera Next
- 🚀 Modern (9): Arc, Zen, SigmaOS, Sidekick, DuckDuckGo, Comet

---

## ⚡ Performance Optimization

**Comprehensive performance guides and benchmarks**

### Main Guides
- **[performance/PERFORMANCE_INDEX.md](performance/PERFORMANCE_INDEX.md)** - Master index for all performance docs
- **[performance/PERFORMANCE_README.md](performance/PERFORMANCE_README.md)** - Overview
- **[performance/OPTIMIZATION_README.md](performance/OPTIMIZATION_README.md)** - Quick start
- **[performance/QUICK_REFERENCE_OPTIMIZATION.md](performance/QUICK_REFERENCE_OPTIMIZATION.md)** - Quick reference card

### Detailed Guides
- **[performance/guides/PERFORMANCE_OPTIMIZATION_GUIDE.md](performance/guides/PERFORMANCE_OPTIMIZATION_GUIDE.md)** - Full implementation guide (430+ lines) ⭐
- **[performance/guides/EDITOR_PERFORMANCE_OPTIMIZATION.md](performance/guides/EDITOR_PERFORMANCE_OPTIMIZATION.md)** - VSCode/Zed optimization
- **[performance/guides/PERFORMANCE_FIX.md](performance/guides/PERFORMANCE_FIX.md)** - Performance fix details
- **[STROKE_OPTIMIZATION.md](STROKE_OPTIMIZATION.md)** - Stroke & pattern validation optimization ⭐ NEW
- **[RAPID_KEYSTROKE_HANDLING.md](RAPID_KEYSTROKE_HANDLING.md)** - Rapid keystroke handling (< 16ms) ⭐ NEW
- **[MEMORY_OPTIMIZATION.md](MEMORY_OPTIMIZATION.md)** - Memory efficiency with RawInputBuffer ⭐ NEW

### Summaries & Benchmarks
- **[performance/summaries/PERFORMANCE_SUMMARY.md](performance/summaries/PERFORMANCE_SUMMARY.md)** - Detailed summary
- **[performance/summaries/PERFORMANCE_COMPARISON.md](performance/summaries/PERFORMANCE_COMPARISON.md)** - Visual benchmarks ⭐
- **[performance/summaries/PERFORMANCE_FIX_SUMMARY.md](performance/summaries/PERFORMANCE_FIX_SUMMARY.md)** - Fix summary
- **[performance/summaries/EDITOR_OPTIMIZATION_SUMMARY.md](performance/summaries/EDITOR_OPTIMIZATION_SUMMARY.md)** - Editor optimization summary
- **[performance/summaries/OPTIMIZATION_STATUS_SUMMARY.md](performance/summaries/OPTIMIZATION_STATUS_SUMMARY.md)** - Status summary
- **[PATTERN_OPTIMIZATION_SUMMARY.md](PATTERN_OPTIMIZATION_SUMMARY.md)** - Pattern & stroke optimization summary ⭐ NEW
- **[MEMORY_OPTIMIZATION_SUMMARY.md](MEMORY_OPTIMIZATION_SUMMARY.md)** - Memory efficiency summary ⭐ NEW

**Achievements:**
- ✅ 47× faster deletion in modern editors (140ms → < 3ms)
- ✅ Latency: < 3ms per keystroke (95% operations < 1ms)
- ✅ Backspace: O(1) for regular chars, O(syllable) for complex
- ✅ Zero heap allocations in hot path (memory optimization)
- ✅ Stroke operations: 87% faster for common patterns (dd → đ)
- ✅ W-as-vowel: 95% faster for simple cases (w → ư)
- ✅ Rapid typing: < 16ms/keystroke at 10+ keys/sec

---

## 🔧 Bug Fixes & Solutions

**Detailed documentation for all major bug fixes**

### Accessibility Permission Fix (2 files) 🆕
- **[ACCESSIBILITY_PERMISSION_FIX.md](ACCESSIBILITY_PERMISSION_FIX.md)** - Complete fix for persistent permission checking (690+ lines) ⭐
- **[ACCESSIBILITY_PERMISSION_FIX_SUMMARY.md](ACCESSIBILITY_PERMISSION_FIX_SUMMARY.md)** - Quick summary (184 lines)

### Backspace Corruption Fix (1 file) 🆕🔥
- **[BACKSPACE_CORRUPTION_FIX.md](BACKSPACE_CORRUPTION_FIX.md)** - Critical fix for character duplication and corruption (413 lines) ⭐⭐⭐

### Memory Leak Fix (1 file) 🆕
- **[MEMORY_LEAK_FIX.md](MEMORY_LEAK_FIX.md)** - Fix for NotificationCenter observer memory leaks (439 lines) ⭐

**Fixed Issues:**
- ✅ Memory growing ~50-200KB per hour during continuous usage
- ✅ NotificationCenter observers not being removed
- ✅ Duplicate observers when settings reloaded

### Backspace Fixes (10 files)
- **[fixes/backspace/BACKSPACE_FIX.md](fixes/backspace/BACKSPACE_FIX.md)** - Complete fix documentation (500+ lines) ⭐
- **[fixes/backspace/BACKSPACE_FIX_SUMMARY.md](fixes/backspace/BACKSPACE_FIX_SUMMARY.md)** - Quick summary
- **[fixes/backspace/TEST_BACKSPACE.md](fixes/backspace/TEST_BACKSPACE.md)** - 14 test cases
- **[fixes/backspace/BACKSPACE_QUICK_TEST.md](fixes/backspace/BACKSPACE_QUICK_TEST.md)** - Quick test guide
- **[fixes/backspace/BACKSPACE_QUICK_TEST_GUIDE.md](fixes/backspace/BACKSPACE_QUICK_TEST_GUIDE.md)** - Test procedures
- **[fixes/backspace/README_FIX_BACKSPACE.md](fixes/backspace/README_FIX_BACKSPACE.md)** - Overview
- **[fixes/backspace/BACKSPACE_OPTIMIZATION_*.md](fixes/backspace/)** - Optimization guides
- **[fixes/backspace/SMART_BACKSPACE_*.md](fixes/backspace/)** - Smart backspace implementation
- **[fixes/backspace/RUST_CORE_BACKSPACE_*.md](fixes/backspace/)** - Rust core changes

**Fixed Issues:**
- ✅ Backspace not working on VSCode/Zed
- ✅ Wrong backspace count (buffer vs screen)
- ✅ Performance: 3-15× faster backspace

### Arrow Key Fixes (4 files)
- **[fixes/arrow-keys/ARROW_KEY_FIX.md](fixes/arrow-keys/ARROW_KEY_FIX.md)** - Complete fix (202 lines) ⭐
- **[fixes/arrow-keys/ARROW_KEY_FIX_SUMMARY.md](fixes/arrow-keys/ARROW_KEY_FIX_SUMMARY.md)** - Summary (102 lines)
- **[fixes/arrow-keys/ARROW_KEY_FIX_CHECKLIST.md](fixes/arrow-keys/ARROW_KEY_FIX_CHECKLIST.md)** - Checklist (119 lines)
- **[fixes/arrow-keys/BUILD_AND_TEST_ARROW_FIX.md](fixes/arrow-keys/BUILD_AND_TEST_ARROW_FIX.md)** - Build & test guide (297 lines)

**Fixed Issues:**
- ✅ Arrow keys captured instead of passed through
- ✅ Return key action mismatch

### Telex Fixes (3 files)
- **[fixes/telex/TELEX_FIX_FINAL.md](fixes/telex/TELEX_FIX_FINAL.md)** - Complete fix details
- **[fixes/telex/TELEX_FIX_SUMMARY.md](fixes/telex/TELEX_FIX_SUMMARY.md)** - Quick summary
- **[fixes/telex/TELEX_VERIFICATION.md](fixes/telex/TELEX_VERIFICATION.md)** - Verification checklist

**Fixed Issues:**
- ✅ Telex not converting (aa → â, vieets → việt)
- ✅ Bridging header size mismatch

### Menu Toggle Fixes (9 files) 🆕
- **[fixes/menubar-toggle/README.md](fixes/menubar-toggle/README.md)** - Index & overview for all toggle/focus/fix docs 🆕
- **[fixes/menubar-toggle/CHANGELOG_TOGGLE_FIX.md](fixes/menubar-toggle/CHANGELOG_TOGGLE_FIX.md)** - Changelog & version history
- **[fixes/menubar-toggle/MENUBAR_APPEARANCE_FIX.md](fixes/menubar-toggle/MENUBAR_APPEARANCE_FIX.md)** - Deprecated API fix
- **[fixes/menubar-toggle/MENUBAR_TOGGLE_CUSTOM_CONTROL.md](fixes/menubar-toggle/MENUBAR_TOGGLE_CUSTOM_CONTROL.md)** - Custom control solution
- **[fixes/menubar-toggle/MENUBAR_TOGGLE_SWIFTUI_DECISION.md](fixes/menubar-toggle/MENUBAR_TOGGLE_SWIFTUI_DECISION.md)** - SwiftUI architecture decision
- **[fixes/menubar-toggle/TOGGLE_FIX_SUMMARY.md](fixes/menubar-toggle/TOGGLE_FIX_SUMMARY.md)** - Executive summary
- **[fixes/menubar-toggle/TOGGLE_TESTING_CHECKLIST.md](fixes/menubar-toggle/TOGGLE_TESTING_CHECKLIST.md)** - Testing checklist
- **[fixes/menubar-toggle/TOGGLE_V2_SUMMARY.md](fixes/menubar-toggle/TOGGLE_V2_SUMMARY.md)** - v2.0.0 summary
- **[fixes/menubar-toggle/TOGGLE_V2.1_SUMMARY.md](fixes/menubar-toggle/TOGGLE_V2.1_SUMMARY.md)** - v2.1.0 summary
- **[fixes/menubar-toggle/TESTING_V2_FOCUS_FIX.md](fixes/menubar-toggle/TESTING_V2_FOCUS_FIX.md)** - Focus/dimming test guide

**Fixed Issues:**
- ✅ NSSwitch intermittent color loss (100% stable now)
- ✅ Blue selection highlight conflicts (completely eliminated)
- ✅ Unstable menu interactions (SwiftUI isolation)
- ✅ Dark mode color inconsistency (automatic support)
- ✅ Focus/dimming issues (full analysis, custom & SwiftUI solutions)

---

## 📋 Project Management

**Project status, roadmap, and changelog**

### Current Status
- **[project/PROJECT_STATUS.md](project/PROJECT_STATUS.md)** - Current project status and roadmap

### Deployment & Distribution
- **[DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md)** - Complete deployment guide (App Store ready)
- **[HOMEBREW_DEPLOYMENT.md](HOMEBREW_DEPLOYMENT.md)** - Homebrew deployment (no Apple Developer Account)
- **[DEPLOYMENT_CHECKLIST.md](DEPLOYMENT_CHECKLIST.md)** - Deployment checklist
- **[DEPLOYMENT_QUICK_REFERENCE.md](DEPLOYMENT_QUICK_REFERENCE.md)** - Quick reference card
- **[project/RUST_CORE_ROADMAP.md](project/RUST_CORE_ROADMAP.md)** - Rust core development roadmap
- **[project/PROJECT_RESTRUCTURE_SUMMARY.md](project/PROJECT_RESTRUCTURE_SUMMARY.md)** - Restructure summary
- **[project/LOG_PATH_MIGRATION.md](project/LOG_PATH_MIGRATION.md)** - Log path cleanup (VietnameseIME → GoxViet)
- **[project/BRANDING_UPDATE_SUMMARY.md](project/BRANDING_UPDATE_SUMMARY.md)** - Complete branding update (Vietnamese IME → Gõ Việt)

### History & Changes
- **[project/CHANGELOG.md](project/CHANGELOG.md)** - Complete project changelog (400+ lines) ⭐
- **[project/COMMIT_MESSAGE_TEMPLATE.md](project/COMMIT_MESSAGE_TEMPLATE.md)** - Commit guidelines

### Release Notes
- **[release-note/RELEASE_NOTE_1.2.0.md](release-note/RELEASE_NOTE_1.2.0.md)** - Version 1.2.0 release notes (Complete rebranding)

**Current Version:** 1.2.0  
**Status:** ✅ Production Ready (macOS)  
**Next:** Settings UI for shortcut customization

---

## 📦 Archive

**Historical documents and completed implementations**

- **[archive/FIX_SUMMARY.md](archive/FIX_SUMMARY.md)** - Initial event handling fix
- **[archive/IMPLEMENTATION_COMPLETE.md](archive/IMPLEMENTATION_COMPLETE.md)** - Integration completion
- **[archive/OPTIMIZATION_COMPLETE.md](archive/OPTIMIZATION_COMPLETE.md)** - Optimization completion summary
- **[archive/UPDATE_SUMMARY_2024.md](archive/UPDATE_SUMMARY_2024.md)** - 2024 update summary
- **[archive/RUST_CORE_NEXT_STEPS.md](archive/RUST_CORE_NEXT_STEPS.md)** - Historical next steps

---

## 🎯 Documentation by Purpose

### For New Users
1. **[getting-started/QUICK_START.md](getting-started/QUICK_START.md)** - Build and run in 5 minutes
2. **[shortcuts/SHORTCUT_QUICK_START.md](shortcuts/SHORTCUT_QUICK_START.md)** - Learn Control+Space
3. **[getting-started/TESTING_GUIDE.md](getting-started/TESTING_GUIDE.md)** - Basic testing

### For Developers
1. **[performance/guides/PERFORMANCE_OPTIMIZATION_GUIDE.md](performance/guides/PERFORMANCE_OPTIMIZATION_GUIDE.md)** - Performance implementation
2. **[shortcuts/SHORTCUT_GUIDE.md](shortcuts/SHORTCUT_GUIDE.md)** - Shortcut implementation
3. **[project/PROJECT_STATUS.md](project/PROJECT_STATUS.md)** - Architecture overview
4. **[fixes/backspace/BACKSPACE_FIX.md](fixes/backspace/BACKSPACE_FIX.md)** - Complex bug fix example

### For Testers
1. **[shortcuts/testing/TEST_SHORTCUT.md](shortcuts/testing/TEST_SHORTCUT.md)** - Shortcut test cases
2. **[fixes/backspace/TEST_BACKSPACE.md](fixes/backspace/TEST_BACKSPACE.md)** - Backspace test cases
3. **[shortcuts/testing/SHORTCUT_VERIFICATION_CHECKLIST.md](shortcuts/testing/SHORTCUT_VERIFICATION_CHECKLIST.md)** - Pre-deployment checklist

### For Project Managers
1. **[shortcuts/roadmap/SHORTCUT_CUSTOMIZATION_ROADMAP.md](shortcuts/roadmap/SHORTCUT_CUSTOMIZATION_ROADMAP.md)** - 7-month roadmap
2. **[project/CHANGELOG.md](project/CHANGELOG.md)** - What's been done
3. **[project/PROJECT_STATUS.md](project/PROJECT_STATUS.md)** - Current status

---

## 📊 Documentation Statistics

| Category | Files | Lines | Status |
|----------|-------|-------|--------|
| **Shortcuts** | 8 | 3,500+ | ✅ Complete + Roadmap |
| **Performance** | 11 | 4,000+ | ✅ Complete |
| **Fixes - Backspace** | 10 | 3,000+ | ✅ Complete |
| **Fixes - Arrow Keys** | 4 | 800+ | ✅ Complete |
| **Fixes - Telex** | 3 | 500+ | ✅ Complete |
| **Fixes - Menu Toggle** | 9 | 3,000+ | ✅ Complete 🆕 |
| **Getting Started** | 2 | 600+ | ✅ Complete |
| **Project** | 5 | 1,500+ | ✅ Complete |
| **Archive** | 5 | 1,000+ | 📦 Historical |
| **TOTAL** | **58** | **16,400+** | |

---

## 🔍 Find Information By Topic

### Accessibility & Browser Support
- Quick overview? → [ACCESSIBILITY_QUICK_REFERENCE.md](ACCESSIBILITY_QUICK_REFERENCE.md) ⭐ START HERE
- Spotlight not working? → [ACCESSIBILITY_API_SUPPORT.md](ACCESSIBILITY_API_SUPPORT.md) (Troubleshooting section)
- Arc browser support? → [BROWSER_SUPPORT.md](BROWSER_SUPPORT.md) (Arc is fully supported!)
- Which browsers supported? → [BROWSER_SUPPORT.md](BROWSER_SUPPORT.md) (38 browsers listed)
- Address bar issues? → [ACCESSIBILITY_API_SUPPORT.md](ACCESSIBILITY_API_SUPPORT.md) (Detection mechanism)
- How to test Accessibility features? → [TEST_ACCESSIBILITY_API.md](TEST_ACCESSIBILITY_API.md) (16 test cases)

## 🔍 Find Information By Topic (continued)

### Performance Issues
- **"Why is deletion slow in VSCode?"** → `performance/guides/EDITOR_PERFORMANCE_OPTIMIZATION.md`
- **"How to optimize backspace?"** → `fixes/backspace/BACKSPACE_FIX.md`
- **"What are the performance targets?"** → `performance/PERFORMANCE_README.md`

### Keyboard Shortcuts
- **"How do I toggle Vietnamese input?"** → `shortcuts/SHORTCUT_QUICK_START.md`
- **"Can I change the shortcut?"** → `shortcuts/roadmap/SHORTCUT_CUSTOMIZATION_ROADMAP.md`
- **"How to test shortcuts?"** → `shortcuts/testing/TEST_SHORTCUT.md`

### Bug Fixes
- **"Backspace not working?"** → `fixes/backspace/BACKSPACE_FIX.md`
- **"Arrow keys not working?"** → `fixes/arrow-keys/ARROW_KEY_FIX.md`
- **"Telex not converting?"** → `fixes/telex/TELEX_FIX_FINAL.md`
- **"Toggle losing color?"** → `fixes/menubar-toggle/TOGGLE_FIX_SUMMARY.md` 🆕
- **"Blue highlight appearing?"** → `fixes/menubar-toggle/MENUBAR_TOGGLE_CUSTOM_CONTROL.md` 🆕
- **"Focus/dimming issues?"** → `fixes/menubar-toggle/README.md` 🆕

### Project Status
- **"What's the current status?"** → `project/PROJECT_STATUS.md`
- **"What changed recently?"** → `project/CHANGELOG.md`
- **"What's next?"** → `shortcuts/roadmap/SHORTCUT_CUSTOMIZATION_ROADMAP.md`

---

## 🎓 Reading Order

### Beginner (30 minutes)
1. **[getting-started/QUICK_START.md](getting-started/QUICK_START.md)** - Build and run
2. **[ACCESSIBILITY_QUICK_REFERENCE.md](ACCESSIBILITY_QUICK_REFERENCE.md)** - Understand browser/app support ⭐
3. **[TEST_ACCESSIBILITY_API.md](TEST_ACCESSIBILITY_API.md)** - Test Spotlight and browsers
4. **[shortcuts/SHORTCUT_QUICK_START.md](shortcuts/SHORTCUT_QUICK_START.md)** - Learn shortcuts
5. **[SMART_PER_APP_MODE.md](SMART_PER_APP_MODE.md)** - Understand per-app mode

### Beginner (30 minutes) - Original
1. `getting-started/QUICK_START.md` - Build and run
2. `shortcuts/SHORTCUT_QUICK_START.md` - Basic usage
3. `getting-started/TESTING_GUIDE.md` - Test it works

### Intermediate (2 hours)
1. `performance/PERFORMANCE_README.md` - Performance overview
2. `shortcuts/SHORTCUT_GUIDE.md` - Shortcut details
3. `project/PROJECT_STATUS.md` - Architecture
4. `fixes/backspace/BACKSPACE_FIX.md` - Complex fix example

### Advanced (4+ hours)
1. `performance/guides/PERFORMANCE_OPTIMIZATION_GUIDE.md` - Full optimization
2. `shortcuts/roadmap/SHORTCUT_CUSTOMIZATION_ROADMAP.md` - Future development
3. All files in `fixes/backspace/` - Deep dive into bugs
4. All files in `performance/` - Performance mastery

---

## 🏆 Key Achievements

### Performance
- ✅ **47× faster** deletion in modern editors
- ✅ **< 3ms** latency per keystroke
- ✅ **O(1)** backspace for regular characters
- ✅ **Zero memory leaks**

### Features
- ✅ **Control+Space** toggle (high priority, never overridden)
- ✅ **System-wide** operation in all apps
- ✅ **Smart text injection** (app-aware)
- ✅ **Per-app IME state** (remembers per app)

### Code Quality
- ✅ **100% memory safe** (Rust core)
- ✅ **Comprehensive testing** (50+ test cases)
- ✅ **15,000+ lines** of documentation
- ✅ **Production ready** (macOS)

---

## 📞 Support

### Getting Help

**Quick Questions:**
- Check `getting-started/QUICK_START.md`
- Check `shortcuts/SHORTCUT_QUICK_START.md`

**Performance Issues:**
- Read `performance/PERFORMANCE_INDEX.md`
- Check `performance/summaries/PERFORMANCE_COMPARISON.md`

**Bug Fixes:**
- Backspace: `fixes/backspace/BACKSPACE_FIX.md`
- Arrow keys: `fixes/arrow-keys/ARROW_KEY_FIX.md`
- Telex: `fixes/telex/TELEX_FIX_FINAL.md`

### Testing
- **Unit tests:** `getting-started/TESTING_GUIDE.md`
- **Shortcut tests:** `shortcuts/testing/TEST_SHORTCUT.md`
- **Backspace tests:** `fixes/backspace/TEST_BACKSPACE.md`

### Contributing
1. Read project guidelines in `.github/instructions/`
2. Follow naming conventions (UPPER_CASE for docs)
3. Add tests for new features
4. Update relevant documentation
5. Submit pull request

---

## 🆕 Recent Updates

### Latest (2025-12-21) 🆕🆕🆕
### Latest (2025-12-21) ���️���️���️
- ✅ **Release Note 1.2.1** - Accessibility Permission & Backspace Corruption Fixes ⭐⭐⭐
  - [docs/release-note/RELEASE_NOTE_1.2.1.md](release-note/RELEASE_NOTE_1.2.1.md)
  - Fixed: Duplicate permission dialogs, permission persistence, auto-detection, log methods, and priority inversion
  - Fixed: Backspace character duplication/corruption, removed batch logic, immediate processing, <5ms latency
  - All test cases pass, documentation updated, no performance regression
- ✅ **Backspace Corruption Fix** - Fixed critical bug causing character duplication ⭐⭐⭐ CRITICAL
  - "gõ " → delete space → "gõ" (not "gg" anymore!)
  - "được" → delete "c" → "đơ" (not "đđư")
  - "đúng" → delete "g" → "đún" (not "đđú")
  - Removed flawed batch processing logic (110 lines)
  - Added simple immediate processing (45 lines)
  - No flicker, no corruption, perfect Vietnamese deletion
- ✅ **Accessibility Permission Issues** - All resolved ⭐
  - Fixed duplicate dialogs (system + custom)
  - Fixed permission not persisting across restarts
  - Added auto-detection on app activation
  - Enhanced user guidance with numbered steps
- ✅ **Priority Inversion Warning** - Fixed by removing duplicate checks
- ✅ **Missing Log Methods** - Added warning() and error()

### Previous (2025-12-21) ���️���️
- ✅ **English Auto-Restore Improvements** - Smart detection of English word patterns ⭐ NEW
  - Fixed: "fix" → stays "fix" (not "fĩ"), "test" → stays "test" (not "tét")
  - Pattern detection: -ix, -ex, -ax endings (fix, text, tax)
  - Smart C+E pattern: test, rest, best preserved
  - Vietnamese typing: 100% preserved (vieets → viết still works)
  - Performance: < 2% overhead (+0.2ms per keystroke)
  - Test coverage: 417 lines, 13 test cases, 10/13 passing
  - Documentation: `ENGLISH_AUTO_RESTORE_IMPROVEMENTS.md` (364 lines)
- ✅ **Accessibility API Documentation** - Complete guide for Spotlight, Arc, and 38 browsers
- ✅ **Quick Reference Guide** - Easy-to-read summary of Accessibility API features (265 lines) ⭐
- ✅ **Browser Support Matrix** - All supported browsers with performance metrics
- ✅ **Accessibility API Testing** - 16 comprehensive test cases with validation procedures
- ✅ **Accessibility API Changelog** - Complete feature changelog (444 lines)
- ✅ **Selection Method** - Optimized for browser address bars (< 8ms latency)
- ✅ **Autocomplete Method** - Special handling for Spotlight with Forward Delete
- ✅ **5 Injection Methods** - Instant, Fast, Slow, Selection, Autocomplete
- ✅ **2,000+ Lines Documentation** - Comprehensive coverage of Accessibility API features

### Previous (2025-12-20) 🆕
- ✅ **Pattern Optimization** - Stroke & rapid keystroke handling improvements
  - Stroke operations: 87% faster (dd → đ: 1.5ms → 0.2ms)
  - W-as-vowel: 95% faster (w → ư: 1.8ms → 0.1ms)
  - Smart backspace: 91% faster for simple chars (3.2ms → 0.3ms)
  - Fast path coverage: 78% of operations < 1ms
  - Rapid typing: < 16ms/keystroke at 10+ keys/sec
  - Documentation: 600+ lines (STROKE_OPTIMIZATION.md, RAPID_KEYSTROKE_HANDLING.md, PATTERN_OPTIMIZATION_SUMMARY.md)

### Previous (2025-12-20)
- ✅ **Menu Toggle Fix** - Migrated from NSSwitch to SwiftUI Toggle
  - Fixed: Intermittent color loss (100% stable)
  - Fixed: Blue selection highlight (completely eliminated)
  - Implementation: NSHostingView bridge for AppKit/SwiftUI integration
  - Documentation: 1,400+ lines across 4 comprehensive documents

### Previous (2024-01-20)
- ✅ **Keyboard Shortcut Toggle** - Control+Space implementation complete
- ✅ **Shortcut Roadmap** - 4-phase plan (7 months)
- ✅ **Documentation Restructure** - Organized into folders by topic
- ✅ **Editor Performance** - 47× faster in VSCode/Zed
- ✅ **Backspace Fix** - All issues resolved
- ✅ **Arrow Key Fix** - Pass-through working

---

## 📝 Documentation Conventions

### File Naming
- **UPPER_CASE.md** - Main documentation files
- **category/UPPER_CASE.md** - Categorized files
- **category/subcategory/UPPER_CASE.md** - Nested categories

### Line Count Guidelines
- Quick Start: 200-300 lines
- Guides: 300-500 lines
- Complete Documentation: 500+ lines
- Roadmaps: 800-1000 lines

### Status Indicators
- ✅ Complete and verified
- 🎯 Next priority
- 🔮 Future planned
- 🌟 Polish phase
- 📦 Historical/archived

---

## 🚀 Next Steps

### For Users
- Build and run the app (`getting-started/QUICK_START.md`)
- Press Control+Space to toggle
- Enjoy Vietnamese typing!

### For Developers
- Review `shortcuts/roadmap/SHORTCUT_CUSTOMIZATION_ROADMAP.md`
- Start Phase 2: Settings UI (10 weeks)
- Implement visual shortcut recorder

### For Contributors
- Pick a feature from roadmap
- Read relevant documentation
- Follow contribution guidelines
- Submit PRs

---

**Made with ❤️ for the Vietnamese community**

**Last Updated:** 2025-12-20  
**Version:** 1.0.2 (Menu Toggle Fix)  
**Status:** Production Ready (macOS)
