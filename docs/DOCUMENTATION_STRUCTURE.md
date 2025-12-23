# DOCUMENTATION STRUCTURE

## Overview

Documentation has been reorganized into logical folders for easier management and navigation.

**Date:** 2025-12-23  
**Total Files:** 7 master files | **Total Lines:** ~7,000+  
**Status:** ✅ Consolidation Complete

---

## 📁 New Structure (Master Files)

```
docs/
├── GETTING_STARTED.md      # Quick start & onboarding (merged)
├── SHORTCUTS.md            # Keyboard shortcuts (merged)
├── FIXES.md                # Bug fixes & troubleshooting (merged)
├── PERFORMANCE.md          # Performance optimization (merged)
├── PROJECT.md              # Project management & roadmap (merged)
├── RELEASE_NOTES.md        # Release notes (merged)
├── ARCHIVE.md              # Historical docs (merged)
```

- All previous topic files are merged into these master files.
- Each master file contains a clear table of contents and up-to-date content.
- See `README.md` for navigation and usage instructions.
## 🔄 File Migration Map

### Getting Started (2 files → `getting-started/`)
- `QUICK_START.md` → `getting-started/QUICK_START.md`
- `TESTING_GUIDE.md` → `getting-started/TESTING_GUIDE.md`

### Deployment (2 files → root `docs/`) 🆕
- `DEPLOYMENT_GUIDE.md` → `docs/DEPLOYMENT_GUIDE.md`
- `DEPLOYMENT_CHECKLIST.md` → `docs/DEPLOYMENT_CHECKLIST.md`

### Accessibility & Browser Support (5 files → root `docs/`) 🆕 2025-12-21
- **NEW:** `ACCESSIBILITY_QUICK_REFERENCE.md` - Quick reference (265 lines) ⭐ START HERE
- **NEW:** `ACCESSIBILITY_API_SUPPORT.md` - Complete Accessibility API guide (691 lines)
- **NEW:** `BROWSER_SUPPORT.md` - Browser support matrix (38 browsers, 422 lines)
- **NEW:** `TEST_ACCESSIBILITY_API.md` - Testing guide (16 test cases, 637 lines)
- **NEW:** `CHANGELOG_ACCESSIBILITY_API.md` - Feature changelog (444 lines)
- `DEPLOYMENT_GUIDE.md` → `docs/DEPLOYMENT_GUIDE.md`
- `DEPLOYMENT_CHECKLIST.md` → `docs/DEPLOYMENT_CHECKLIST.md`

### Shortcuts (8 files → `shortcuts/`)
- `SHORTCUT_GUIDE.md` → `shortcuts/SHORTCUT_GUIDE.md`
- `SHORTCUT_QUICK_START.md` → `shortcuts/SHORTCUT_QUICK_START.md`
- `SHORTCUT_IMPLEMENTATION_SUMMARY.md` → `shortcuts/implementation/`
- `TEST_SHORTCUT.md` → `shortcuts/testing/`
- `SHORTCUT_VERIFICATION_CHECKLIST.md` → `shortcuts/testing/`
- `SHORTCUT_CUSTOMIZATION_ROADMAP.md` → `shortcuts/roadmap/`
- `SHORTCUT_ROADMAP_SUMMARY.md` → `shortcuts/roadmap/`

### Performance (11 files → `performance/`)
**Main:**
- `PERFORMANCE_INDEX.md` → `performance/PERFORMANCE_INDEX.md`
- `PERFORMANCE_README.md` → `performance/PERFORMANCE_README.md`
- `OPTIMIZATION_README.md` → `performance/OPTIMIZATION_README.md`
- `QUICK_REFERENCE_OPTIMIZATION.md` → `performance/QUICK_REFERENCE_OPTIMIZATION.md`

**Guides:**
- `PERFORMANCE_OPTIMIZATION_GUIDE.md` → `performance/guides/`
- `EDITOR_PERFORMANCE_OPTIMIZATION.md` → `performance/guides/`
- `PERFORMANCE_FIX.md` → `performance/guides/`

