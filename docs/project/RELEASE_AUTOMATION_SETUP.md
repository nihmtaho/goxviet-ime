# Gõ Việt (GoxViet) - Release Automation Setup Summary

**Created:** 2025-12-21  
**Last Updated:** 2025-12-21  
**Status:** ✅ Complete

---

## 📋 Overview

Hệ thống tự động hóa release cho **Gõ Việt (GoxViet)** đã được thiết lập hoàn chỉnh với GitHub Actions, bao gồm:

- ✅ Tự động tạo version tags
- ✅ Tự động build Rust core + macOS app
- ✅ Tự động tạo DMG installer
- ✅ Tự động tạo GitHub Release
- ✅ Tự động upload assets

---

## 🗂️ Files Created

### 1. GitHub Actions Workflows

```
.github/workflows/
├── release.yml        # Main release workflow (build & publish)
├── auto-tag.yml       # Automatic tag creation workflow
└── README.md          # Workflow documentation
```

### 2. Helper Scripts

```
scripts/
└── bump_version.sh    # Interactive version bump script
```

### 3. Version Management

```
VERSION                # Single source of truth for version
```

### 4. Documentation

```
docs/project/
├── RELEASE_WORKFLOW.md          # Detailed release workflow guide
└── RELEASE_AUTOMATION_SETUP.md  # This file
```

---

## 🚀 How It Works

### Workflow Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     RELEASE AUTOMATION FLOW                      │
└─────────────────────────────────────────────────────────────────┘

Step 1: Developer Updates Version
┌──────────────────────────────────┐
│ ./scripts/bump_version.sh patch  │  ← Interactive script
│                                   │
│ Updates:                          │
│ • VERSION file                    │
│ • core/Cargo.toml                 │
│ • Info.plist                      │
│ • CHANGELOG.md                    │
└────────────┬─────────────────────┘
             │
             ↓
┌────────────────────────────────────┐
│ git commit -m "bump version"       │
│ git push origin main               │
└────────────┬───────────────────────┘
             │
             ↓
Step 2: Auto Tag Workflow Triggers
┌────────────────────────────────────┐
│ .github/workflows/auto-tag.yml     │
│                                     │
│ 1. Detect VERSION file change      │
│ 2. Extract version (e.g., 1.0.0)   │
│ 3. Create tag v1.0.0               │
│ 4. Push tag to GitHub              │
└────────────┬───────────────────────┘
             │
             ↓
Step 3: Release Workflow Triggers
┌────────────────────────────────────┐
│ .github/workflows/release.yml      │
│                                     │
│ 1. Build Rust core library         │
│ 2. Build macOS app (Xcode)         │
│ 3. Create DMG installer            │
│ 4. Generate release notes          │
│ 5. Create GitHub Release           │
│ 6. Upload DMG to release           │
└────────────┬───────────────────────┘
             │
             ↓
Step 4: Release Published!
┌────────────────────────────────────┐
│ GitHub Release with Assets         │
│                                     │
│ • GoxViet.dmg (installer)          │
│ • Automatic release notes          │
│ • Installation instructions        │
│ • Download links                   │
└────────────────────────────────────┘
```

---

## 📝 Usage Examples

### Method 1: Automated (Recommended)

```bash
# Interactive mode - script will guide you
./scripts/bump_version.sh

# Or specify bump type directly
./scripts/bump_version.sh patch   # 0.1.0 → 0.1.1
./scripts/bump_version.sh minor   # 0.1.1 → 0.2.0
./scripts/bump_version.sh major   # 0.2.0 → 1.0.0
```

**What happens:**
1. ✅ Script updates all version files
2. ✅ Script adds CHANGELOG entry (you fill in details)
3. ✅ Script commits and pushes to main
4. ✅ Auto-tag workflow creates tag automatically
5. ✅ Release workflow builds and publishes
6. ✅ DMG appears in GitHub Releases!

**Time:** ~10-15 minutes (automatic)

---

### Method 2: Semi-Automatic

```bash
# 1. Update version manually
echo "1.0.0" > VERSION

# 2. Update CHANGELOG.md
vim CHANGELOG.md

# 3. Commit and push
git add VERSION CHANGELOG.md
git commit -m "chore: bump version to 1.0.0"
git push origin main

