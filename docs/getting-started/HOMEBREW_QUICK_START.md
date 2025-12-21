# HOMEBREW QUICK START - GÕ VIỆT (GOXVIET)

**Deploy Gõ Việt (GoxViet) via Homebrew in 15 minutes - NO Apple Developer Account needed!**

**Date:** 2025-12-20 | **Version:** 1.0.0

---

## 🎯 Goal

Deploy Gõ Việt (GoxViet) so users can install with:

```bash
brew tap yourusername/goxviet
brew install --cask goxviet
```

**Total Time:** ~15 minutes  
**Cost:** $0 (FREE!)

---

## ✅ Prerequisites

- macOS with Xcode installed
- Rust installed (`cargo --version`)
- Git and GitHub account
- **NO Apple Developer Account needed!**

---

## 🚀 5-Step Deployment

### Step 1: Build Unsigned DMG (5 min)

```bash
# Build DMG
./scripts/build-dmg.sh 1.2.0
```

**Output:** `platforms/macos/goxviet/dist/GoxViet-1.2.0.dmg`

**Verify:**
```bash
open platforms/macos/goxviet/dist/GoxViet-1.2.0.dmg
# Should open and show app
```

---

### Step 2: Upload to GitHub Release (3 min)

1. **Create release:**
   ```bash
   git tag -a v1.2.0 -m "Release version 1.2.0"
   git push origin v1.2.0
   ```

2. **Go to GitHub:**
   - Visit: `https://github.com/yourusername/goxviet/releases/new`
   - Tag: `v1.2.0`
   - Title: `Gõ Việt (GoxViet) v1.2.0`
   - Upload: `GoxViet-1.2.0.dmg`