**Summaries:**
- `PERFORMANCE_SUMMARY.md` → `performance/summaries/`
- `PERFORMANCE_COMPARISON.md` → `performance/summaries/`
- `PERFORMANCE_FIX_SUMMARY.md` → `performance/summaries/`
- `EDITOR_OPTIMIZATION_SUMMARY.md` → `performance/summaries/`
- `OPTIMIZATION_STATUS_SUMMARY.md` → `performance/summaries/`

**Core Optimizations (4 files → root `docs/`):** 🆕
- `STROKE_OPTIMIZATION.md` → `docs/STROKE_OPTIMIZATION.md`
- `RAPID_KEYSTROKE_HANDLING.md` → `docs/RAPID_KEYSTROKE_HANDLING.md`
- `MEMORY_OPTIMIZATION.md` → `docs/MEMORY_OPTIMIZATION.md`
- `ULTIMATE_ENGLISH_DETECTION_GUIDE.md` → `docs/ULTIMATE_ENGLISH_DETECTION_GUIDE.md` ⭐⭐⭐ NEW (1000+ lines)

### Fixes - Backspace (10 files → `fixes/backspace/`)
- `BACKSPACE_FIX.md` → `fixes/backspace/`
- `BACKSPACE_FIX_SUMMARY.md` → `fixes/backspace/`
- `TEST_BACKSPACE.md` → `fixes/backspace/`
- `BACKSPACE_QUICK_TEST.md` → `fixes/backspace/`
- `BACKSPACE_QUICK_TEST_GUIDE.md` → `fixes/backspace/`
- `README_FIX_BACKSPACE.md` → `fixes/backspace/`
- `BACKSPACE_OPTIMIZATION_*.md` → `fixes/backspace/`
- `SMART_BACKSPACE_*.md` → `fixes/backspace/`
- `RUST_CORE_BACKSPACE_*.md` → `fixes/backspace/`
- `COMMIT_SMART_BACKSPACE.md` → `fixes/backspace/`

### Fixes - Arrow Keys (4 files → `fixes/arrow-keys/`)
- `ARROW_KEY_FIX.md` → `fixes/arrow-keys/`
- `ARROW_KEY_FIX_SUMMARY.md` → `fixes/arrow-keys/`
- `ARROW_KEY_FIX_CHECKLIST.md` → `fixes/arrow-keys/`
- `BUILD_AND_TEST_ARROW_FIX.md` → `fixes/arrow-keys/`

### Fixes - Menu Bar Toggle (9 files → `fixes/menubar-toggle/`)
- `MENUBAR_APPEARANCE_FIX.md` → `fixes/menubar-toggle/`
- `MENUBAR_TOGGLE_CUSTOM_CONTROL.md` → `fixes/menubar-toggle/`
- `MENUBAR_TOGGLE_SWIFTUI_DECISION.md` → `fixes/menubar-toggle/`
- `CHANGELOG_TOGGLE_FIX.md` → `fixes/menubar-toggle/`
- `TOGGLE_FIX_SUMMARY.md` → `fixes/menubar-toggle/`
- `TOGGLE_TESTING_CHECKLIST.md` → `fixes/menubar-toggle/`
- `TOGGLE_V2_SUMMARY.md` → `fixes/menubar-toggle/`
- `TOGGLE_V2.1_SUMMARY.md` → `fixes/menubar-toggle/`
- `TESTING_V2_FOCUS_FIX.md` → `fixes/menubar-toggle/`

### Fixes - Telex (3 files → `fixes/telex/`)
- `TELEX_FIX_FINAL.md` → `fixes/telex/`
- `TELEX_FIX_SUMMARY.md` → `fixes/telex/`
- `TELEX_VERIFICATION.md` → `fixes/telex/`

### Project (7 files → `project/`)
- `PROJECT_STATUS.md` → `project/PROJECT_STATUS.md`
- `CHANGELOG.md` → `project/CHANGELOG.md`
- `RUST_CORE_ROADMAP.md` → `project/RUST_CORE_ROADMAP.md`
- `COMMIT_MESSAGE_TEMPLATE.md` → `project/COMMIT_MESSAGE_TEMPLATE.md`
- `PROJECT_RESTRUCTURE_SUMMARY.md` → `project/PROJECT_RESTRUCTURE_SUMMARY.md`
- `LOG_PATH_MIGRATION.md` → `project/LOG_PATH_MIGRATION.md` (VietnameseIME → GoxViet cleanup)
- `BRANDING_UPDATE_SUMMARY.md` → `project/BRANDING_UPDATE_SUMMARY.md` (v1.2.0 Complete rebranding)