# 4. Auto-tag workflow creates tag
# 5. Release workflow publishes
```

**Time:** ~10-15 minutes (automatic after push)

---

### Method 3: Fully Manual

```bash
# 1. Update all version files manually
echo "1.0.0" > VERSION
vim core/Cargo.toml
vim platforms/macos/goxviet/goxviet/Info.plist
vim CHANGELOG.md

# 2. Commit
git add -A
git commit -m "chore: bump version to 1.0.0"
git push origin main

# 3. Create and push tag manually
git tag -a v1.0.0 -m "GoxViet 1.0.0"
git push origin v1.0.0

# 4. Release workflow builds and publishes automatically
```

**Time:** ~10-15 minutes (automatic after tag push)

---

## 🎯 Key Features

### 1. Single Source of Truth

**VERSION file** là nguồn chính xác duy nhất:

```
goxviet/VERSION
```

Format: `x.y.z` (Semantic Versioning)

### 2. Automatic Tag Creation

Khi bạn push thay đổi `VERSION` lên main branch:
- ✅ Auto-tag workflow tự động tạo tag
- ✅ Không cần tạo tag thủ công
- ✅ Luôn đồng bộ giữa version file và git tag

### 3. Automatic Release Publishing

Khi tag được tạo (manual hoặc automatic):
- ✅ Build tất cả components
- ✅ Tạo DMG installer
- ✅ Publish GitHub Release
- ✅ Upload DMG vào release assets

### 4. Smart Caching

Workflow sử dụng cache cho:
- ✅ Cargo dependencies (Rust)
- ✅ Build artifacts
- ✅ Faster subsequent builds

### 5. Comprehensive Release Notes

Release notes tự động bao gồm:
- ✅ Installation instructions
- ✅ What's new (from CHANGELOG)
- ✅ Bug reports link
- ✅ Full changelog link

---

## 🔧 Configuration

### Workflow Triggers

#### Release Workflow (`release.yml`)
```yaml
on:
  push:
    tags:
      - 'v*.*.*'  # Matches v1.0.0, v1.2.3, etc.
```

#### Auto Tag Workflow (`auto-tag.yml`)
```yaml
on:
  push:
    branches:
      - main
    paths:
      - 'VERSION'
      - 'core/Cargo.toml'
      - 'platforms/macos/goxviet/goxviet/Info.plist'
```

### Environment Variables

```yaml
APP_NAME: GoxViet
BUNDLE_ID: com.goxviet.ime
RUST_BACKTRACE: 1
```

### Permissions

Repository settings → Actions → Workflow permissions:
- ✅ Set to "Read and write permissions"
- ✅ Allows workflows to create releases and push tags

---

## 📊 Monitoring

### View Workflow Status

```
GitHub Repository → Actions Tab
https://github.com/YOUR_USERNAME/goxviet/actions
```

### Workflow Logs

Each workflow run provides detailed logs:
- Build output (Rust + Xcode)
- Test results
- DMG creation logs
- Release creation status

### Notifications

GitHub automatically sends email notifications:
- ✅ When workflow succeeds
- ❌ When workflow fails

---

## 🐛 Common Issues & Solutions

### Issue 1: Tag Already Exists

**Error:** `tag 'v1.0.0' already exists`

**Solution:**
```bash
# Delete existing tag
git tag -d v1.0.0
git push origin :refs/tags/v1.0.0

# Create new tag
git tag -a v1.0.0 -m "GoxViet 1.0.0"
git push origin v1.0.0
```

### Issue 2: Build Fails

**Error:** `cargo build failed` or `xcodebuild failed`

**Solution:**
```bash
# Test locally first
cd core && cargo build --release && cargo test
cd platforms/macos/goxviet
xcodebuild -project goxviet.xcodeproj -scheme goxviet clean build
```

### Issue 3: DMG Creation Fails

**Error:** `DMG creation failed`

**Solution:**
```bash
# Ensure script is executable
chmod +x scripts/create_dmg.sh

