# Output Encoding User Guide

**Feature:** Output Encoding Selection  
**Introduced:** Phase 2.9  
**Location:** Settings → Advanced Tab → Character Encoding

---

## What is Output Encoding?

Output Encoding determines how Vietnamese characters are represented in the output. Different encodings are used for compatibility with various software and systems.

---

## Available Encodings

### 🌟 Unicode (UTF-8) - **RECOMMENDED**

**Best for:**
- All modern applications (web browsers, text editors, email)
- New documents and communications
- Cross-platform compatibility (Windows, macOS, Linux)
- International standards

**Advantages:**
- ✅ Works everywhere
- ✅ Correct display on all systems
- ✅ Future-proof
- ✅ No conversion needed

**Use this unless you have a specific reason to use legacy encodings.**

---

### 📦 Legacy Encodings

**⚠️ Warning:** These encodings may display incorrectly in modern applications.

#### TCVN3 (ABC)
- Vietnamese standard from 1990s
- Used by old Vietnamese software
- May show garbled text in modern apps

**Use only when:**
- Opening old Vietnamese documents
- Working with legacy Vietnamese software
- Compatibility with TCVN3-only systems

#### VNI Windows
- VNI Corporation encoding
- Popular in Vietnam in 2000s
- Limited modern support

**Use only when:**
- Working with old VNI software
- Editing VNI-encoded documents
- Compatibility requirements

#### Windows-1258 (CP1258)
- Microsoft Vietnamese code page
- Windows-specific encoding
- Limited cross-platform support

**Use only when:**
- Old Windows applications require it
- Legacy Windows documents
- Windows-specific compatibility

---

## Changing Encoding

### Step-by-Step

1. Open GoxViet Settings
2. Navigate to **Advanced** tab
3. Find **Character Encoding** section
4. Click encoding dropdown
5. Select desired encoding
6. If selecting legacy encoding:
   - Read warning dialog
   - Click "Change" to confirm

### Warning Dialog

When switching to legacy encoding, you'll see:

```
⚠️ Legacy Encoding Warning

TCVN3 (ABC) is a legacy encoding that may not display 
correctly in modern applications.

For best compatibility, we recommend using Unicode (UTF-8).

Continue with TCVN3?
```

---

## When to Use Each Encoding

### Use Unicode (UTF-8) when:
- ✅ Writing emails
- ✅ Creating new documents
- ✅ Browsing the web
- ✅ Using modern text editors
- ✅ Cross-platform work
- ✅ **Default choice**

### Use TCVN3 when:
- 📄 Opening old TCVN3 documents
- 🔧 Working with legacy Vietnamese software
- 🏢 Company requires TCVN3 format

### Use VNI Windows when:
- 📄 Editing VNI-encoded files
- 🔧 Legacy VNI software compatibility
- 🏢 Specific VNI requirements

### Use Windows-1258 when:
- 🪟 Old Windows applications
- 📄 CP1258-encoded documents
- 🏢 Windows-specific compatibility

---

## Encoding Comparison

| Feature | Unicode | TCVN3 | VNI | CP1258 |
|---------|---------|-------|-----|--------|
| Modern apps | ✅ Perfect | ❌ May fail | ❌ May fail | ⚠️ Limited |
| Web browsers | ✅ Yes | ❌ No | ❌ No | ⚠️ Limited |
| Email | ✅ Yes | ❌ No | ❌ No | ⚠️ Limited |
| Cross-platform | ✅ Yes | ❌ No | ❌ No | ❌ No |
| Legacy software | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
| Recommended | ✅✅✅ | ❌ | ❌ | ❌ |

---

## Troubleshooting

### Text Displays as Gibberish

**Problem:** Vietnamese text shows as strange characters.

**Solution:**
1. Check encoding setting in GoxViet
2. Switch to Unicode (UTF-8)
3. If working with legacy document, switch to matching encoding

### Can't Open Old Document

**Problem:** Old Vietnamese document displays incorrectly.

**Solution:**
1. Identify document encoding (check file properties or ask creator)
2. Switch GoxViet to matching encoding
3. Open document
4. Consider converting to Unicode for future use

### Encoding Changes Don't Apply

**Problem:** Changed encoding but output unchanged.

**Solution:**
1. Close and reopen application
2. Type new text to see effect
3. Restart GoxViet if needed

---

## Converting Between Encodings

### Manual Conversion

1. Open document in correct encoding
2. Select all text (Cmd+A)
3. Copy (Cmd+C)
4. Switch GoxViet to Unicode
5. Paste in new document (Cmd+V)
6. Save as new file

### Future Feature

Automatic encoding detection and conversion planned for future release.

---

## Best Practices

### For New Users
- ✅ Always use Unicode (UTF-8)
- ✅ Never change unless required
- ✅ Educate others to use Unicode

### For Legacy Support
- 🔄 Convert old documents to Unicode when possible
- 📝 Document which encoding each file uses
- 🔧 Use virtual machine for very old software

### For Organizations
- 📋 Standardize on Unicode for all new documents
- 📚 Create conversion plan for legacy files
- 👥 Train staff on encoding importance

---

## FAQ

### Q: What encoding should I use?

**A:** Unicode (UTF-8) unless you specifically need legacy encoding for old documents/software.

---

### Q: Will changing encoding affect existing text?

**A:** No. Encoding only affects new text you type after changing the setting.

---

### Q: Can I convert old documents to Unicode?

**A:** Yes, manually by copying/pasting. Automatic conversion planned for future.

---

### Q: Why do legacy encodings exist?

**A:** They were created before Unicode became standard. They're maintained for compatibility with old documents and software.

---

### Q: Is Unicode the same as UTF-8?

**A:** UTF-8 is the most common encoding of Unicode. When we say "Unicode", we mean UTF-8.

---

### Q: Can I use multiple encodings simultaneously?

**A:** No. GoxViet uses one encoding at a time for all output.

---

## Technical Details

### Character Representation

**Unicode (UTF-8):**
- `à` = U+00E0 (1 code point)
- `ớ` = U+1EDB (1 code point)

**TCVN3:**
- `à` = 0xB5 (1 byte)
- Uses 1-byte per character

**VNI:**
- `à` = 0xE0 (1 byte) 
- Different mapping than TCVN3

**Windows-1258:**
- `à` = 0xE0 (1 byte)
- Microsoft-specific mapping

---

## Support

### Get Help

- **Documentation:** `.docs/features/platform/macos/settings_features.md`
- **GitHub Issues:** Report encoding bugs
- **Email:** support@goxviet.com (if available)

---

## Encoding Resources

### Learn More
- Unicode Consortium: https://unicode.org
- Vietnamese encoding history (Wikipedia)
- Character encoding basics (MDN)

---

**For best results, always use Unicode! 🌍**
