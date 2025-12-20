# Homebrew Cask for Vietnamese IME

This directory contains the Homebrew Cask formula for Vietnamese IME.

## Installation

### From Custom Tap (Recommended)

```bash
# Add your custom tap
brew tap YOURUSERNAME/vietnamese-ime https://github.com/YOURUSERNAME/homebrew-vietnamese-ime

# Install the cask
brew install --cask vietnamese-ime
```

### Direct Install (Testing)

```bash
# Install directly from local cask file
brew install --cask homebrew/vietnamese-ime.rb
```

## Post-Installation

1. **Bypass Gatekeeper:**
   ```bash
   xattr -cr /Applications/VietnameseIMEFast.app
   ```

2. **Launch the app:**
   ```bash
   open /Applications/VietnameseIMEFast.app
   ```

3. **Grant Accessibility permission** when prompted

4. **Start using:**
   - Menu bar icon appears (🇻🇳)
   - Click to toggle Vietnamese/English
   - Or use keyboard shortcut: Cmd+Shift+V

## Updating the Cask

When releasing a new version:

1. Build new unsigned DMG:
   ```bash
   ./scripts/build-dmg-unsigned.sh 1.1.0
   ```

2. Upload DMG to GitHub Releases

3. Generate new cask:
   ```bash
   ./scripts/create-homebrew-cask.sh 1.1.0 https://github.com/USER/REPO/releases/download/v1.1.0/VietnameseIME-1.1.0-unsigned.dmg
   ```

4. Update tap repository:
   ```bash
   cd ../homebrew-vietnamese-ime
   cp ../vietnamese-ime/homebrew/vietnamese-ime.rb Casks/
   git add Casks/vietnamese-ime.rb
   git commit -m "Update Vietnamese IME to v1.1.0"
   git push
   ```

5. Users update with:
   ```bash
   brew upgrade --cask vietnamese-ime
   ```

## Creating Your Own Tap

1. **Create new repository:** `homebrew-vietnamese-ime`

2. **Create Casks directory:**
   ```bash
   mkdir -p Casks
   cp vietnamese-ime.rb Casks/
   ```

3. **Commit and push:**
   ```bash
   git init
   git add .
   git commit -m "Initial commit"
   git remote add origin https://github.com/YOURUSERNAME/homebrew-vietnamese-ime.git
   git push -u origin main
   ```

4. **Users can now install:**
   ```bash
   brew tap YOURUSERNAME/vietnamese-ime
   brew install --cask vietnamese-ime
   ```

## Submitting to Official Homebrew Cask

To submit to official homebrew-cask repository:

1. Fork: https://github.com/Homebrew/homebrew-cask

2. Add your cask:
   ```bash
   cp vietnamese-ime.rb /path/to/homebrew-cask/Casks/v/
   ```

3. Test locally:
   ```bash
   brew install --cask Casks/v/vietnamese-ime.rb
   brew audit --cask vietnamese-ime
   ```

4. Submit PR to homebrew-cask

**Note:** Official homebrew-cask has strict requirements:
- Must have 75+ GitHub stars
- Must be actively maintained
- Must pass all audits

For smaller projects, using a custom tap is recommended.

## Files Generated

- `vietnamese-ime.rb` - Homebrew Cask formula
- `README.md` - This file

## Testing

```bash
# Audit cask
brew audit --cask vietnamese-ime.rb

# Test installation
brew install --cask vietnamese-ime.rb

# Test uninstallation
brew uninstall --cask vietnamese-ime

# Test upgrade
brew upgrade --cask vietnamese-ime
```

## Troubleshooting

### "App is damaged and can't be opened"

```bash
xattr -cr /Applications/VietnameseIMEFast.app
```

### "App doesn't have permission"

Grant Accessibility permission:
- System Preferences → Security & Privacy → Privacy → Accessibility
- Add Vietnamese IME and enable

### Reinstall

```bash
brew uninstall --cask vietnamese-ime
brew install --cask vietnamese-ime
xattr -cr /Applications/VietnameseIMEFast.app
```

## Support

- GitHub Issues: https://github.com/YOURUSERNAME/vietnamese-ime/issues
- Documentation: https://github.com/YOURUSERNAME/vietnamese-ime/docs

---

**Generated:** 2025-12-20  
**Version:** 1.0.0
