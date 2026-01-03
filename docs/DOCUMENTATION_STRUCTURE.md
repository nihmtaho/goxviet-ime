# DOCUMENTATION STRUCTURE

**Cập nhật:** 2025-12-31  
**Trạng thái:** ✅ Đã tổ chức lại, đồng bộ mục lục, bổ sung tính năng Multi-Language Support.

Tài liệu được chia theo chủ đề, dễ tra cứu, dễ bảo trì. Khi thêm tài liệu mới, luôn cập nhật mục lục tại đây và `README.md`.

---

## 📁 Cấu trúc thư mục tài liệu (ngắn gọn)

```
docs/
├── getting-started/           # Hướng dẫn bắt đầu, test
├── shortcuts/                 # Phím tắt & roadmap
├── fixes/                     # Tổng hợp fix (backspace, arrow, telex, menubar, ...)
├── performance/               # Tối ưu hiệu năng, benchmark, guides
├── project/                   # Quản lý dự án, changelog, roadmap
├── release-note/              # Ghi chú phát hành (bản mới nhất: RELEASE_NOTE_1.3.2.md)
├── archive/                   # Lưu trữ tài liệu cũ, tổng hợp lịch sử
├── README.md                  # Danh mục tài liệu & hướng dẫn tra cứu
├── DOCUMENTATION_STRUCTURE.md # File này - hướng dẫn cấu trúc
├── STRUCTURE_VISUAL.md        # Sơ đồ visual cấu trúc docs
```

- Mỗi chủ đề có thư mục riêng, tài liệu chính nằm ở các file đầu mục.
- Khi thêm bản phát hành mới, cập nhật vào `release-note/` và mục lục tại đây.
## 🔄 Mục lục & phân loại

- **getting-started/**: QUICK_START.md, TESTING_GUIDE.md
- **Multi-Language Support**: MULTI_LANGUAGE_SUPPORT.md ⭐ NEW (auto-disable Vietnamese for non-Latin keyboards)
- **shortcuts/**: SHORTCUT_GUIDE.md, SHORTCUT_QUICK_START.md, roadmap, testing
- **fixes/**: backspace/, arrow-keys/, menubar-toggle/, telex/
- **performance/**: guides/, summaries/, benchmark, tối ưu hóa, ENGLISH_DETECTION_PATTERNS.md ⭐ NEW
- **project/**: PROJECT_STATUS.md, CHANGELOG.md, roadmap, branding
- **release-note/**: RELEASE_NOTE_1.3.2.md (mới nhất), các bản trước
- **archive/**: Tổng hợp lịch sử, tài liệu cũ

**Khi có bản phát hành mới:**  
→ Thêm file vào `release-note/`, cập nhật mục lục tại đây và `README.md`.

**Xem sơ đồ trực quan:** `STRUCTURE_VISUAL.md`

**Tính năng mới (2025-12-31):**  
→ `MULTI_LANGUAGE_SUPPORT.md` - Tự động tắt Vietnamese khi dùng bàn phím Nhật/Hàn/Trung/...

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

## 🔍 Tra cứu nhanh

- **Bắt đầu:** `getting-started/QUICK_START.md`
- **Phím tắt:** `shortcuts/SHORTCUT_GUIDE.md`, `SHORTCUT_QUICK_START.md`
- **Fix lỗi:** `fixes/backspace/BACKSPACE_FIX.md`, `fixes/arrow-keys/ARROW_KEY_FIX.md`, `fixes/telex/TELEX_FIX_FINAL.md`
- **Tối ưu hiệu năng:** `performance/guides/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Trạng thái dự án:** `project/PROJECT_STATUS.md`
- **Lịch sử phát hành:** `release-note/RELEASE_NOTE_1.3.2.md` (mới nhất)
- **Tài liệu cũ:** `archive/`

**Lưu ý:** Luôn dùng đường dẫn tương đối, cập nhật link khi đổi vị trí file.

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

## ✅ Kết luận

- Tài liệu đã chia rõ theo chủ đề, dễ tìm, dễ cập nhật.
- Khi thêm tài liệu mới, luôn cập nhật mục lục tại đây và `README.md`.
- Bản phát hành mới nhất: `release-note/RELEASE_NOTE_1.3.2.md` (24/12/2025).

**Mọi tài liệu cũ đều được lưu trữ, không mất dữ liệu.**

---