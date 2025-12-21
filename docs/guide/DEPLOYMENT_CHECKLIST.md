# DEPLOYMENT CHECKLIST - VIETNAMESE IME

Quick reference checklist for deploying Vietnamese IME to production.

**Date:** 2025-12-20  
**Version:** 1.0.0

---

## ✅ PRE-RELEASE CHECKLIST

### Code Quality
- [ ] All unit tests passing
- [ ] Integration tests passing
- [ ] Performance benchmarks met (< 16ms latency)
- [ ] No memory leaks detected
- [ ] Code reviewed and approved
- [ ] No compiler warnings in Release mode

### Documentation
- [ ] User guide updated
- [ ] CHANGELOG.md updated with all changes
- [ ] Known issues documented
- [ ] Installation instructions tested
- [ ] README.md download links prepared

### Version Management
- [ ] Version number updated in `Info.plist`
- [ ] Build number incremented
- [ ] Git branch is clean (no uncommitted changes)
- [ ] All changes committed and pushed

### Legal & Security
- [ ] LICENSE file included
- [ ] Privacy policy reviewed (if applicable)
- [ ] No hardcoded secrets or API keys
- [ ] Third-party licenses acknowledged
- [ ] Code signed with valid Developer ID certificate

---

## 🔨 BUILD CHECKLIST

### Rust Core
- [ ] Clean build: `cd core && cargo clean`
- [ ] Release build: `cargo build --release`
- [ ] Verify library exists: `core/target/release/libvietnamese_ime_core.dylib`
- [ ] Check library dependencies: `otool -L core/target/release/libvietnamese_ime_core.dylib`

### macOS App
- [ ] Clean Xcode build
- [ ] Set scheme to **Release** configuration
- [ ] Build succeeds without errors
- [ ] Archive created successfully
- [ ] App exported to `dist/` folder

### Automated Build (Recommended)
```bash
# Run complete build
./scripts/build-release.sh 1.0.0

# Verify output
ls -la platforms/macos/VietnameseIMEFast/dist/VietnameseIMEFast.app
```

---

## 🔐 CODE SIGNING CHECKLIST

### Prerequisites
- [ ] Apple Developer account enrolled ($99/year)
- [ ] Developer ID Application certificate installed
- [ ] Certificate visible in Keychain Access
- [ ] Certificate not expired

### Sign Rust Library
```bash
codesign --force --sign "Developer ID Application: Your Name (TEAM_ID)" \
  --options runtime \
  --timestamp \
  core/target/release/libvietnamese_ime_core.dylib
```
- [ ] Signing successful
- [ ] Verify: `codesign --verify --verbose core/target/release/libvietnamese_ime_core.dylib`

### Sign macOS App
- [ ] App signed during Xcode Archive/Export
- [ ] Or manually: `codesign --force --sign "Developer ID Application" --deep VietnameseIMEFast.app`
- [ ] Verify: `codesign -dvv VietnameseIMEFast.app`

---

## 📦 DMG CREATION CHECKLIST

### Create DMG
```bash
# Automated (recommended)
./scripts/create-dmg.sh 1.0.0

# Verify output
ls -la platforms/macos/VietnameseIMEFast/dist/VietnameseIME-1.0.0.dmg
```

### Manual DMG Creation
- [ ] Create folder with app and README
- [ ] Create Applications symlink
- [ ] Create DMG: `hdiutil create -volname "Vietnamese IME" -srcfolder dist/ -format UDZO VietnameseIME-1.0.0.dmg`
- [ ] Sign DMG: `codesign --sign "Developer ID Application" VietnameseIME-1.0.0.dmg`

### Verify DMG
- [ ] DMG file exists
- [ ] DMG is signed: `codesign -dvv VietnameseIME-1.0.0.dmg`
- [ ] DMG is mountable: `hdiutil verify VietnameseIME-1.0.0.dmg`
- [ ] Calculate checksum: `shasum -a 256 VietnameseIME-1.0.0.dmg > VietnameseIME-1.0.0.dmg.sha256`

---

## 🍎 NOTARIZATION CHECKLIST

### Prerequisites
- [ ] App-specific password created at appleid.apple.com
- [ ] Notarytool configured with credentials
- [ ] Profile stored: `xcrun notarytool store-credentials "notary-profile"`

### Submit for Notarization
```bash
# Automated (recommended)
./scripts/notarize.sh dist/VietnameseIME-1.0.0.dmg

# Manual
xcrun notarytool submit dist/VietnameseIME-1.0.0.dmg \
  --keychain-profile "notary-profile" \
  --wait
```

### Checklist
- [ ] Submission accepted (not rejected)
- [ ] Wait for approval (5-10 minutes)
- [ ] Staple ticket: `xcrun stapler staple VietnameseIME-1.0.0.dmg`
- [ ] Verify staple: `xcrun stapler validate VietnameseIME-1.0.0.dmg`
- [ ] Gatekeeper check: `spctl -a -t open --context context:primary-signature -v VietnameseIME-1.0.0.dmg`
- [ ] Update checksum (stapling changes file)

### If Notarization Fails
- [ ] Get detailed log: `xcrun notarytool log SUBMISSION_ID --keychain-profile "notary-profile"`
- [ ] Fix issues (unsigned libraries, missing entitlements, etc.)
- [ ] Re-sign and re-submit

---