### Release Notes (4 files → `release-note/`)
- `RELEASE_NOTE_1.2.3.md` → `release-note/RELEASE_NOTE_1.2.3.md` (Version 1.2.3 - Memory Leak & Bloat Fix, Stability)
- `RELEASE_NOTE_1.2.2.md` → `release-note/RELEASE_NOTE_1.2.2.md` (Version 1.2.2 - Minor improvements)
- `RELEASE_NOTE_1.2.1.md` → `release-note/RELEASE_NOTE_1.2.1.md` (Version 1.2.1 - Accessibility & Backspace Fix)
- `RELEASE_NOTE_1.2.0.md` → `release-note/RELEASE_NOTE_1.2.0.md` (Version 1.2.0 - Complete rebranding)

### Archive (5 files → `archive/`)
- `FIX_SUMMARY.md` → `archive/`
- `IMPLEMENTATION_COMPLETE.md` → `archive/`
- `OPTIMIZATION_COMPLETE.md` → `archive/`
- `UPDATE_SUMMARY_2024.md` → `archive/`
- `RUST_CORE_NEXT_STEPS.md` → `archive/`

---

## 🎯 Benefits of New Structure

### 1. Better Organization
- **Topic-based folders** make finding docs easier
- **Logical grouping** reduces cognitive load
- **Clear hierarchy** shows relationships

### 2. Easier Maintenance
- **Update related docs** in one folder
- **Add new features** to appropriate category
- **Archive old docs** without cluttering main folders

### 3. Better Discoverability
- **New users** start in `getting-started/`
- **Developers** go to relevant category
- **Testers** find test procedures in one place

### 4. Scalability
- Easy to add new categories
- Subcategories for complex topics
- Clear structure for future growth

---

## 📖 How to Find Documents

### By Category

**"I want to get started"**
→ `getting-started/`

**"I want to learn about shortcuts"**
→ `shortcuts/`

**"I need to fix a bug"**
→ `fixes/` (then choose: backspace, arrow-keys, or telex)

**"I want to optimize performance"**
→ `performance/`

**"I want to see project status"**
→ `project/`

**"I want to see old documents"**
→ `archive/`

### By Use Case

**New User Setup:**
1. `getting-started/QUICK_START.md`
2. `shortcuts/SHORTCUT_QUICK_START.md`
3. `getting-started/TESTING_GUIDE.md`

**Bug Fixing:**
1. `fixes/backspace/BACKSPACE_FIX.md`
2. `fixes/arrow-keys/ARROW_KEY_FIX.md`
3. `fixes/telex/TELEX_FIX_FINAL.md`

**Performance Optimization:**
1. `performance/PERFORMANCE_INDEX.md`
2. `performance/guides/PERFORMANCE_OPTIMIZATION_GUIDE.md`
3. `performance/summaries/PERFORMANCE_COMPARISON.md`

**Shortcut Development:**
1. `shortcuts/SHORTCUT_GUIDE.md`
2. `shortcuts/implementation/SHORTCUT_IMPLEMENTATION_SUMMARY.md`
3. `shortcuts/roadmap/SHORTCUT_CUSTOMIZATION_ROADMAP.md`

---

## 🔍 Quick Reference

### File Count by Category

| Category | Files | Total Lines |
|----------|-------|-------------|
| Getting Started | 2 | 600+ |
| Deployment | 2 | 1,200+ |
| Shortcuts | 8 | 3,500+ |
| Fixes - Backspace | 10 | 3,000+ |
| Fixes - Arrow Keys | 4 | 800+ |
| Fixes - Menu Toggle | 9 | 3,000+ |
| Fixes - Telex | 3 | 500+ |
| Performance | 11 | 4,000+ |
| Core Optimizations | 2 | 600+ |
| Project | 5 | 1,500+ |
| Archive | 5 | 1,000+ |
| **TOTAL** | **67** | **19,800+** |

### Most Important Files

**For Users:**
1. `getting-started/QUICK_START.md` ⭐
2. `shortcuts/SHORTCUT_QUICK_START.md` ⭐
3. `getting-started/TESTING_GUIDE.md`

