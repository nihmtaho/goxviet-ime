# HOMEBREW DEPLOYMENT GUIDE - GÕ VIỆT (GOXVIET)

Complete guide for deploying Gõ Việt (GoxViet) via Homebrew **without Apple Developer Account**.

**Date:** 2025-12-20  
**Version:** 1.0.0  
**Target:** Open Source Distribution

---

## 📋 Table of Contents

1. [Overview](#overview)
2. [Why Homebrew?](#why-homebrew)
3. [Quick Start](#quick-start)
4. [Building Unsigned DMG](#building-unsigned-dmg)
5. [Creating Homebrew Cask](#creating-homebrew-cask)
6. [Custom Tap Setup](#custom-tap-setup)
7. [Testing](#testing)
8. [User Installation](#user-installation)
9. [Updates & Maintenance](#updates--maintenance)
10. [Official Homebrew Submission](#official-homebrew-submission)
11. [Troubleshooting](#troubleshooting)

---

## 1. Overview

This guide shows you how to distribute Gõ Việt (GoxViet) through Homebrew **without paying for Apple Developer Program** ($99/year).

### What You'll Do:

1. ✅ Build **unsigned** DMG (no code signing needed)
1. Build unsigned DMG package
2. Upload to GitHub Releases
3. Create Homebrew Cask formula
4. Set up custom Homebrew Tap
5. Users install with: `brew install --cask goxviet`

### What Users Need to Do:

- Bypass Gatekeeper manually (one-time setup)
- Grant Accessibility permission

---

## 2. Why Homebrew?

### ✅ Advantages

- **No Apple Developer Account needed** - Save $99/year
- **Easy updates** - Users run `brew upgrade`
- **Community distribution** - Developers love Homebrew
- **Automated installation** - Script-friendly
- **No manual downloads** - Direct from command line

### ⚠️ Limitations

- **Gatekeeper bypass required** - Users must manually approve
- **Not for App Store** - Only for direct distribution
- **Terminal knowledge needed** - Target audience: developers/power users
- **No auto-updates in app** - Users must run brew upgrade

### 📊 Comparison

| Method | Code Signing | Notarization | Cost | User Friction |
|--------|--------------|--------------|------|---------------|
| **App Store** | Required | Required | $99/year | None |
| **Notarized DMG** | Required | Required | $99/year | Low |
| **Homebrew (Unsigned)** | Not required | Not required | Free | Medium |
| **Manual DMG** | Not required | Not required | Free | High |

**Recommendation:** Homebrew for open-source projects targeting developers.

---

## 3. Quick Start

### One-Command Release

```bash
# 1. Build unsigned DMG
./scripts/build-dmg-unsigned.sh 1.0.0

# 2. Upload to GitHub Releases (manual)
# Go to: https://github.com/yourusername/goxviet/releases/new

# 3. Generate Homebrew Cask
./scripts/create-homebrew-cask.sh 1.0.0 \
  https://github.com/yourusername/goxviet/releases/download/v1.0.0/GoxViet-1.0.0-unsigned.dmg

# 4. Set up custom tap (see section 6)

# 5. Done! Users install with:
brew tap yourusername/goxviet
brew install --cask goxviet
```

---

## 4. Building Unsigned DMG

### 4.1. Build Script

The unsigned build script does NOT require:
- ❌ Apple Developer Account
- ❌ Code signing certificate
- ❌ Notarization

**Run build:**

```bash
./scripts/build-dmg.sh 1.2.0
```

**What it does:**

1. Cleans previous builds
2. Builds Rust core (release mode)
3. Builds macOS app **without code signing**
4. Creates DMG with:
   - App bundle
   - Applications symlink (drag-to-install)
   - README.txt (Gatekeeper bypass instructions)
5. Calculates SHA-256 checksum

**Output:**

```
platforms/macos/GoxViet/dist/
├── GoxViet-1.0.0-unsigned.dmg        # Distributable DMG
└── GoxViet-1.0.0-unsigned.dmg.sha256 # Checksum file
```

**Build time:** ~5-10 minutes

### 4.2. Test Locally

```bash
# Open DMG
**Output:** `platforms/macos/goxviet/dist/GoxViet-1.2.0.dmg`

# Drag to Applications
# Then bypass Gatekeeper:
xattr -cr /Applications/GoxViet.app

# Launch
open /Applications/GoxViet.app
```

### 4.3. Verify Checksum

```bash
cd platforms/macos/goxviet/dist
shasum -a 256 GoxViet-1.2.0.dmg
# Should output: GoxViet-1.0.0-unsigned.dmg: OK
```

---

## 5. Creating Homebrew Cask

### 5.1. Generate Cask Formula

After uploading DMG to GitHub Releases:

```bash
./scripts/create-cask.sh 1.2.0 \
  https://github.com/yourusername/goxviet/releases/download/v1.2.0/GoxViet-1.2.0.dmg
```

**Output:**

```
homebrew/
├── goxviet.rb  # Homebrew Cask formula
└── README.md          # Instructions
```

### 5.2. Cask Structure

The generated cask looks like:

```ruby
cask "goxviet" do
  version "1.0.0"
  sha256 "abc123..."

  url "https://github.com/yourusername/goxviet/releases/download/v#{version}/GoxViet-#{version}.dmg"
  name "GoxViet"
  desc "Gõ Việt - Vietnamese Input Method Editor for macOS"
  homepage "https://github.com/yourusername/goxviet"

  depends_on macos: ">= :catalina"

  app "GoxViet.app"

  postflight do
    puts "⚠️  IMPORTANT: Gatekeeper Bypass Required"
    puts "xattr -cr /Applications/GoxViet.app"
  end

  zap trash: [
    "~/Library/Logs/GoxViet",
    "~/Library/Preferences/com.vietnamese.ime.plist",
  ]
end
```

### 5.3. Manual Cask Creation

If you want to create manually:

```bash
# Create cask file
mkdir -p homebrew/Casks
nano homebrew/Casks/goxviet.rb

# Paste cask formula (see above)
# Update: version, sha256, url, homepage
```

**Get SHA-256:**

```bash
shasum -a 256 GoxViet-1.0.0-unsigned.dmg
```

---

## 6. Custom Tap Setup

### 6.1. Create Tap Repository

A "tap" is a GitHub repository containing Homebrew formulas.

**Naming convention:** `homebrew-<tap-name>`

**Example:** `homebrew-goxviet`

**Steps:**

1. **Create new GitHub repository:**
   ```
   Name: homebrew-goxviet
   Public repository
   Initialize with README
   ```

2. **Clone repository:**
   ```bash
   git clone https://github.com/yourusername/homebrew-goxviet.git
   cd homebrew-goxviet
   ```

3. **Create Casks directory:**
   ```bash
   mkdir -p Casks
   ```

4. **Copy cask formula:**
   ```bash
   cp ../goxviet/homebrew-cask/goxviet.rb Casks/
   ```

5. **Create README:**
   ```bash
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
   
   ## Updates
   
   \`\`\`bash
   brew upgrade --cask goxviet
   \`\`\`
   EOF
   ```

6. **Commit and push:**
   ```bash
   git add .
   git commit -m "Add Gõ Việt (GoxViet) cask v1.0.0"
   git push origin main
   ```

### 6.2. Repository Structure

Your tap repository should look like:

```
homebrew-goxviet/
├── README.md
└── Casks/
    └── goxviet.rb
```

### 6.3. Multiple Versions

To support multiple versions:

```
Casks/
├── goxviet.rb              # Latest stable
├── goxviet@1.0.rb          # Version 1.0.x
└── goxviet-beta.rb         # Beta releases
```

---

## 7. Testing

### 7.1. Local Testing

Before pushing to tap:

```bash
# Audit cask
brew audit --cask homebrew/goxviet.rb

# Test installation
brew install --cask homebrew/goxviet.rb

# Bypass Gatekeeper
xattr -cr /Applications/GoxViet.app

# Test app
open /Applications/GoxViet.app

# Test uninstallation
brew uninstall --cask goxviet
```

### 7.2. Test from Tap

After pushing to GitHub:

```bash
# Add tap
brew tap yourusername/goxviet

# Install
brew install --cask goxviet

# Verify installation
ls -la /Applications/GoxViet.app

# Test upgrade
brew upgrade --cask goxviet
```

### 7.3. Test Gatekeeper Bypass

```bash
# Method 1: Remove quarantine
xattr -cr /Applications/GoxViet.app
open /Applications/GoxViet.app

# Method 2: List quarantine attributes
xattr -l /Applications/GoxViet.app

# Method 3: Check Gatekeeper status
spctl --assess --verbose /Applications/GoxViet.app
```

---

## 8. User Installation

### 8.1. Installation Instructions

Provide users with these instructions:

```markdown
## Installation

### Step 1: Install via Homebrew

```bash
# Add tap
brew tap yourusername/goxviet

# Install app
brew install --cask goxviet
```

### Step 2: Bypass Gatekeeper

This app is not code-signed. Run this command:

```bash
xattr -cr /Applications/GoxViet.app
```

### Step 3: Launch App

```bash
open /Applications/GoxViet.app
```

### Step 4: Grant Permission

When prompted:
- Click "Open System Preferences"
- Go to Security & Privacy → Privacy → Accessibility
- Enable Gõ Việt (GoxViet)
- Restart app

### Step 5: Start Using

- Menu bar icon appears (🇻🇳)
- Click to toggle Vietnamese/English
- Or use: Cmd+Shift+V
- Type: `hoa` → `hòa`

## Uninstallation

```bash
brew uninstall --cask goxviet
```
```

### 8.2. README Badge

Add installation badge to your README.md:

```markdown
[![Install with Homebrew](https://img.shields.io/badge/Install%20with-Homebrew-orange)](https://github.com/yourusername/homebrew-goxviet)

```bash
brew tap yourusername/goxviet
brew install --cask goxviet
```
```

### 8.3. One-Liner Install

For advanced users:

**User Install:**
```bash
brew tap yourusername/goxviet
brew install --cask goxviet
xattr -cr /Applications/GoxViet.app
```

---

## 9. Updates & Maintenance

### 9.1. Release New Version

**Complete workflow:**

```bash
# 1. Update version in code
# Edit: platforms/macos/GoxViet/GoxViet/Info.plist

# 2. Build new DMG
./scripts/build-dmg-unsigned.sh 1.1.0

# 3. Create git tag
git tag -a v1.3.0 -m "Release version 1.3.0"
git push origin v1.3.0

# 4. Create GitHub Release
# Upload: GoxViet-1.1.0-unsigned.dmg

# 5. Generate new cask
./scripts/create-cask.sh 1.3.0 \
  https://github.com/yourusername/goxviet/releases/download/v1.3.0/GoxViet-1.3.0.dmg

# 6. Update tap
cd ../homebrew-goxviet
cp ../goxviet/homebrew-cask/goxviet.rb Casks/
git add Casks/goxviet.rb
git commit -m "Update Gõ Việt (GoxViet) to v1.1.0"
git push

# 7. Users upgrade
brew upgrade --cask goxviet
```

### 9.2. Update Checklist

- [ ] Build new unsigned DMG
- [ ] Upload to GitHub Releases
- [ ] Update cask with new version and SHA-256
- [ ] Test installation locally
- [ ] Push to tap repository
- [ ] Announce update to users
- [ ] Update documentation

### 9.3. Version Numbering

Follow semantic versioning:

- **Major (1.0.0 → 2.0.0):** Breaking changes
- **Minor (1.0.0 → 1.1.0):** New features
- **Patch (1.0.0 → 1.0.1):** Bug fixes

### 9.4. Deprecating Old Versions

```ruby
# In old version cask
cask "goxviet@1.0" do
  version "1.0.0"
  # ...
  
  deprecate! date: "2025-12-31", because: :discontinued
end
```

---

## 10. Official Homebrew Submission

### 10.1. Requirements

To submit to official `homebrew-cask`:

- ✅ Public GitHub repository
- ✅ **75+ GitHub stars** (minimum)
- ✅ Stable release (1.0.0+)
- ✅ Active maintenance (commits in last 3 months)
- ✅ Pass `brew audit --cask` with no errors
- ✅ Clear documentation
- ✅ No trademark violations

### 10.2. Submission Process

**Step 1: Fork homebrew-cask**

```bash
# Fork on GitHub
https://github.com/Homebrew/homebrew-cask

# Clone your fork
git clone https://github.com/yourusername/homebrew-cask.git
cd homebrew-cask
```

**Step 2: Add your cask**

```bash
# Create branch
git checkout -b goxviet

# Add cask (alphabetically in Casks/v/)
cp ../goxviet/homebrew/goxviet.rb Casks/v/

# Audit
brew audit --cask Casks/v/goxviet.rb

# Test installation
brew install --cask Casks/v/goxviet.rb
```

**Step 3: Submit Pull Request**

```bash
git add Casks/v/goxviet.rb
git commit -m "Add Gõ Việt (GoxViet) v1.2.0"
git push origin goxviet

# Create PR on GitHub
# Title: "Add Gõ Việt (GoxViet) v1.0.0"
# Description: Follow PR template
```

**Step 4: Address Review Comments**

Homebrew maintainers will review:
- Cask syntax
- URL validity
- SHA-256 correctness
- App name consistency
- License clarity

### 10.3. Audit Checklist

```bash
# Run all audits
brew audit --cask --strict goxviet

# Check for issues:
# - Missing required fields
# - Incorrect syntax
# - Invalid URLs
# - Wrong SHA-256
# - Trademark violations
```

### 10.4. Alternative: Custom Tap

If you don't meet requirements (e.g., < 75 stars):

- ✅ Use custom tap (recommended for new projects)
- ✅ Build community first
- ✅ Submit to official homebrew later

---

## 11. Troubleshooting

### 11.1. Build Issues

**Error: "Library not found"**

```bash
# Rebuild Rust core
cd core
cargo clean
cargo build --release

# Verify library exists
ls -la target/release/libvietnamese_ime_core.dylib
```

**Error: "Xcode build failed"**

```bash
# Clean Xcode build
cd platforms/macos/goxviet
xcodebuild clean -project GoxViet.xcodeproj

# Rebuild
cd ../../..
./scripts/build-dmg-unsigned.sh 1.0.0
```

### 11.2. DMG Issues

**Error: "DMG won't mount"**

```bash
# Verify DMG
hdiutil verify GoxViet-1.0.0-unsigned.dmg

# Check corruption
shasum -a 256 GoxViet-1.0.0-unsigned.dmg
```

**Error: "Resource busy"**

```bash
# Unmount all DMGs
hdiutil detach /Volumes/Vietnamese\ IME*

# Try again
open GoxViet-1.0.0-unsigned.dmg
```

### 11.3. Gatekeeper Issues

**Error: "App is damaged and can't be opened"**

**Solution:**

```bash
# Remove quarantine attribute
xattr -cr /Applications/GoxViet.app

# Verify removed
xattr -l /Applications/GoxViet.app
# Should show no com.apple.quarantine
```

**Error: "App can't be opened because Apple cannot check it"**

**Solution:**

```bash
# Method 1: Right-click Open
# Right-click app → Open → Click "Open"

# Method 2: System Settings (macOS 13+)
# Try to open → Go to System Settings → Click "Open Anyway"

# Method 3: Command line
sudo spctl --master-disable  # Disable Gatekeeper (not recommended)
# Open app
sudo spctl --master-enable   # Re-enable Gatekeeper
```

### 11.4. Homebrew Issues

**Error: "Cask not found"**

```bash
# Update Homebrew
brew update

# Re-add tap
brew untap yourusername/goxviet
brew tap yourusername/goxviet
```

**Error: "Checksum mismatch"**

```bash
# Recalculate SHA-256
shasum -a 256 GoxViet-1.0.0-unsigned.dmg

# Update cask with correct checksum
nano homebrew-cask/goxviet.rb
# Update sha256 line
```

**Error: "Download failed"**

```bash
# Check URL is accessible
curl -I https://github.com/user/repo/releases/download/v1.0.0/GoxViet-1.0.0-unsigned.dmg

# Verify file exists on GitHub Releases
```

### 11.5. Permission Issues

**Error: "App doesn't have Accessibility permission"**

**Solution:**

1. System Preferences → Security & Privacy
2. Privacy tab → Accessibility
3. Click lock to make changes
4. Add Gõ Việt (GoxViet) (+)
5. Enable checkbox
6. Restart app

**Error: "Permission request doesn't appear"**

```bash
# Reset TCC database (requires reboot)
tccutil reset Accessibility com.vietnamese.ime

# Relaunch app
open /Applications/GoxViet.app
```

### 11.6. User Support

Common user issues:

**Issue 1: "Homebrew not installed"**

```bash
# Install Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

**Issue 2: "xattr command not found"**

```bash
# xattr is built-in on macOS
# Verify macOS version
sw_vers

# If missing, install Xcode Command Line Tools
xcode-select --install
```

**Issue 3: "App won't start"**

```bash
# Check logs
tail -f ~/Library/Logs/GoxViet/keyboard.log

# Check Console.app for crash logs
open /Applications/Utilities/Console.app
```

---

## 12. Best Practices

### 12.1. Release Checklist

- [ ] Test build on clean macOS
- [ ] Verify DMG mounts correctly
- [ ] Test Gatekeeper bypass methods
- [ ] Update version in all files
- [ ] Create git tag
- [ ] Upload to GitHub Releases with release notes
- [ ] Update Homebrew cask
- [ ] Test installation from tap
- [ ] Update documentation
- [ ] Announce release

### 12.2. Documentation

Always include in your README:

- ✅ Clear installation instructions
- ✅ Gatekeeper bypass steps
- ✅ Permission requirements
- ✅ Troubleshooting section
- ✅ Uninstallation steps
- ✅ Known issues

### 12.3. User Communication

**In your README.md:**

```markdown
## ⚠️ Important Notes

This app is **not code-signed or notarized** by Apple. This is intentional to avoid Apple Developer fees and keep the project free.

You will need to bypass Gatekeeper once after installation. This is safe for open-source software where you can inspect the code.
```

### 12.4. Security

- ✅ Open source code for transparency
- ✅ Provide checksums for verification
- ✅ Sign commits with GPG
- ✅ Document build process
- ✅ Clear changelog

---

## 13. Comparison: Custom Tap vs Official Homebrew

| Aspect | Custom Tap | Official Homebrew |
|--------|------------|-------------------|
| **Setup** | Easy (5 min) | Complex (PR review) |
| **Requirements** | None | 75+ stars, audits |
| **Control** | Full | Limited (community managed) |
| **Discoverability** | Low | High |
| **Maintenance** | You maintain | Community helps |
| **Speed** | Instant updates | PR approval needed |

**Recommendation:**
- **Start with custom tap** - Quick to set up, full control
- **Move to official** - When project is mature (75+ stars)

---

## 14. Resources

### Documentation

- **Homebrew Cask Documentation:** https://docs.brew.sh/Cask-Cookbook
- **Homebrew Formula Cookbook:** https://docs.brew.sh/Formula-Cookbook
- **Gatekeeper Info:** https://support.apple.com/en-us/HT202491

### Tools

- **Homebrew:** https://brew.sh
- **Cask Room:** https://github.com/Homebrew/homebrew-cask

### Examples

Look at other successful unsigned casks:
- Open source developer tools
- Community projects
- Academic software

### Support

- **Homebrew Discourse:** https://discourse.brew.sh
- **GitHub Issues:** Your repository
- **Stack Overflow:** Tag `homebrew`

---

## 15. Summary

### Quick Reference

```bash
# Build
./scripts/build-dmg.sh 1.2.0

# Generate cask
./scripts/create-cask.sh 1.2.0 <GITHUB_RELEASE_URL>

# Set up tap (one-time)
# 1. Create repo: homebrew-goxviet
# 2. Copy cask to Casks/
# 3. Push to GitHub

# Users install
brew tap yourusername/goxviet
brew install --cask goxviet
xattr -cr /Applications/GoxViet.app
```

### Key Takeaways

1. ✅ **No Apple Developer Account needed** - Completely free
2. ✅ **Homebrew = Professional** - Developers love it
3. ✅ **Custom tap = Easy** - Full control, quick updates
4. ⚠️ **Gatekeeper bypass required** - One-time user action
5. ✅ **Open source = Trust** - Code transparency

### Next Steps

1. Build your first unsigned DMG
2. Upload to GitHub Releases
3. Create Homebrew cask
4. Set up custom tap
5. Test installation
6. Document for users
7. Announce release!

---

**Status:** ✅ Production Ready  
**Last Updated:** 2025-12-20  
**Maintainer:** Gõ Việt (GoxViet) Team

---

**Questions?** Open an issue on GitHub or check the troubleshooting section above.

**Good luck with your Homebrew deployment! 🍺🚀**