## 🧪 TESTING BEFORE RELEASE

### Functional Testing
- [ ] App launches successfully
- [ ] Menu bar icon appears
- [ ] Toggle Vietnamese/English works (Cmd+Shift+V)
- [ ] Telex input works correctly (e.g., `hoa` → `hòa`)
- [ ] VNI input works (if implemented)
- [ ] Tone marks applied correctly
- [ ] Smart backspace works
- [ ] Arrow keys don't interfere with text

### App-Specific Testing
Test in at least 5 different applications:
- [ ] TextEdit
- [ ] Notes
- [ ] Safari/Chrome (address bar)
- [ ] VSCode or other code editor
- [ ] Terminal/iTerm2

### Performance Testing
- [ ] Keystroke latency < 16ms (no noticeable lag)
- [ ] No lag with fast typing (>100 WPM)
- [ ] Memory usage stable (< 50MB idle)
- [ ] CPU usage low (< 5% idle, < 20% typing)

### Clean Installation Test
- [ ] Test on VM or clean macOS installation
- [ ] No development tools installed
- [ ] DMG opens without errors
- [ ] App installs to Applications folder
- [ ] First launch requests Accessibility permission correctly
- [ ] App works after granting permission
- [ ] No crashes or errors in Console.app

### Permission Testing
- [ ] App requests Accessibility permission on first launch
- [ ] App detects when permission is revoked
- [ ] App shows helpful instructions for granting permission
- [ ] App works correctly after permission granted

---

## 🏷️ GIT TAG CHECKLIST

### Create Tag
```bash
# Create annotated tag
git tag -a v1.0.0 -m "Release version 1.0.0"

# Verify tag
git show v1.0.0
```

### Checklist
- [ ] Tag follows semver format (v1.0.0)
- [ ] Tag is annotated (not lightweight)
- [ ] Tag message is descriptive
- [ ] No uncommitted changes before tagging
- [ ] Tag pushed to remote: `git push origin v1.0.0`

---

## 📝 GITHUB RELEASE CHECKLIST

### Prepare Release Notes
- [ ] Copy from `RELEASE_NOTES_v1.0.0.md` (auto-generated by release script)
- [ ] Include: version, date, changes, installation instructions
- [ ] Include: requirements (macOS 10.15+)
- [ ] Include: SHA-256 checksum for verification
- [ ] Include: known issues (if any)

### Create GitHub Release
- [ ] Go to: `https://github.com/yourusername/vietnamese-ime/releases/new`
- [ ] Select tag: `v1.0.0`
- [ ] Title: `Vietnamese IME v1.0.0`
- [ ] Description: Paste release notes
- [ ] Upload: `VietnameseIME-1.0.0.dmg`
- [ ] Upload: `VietnameseIME-1.0.0.dmg.sha256`
- [ ] Mark as latest release
- [ ] Publish release

### Verify Release
- [ ] Release appears on releases page
- [ ] DMG download link works
- [ ] Checksum file download works
- [ ] Release notes display correctly

---

## 📢 POST-RELEASE CHECKLIST

### Documentation Updates
- [ ] Update README.md with download link
- [ ] Update CHANGELOG.md with release
- [ ] Update docs with new features (if any)
- [ ] Tag documentation version

### Announcements
- [ ] Post on social media (Twitter, etc.)
- [ ] Send email to mailing list (if exists)
- [ ] Post in community channels (Discord, Slack)
- [ ] Update website (if exists)

### Monitoring
- [ ] Monitor GitHub Issues for bug reports
- [ ] Monitor download statistics
- [ ] Watch for user feedback
- [ ] Check crash reports (if analytics enabled)

### Backup
- [ ] Archive build artifacts
- [ ] Backup signing certificates
- [ ] Document release process improvements
- [ ] Update this checklist if needed

---

## 🚀 QUICK RELEASE COMMAND

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

## ⚠️ COMMON ISSUES

### "Code signing failed"
- **Solution:** Verify certificate in Keychain Access, check it's not expired

### "Notarization rejected"
- **Solution:** Get detailed log, check all libraries are signed, verify entitlements

### "App is damaged and can't be opened"
- **Solution:** Remove quarantine: `xattr -d com.apple.quarantine VietnameseIMEFast.app`

### "Gatekeeper blocks app"
- **Solution:** Ensure notarization stapled, or user right-clicks → Open

### "DMG not mountable"
- **Solution:** Verify DMG creation didn't fail, check disk space

---

## 📊 RELEASE METRICS

Track these metrics for each release:

- [ ] Build time: _____ minutes
- [ ] DMG size: _____ MB
- [ ] Notarization time: _____ minutes
- [ ] Total release time: _____ minutes
- [ ] Download count (1 week): _____
- [ ] Issues reported (1 week): _____

---

## ✅ FINAL SIGN-OFF

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

## 📚 RELATED DOCUMENTATION

- **[DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md)** - Complete deployment guide (read this first!)
- **[getting-started/QUICK_START.md](getting-started/QUICK_START.md)** - Development setup
- **[getting-started/TESTING_GUIDE.md](getting-started/TESTING_GUIDE.md)** - Testing procedures
- **[project/CHANGELOG.md](project/CHANGELOG.md)** - Version history

---

**Status:** ✅ Ready for Use  
**Last Updated:** 2025-12-20  
**Maintainer:** Vietnamese IME Team