**For Release Managers:**
1. `DEPLOYMENT_GUIDE.md` ⭐
2. `DEPLOYMENT_CHECKLIST.md` ⭐
3. `project/CHANGELOG.md`

**For Developers:**
1. `performance/guides/PERFORMANCE_OPTIMIZATION_GUIDE.md` ⭐
2. `STROKE_OPTIMIZATION.md` ⭐ 🆕
3. `RAPID_KEYSTROKE_HANDLING.md` ⭐ 🆕
4. `MEMORY_OPTIMIZATION.md` ⭐ 🆕
5. `shortcuts/SHORTCUT_GUIDE.md` ⭐
5. `project/PROJECT_STATUS.md` ⭐

**For Testers:**
1. `shortcuts/testing/TEST_SHORTCUT.md` ⭐
2. `fixes/backspace/TEST_BACKSPACE.md` ⭐
3. `shortcuts/testing/SHORTCUT_VERIFICATION_CHECKLIST.md`

**For Project Managers:**
1. `project/PROJECT_STATUS.md` ⭐
2. `project/CHANGELOG.md` ⭐
3. `shortcuts/roadmap/SHORTCUT_CUSTOMIZATION_ROADMAP.md` ⭐

---

## 🔗 Updating Links

### Internal Links

When referencing docs in other docs, use relative paths and always point to the new master files:

**From root README:**
```markdown
[GETTING_STARTED](docs/GETTING_STARTED.md)
[SHORTCUTS](docs/SHORTCUTS.md)
[FIXES](docs/FIXES.md)
[PERFORMANCE](docs/PERFORMANCE.md)
[PROJECT](docs/PROJECT.md)
[RELEASE_NOTES](docs/RELEASE_NOTES.md)
[ARCHIVE](docs/ARCHIVE.md)
```

**From within docs/:**
```markdown
[GETTING_STARTED](GETTING_STARTED.md)
[SHORTCUTS](SHORTCUTS.md)
...
```

**From within docs/ subdirectories (if any remain):**
```markdown
[PERFORMANCE](../PERFORMANCE.md)
[GETTING_STARTED](../GETTING_STARTED.md)
```

- Remove or update all links to old sub-files (e.g. `shortcuts/SHORTCUT_GUIDE.md`) to point to the relevant master file.
- Update navigation tables and quick references accordingly.

