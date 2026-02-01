# Shift+Backspace Word Deletion Guide

**Feature:** Shift+Backspace to Delete Word  
**Introduced:** Phase 2.9  
**Location:** Settings → General Tab → Restore Settings

---

## What is Shift+Backspace?

Shift+Backspace is a productivity feature that allows you to delete an entire word with a single keystroke, instead of pressing Backspace multiple times.

---

## How It Works

### Normal Backspace
- Deletes one character at a time
- Requires multiple presses to delete a word

**Example:**
```
Text: "Hello world"
Press: Backspace (5 times)
Result: "Hello "
```

### Shift+Backspace
- Deletes entire word to the left of cursor
- Only requires one press

**Example:**
```
Text: "Hello world"
Press: Shift+Backspace (once)
Result: "Hello "
```

---

## Enabling the Feature

### Step-by-Step

1. Open GoxViet Settings
2. Navigate to **General** tab
3. Scroll to **Restore Settings** section
4. Find "Shift+Backspace to delete word"
5. Toggle the switch **ON**
6. Feature is now active

### Disabling

Follow same steps and toggle switch **OFF**.

---

## Using Shift+Backspace

### Basic Usage

1. Type some words: `Hello world example`
2. Place cursor after a word: `Hello world example|`
3. Press **Shift+Backspace**
4. Entire word deleted: `Hello world |`

### Multiple Words

Delete multiple words by pressing Shift+Backspace repeatedly:

```
Text: "The quick brown fox"
Cursor position: "The quick brown fox|"

Press Shift+Backspace once → "The quick brown |"
Press Shift+Backspace again → "The quick |"
Press Shift+Backspace again → "The |"
Press Shift+Backspace again → "|"
```

---

## Word Boundaries

### What Counts as a "Word"?

GoxViet uses the system's word boundary detection, which varies by application:

**Typical word boundaries:**
- Spaces: `Hello world` (2 words)
- Punctuation: `Hello, world` (2 words)
- Line breaks: `Hello\nworld` (2 words)

### Cross-App Compatibility

Shift+Backspace works in **all macOS applications**:
- ✅ TextEdit
- ✅ Chrome / Safari
- ✅ VSCode / Xcode
- ✅ Terminal
- ✅ Notes
- ✅ Mail
- ✅ Microsoft Office
- ✅ Slack / Discord
- ✅ Any text input field

---

## Examples

### Example 1: Text Editor

```
Type: "This is a test sentence"
Cursor: "This is a test sentence|"
Shift+Backspace: "This is a test |"
```

### Example 2: Code Editor

```
Type: "function calculateTotal() {"
Cursor: "function calculateTotal() {|"
Shift+Backspace: "function calculateTotal() |"
Shift+Backspace: "function calculateTotal|"
Shift+Backspace: "function |"
```

### Example 3: Vietnamese Text

```
Type: "Xin chào bạn"
Cursor: "Xin chào bạn|"
Shift+Backspace: "Xin chào |"
```

---

## Technical Implementation

### How It Works Internally

When you press Shift+Backspace:

1. **Detection**: InputManager detects keyCode 51 (Backspace) + Shift modifier
2. **Selection**: Simulates Cmd+Shift+Left Arrow (select word backward)
3. **Deletion**: Presses Delete key to remove selected text
4. **Cleanup**: Clears engine buffer to maintain state

### Why This Method?

- ✅ **Universal**: Works in all applications
- ✅ **Reliable**: Uses system text selection
- ✅ **Respects app logic**: Each app defines own word boundaries
- ✅ **No parsing**: No need to manually detect words

### Performance

- Overhead: ~1ms for word selection
- Imperceptible to user
- No impact on typing speed

---

## Troubleshooting

### Shift+Backspace Not Working

**Problem:** Press Shift+Backspace but character deleted instead of word.

**Solutions:**
1. Check feature is enabled in Settings
2. Verify you're pressing Shift+Backspace (not just Backspace)
3. Try in different application (TextEdit)
4. Restart GoxViet

### Wrong Text Deleted

**Problem:** More/less text deleted than expected.

