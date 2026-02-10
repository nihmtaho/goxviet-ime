# Release Notes Guide

This guide explains how the release notes system works for GoxViet releases.

## Overview

The release notes system has been redesigned to use CHANGELOG.md as the single source of truth, with optional detailed release notes in `.release-notes/` directory.

## File Structure

```
├── CHANGELOG.md                          # Single source of truth
├── .release-notes/                       # Optional detailed notes
│   └── release_note_X.Y.Z.md
└── scripts/
    └── extract-changelog.sh              # Extraction script
```

## How It Works

### 1. CHANGELOG.md Format

Each version entry in CHANGELOG.md should follow this format:

```markdown
## [X.Y.Z] - YYYY-MM-DD

> 📝 **Release Note**: [.release-notes/release_note_X.Y.Z.md](.release-notes/release_note_X.Y.Z.md)

### ✨ Features
- Feature 1
- Feature 2

### 🐛 Bug Fixes
- Bug fix 1
- Bug fix 2

### ⚡ Improvements
- Improvement 1

---
```

**Important**: 
- The reference line `> 📝 **Release Note**: ...` is optional but recommended
- The separator `---` marks the end of the section
- Version header must match pattern: `## [X.Y.Z] - YYYY-MM-DD`

### 2. Optional Detailed Release Notes

For major releases, you can create a detailed release note file at:
```
.release-notes/release_note_X.Y.Z.md
```

This file can contain:
- Detailed explanations
- Migration guides
- Breaking changes
- Screenshots
- Code examples

### 3. Release Workflow

When a tag is pushed (e.g., `v2.0.9`), the GitHub Actions workflow:

1. **Extract changelog** from CHANGELOG.md using `scripts/extract-changelog.sh`
2. **Generate release notes**:
   - Header: `# Gõ Việt (GoxViet) vX.Y.Z`
   - Changelog content (from CHANGELOG.md)
   - Reference to detailed note (if exists)
   - Installation instructions
3. **Create GitHub Release** with the generated notes and DMG file

## Usage

### For Maintainers

#### Creating a New Release

1. **Update CHANGELOG.md**:
   ```bash
   # Add new version entry at the top
   vim CHANGELOG.md
   ```

2. **Optional: Create detailed release note**:
   ```bash
   # Create detailed note if needed
   vim .release-notes/release_note_2.0.9.md
   ```

3. **Add reference in CHANGELOG.md**:
   ```markdown
   ## [2.0.9] - 2026-02-10
   
   > 📝 **Release Note**: [.release-notes/release_note_2.0.9.md](.release-notes/release_note_2.0.9.md)
   
   ### ✨ Features
   ...
   ```

4. **Commit and push**:
   ```bash
   git add CHANGELOG.md .release-notes/
   git commit -m "docs: release 2.0.9"
   git push origin main
   ```

5. **Create and push tag**:
   ```bash
   git tag v2.0.9
   git push origin v2.0.9
   ```

The release workflow will automatically:
- Build the app
- Extract changelog from CHANGELOG.md
- Create GitHub Release with proper notes

### Testing Extraction Script

Test the extraction script locally:

```bash
# Extract changelog for a version
./scripts/extract-changelog.sh 2.0.9

# Test with another version
./scripts/extract-changelog.sh 2.0.8
```

## Migration from Old System

The old system used:
```
docs/release-note/RELEASE_NOTE_X.Y.Z.md
```

This has been replaced by:
1. Short changelog in `CHANGELOG.md` (required)
2. Optional detailed note in `.release-notes/release_note_X.Y.Z.md`

### Why?

- **Single source of truth**: CHANGELOG.md is always up-to-date
- **Better GitHub integration**: Changelog is visible in repository
- **Flexibility**: Can still add detailed notes when needed
- **Automation**: No need to maintain separate release note files for simple releases

## Example

See the release workflow in action:
- CHANGELOG.md entry: [2.0.9](../../CHANGELOG.md#209---2026-02-10)
- Detailed note: [release_note_2.0.9.md](../release_note_2.0.9.md)
- Extraction script: [extract-changelog.sh](../../scripts/extract-changelog.sh)

## Troubleshooting

### Changelog not extracted

**Problem**: Workflow shows "⚠️ Failed to extract changelog"

**Solutions**:
1. Check version format in CHANGELOG.md matches `## [X.Y.Z] - YYYY-MM-DD`
2. Ensure there's a `---` separator or next version header
3. Test locally: `./scripts/extract-changelog.sh X.Y.Z`

### Reference link broken

**Problem**: Reference to `.release-notes/release_note_X.Y.Z.md` is 404

**Solutions**:
1. Create the file if you want detailed notes
2. Or remove the reference line from CHANGELOG.md
3. The release will still work without detailed notes

## Best Practices

1. **Keep CHANGELOG.md concise**: Use bullet points, avoid long explanations
2. **Use detailed notes for major releases**: Breaking changes, migration guides
3. **Always test extraction**: Run `./scripts/extract-changelog.sh` before tagging
4. **Consistent formatting**: Follow emoji conventions (✨🐛⚡📊)
5. **Add reference line**: Link to detailed note even if file doesn't exist yet

---

**Last updated**: 2026-02-10
**Maintained by**: GoxViet Team
