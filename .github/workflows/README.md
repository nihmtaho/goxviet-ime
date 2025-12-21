# Gõ Việt (GoxViet) - GitHub Actions Workflows

**Last Updated:** 2025-12-21

---

## 📋 Overview

This directory contains GitHub Actions workflows for automating the release process of Gõ Việt (GoxViet) IME.

---

## 🔄 Available Workflows

### 1. **Release Workflow** (`release.yml`)

**Trigger:** Push tag with format `v*.*.*` (e.g., `v1.0.0`)

**Purpose:** Automate the complete release process

**Steps:**
1. ✅ Checkout code
2. ✅ Build Rust core library (`core/`)
3. ✅ Build macOS app (Xcode)
4. ✅ Create DMG installer
5. ✅ Generate release notes
6. ✅ Create GitHub Release
7. ✅ Upload DMG to release assets

**Usage:**
```bash
# Manual trigger by creating and pushing tag
git tag -a v1.0.0 -m "GoxViet 1.0.0"
git push origin v1.0.0
```

---

### 2. **Auto Tag Workflow** (`auto-tag.yml`)

**Trigger:** Push to `main` branch with changes in:
- `VERSION` file
- `core/Cargo.toml`
- `platforms/macos/goxviet/goxviet/Info.plist`

**Purpose:** Automatically create version tags when version is bumped

**Steps:**
1. ✅ Extract version from `VERSION` file
2. ✅ Validate version format (x.y.z)
3. ✅ Check if tag already exists
4. ✅ Create annotated tag
5. ✅ Push tag to GitHub (triggers release workflow)

**Usage:**
```bash
# Update version and push to main
echo "1.0.0" > VERSION
git add VERSION
git commit -m "chore: bump version to 1.0.0"
git push origin main
# Tag is created automatically!
```

---

## 🚀 Quick Start

### Method 1: Automatic (Recommended)

Use the version bump script:

```bash
# Interactive mode
./scripts/bump_version.sh

# Or specify bump type
./scripts/bump_version.sh patch  # 0.0.x
./scripts/bump_version.sh minor  # 0.x.0
./scripts/bump_version.sh major  # x.0.0
```

The script will:
1. Update `VERSION`, `Cargo.toml`, `Info.plist`
2. Add changelog entry
3. Commit changes
4. Push to main (triggers auto-tag)
5. Auto-tag creates tag (triggers release)

### Method 2: Manual

```bash
# 1. Update version
echo "1.0.0" > VERSION

# 2. Update CHANGELOG.md manually

# 3. Commit and push
git add VERSION CHANGELOG.md
git commit -m "chore: bump version to 1.0.0"
git push origin main

# 4. Wait for auto-tag workflow, or create tag manually
git tag -a v1.0.0 -m "GoxViet 1.0.0"
git push origin v1.0.0
```

---

## 📊 Workflow Diagram

```
Developer Action          Auto Tag Workflow       Release Workflow        Result
─────────────────         ─────────────────       ────────────────        ──────
Update VERSION file  →    Extract version    →    Build Rust core    →    GitHub
Commit & push main        Create tag v1.0.0       Build macOS app         Release
                          Push tag                Create DMG              with DMG
                                                  Upload assets
```

---

## 🔧 Configuration

### Environment Variables

Defined in `release.yml`:

```yaml
APP_NAME: GoxViet              # App display name
BUNDLE_ID: com.goxviet.ime     # macOS bundle identifier
RUST_BACKTRACE: 1              # Enable Rust backtraces for debugging
```

### Secrets

Required secrets (automatically provided by GitHub):

- `GITHUB_TOKEN` - Used to create releases and upload assets

---

## 📝 Version Management

Version is managed in `VERSION` file at project root:

**Format:** `x.y.z` (Semantic Versioning)

- **Patch (0.0.x):** Bug fixes, minor improvements
- **Minor (0.x.0):** New features, backwards compatible
- **Major (x.0.0):** Breaking changes

**Example:**
```
0.1.0  # Initial release
0.2.0  # New feature
0.2.1  # Bug fix
1.0.0  # First stable release
```

---

## 🐛 Troubleshooting

### Workflow doesn't trigger

**Check:**
- Tag format is correct (`v1.0.0`, not `1.0.0`)
- You have push permission to repository
- GitHub Actions are enabled in repository settings

### Build fails

**Check:**
- Local build works: `cargo build --release`
- Xcode build works: `xcodebuild clean build`
- All tests pass: `cargo test --release`

### DMG creation fails

**Check:**
- Script exists: `scripts/create_dmg.sh`
- Script is executable: `chmod +x scripts/create_dmg.sh`
- Test locally: `./scripts/create_dmg.sh`

### Release creation fails

**Check:**
- Repository settings → Actions → Workflow permissions
- Set to "Read and write permissions"
- Tag doesn't already exist

---

## 📖 Documentation

For detailed documentation, see:

- [Release Workflow Guide](../../docs/project/RELEASE_WORKFLOW.md)
- [Build Instructions](../../docs/getting-started/BUILD.md)
- [Distribution Guide](../../docs/project/DISTRIBUTION.md)

---

## 🔗 Related Files

```
goxviet/
├── .github/
│   └── workflows/
│       ├── README.md           # This file
│       ├── release.yml         # Release automation
│       └── auto-tag.yml        # Auto tagging
├── scripts/
│   ├── bump_version.sh         # Version bump helper
│   └── create_dmg.sh           # DMG creation script
├── VERSION                     # Version file
└── CHANGELOG.md                # Release notes
```

---

## 🎯 Best Practices

1. ✅ **Always update CHANGELOG.md** before releasing
2. ✅ **Test locally** before pushing tags
3. ✅ **Use semantic versioning** (x.y.z)
4. ✅ **Review generated release notes** after workflow completes
5. ✅ **Monitor workflow status** in Actions tab
6. ✅ **Keep VERSION file in sync** with Cargo.toml and Info.plist

---

## 📞 Support

If you encounter issues:

1. Check [GitHub Actions logs](../../actions)
2. Review [Troubleshooting section](#troubleshooting)
3. See [Release Workflow Documentation](../../docs/project/RELEASE_WORKFLOW.md)
4. Open an issue on GitHub

---

**Maintained by:** GoxViet Development Team  
**Project:** Gõ Việt (Vietnamese IME)  
**License:** See LICENSE file in root directory