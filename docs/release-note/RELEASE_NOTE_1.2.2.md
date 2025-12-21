# Release Notes – Gõ Việt (GoxViet) v1.2.1

**Release Date:** December 21, 2025  
**Version:** 1.2.1  
**Type:** Critical Bugfix & Stability Release

---

## 🚨 What’s New in v1.2.1

This update delivers two critical fixes for Gõ Việt (GoxViet) on macOS, ensuring a seamless and reliable typing experience for all users.

---

### 1. Accessibility Permission Fix

#### 🛠️ Issues Addressed
- **Duplicate Permission Dialogs:** Only a single, clear custom dialog is now shown (system dialog suppressed).
- **Permission Not Persisting:** Accessibility permission is now remembered across app restarts.
- **No Auto-Detection:** The app now detects when permission is granted in System Preferences and activates automatically.
- **Priority Inversion Warning:** Removed duplicate permission checks to prevent thread priority issues.
- **Missing Log Methods:** Added missing `Log.warning()` and `Log.error()` to eliminate compile errors.

#### ✨ Improvements
- **User Experience:** Custom dialog includes clear, numbered steps, troubleshooting tips, and a “Restart Now” button.
- **Robustness:** No more repeated permission requests or confusing dialogs.
- **Code Quality:** Cleaner permission logic and improved logging.

---

### 2. Backspace Corruption Fix

#### 🛠️ Issues Addressed
- **Character Duplication & Corruption:** Fixed bug where deleting with Backspace could cause duplicated or corrupted Vietnamese characters (e.g., “gõ” → “gg”, “được” → “đđư”).
- **State Machine Violation:** Removed batch/coalescing logic that desynchronized the Rust engine’s state from the actual text buffer.
- **Performance Concerns:** Immediate processing of each DELETE event, maintaining <5ms latency per operation.

#### ✨ Improvements
- **Reliability:** Engine and screen state are always in sync—no more ghost characters or corruption.
- **Performance:** No flicker or lag, even during rapid or held Backspace.
- **Testing:** All Vietnamese and English deletion scenarios pass, including edge cases.

---

## 🧪 Quality Assurance

- ✅ All accessibility permission scenarios tested and passed.
- ✅ All backspace and deletion test cases pass (Vietnamese & English).
- ✅ No performance regression: <5ms per DELETE, 60fps maintained.
- ✅ Documentation updated for all changes.

---

## 📚 Documentation

- `docs/ACCESSIBILITY_PERMISSION_FIX.md` – Full details on permission logic and UX.
- `docs/BACKSPACE_CORRUPTION_FIX.md` – Technical breakdown of the backspace fix.
- `docs/COMMIT_ACCESSIBILITY_FIX.md` – Commit summary for this release.
- `docs/README.md` – Updated index and quick reference.
- `docs/project/CHANGELOG.md` – Full changelog.

---

## 📝 How to Update

**Homebrew:**
```bash
brew update
brew upgrade --cask goxviet
```

**Manual Download:**  
Visit [GoxViet Releases](https://github.com/your-repo/goxviet/releases/tag/v1.2.1) for the latest DMG.

---

## ❓ FAQ

- **Q:** Why did I see two permission dialogs before?  
  **A:** This is now fixed—only one clear dialog will appear.

- **Q:** Why did backspace sometimes duplicate or corrupt Vietnamese text?  
  **A:** This was a bug in the event batching logic. It’s now fully resolved.

- **Q:** Do I need to reconfigure anything?  
  **A:** No. All settings and preferences are preserved.

---

## 🔗 References

- **Pull Requests:**  
  - [#15 – Accessibility & Backspace Fix](https://github.com/your-repo/goxviet/pull/15)  
  - [#16 – Release to main](https://github.com/your-repo/goxviet/pull/16)
- **Issue:**  
  - [#13 – Backspace corruption & permission bug](https://github.com/your-repo/goxviet/issues/13)

---

## 🙏 Thank You!

Thank you for your feedback and support. This release is dedicated to everyone who reported issues and helped us make Gõ Việt (GoxViet) better!

---

*Gõ Việt (GoxViet) – Type Vietnamese Naturally. Now more reliable than ever.* ✨