# Test locally
./scripts/create_dmg.sh
```

### Issue 4: Release Not Created

**Error:** `failed to create release`

**Solution:**
- Check repository Settings → Actions → Workflow permissions
- Set to "Read and write permissions"
- Ensure no release exists for this tag already

---

## 📖 Related Documentation

- **[RELEASE_WORKFLOW.md](./RELEASE_WORKFLOW.md)** - Detailed workflow guide
- **[.github/workflows/README.md](../../.github/workflows/README.md)** - Workflow overview
- **[BUILD.md](../getting-started/BUILD.md)** - Build instructions
- **[DISTRIBUTION.md](./DISTRIBUTION.md)** - Distribution guide

---

## 🎓 Version Numbering Guide

### Semantic Versioning (x.y.z)

- **x (Major):** Breaking changes, major rewrites
  - Example: `0.9.9` → `1.0.0` (stable release)
  - Example: `1.2.3` → `2.0.0` (breaking API changes)

- **y (Minor):** New features, backwards compatible
  - Example: `1.0.0` → `1.1.0` (new feature added)
  - Example: `1.1.0` → `1.2.0` (another feature)

- **z (Patch):** Bug fixes, minor improvements
  - Example: `1.1.0` → `1.1.1` (bug fix)
  - Example: `1.1.1` → `1.1.2` (another bug fix)

### Pre-release Versions

For beta/RC releases, use tags like:
- `v1.0.0-beta.1`
- `v1.0.0-beta.2`
- `v1.0.0-rc.1`

Update workflow to mark as pre-release:
```yaml
prerelease: true  # In release.yml
```

---

## ✅ Verification Checklist

After setup, verify everything works:

- [ ] `VERSION` file exists in root
- [ ] Scripts are executable (`chmod +x scripts/*.sh`)
- [ ] Workflows exist in `.github/workflows/`
- [ ] Repository has write permissions for Actions
- [ ] Test workflow locally:
  ```bash
  ./scripts/bump_version.sh patch
  ```
- [ ] Push to main and verify auto-tag workflow runs
- [ ] Verify release workflow triggers on tag push
- [ ] Check GitHub Releases page for published release
- [ ] Download and test DMG installer

---

## 🚦 Release Workflow Status

| Component | Status | Notes |
|-----------|--------|-------|
| Version Management | ✅ Complete | VERSION file + bump script |
| Auto Tag Workflow | ✅ Complete | Triggers on VERSION change |
| Release Workflow | ✅ Complete | Builds + publishes release |
| DMG Creation | ✅ Complete | Uses existing create_dmg.sh |
| Documentation | ✅ Complete | Comprehensive guides |
| Helper Scripts | ✅ Complete | bump_version.sh |

---

## 🎯 Next Steps

### Immediate
- [x] Setup GitHub Actions workflows
- [x] Create VERSION file
- [x] Write helper scripts
- [x] Document everything

### Future Enhancements
- [ ] Add code signing for DMG
- [ ] Add notarization for macOS Gatekeeper
- [ ] Setup Homebrew tap automation
- [ ] Add release notes templates
- [ ] Add changelog generation from git commits
- [ ] Setup multiple platform builds (Intel + Apple Silicon)
- [ ] Add automated testing before release
- [ ] Setup Discord/Slack notifications

---

## 📞 Support

Need help with the release automation?

1. **Read the docs:**
   - [RELEASE_WORKFLOW.md](./RELEASE_WORKFLOW.md)
   - [.github/workflows/README.md](../../.github/workflows/README.md)

2. **Check workflow logs:**
   - GitHub → Actions tab

3. **Test locally:**
   ```bash
   ./scripts/bump_version.sh
   ```

4. **Open an issue:**
   - Include workflow logs
   - Describe what you expected vs what happened

---

## 🎉 Summary

**Setup Status:** ✅ COMPLETE

You now have a fully automated release pipeline:
1. Developer updates VERSION file
2. Push to main triggers auto-tag
3. Tag triggers release workflow
4. Release is published with DMG automatically!

**Total Time:** ~10-15 minutes per release (mostly automated)

**Manual Steps:** Only updating VERSION and CHANGELOG

**Everything else is automatic!** 🚀

---

**Maintained by:** GoxViet Development Team  
**Project:** Gõ Việt (Vietnamese IME)  
**License:** See LICENSE file in root directory