# Requesting English Words for Auto-Restore

## For Users

If you encounter an English word that gets incorrectly transformed to Vietnamese, you can request it to be added to the auto-restore dictionary.

### How to Request

1. Go to [GitHub Issues](https://github.com/nihmtaho/goxviet-ime/issues/new/choose)
2. Select "English Word Auto-Restore Request"
3. Fill in the template:
   - **Word to add**: The English word (e.g., "mason", "nurses")
   - **Current behavior**: What happens when you type it
   - **Expected behavior**: What should happen instead
4. Submit the issue

### Example Request

**Word to add:** mason

**Current behavior:** When I type "mason" in Telex mode, it becomes "máon" with Vietnamese tone marks.

**Expected behavior:** The word "mason" should remain as "mason" without any Vietnamese transforms.

### What Happens Next

1. A developer will review your request
2. The word will be added to the dictionary
3. A new version will be released
4. You'll be notified when it's available

## For Developers

See [ADDING_ENGLISH_WORDS.md](core/docs/ADDING_ENGLISH_WORDS.md) for detailed instructions on processing word requests.

### Quick Process

1. Calculate hash: `python3 scripts/calc_hash.py <word>`
2. Add to dictionary in `core/src/engine_v2/english/dictionary.rs`
3. Add test case in `core/tests/double_consonant_test.rs`
4. Run tests: `cargo test`
5. Build: `./scripts/rust_build_lib_universal_for_macos.sh`

## Recently Added Words

### Version 1.5.2 (January 2026)

**Double consonant endings (-son, -ton, -ron):**
- mason, season, reason, person, poison, prison, lesson
- button, cotton

**Multiple modifiers:**
- nurses, horses, verses, houses

**Total:** 13 new words added

## Statistics

- **Vietnamese accuracy:** 100% ✓
- **English detection:** Significantly improved
- **Dictionary size:** 50+ common English words
- **Response time:** O(1) instant lookup

## FAQ

**Q: Why does my English word get Vietnamese tone marks?**

A: GoxViet uses Telex/VNI input methods where certain keys (s, f, r, x, j) are tone modifiers. If an English word contains these letters, they may trigger Vietnamese transforms. Adding the word to the dictionary prevents this.

**Q: How long does it take to add a word?**

A: Usually 1-2 days for review and implementation, then included in the next release.

**Q: Can I add multiple words in one request?**

A: Yes! If you have several related words (e.g., "nurse", "nurses", "nursing"), please list them all in one request.

**Q: What if the word is very rare?**

A: We prioritize common words and technical terms. Very rare words may be handled via phonotactic patterns instead of dictionary entries.

**Q: Does this work for all typing modes?**

A: Dictionary lookup works for all modes (Telex, VNI, All). However, in Telex/VNI modes, you may see partial transforms before the word is complete.