---
```

### External References

If other files reference old paths, update them:

**Old:**
```markdown
See [PERFORMANCE_OPTIMIZATION_GUIDE.md](docs/PERFORMANCE_OPTIMIZATION_GUIDE.md)
```

**New:**
```markdown
See [PERFORMANCE_OPTIMIZATION_GUIDE.md](docs/performance/guides/PERFORMANCE_OPTIMIZATION_GUIDE.md)
```

---

## ✅ Migration Checklist

### For Contributors

- Always add new documentation as a section in the relevant master file.
- Update the table of contents in the master file if you add a new section.
- If a new topic is large and justified, create a new master file and update this structure.
- Keep all documentation in English or Vietnamese (no mixed/auto-translated sections).
- Use clear, descriptive section headers for each topic.
- Follow naming conventions: ALL CAPS for master files, no spaces, use underscores.
- Update this structure file and the main README if you reorganize or add docs.

---

When adding new documentation:

- [ ] Determine correct category (getting-started, shortcuts, fixes, performance, project)
- [ ] Use UPPER_CASE naming convention
- [ ] Place file in appropriate subfolder
- [ ] Update `docs/README.md` with new file
- [ ] Use relative links for cross-references
- [ ] Add to table of contents in category
- [ ] Include line count estimate
- [ ] Mark status (✅ Complete, 🎯 Next, 🔮 Future, etc.)

### For Document Updates

When updating existing docs:

- [ ] Check if file moved to new location
- [ ] Update internal links to new paths
- [ ] Update references in other docs
- [ ] Update README.md if needed
- [ ] Test all links work correctly

---

## 📝 Naming Conventions

### File Names
- **UPPER_CASE.md** - Main topic files
- **CATEGORY_TOPIC.md** - Category-specific files
- **TOPIC_SUMMARY.md** - Summary documents
- **TEST_TOPIC.md** - Testing procedures

### Folder Names
- **lowercase-with-dashes** - All folders use lowercase
- **descriptive-names** - Clear, self-explanatory
- **plural-for-collections** - `fixes/`, `guides/`, `summaries/`

### Documentation Types
- **README.md** - Index/overview files
- **GUIDE.md** - Comprehensive guides
- **SUMMARY.md** - Quick summaries
- **TEST_*.md** - Testing procedures
- ***_CHECKLIST.md** - Verification checklists

---

## 🚀 Benefits Summary

### Before Reorganization ❌
- All 54 files in one folder
- Hard to find specific topics
- No clear categorization
- Difficult to maintain
- Overwhelming for new users

### After Reorganization ✅
- **7 logical categories** with subcategories
- **Easy navigation** by topic
- **Clear hierarchy** and relationships
- **Easier maintenance** and updates
- **Better discoverability** for all users

---

## 🎓 Best Practices

### When Creating New Docs

1. **Choose Category First**
   - Is it a guide? → `getting-started/` or relevant category
   - Is it a fix? → `fixes/` (choose subcategory)
   - Is it performance? → `performance/` (guide or summary)
   - Is it about shortcuts? → `shortcuts/` (choose subcategory)

2. **Use Consistent Format**
   - Title with # heading
   - Overview section
   - Clear sections with ## headings
   - Code examples with file paths
   - Summary at end

3. **Link to Related Docs**
   - Reference related guides
   - Link to prerequisites
   - Point to next steps

4. **Update Index Files**
   - Add to `docs/README.md`
   - Add to category README if exists
   - Update table of contents

---

## 📊 Statistics

### Documentation Growth

| Metric | Count |
|--------|-------|
| Total Files | 67 |
| Total Lines | 19,800+ |
| Categories | 9 |
| Subcategories | 10 |
| Archived Files | 5 |

### Category Breakdown

```
Performance:        11 files (4,000+ lines) - 20%
Core Optimizations:  2 files (600+ lines)   - 3%
Fixes:              26 files (7,300+ lines) - 37%
Shortcuts:           8 files (3,500+ lines) - 18%
Deployment:          2 files (1,200+ lines) - 6%
Project:             5 files (1,500+ lines) - 8%
Getting Started:     2 files (600+ lines)   - 3%
Archive:             5 files (1,000+ lines) - 5%
Other:               6 files (100+ lines)   - <1%
```

---

## 🔮 Future Improvements

### Planned Enhancements

1. **Category README Files**
   - Add README.md to each category folder
   - Provide category-specific navigation
   - Include quick reference

2. **Auto-Generated Index**
   - Script to generate docs/README.md
   - Keep file list up-to-date
   - Calculate statistics automatically

3. **Link Validation**
   - Script to check all internal links
   - Detect broken references
   - Report missing files

4. **Documentation Templates**
   - Standard templates for each doc type
   - Consistent formatting
   - Easier for contributors

---

## 📞 Support

### Questions About Structure

**"Where should I put my new document?"**
→ Choose the most relevant category from the 7 main folders

**"Can I create new categories?"**
→ Yes, but discuss with team first to ensure it fits the structure

**"What if a document fits multiple categories?"**
→ Put it in the primary category, reference from others

**"Should I update old links?"**
→ Yes, update all references to point to new locations

---

## ✅ Conclusion

The new documentation structure provides:
- ✅ **Better organization** by topic
- ✅ **Easier navigation** for all users
- ✅ **Simpler maintenance** and updates
- ✅ **Clear growth path** for future docs
- ✅ **Professional structure** for large projects

**All existing documentation has been preserved and categorized for optimal accessibility.**

---

**Status:** ✅ Reorganization Complete  
**Date:** 2025-12-20  
**Version:** 2.2  
**Total Files:** 67 → Organized into 9 categories  
**Total Lines:** 19,800+ lines of documentation  
**Latest Additions:** 
- Deployment documentation (DEPLOYMENT_GUIDE.md, DEPLOYMENT_CHECKLIST.md)
- Core optimizations (STROKE_OPTIMIZATION.md, RAPID_KEYSTROKE_HANDLING.md, MEMORY_OPTIMIZATION.md) 🆕