**Explanation:** Each app has different word boundary logic:
- TextEdit: Deletes to previous space
- VSCode: Deletes to previous space/symbol
- Terminal: May include path separators

This is expected behavior - Shift+Backspace respects each app's logic.

### Feature Disabled

**Problem:** Feature worked before, now doesn't.

**Solutions:**
1. Open Settings → General
2. Check "Shift+Backspace to delete word" is ON
3. Toggle OFF then ON
4. Restart GoxViet

---

## Tips & Best Practices

### Keyboard Placement

- Backspace is top-right (usually)
- Shift is bottom-left (usually)
- Takes practice to press simultaneously

**Practice tip:** Type "word word word", then practice Shift+Backspace.

### When to Use

✅ **Good for:**
- Deleting entire wrong words
- Fast editing
- Cleaning up mistakes
- Code refactoring

❌ **Not ideal for:**
- Deleting single characters (use normal Backspace)
- Precise editing (use mouse selection)
- Complex selections (use Cmd+Shift+Arrow)

### Muscle Memory

After ~1 week of use, Shift+Backspace becomes automatic:
- Week 1: Think before pressing
- Week 2: Press naturally
- Week 3+: Can't live without it

---

## Alternatives

### macOS Built-in Shortcuts

GoxViet's Shift+Backspace is similar to:
- **Option+Backspace**: Delete word backward (built-in)
- **Cmd+Backspace**: Delete to start of line

**Why Shift+Backspace?**
- Easier to press than Option+Backspace
- More ergonomic for frequent use
- Matches some Windows keyboard layouts

### Can Use Both

You can still use Option+Backspace even with Shift+Backspace enabled.

---

## Comparison

| Shortcut | Deletes | Built-in? | Ergonomic? |
|----------|---------|-----------|------------|
| Backspace | 1 char | ✅ Yes | ✅ Easy |
| Shift+Backspace | 1 word | ❌ No (GoxViet) | ✅ Easy |
| Option+Backspace | 1 word | ✅ Yes | ⚠️ Harder to press |
| Cmd+Backspace | To line start | ✅ Yes | ✅ Easy |

---

## FAQ

### Q: Does this work when GoxViet is disabled?

**A:** Yes! Shift+Backspace works even when Vietnamese input is disabled, as long as GoxViet app is running.

---

### Q: Can I customize the keyboard shortcut?

**A:** Not yet. Shift+Backspace is fixed. Customization planned for future.

---

### Q: Does it work in Terminal?

**A:** Yes, works in all applications including Terminal.

---

### Q: What if my app already uses Shift+Backspace?

**A:** Very rare, but if conflict occurs, you can disable the feature in Settings.

---

### Q: Is this safe? Will it cause data loss?

**A:** Safe. It's just a shortcut for built-in text selection + deletion. Standard undo (Cmd+Z) works.

---

### Q: Performance impact?

**A:** Negligible. ~1ms overhead, imperceptible to user.

---

### Q: Can I undo after Shift+Backspace?

**A:** Yes, standard Cmd+Z (Undo) works in all applications.

---

## Advanced Usage

### Combining with Other Shortcuts

**Fast Line Deletion:**
```
Cmd+Left (start of line)
Shift+Backspace repeatedly (delete words)
```

**Fast Paragraph Cleanup:**
```
Cmd+Up (start of paragraph)
Cmd+Shift+Down (select paragraph)
Backspace (delete)
```

### Scripting (AppleScript)

Currently not exposed to AppleScript. May be added in future.

---

## Future Enhancements

Planned improvements:
- [ ] Customizable shortcut (e.g., Ctrl+Backspace)
- [ ] Option: Delete word forward (Shift+Delete)
- [ ] Option: Delete to line start (Shift+Cmd+Backspace)
- [ ] Visual feedback (brief highlight)

---

## Support

### Report Issues

If Shift+Backspace doesn't work in specific app:
1. Note app name and version
2. Describe expected vs actual behavior
3. Report via GitHub Issues

---

**Delete words faster with Shift+Backspace! ⌨️**