3. **Copy download URL** (you'll need it next)
   - Example: `https://github.com/yourusername/goxviet/releases/download/v1.2.0/GoxViet-1.2.0.dmg`

---

### Step 3: Generate Homebrew Cask (1 min)

```bash
./scripts/create-cask.sh 1.2.0 \
  https://github.com/yourusername/goxviet/releases/download/v1.2.0/GoxViet-1.2.0.dmg
```

**Output:** `homebrew-cask/goxviet.rb`

**What it does:**
- Calculates SHA-256 checksum
- Generates Homebrew cask formula
- Adds Gatekeeper bypass instructions

---

### Step 4: Create Homebrew Tap (5 min)

**4.1. Create new GitHub repository:**
```
Name: homebrew-goxviet
Public: Yes
Initialize with README: Yes
```

**4.2. Clone and setup:**
```bash
git clone https://github.com/yourusername/homebrew-goxviet.git
cd homebrew-goxviet

# Create Casks directory
mkdir -p Casks

# Copy cask formula
cp ../goxviet/homebrew-cask/goxviet.rb Casks/

# Update README
cat > README.md <<EOF
# Gõ Việt (GoxViet) Homebrew Tap

## Installation

\`\`\`bash
brew tap yourusername/goxviet
brew install --cask goxviet
\`\`\`

## Post-Installation

Bypass Gatekeeper:
\`\`\`bash
xattr -cr /Applications/GoxViet.app
\`\`\`

Then launch and grant Accessibility permission.
EOF

# Commit and push
git add .
git commit -m "Add Gõ Việt (GoxViet) v1.2.0"
git push origin main
```

---

### Step 5: Test Installation (1 min)

```bash
# Add your tap
brew tap yourusername/goxviet

# Install
brew install --cask goxviet

# Bypass Gatekeeper
xattr -cr /Applications/GoxViet.app

# Launch
open /Applications/GoxViet.app
```

**✅ Done! Your app is now installable via Homebrew!**

---

## 📝 User Installation Instructions

Add this to your README.md:

```markdown
## Installation

### Via Homebrew (Recommended)

```bash
# Add tap
brew tap yourusername/goxviet

# Install
brew install --cask goxviet

# Bypass Gatekeeper (one-time)
xattr -cr /Applications/GoxViet.app

# Launch
open /Applications/GoxViet.app
```

### Post-Installation

1. Grant Accessibility permission when prompted
2. Menu bar icon appears (🇻🇳)
3. Click icon to toggle Vietnamese/English
4. Or use keyboard shortcut: Cmd+Shift+V
5. Type: `hoa` → `hòa`

### Updates

```bash
brew upgrade --cask goxviet
```

### Uninstall

```bash
brew uninstall --cask goxviet
```
```

---

## 🔄 Updating for New Releases

When releasing version 1.3.0:

```bash
# 1. Build new DMG
./scripts/build-dmg.sh 1.3.0

# 2. Create git tag
git tag -a v1.3.0 -m "Release version 1.3.0"
git push origin v1.3.0

# 3. Upload to GitHub Releases
# (Go to GitHub, create release, upload DMG)

# 4. Generate new cask
./scripts/create-cask.sh 1.3.0 \
  https://github.com/yourusername/goxviet/releases/download/v1.3.0/GoxViet-1.3.0.dmg

# 5. Update tap
cd ../homebrew-goxviet
cp ../goxviet/homebrew-cask/goxviet.rb Casks/
git add Casks/goxviet.rb
git commit -m "Update to v1.3.0"
git push

# 6. Users upgrade with:
brew upgrade --cask goxviet
```

---

## ⚠️ Important Notes

### Gatekeeper Bypass

**Why needed?**
- App is NOT code-signed (no Apple Developer Account)
- macOS blocks unsigned apps by default

**Is it safe?**
- ✅ Yes for open source projects
- ✅ Users can inspect code
- ✅ Transparent build process

**User instructions:**
```bash
# Remove quarantine attribute
xattr -cr /Applications/GoxViet.app
```

### Alternative Methods

**Method 1: Right-click Open**
1. Right-click app in Applications
2. Select "Open"
3. Click "Open" in dialog

**Method 2: System Settings (macOS 13+)**
1. Try to open app (will be blocked)
2. System Settings → Security & Privacy
3. Click "Open Anyway"

---

## 🐛 Troubleshooting

### Build Failed

```bash
# Clean and rebuild
cd core && cargo clean && cd ..
./scripts/build-dmg.sh 1.2.0
```

### DMG Won't Mount

```bash
# Verify DMG
hdiutil verify platforms/macos/goxviet/dist/GoxViet-1.2.0.dmg
```

### Cask Install Failed

```bash
# Update Homebrew
brew update

# Re-add tap
brew untap yourusername/goxviet
brew tap yourusername/goxviet

# Retry
brew install --cask goxviet
```

### Checksum Mismatch

```bash
# Recalculate SHA-256
cd platforms/macos/goxviet/dist
shasum -a 256 GoxViet-1.2.0.dmg

# Update cask file with correct SHA-256
nano ../../../homebrew-cask/goxviet.rb
```

### App Won't Launch

```bash
# Check Gatekeeper bypass
xattr -l /Applications/GoxViet.app
# Should show NO com.apple.quarantine

# If still present, remove it
xattr -cr /Applications/GoxViet.app

# Check logs
tail -f ~/Library/Logs/GoxViet/keyboard.log
```

---

## 📊 Comparison: Homebrew vs App Store

| Aspect | Homebrew (Unsigned) | App Store / Notarized |
|--------|---------------------|------------------------|
| **Cost** | FREE | $99/year |
| **Setup Time** | 15 min | Hours + approval |
| **User Install** | `brew install` | Mac App Store |
| **Gatekeeper** | Manual bypass | Automatic |
| **Updates** | `brew upgrade` | Automatic |
| **Target Audience** | Developers | General users |
| **Distribution** | GitHub + Homebrew | App Store |

**Recommendation:**
- **Homebrew** - For open source, developer-focused tools
- **App Store** - For general consumer apps

---

## 🎓 Learn More

### Full Documentation
- **[HOMEBREW_DEPLOYMENT.md](HOMEBREW_DEPLOYMENT.md)** - Complete guide
- **[DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md)** - Code signing & notarization

### Scripts
- **[scripts/build-dmg-unsigned.sh](../scripts/build-dmg-unsigned.sh)** - Build script
- **[scripts/create-homebrew-cask.sh](../scripts/create-homebrew-cask.sh)** - Cask generator
- **[scripts/README.md](../scripts/README.md)** - Scripts documentation

### External Resources
- **Homebrew Documentation:** https://docs.brew.sh
- **Homebrew Cask:** https://github.com/Homebrew/homebrew-cask
- **Gatekeeper Info:** https://support.apple.com/en-us/HT202491

---

## ✅ Summary

**You just learned how to:**
1. ✅ Build unsigned DMG (no code signing)
2. ✅ Upload to GitHub Releases
3. ✅ Create Homebrew Cask formula
4. ✅ Set up custom Homebrew Tap
5. ✅ Deploy for FREE (no Apple Developer Account)

**Users can now install with:**
```bash
brew tap yourusername/goxviet
brew install --cask goxviet
```

**Total Cost:** $0  
**Total Time:** ~15 minutes  
**Target Audience:** Developers, power users  
**Updates:** Via `brew upgrade`

---

## 🚀 Next Steps

1. **Announce your release:**
   - GitHub README
   - Social media
   - Developer communities

2. **Add installation badge:**
   ```markdown
   [![Homebrew](https://img.shields.io/badge/Install%20with-Homebrew-orange)](https://github.com/yourusername/homebrew-goxviet)
   ```

3. **Monitor feedback:**
   - GitHub Issues
   - User questions
   - Bug reports

4. **Plan next release:**
   - New features
   - Bug fixes
   - Performance improvements

---

**Congratulations! You've successfully deployed Gõ Việt (GoxViet) via Homebrew! 🎉🍺**

---

**Status:** ✅ Ready to Use  
**Last Updated:** 2025-12-21  
**Maintainer:** Gõ Việt (GoxViet) Team