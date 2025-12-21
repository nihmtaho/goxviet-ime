# DEPLOYMENT QUICK REFERENCE

One-page quick reference for deploying Vietnamese IME to production.

**Version:** 1.0.0 | **Date:** 2025-12-20

---

## 🚀 ONE-COMMAND RELEASE

```bash
# Complete release (recommended)
./scripts/release.sh 1.0.0

# Testing mode (skip notarization)
./scripts/release.sh 1.0.0 true
```

---

## 📋 STEP-BY-STEP COMMANDS

### 1. Build (5-10 min)
```bash
./scripts/build-release.sh 1.0.0
```

### 2. Create DMG (2-3 min)
```bash
./scripts/create-dmg.sh 1.0.0
```

### 3. Sign DMG
```bash
codesign --sign "Developer ID Application: Your Name (TEAM_ID)" \
  --timestamp \
  platforms/macos/VietnameseIMEFast/dist/VietnameseIME-1.0.0.dmg
```

### 4. Notarize (5-10 min)
```bash
./scripts/notarize.sh platforms/macos/VietnameseIMEFast/dist/VietnameseIME-1.0.0.dmg
```

### 5. Create Git Tag
```bash
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0
```

---

## ⚙️ PREREQUISITES

### First-Time Setup

1. **Install Tools**
   ```bash
   # Check if installed
   cargo --version
   xcodebuild -version
   codesign --version
   ```

2. **Apple Developer Certificate**
   - Create at: https://developer.apple.com/account/resources/certificates
   - Type: "Developer ID Application"
   - Install in Keychain Access

3. **App-Specific Password**
   - Create at: https://appleid.apple.com → Security
   - Store credentials:
   ```bash
   xcrun notarytool store-credentials "notary-profile" \
     --apple-id "your@email.com" \
     --team-id "YOUR_TEAM_ID" \
     --password "xxxx-xxxx-xxxx-xxxx"
   ```

4. **Make Scripts Executable**
   ```bash
   chmod +x scripts/*.sh
   ```

---

## ✅ PRE-RELEASE CHECKLIST

- [ ] All tests passing
- [ ] Version updated in `Info.plist`
- [ ] CHANGELOG.md updated
- [ ] No uncommitted changes
- [ ] Git tag doesn't exist yet

---

## 🧪 TESTING

### Quick Test
```bash
# Open DMG
open platforms/macos/VietnameseIMEFast/dist/VietnameseIME-1.0.0.dmg

# Verify checksum
cd platforms/macos/VietnameseIMEFast/dist
shasum -a 256 -c VietnameseIME-1.0.0.dmg.sha256
```

### Test Installation
1. Drag app to Applications
2. Launch app
3. Grant Accessibility permission
4. Toggle Vietnamese/English (Cmd+Shift+V)
5. Type: `hoa` → should become `hòa`

---

## 🐛 TROUBLESHOOTING

### Permission Denied
```bash
chmod +x scripts/*.sh
```

### Code Signing Failed
- Check certificate in Keychain Access
- Verify not expired
- Update Team ID in ExportOptions.plist

### Notarization Rejected
```bash
# Get detailed log
xcrun notarytool log SUBMISSION_ID --keychain-profile "notary-profile"
```

### Build Failed
```bash
# Clean and rebuild
cd core && cargo clean && cd ..
./scripts/build-release.sh 1.0.0
```

### DMG Won't Mount
```bash
# Remove quarantine
xattr -d com.apple.quarantine VietnameseIME-1.0.0.dmg
```

---

## 📦 OUTPUT FILES

After successful release:

```
platforms/macos/VietnameseIMEFast/dist/
├── VietnameseIMEFast.app              # App bundle
├── VietnameseIME-1.0.0.dmg            # Distributable DMG
├── VietnameseIME-1.0.0.dmg.sha256     # Checksum
└── RELEASE_NOTES_v1.0.0.md            # Release notes
```

---

## 🎯 POST-RELEASE TASKS

### 1. Create GitHub Release
```
URL: https://github.com/yourusername/vietnamese-ime/releases/new
Tag: v1.0.0
Title: Vietnamese IME v1.0.0
Files: VietnameseIME-1.0.0.dmg + .sha256
```

### 2. Update Documentation
- [ ] README.md (download link)
- [ ] CHANGELOG.md (version entry)
- [ ] docs/PROJECT_STATUS.md

### 3. Announce
- [ ] GitHub release notes
- [ ] Social media
- [ ] Email/Discord/Slack

---

## 🔍 VERIFICATION COMMANDS

```bash
# Verify code signing
codesign -dvv platforms/macos/VietnameseIMEFast/dist/VietnameseIME-1.0.0.dmg

# Verify notarization stapled
xcrun stapler validate platforms/macos/VietnameseIMEFast/dist/VietnameseIME-1.0.0.dmg

# Verify Gatekeeper
spctl -a -t open --context context:primary-signature -v platforms/macos/VietnameseIMEFast/dist/VietnameseIME-1.0.0.dmg

# Check DMG
hdiutil verify platforms/macos/VietnameseIMEFast/dist/VietnameseIME-1.0.0.dmg

# Calculate checksum
shasum -a 256 platforms/macos/VietnameseIMEFast/dist/VietnameseIME-1.0.0.dmg
```

---

## 📊 EXPECTED TIMINGS

| Task | Time |
|------|------|
| Build | 5-10 min |
| Create DMG | 2-3 min |
| Sign | < 1 min |
| Notarize | 5-10 min |
| Git tag | < 1 min |
| **Total** | **15-25 min** |

---

## 🆘 HELP

### Documentation
- **Full Guide:** [DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md)
- **Checklist:** [DEPLOYMENT_CHECKLIST.md](DEPLOYMENT_CHECKLIST.md)
- **Scripts:** [../scripts/README.md](../scripts/README.md)

### Support
- GitHub Issues: https://github.com/yourusername/vietnamese-ime/issues
- Team: Vietnamese IME Development Team

---

## 📝 COMMON COMMANDS

```bash
# View git tags
git tag -l

# Delete local tag
git tag -d v1.0.0

# Delete remote tag
git push origin :refs/tags/v1.0.0

# View tag details
git show v1.0.0

# Find Team ID
security find-identity -v -p codesigning

# List notarization history
xcrun notarytool history --keychain-profile "notary-profile"

# Check Xcode signing identity
security find-identity -v -p codesigning | grep "Developer ID Application"
```

---

## 🔐 SECURITY NOTES

- ✅ Never commit certificates or passwords
- ✅ Use app-specific passwords (not main Apple ID password)
- ✅ Store credentials in Keychain (via notarytool)
- ✅ Sign all binaries (.dylib, .app, .dmg)
- ✅ Always notarize before public release

---

## 📈 VERSION NUMBERING

Follow Semantic Versioning (semver):

- **Major (1.0.0):** Breaking changes
- **Minor (0.1.0):** New features (backward compatible)
- **Patch (0.0.1):** Bug fixes

Examples:
```bash
./scripts/release.sh 1.0.0    # Initial release
./scripts/release.sh 1.1.0    # Added VNI support
./scripts/release.sh 1.1.1    # Fixed backspace bug
./scripts/release.sh 2.0.0    # Complete rewrite
```

---

**Print this page for quick reference during releases!**

**Last Updated:** 2025-12-20  
**Status:** ✅ Production Ready