# Adding English Words to Auto-Restore Dictionary

This guide explains how to add English words to the auto-restore dictionary when users request them via GitHub issues.

## Quick Steps

1. **Calculate hash** for the word
2. **Add to dictionary** in the appropriate length function
3. **Add test case** to verify it works
4. **Run tests** to ensure no regressions

## Step-by-Step Guide

### 1. Calculate Word Hash

Use the hash calculation script:

```bash
python3 scripts/calc_hash.py <word>
```

Example:
```bash
$ python3 scripts/calc_hash.py mason
mason: 42483933
```

### 2. Add to Dictionary

Edit `core/src/engine_v2/english/dictionary.rs`:

**Determine word length:**
- 5 letters → `is_english_len5()`
- 6 letters → `is_english_len6()`
- 7 letters → `is_english_len7()`
- 8 letters → `is_english_len8()`

**Add the hash:**

```rust
fn is_english_len5(hash: u32) -> bool {
    matches!(
        hash,
        16388432 | 16388414 | 1365045 | 16053307 | // existing words
        42483933 // mason (NEW)
    )
}
```

**Add comment with word name** for maintainability.

### 3. Check if Phonotactic Pattern Update Needed

Some words may require phonotactic pattern updates in `core/src/engine_v2/english/phonotactic.rs`:

**Words ending in -son, -ton, -ron:**
- Already supported via `is_english_coda()`

**Words ending in -tion, -sion:**
- Already supported via `is_english_suffix()`

**New patterns:**
- Update `is_english_coda()` for new coda patterns
- Update `is_english_suffix()` for new suffix patterns

### 4. Add Test Case

Add to `core/tests/double_consonant_test.rs` or create new test file:

```rust
#[test]
fn test_new_word() {
    let words = vec!["mason", "your_new_word"];
    
    for word in words {
        let output = type_word(word, 2); // All mode
        assert_eq!(
            output, word,
            "Expected '{}' to remain unchanged, got '{}'",
            word, output
        );
    }
}
```

### 5. Run Tests

```bash
# Run specific test
cargo test --test double_consonant_test -- --nocapture

# Run all tests
cargo test --lib
```

### 6. Build and Verify

```bash
# Build universal library for macOS
./scripts/rust_build_lib_universal_for_macos.sh

# Test in actual application
```

## Common Patterns

### Words with Double Consonants (-son, -ton, -ron)

Examples: mason, reason, poison, button, cotton

**Hash calculation:**
```bash
python3 scripts/calc_hash.py mason reason poison button cotton
```

**Add to dictionary:**
```rust
fn is_english_len6(hash: u32) -> bool {
    matches!(
        hash,
        442368526 | 1031664297 | // reason, poison
        344997123 | 258186149    // button, cotton
    )
}
```

### Words with Multiple Modifiers

Examples: nurses, horses, verses, houses

These words have 'r' and 's' which are tone modifiers in Telex mode.

**Important:** Test in All mode to avoid modifier interference.

### Words with -tion/-sion Suffix

Examples: action, nation, version, session

These are automatically detected by suffix pattern matching, but adding to dictionary provides instant O(1) lookup.

## Checklist

When processing a user request:

- [ ] Calculate hash using `scripts/calc_hash.py`
- [ ] Add hash to appropriate `is_english_lenN()` function
- [ ] Add comment with word name
- [ ] Check if phonotactic pattern update needed
- [ ] Add test case
- [ ] Run tests (`cargo test`)
- [ ] Build library
- [ ] Update issue with status
- [ ] Close issue when merged

## Example PR Description

```markdown
## Add English word: mason

Fixes #123

### Changes
- Added "mason" to `is_english_len5()` dictionary (hash: 42483933)
- Added test case in `double_consonant_test.rs`

### Testing
- ✓ All new tests passing (1/1)
- ✓ Existing tests passing (187/203)
- ✓ Verified in macOS app

### Word Details
- Length: 5 letters
- Pattern: -son ending (double consonant)
- Frequency: Common English word
```

## Troubleshooting

### Test Failures

**Issue:** Word still gets Vietnamese transforms

**Solution:** 
1. Verify hash is correct
2. Check word length matches function name
3. Ensure hash is in the right `is_english_lenN()` function

**Issue:** Vietnamese words incorrectly detected as English

**Solution:**
1. Check if hash conflicts with Vietnamese word
2. Verify phonotactic patterns don't over-match
3. Add Vietnamese word to `is_vietnamese_lenN()` if needed

### Build Errors

**Issue:** Compilation fails

**Solution:**
1. Check Rust syntax (commas, pipes)
2. Ensure all `matches!()` arms are correct
3. Run `cargo check` for detailed errors

## Resources

- [Implementation Plan](file:///Users/nihmtaho/.gemini/antigravity/brain/a5e0f20b-401f-45dc-81d9-9376431bba68/implementation_plan.md)
- [Walkthrough](file:///Users/nihmtaho/.gemini/antigravity/brain/a5e0f20b-401f-45dc-81d9-9376431bba68/walkthrough.md)
- [Hash Calculator Script](file:///Users/nihmtaho/developer/personal-projects/cmlia/goxviet/scripts/calc_hash.py)
