# DEPLOYMENT GUIDE - VIETNAMESE IME PRODUCTION

## Overview

This guide covers the complete process for deploying Vietnamese IME to production for end users.

**Date:** 2025-12-20  
**Version:** 1.0.0  
**Platform:** macOS 10.15+  
**Target:** End Users (Non-Developers)

---

## 📋 Table of Contents

1. [Pre-Deployment Checklist](#pre-deployment-checklist)
2. [Build Configuration](#build-configuration)
3. [Code Signing & Notarization](#code-signing--notarization)
4. [Build Process](#build-process)
5. [Testing Before Release](#testing-before-release)
6. [Distribution Methods](#distribution-methods)
7. [Installation Instructions](#installation-instructions)
8. [Troubleshooting](#troubleshooting)
9. [Post-Deployment](#post-deployment)

---

## 1. Pre-Deployment Checklist

### ✅ Code Quality

- [ ] All unit tests passing
- [ ] Integration tests passing
- [ ] Performance benchmarks meet requirements (< 16ms latency)
- [ ] No memory leaks detected
- [ ] Code reviewed and approved

### ✅ Documentation

- [ ] User guide updated
- [ ] Changelog updated
- [ ] Known issues documented
- [ ] Installation instructions tested

### ✅ Legal & Security

- [ ] License file included
- [ ] Privacy policy reviewed
- [ ] No hardcoded secrets or API keys
- [ ] Third-party licenses acknowledged

### ✅ Version Management

- [ ] Version number updated in `Info.plist`
- [ ] Build number incremented
- [ ] Git tag created for release
- [ ] Changelog entry added

---

## 2. Build Configuration

### 2.1. Update Version Information

Update version in `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/Info.plist`:

```xml
<key>CFBundleShortVersionString</key>
<string>1.0.0</string>
<key>CFBundleVersion</key>
<string>100</string>
```

**Version Scheme:**
- `CFBundleShortVersionString`: User-visible version (e.g., 1.0.0)
- `CFBundleVersion`: Build number (increment for each build)

### 2.2. Configure Build Settings

In Xcode, set the following for **Release** configuration:

**General Settings:**
- **Product Name:** `VietnameseIMEFast`
- **Bundle Identifier:** `com.vietnamese.ime`
- **Minimum macOS Version:** `10.15`

**Build Settings:**
- **Optimization Level:** `-O` (Fast)
- **Swift Compilation Mode:** `Whole Module`
- **Debug Information Format:** `DWARF with dSYM File`
- **Strip Debug Symbols During Copy:** `YES`
- **Enable Bitcode:** `NO` (not supported for macOS apps)

**Signing & Capabilities:**
- **Automatically manage signing:** `NO` (for production)
- **Team:** Your Apple Developer Team
- **Signing Certificate:** `Developer ID Application`
- **Provisioning Profile:** None (for direct distribution)

### 2.3. Build Rust Core in Release Mode

```bash
cd core
cargo build --release
```

This creates optimized binary at `core/target/release/libvietnamese_ime_core.dylib`

Verify the library:
```bash
file core/target/release/libvietnamese_ime_core.dylib
otool -L core/target/release/libvietnamese_ime_core.dylib
```

---

## 3. Code Signing & Notarization

### 3.1. Prerequisites

**Apple Developer Account:**
- Enrolled in Apple Developer Program ($99/year)
- Developer ID Application certificate installed
- App-specific password created for notarization

**Install Developer Certificate:**
1. Go to [Apple Developer Certificates](https://developer.apple.com/account/resources/certificates/list)
2. Create "Developer ID Application" certificate
3. Download and install in Keychain Access

### 3.2. Code Signing

**Sign the Rust Library:**
```bash
codesign --force --sign "Developer ID Application: Your Name (TEAM_ID)" \
  --options runtime \
  --timestamp \
  core/target/release/libvietnamese_ime_core.dylib
```

**Verify Signature:**
```bash
codesign --verify --verbose core/target/release/libvietnamese_ime_core.dylib
codesign -dvv core/target/release/libvietnamese_ime_core.dylib
```

**Sign the macOS App:**

In Xcode, signing happens automatically during Archive if configured correctly. To manually sign:

```bash
codesign --force --sign "Developer ID Application: Your Name (TEAM_ID)" \
  --options runtime \
  --timestamp \
  --entitlements platforms/macos/VietnameseIMEFast/VietnameseIMEFast/VietnameseIMEFast.entitlements \
  --deep \
  /path/to/VietnameseIMEFast.app
```

### 3.3. Notarization

**Why Notarize:**
- Required for macOS 10.15+ (Catalina and later)
- Prevents "unidentified developer" warning
- Builds user trust

**Step 1: Create App-Specific Password**
1. Go to [appleid.apple.com](https://appleid.apple.com)
2. Sign in → Security → App-Specific Passwords
3. Generate password → Save it securely

**Step 2: Store Credentials**
```bash
xcrun notarytool store-credentials "notary-profile" \
  --apple-id "your-email@example.com" \
  --team-id "YOUR_TEAM_ID" \
  --password "xxxx-xxxx-xxxx-xxxx"
```

**Step 3: Create DMG**
```bash
# Create a folder for distribution
mkdir -p dist/VietnameseIME

# Copy app bundle
cp -R /path/to/VietnameseIMEFast.app dist/VietnameseIME/

# Create DMG
hdiutil create -volname "Vietnamese IME" \
  -srcfolder dist/VietnameseIME \
  -ov -format UDZO \
  dist/VietnameseIME-1.0.0.dmg
```

**Step 4: Sign DMG**
```bash
codesign --sign "Developer ID Application: Your Name (TEAM_ID)" \
  --timestamp \
  dist/VietnameseIME-1.0.0.dmg
```

**Step 5: Submit for Notarization**
```bash
xcrun notarytool submit dist/VietnameseIME-1.0.0.dmg \
  --keychain-profile "notary-profile" \
  --wait
```

This will output a submission ID. Wait for approval (usually 5-10 minutes).

**Step 6: Staple Notarization Ticket**
```bash
xcrun stapler staple dist/VietnameseIME-1.0.0.dmg
```

**Step 7: Verify Notarization**
```bash
spctl -a -t open --context context:primary-signature -v dist/VietnameseIME-1.0.0.dmg
```

Should output: `accepted`

---

## 4. Build Process

### 4.1. Clean Build

```bash
# Clean Rust build
cd core
cargo clean
cargo build --release

# Clean Xcode build
cd ../platforms/macos/VietnameseIMEFast
xcodebuild clean -project VietnameseIMEFast.xcodeproj -scheme VietnameseIMEFast
```

### 4.2. Build via Xcode (Recommended)

1. Open `platforms/macos/VietnameseIMEFast/VietnameseIMEFast.xcodeproj`
2. Select **Product → Scheme → Edit Scheme**
3. Change **Run** to **Release** configuration
4. Select **Product → Archive**
5. Wait for archive to complete
6. In Organizer, select archive → **Distribute App**
7. Choose **Developer ID** → **Export**
8. Save to `dist/` folder

### 4.3. Build via Command Line

```bash
cd platforms/macos/VietnameseIMEFast

xcodebuild archive \
  -project VietnameseIMEFast.xcodeproj \
  -scheme VietnameseIMEFast \
  -configuration Release \
  -archivePath build/VietnameseIMEFast.xcarchive

xcodebuild -exportArchive \
  -archivePath build/VietnameseIMEFast.xcarchive \
  -exportPath dist/ \
  -exportOptionsPlist ExportOptions.plist
```

**ExportOptions.plist:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>method</key>
    <string>developer-id</string>
    <key>teamID</key>
    <string>YOUR_TEAM_ID</string>
</dict>
</plist>
```

---

## 5. Testing Before Release

### 5.1. Functional Testing

**Basic Functionality:**
- [ ] App launches successfully
- [ ] Menu bar icon appears
- [ ] Toggle Vietnamese/English works
- [ ] Telex input works correctly
- [ ] VNI input works correctly (if implemented)
- [ ] Tone marks applied correctly
- [ ] Backspace works correctly
- [ ] Arrow keys don't interfere

**App-Specific Testing:**
- [ ] Works in TextEdit
- [ ] Works in Notes
- [ ] Works in Safari/Chrome address bar
- [ ] Works in VSCode
- [ ] Works in Terminal/iTerm2
- [ ] Works in Slack/Discord

**Performance Testing:**
- [ ] Keystroke latency < 16ms
- [ ] No lag with fast typing
- [ ] Memory usage stable (< 50MB)
- [ ] CPU usage low (< 5% idle)

### 5.2. Clean macOS Installation Test

**Test on Fresh VM or Clean Install:**
```bash
# Verify app runs on clean system
# No development tools installed
# No Xcode, no Homebrew

# Install app
# Test all functionality
# Check logs for errors
```

### 5.3. Accessibility Permissions

**Test Permission Flow:**
1. First launch → App requests Accessibility permission
2. User grants permission → App works
3. User denies permission → App shows instructions
4. User revokes permission → App detects and shows alert

### 5.4. Update Testing

**Test Update Scenario:**
1. Install old version
2. Configure settings
3. Install new version
4. Verify settings preserved
5. Verify functionality improved

---

## 6. Distribution Methods

### 6.1. Direct Download (DMG)

**Pros:**
- Full control
- No store fees
- Instant updates

**Cons:**
- Manual updates
- No discovery
- Hosting costs

**Implementation:**
1. Create DMG (see Section 3.3)
2. Upload to GitHub Releases or hosting
3. Provide download link on website

### 6.2. GitHub Releases

**Setup:**
```bash
# Create git tag
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0

# Create release on GitHub
# Upload DMG file
# Add release notes
```

**Release Notes Template:**
```markdown
## Vietnamese IME v1.0.0

### 🎉 New Features
- Feature 1
- Feature 2

### 🐛 Bug Fixes
- Fix 1
- Fix 2

### ⚡ Performance Improvements
- Improvement 1
- Improvement 2

### 📦 Installation
1. Download `VietnameseIME-1.0.0.dmg`
2. Open DMG file
3. Drag app to Applications folder
4. Launch and grant Accessibility permission

### 📋 Requirements
- macOS 10.15 (Catalina) or later
- 50MB disk space
```

### 6.3. Homebrew Cask

**Create Cask Formula:**
```ruby
cask "vietnamese-ime" do
  version "1.0.0"
  sha256 "checksum_here"

  url "https://github.com/yourusername/vietnamese-ime/releases/download/v#{version}/VietnameseIME-#{version}.dmg"
  name "Vietnamese IME"
  desc "Fast Vietnamese input method editor"
  homepage "https://github.com/yourusername/vietnamese-ime"

  depends_on macos: ">= :catalina"

  app "VietnameseIMEFast.app"

  zap trash: [
    "~/Library/Logs/VietnameseIME",
    "~/Library/Preferences/com.vietnamese.ime.plist",
  ]
end
```

**Submit to Homebrew:**
```bash
# Fork homebrew/cask
# Add formula to Casks/vietnamese-ime.rb
# Submit PR
```

### 6.4. Mac App Store (Future)

**Pros:**
- Automatic updates
- User trust
- Discovery

**Cons:**
- Approval process
- Store fees (30%)
- Additional requirements

**Requirements:**
- App Store Connect account
- App sandbox enabled
- No private APIs
- No accessibility bypass

---

## 7. Installation Instructions

### 7.1. For End Users

**Installation Steps:**

1. **Download the App**
   - Go to [Release Page]
   - Download `VietnameseIME-1.0.0.dmg`

2. **Install the App**
   - Open the DMG file
   - Drag `VietnameseIMEFast.app` to Applications folder
   - Eject the DMG

3. **First Launch**
   - Open Applications folder
   - Double-click `VietnameseIMEFast.app`
   - If you see "unidentified developer" warning:
     - Right-click → Open
     - Click "Open" in dialog

4. **Grant Permissions**
   - System will ask for Accessibility permission
   - Click "Open System Preferences"
   - Enable permission for Vietnamese IME
   - Restart the app

5. **Start Using**
   - Menu bar icon appears (🇻🇳)
   - Click to toggle Vietnamese/English
   - Or use keyboard shortcut (Cmd+Shift+V)

### 7.2. Uninstallation

**Manual Uninstall:**
```bash
# 1. Quit the app
# 2. Delete app from Applications
rm -rf /Applications/VietnameseIMEFast.app

# 3. Remove preferences (optional)
rm -rf ~/Library/Logs/VietnameseIME
rm -rf ~/Library/Preferences/com.vietnamese.ime.plist

# 4. Revoke Accessibility permission (optional)
# System Preferences → Security & Privacy → Privacy → Accessibility
# Remove Vietnamese IME from list
```

### 7.3. Troubleshooting Common Issues

**Issue 1: "App is damaged and can't be opened"**

**Solution:**
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine /Applications/VietnameseIMEFast.app
```

**Issue 2: "App doesn't have permission to control your computer"**

**Solution:**
1. System Preferences → Security & Privacy
2. Privacy tab → Accessibility
3. Click lock to make changes
4. Add Vietnamese IME to list
5. Restart app

**Issue 3: "Typing doesn't work"**

**Solution:**
1. Check menu bar icon is visible
2. Click icon to verify mode (Vietnamese/English)
3. Check Accessibility permission granted
4. Check app logs: `~/Library/Logs/VietnameseIME/`

**Issue 4: "App crashes on launch"**

**Solution:**
1. Check macOS version (10.15+)
2. Check Console.app for crash logs
3. Reset preferences:
   ```bash
   rm ~/Library/Preferences/com.vietnamese.ime.plist
   ```
4. Reinstall app

---

## 8. Troubleshooting

### 8.1. Build Errors

**Error: "Code signing failed"**

**Solution:**
- Verify certificate installed in Keychain
- Check certificate not expired
- Verify Team ID matches

**Error: "Library not found"**

**Solution:**
```bash
# Rebuild Rust library
cd core
cargo clean
cargo build --release

# Verify library exists
ls -la target/release/libvietnamese_ime_core.dylib
```

**Error: "Notarization failed"**

**Solution:**
```bash
# Check detailed notarization log
xcrun notarytool log SUBMISSION_ID --keychain-profile "notary-profile"

# Common issues:
# - Library not signed
# - Missing entitlements
# - Unsigned frameworks
```

### 8.2. Testing Issues

**Performance Degradation:**
- Profile with Instruments
- Check debug symbols stripped
- Verify Release build configuration

**Memory Leaks:**
- Run with Instruments → Leaks
- Check FFI boundary
- Verify proper cleanup

---

## 9. Post-Deployment

### 9.1. Monitoring

**User Feedback:**
- Create GitHub Issues template
- Monitor user reports
- Track common issues

**Analytics (Optional):**
- Crash reporting
- Usage statistics
- Performance metrics

### 9.2. Updates

**Update Checklist:**
- [ ] Version number incremented
- [ ] Changelog updated
- [ ] Build tested
- [ ] Notarization successful
- [ ] Release notes prepared
- [ ] GitHub release created

**Update Frequency:**
- **Major updates:** New features (1.0.0 → 2.0.0)
- **Minor updates:** Improvements (1.0.0 → 1.1.0)
- **Patch updates:** Bug fixes (1.0.0 → 1.0.1)

### 9.3. Support

**User Support Channels:**
- GitHub Issues: Bug reports, feature requests
- Email: Direct support
- Documentation: FAQ, guides

**Common Support Requests:**
1. Installation help
2. Permission issues
3. Compatibility questions
4. Feature requests

---

## 10. Automation Scripts

### 10.1. Build Script

Create `scripts/build-release.sh`:

```bash
#!/bin/bash
set -e

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Usage: ./build-release.sh <version>"
    exit 1
fi

echo "Building Vietnamese IME v$VERSION"

# Build Rust core
echo "Building Rust core..."
cd core
cargo clean
cargo build --release
cd ..

# Build macOS app
echo "Building macOS app..."
cd platforms/macos/VietnameseIMEFast
xcodebuild archive \
    -project VietnameseIMEFast.xcodeproj \
    -scheme VietnameseIMEFast \
    -configuration Release \
    -archivePath build/VietnameseIMEFast.xcarchive

# Export app
echo "Exporting app..."
xcodebuild -exportArchive \
    -archivePath build/VietnameseIMEFast.xcarchive \
    -exportPath dist/ \
    -exportOptionsPlist ExportOptions.plist

echo "Build complete: dist/VietnameseIMEFast.app"
```

### 10.2. Notarization Script

Create `scripts/notarize.sh`:

```bash
#!/bin/bash
set -e

DMG_PATH=$1
if [ -z "$DMG_PATH" ]; then
    echo "Usage: ./notarize.sh <dmg-path>"
    exit 1
fi

echo "Notarizing $DMG_PATH"

# Submit for notarization
xcrun notarytool submit "$DMG_PATH" \
    --keychain-profile "notary-profile" \
    --wait

# Staple ticket
xcrun stapler staple "$DMG_PATH"

# Verify
spctl -a -t open --context context:primary-signature -v "$DMG_PATH"

echo "Notarization complete!"
```

### 10.3. Release Script

Create `scripts/release.sh`:

```bash
#!/bin/bash
set -e

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Usage: ./release.sh <version>"
    exit 1
fi

echo "Releasing v$VERSION"

# Build
./scripts/build-release.sh "$VERSION"

# Create DMG
./scripts/create-dmg.sh "$VERSION"

# Notarize
./scripts/notarize.sh "dist/VietnameseIME-$VERSION.dmg"

# Create git tag
git tag -a "v$VERSION" -m "Release version $VERSION"
git push origin "v$VERSION"

echo "Release v$VERSION complete!"
echo "Upload dist/VietnameseIME-$VERSION.dmg to GitHub Releases"
```

---

## 11. Security Best Practices

### 11.1. Code Security

- ✅ No hardcoded secrets
- ✅ Input validation on all user input
- ✅ Secure FFI boundaries
- ✅ Memory safety (Rust)
- ✅ No shell command injection

### 11.2. Distribution Security

- ✅ Code signed with Developer ID
- ✅ Notarized by Apple
- ✅ DMG signed and stapled
- ✅ HTTPS for downloads
- ✅ Checksums provided

### 11.3. Privacy

- ✅ No telemetry without consent
- ✅ No network requests (unless needed)
- ✅ Local-only data processing
- ✅ No user data collection
- ✅ Privacy policy available

---

## 12. Checklist Summary

---
### Checklist Summary

> **Note:** Checklist dưới đây đã được hợp nhất và chi tiết hóa từ tài liệu `DEPLOYMENT_CHECKLIST.md` (đã deprecated, xem lịch sử git nếu cần bản gốc). Checklist này bao gồm tất cả các bước kiểm tra, xác nhận và quy trình triển khai Vietnamese IME production.

---

#### ✅ PRE-RELEASE CHECKLIST

**Code Quality**
- [ ] All unit tests passing
- [ ] Integration tests passing
- [ ] Performance benchmarks met (< 16ms latency)
- [ ] No memory leaks detected
- [ ] Code reviewed and approved
- [ ] No compiler warnings in Release mode

**Documentation**
- [ ] User guide updated
- [ ] CHANGELOG.md updated with all changes
- [ ] Known issues documented
- [ ] Installation instructions tested
- [ ] README.md download links prepared

**Version Management**
- [ ] Version number updated in `Info.plist`
- [ ] Build number incremented
- [ ] Git branch is clean (no uncommitted changes)
- [ ] All changes committed and pushed

**Legal & Security**
- [ ] LICENSE file included
- [ ] Privacy policy reviewed (if applicable)
- [ ] No hardcoded secrets or API keys
- [ ] Third-party licenses acknowledged
- [ ] Code signed with valid Developer ID certificate

---

#### 🔨 BUILD CHECKLIST

**Rust Core**
- [ ] Clean build: `cd core && cargo clean`
- [ ] Release build: `cargo build --release`
- [ ] Verify library exists: `core/target/release/libvietnamese_ime_core.dylib`
- [ ] Check library dependencies: `otool -L core/target/release/libvietnamese_ime_core.dylib`

**macOS App**
- [ ] Clean Xcode build
- [ ] Set scheme to **Release** configuration
- [ ] Build succeeds without errors
- [ ] Archive created successfully
- [ ] App exported to `dist/` folder

**Automated Build (Recommended)**
```bash
# Run complete build
./scripts/build-release.sh 1.0.0

# Verify output
ls -la platforms/macos/VietnameseIMEFast/dist/VietnameseIMEFast.app
```

---

#### 🔐 CODE SIGNING CHECKLIST

**Prerequisites**
- [ ] Apple Developer account enrolled ($99/year)
- [ ] Developer ID Application certificate installed
- [ ] Certificate visible in Keychain Access
- [ ] Certificate not expired

**Sign Rust Library**
```bash
codesign --force --sign "Developer ID Application: Your Name (TEAM_ID)" \
  --options runtime \
  --timestamp \
  core/target/release/libvietnamese_ime_core.dylib
```
- [ ] Signing successful
- [ ] Verify: `codesign --verify --verbose core/target/release/libvietnamese_ime_core.dylib`

**Sign macOS App**
- [ ] App signed during Xcode Archive/Export
- [ ] Or manually: `codesign --force --sign "Developer ID Application" --deep VietnameseIMEFast.app`
- [ ] Verify: `codesign -dvv VietnameseIMEFast.app`

---

#### 📦 DMG CREATION CHECKLIST

**Create DMG**
```bash
# Automated (recommended)
./scripts/create-dmg.sh 1.0.0

# Verify output
ls -la platforms/macos/VietnameseIMEFast/dist/VietnameseIME-1.0.0.dmg
```

**Manual DMG Creation**
- [ ] Create folder with app and README
- [ ] Create Applications symlink
- [ ] Create DMG: `hdiutil create -volname "Vietnamese IME" -srcfolder dist/ -format UDZO VietnameseIME-1.0.0.dmg`
- [ ] Sign DMG: `codesign --sign "Developer ID Application" VietnameseIME-1.0.0.dmg`

**Verify DMG**
- [ ] DMG file exists
- [ ] DMG is signed: `codesign -dvv VietnameseIME-1.0.0.dmg`
- [ ] DMG is mountable: `hdiutil verify VietnameseIME-1.0.0.dmg`
- [ ] Calculate checksum: `shasum -a 256 VietnameseIME-1.0.0.dmg > VietnameseIME-1.0.0.dmg.sha256`

---

#### 🍎 NOTARIZATION CHECKLIST

**Prerequisites**
- [ ] App-specific password created at appleid.apple.com
- [ ] Notarytool configured with credentials
- [ ] Profile stored: `xcrun notarytool store-credentials "notary-profile"`

**Submit for Notarization**
```bash
# Automated (recommended)
./scripts/notarize.sh dist/VietnameseIME-1.0.0.dmg

# Manual
xcrun notarytool submit dist/VietnameseIME-1.0.0.dmg \
  --keychain-profile "notary-profile" \
  --wait
```

**Checklist**
- [ ] Submission accepted (not rejected)
- [ ] Wait for approval (5-10 minutes)
- [ ] Staple ticket: `xcrun stapler staple VietnameseIME-1.0.0.dmg`
- [ ] Verify staple: `xcrun stapler validate VietnameseIME-1.0.0.dmg`
- [ ] Gatekeeper check: `spctl -a -t open --context context:primary-signature -v VietnameseIME-1.0.0.dmg`
- [ ] Update checksum (stapling changes file)

**If Notarization Fails**
- [ ] Get detailed log: `xcrun notarytool log SUBMISSION_ID --keychain-profile "notary-profile"`
- [ ] Fix issues (unsigned libraries, missing entitlements, etc.)
- [ ] Re-sign and re-submit

---

#### 🧪 TESTING BEFORE RELEASE

**Functional Testing**
- [ ] App launches successfully
- [ ] Menu bar icon appears
- [ ] Toggle Vietnamese/English works (Cmd+Shift+V)
- [ ] Telex input works correctly (e.g., `hoa` → `hòa`)
- [ ] VNI input works (if implemented)
- [ ] Tone marks applied correctly
- [ ] Smart backspace works
- [ ] Arrow keys don't interfere with text

**App-Specific Testing**
Test in at least 5 different applications:
- [ ] TextEdit
- [ ] Notes
- [ ] Safari/Chrome (address bar)
- [ ] VSCode or other code editor
- [ ] Terminal/iTerm2

**Performance Testing**
- [ ] Keystroke latency < 16ms (no noticeable lag)
- [ ] No lag with fast typing (>100 WPM)
- [ ] Memory usage stable (< 50MB idle)
- [ ] CPU usage low (< 5% idle, < 20% typing)

**Clean Installation Test**
- [ ] Test on VM or clean macOS installation
- [ ] No development tools installed
- [ ] DMG opens without errors
- [ ] App installs to Applications folder
- [ ] First launch requests Accessibility permission correctly
- [ ] App works after granting permission
- [ ] No crashes or errors in Console.app

**Permission Testing**
- [ ] App requests Accessibility permission on first launch
- [ ] App detects when permission is revoked
- [ ] App shows helpful instructions for granting permission
- [ ] App works correctly after permission granted

---

#### 🏷️ GIT TAG CHECKLIST

**Create Tag**
```bash
# Create annotated tag
git tag -a v1.0.0 -m "Release version 1.0.0"

# Verify tag
git show v1.0.0
```

**Checklist**
- [ ] Tag follows semver format (v1.0.0)
- [ ] Tag is annotated (not lightweight)
- [ ] Tag message is descriptive
- [ ] No uncommitted changes before tagging
- [ ] Tag pushed to remote: `git push origin v1.0.0`

---

#### 📝 GITHUB RELEASE CHECKLIST

**Prepare Release Notes**
- [ ] Copy from `RELEASE_NOTES_v1.0.0.md` (auto-generated by release script)
- [ ] Include: version, date, changes, installation instructions
- [ ] Include: requirements (macOS 10.15+)
- [ ] Include: SHA-256 checksum for verification
- [ ] Include: known issues (if any)

**Create GitHub Release**
- [ ] Go to: `https://github.com/yourusername/vietnamese-ime/releases/new`
- [ ] Select tag: `v1.0.0`
- [ ] Title: `Vietnamese IME v1.0.0`
- [ ] Description: Paste release notes
- [ ] Upload: `VietnameseIME-1.0.0.dmg`
- [ ] Upload: `VietnameseIME-1.0.0.dmg.sha256`
- [ ] Mark as latest release
- [ ] Publish release

**Verify Release**
- [ ] Release appears on releases page
- [ ] DMG download link works
- [ ] Checksum file download works
- [ ] Release notes display correctly

---

#### 📢 POST-RELEASE CHECKLIST

**Documentation Updates**
- [ ] Update README.md with download link
- [ ] Update CHANGELOG.md with release
- [ ] Update docs with new features (if any)
- [ ] Tag documentation version

**Announcements**
- [ ] Post on social media (Twitter, etc.)
- [ ] Send email to mailing list (if exists)
- [ ] Post in community channels (Discord, Slack)
- [ ] Update website (if exists)

**Monitoring**
- [ ] Monitor GitHub Issues for bug reports
- [ ] Monitor download statistics
- [ ] Watch for user feedback
- [ ] Check crash reports (if analytics enabled)

**Backup**
- [ ] Archive build artifacts
- [ ] Backup signing certificates
- [ ] Document release process improvements
- [ ] Update this checklist if needed

---

#### 🚀 QUICK RELEASE COMMAND

For experienced users, use the automated release script:

```bash
# Complete release process (all steps)
./scripts/release.sh 1.0.0

# Skip notarization (for testing)
./scripts/release.sh 1.0.0 true
```

This script will:
1. Build Rust core (release mode)
2. Build macOS app (archive)
3. Create DMG
4. Notarize DMG (unless skipped)
5. Create git tag
6. Generate release notes

You still need to manually:
- Upload to GitHub Releases
- Update documentation
- Announce release

---

#### ⚠️ COMMON ISSUES

**"Code signing failed"**
- **Solution:** Verify certificate in Keychain Access, check it's not expired

**"Notarization rejected"**
- **Solution:** Get detailed log, check all libraries are signed, verify entitlements

**"App is damaged and can't be opened"**
- **Solution:** Remove quarantine: `xattr -d com.apple.quarantine VietnameseIMEFast.app`

**"Gatekeeper blocks app"**
- **Solution:** Ensure notarization stapled, or user right-clicks → Open

**"DMG not mountable"**
- **Solution:** Verify DMG creation didn't fail, check disk space

---

#### 📊 RELEASE METRICS

Track these metrics for each release:

- [ ] Build time: _____ minutes
- [ ] DMG size: _____ MB
- [ ] Notarization time: _____ minutes
- [ ] Total release time: _____ minutes
- [ ] Download count (1 week): _____
- [ ] Issues reported (1 week): _____

---

#### ✅ FINAL SIGN-OFF

Before announcing release publicly:

- [ ] All checklist items completed
- [ ] Release tested on at least 2 different Macs
- [ ] Download from GitHub works
- [ ] Installation instructions verified
- [ ] Support channels ready for user questions
- [ ] Team notified of release

**Released by:** _____________  
**Date:** _____________  
**Version:** _____________  
**Sign-off:** _____________

---

#### 📚 RELATED DOCUMENTATION

- **[DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md)** - Complete deployment guide (read this first!)
- **[getting-started/QUICK_START.md](getting-started/QUICK_START.md)** - Development setup
- **[getting-started/TESTING_GUIDE.md](getting-started/TESTING_GUIDE.md)** - Testing procedures
- **[project/CHANGELOG.md](project/CHANGELOG.md)** - Version history

---

---

## 13. Resources

### Documentation
- [Apple Code Signing Guide](https://developer.apple.com/support/code-signing/)
- [Notarization Guide](https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution)
- [App Distribution Overview](https://developer.apple.com/documentation/xcode/distributing-your-app-for-beta-testing-and-releases)

### Tools
- Xcode 14.0+
- Command Line Tools
- macOS 10.15+

### Support
- GitHub Issues: [Project Issues]
- Email: support@example.com
- Documentation: [Project Docs]

---

## Conclusion

This guide covers the complete deployment process for Vietnamese IME. Follow each section carefully to ensure a successful production release.

**Key Takeaways:**
1. Always test on clean macOS installation
2. Never skip code signing and notarization
3. Document all changes in changelog
4. Monitor user feedback post-release
5. Automate repetitive tasks

**Next Steps:**
1. Complete pre-deployment checklist
2. Build and test release build
3. Submit for notarization
4. Create GitHub release
5. Announce to users

---

**Status:** ✅ Complete  
**Last Updated:** 2025-12-20  
**Version:** 1.0.0  
**Maintainer:** Vietnamese IME Team