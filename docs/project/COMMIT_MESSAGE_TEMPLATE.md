# COMMIT MESSAGE TEMPLATE

Template này giúp viết commit messages rõ ràng và nhất quán cho Vietnamese IME project.

---

## Format Chuẩn

```
<type>(<scope>): <subject>

<body>

<footer>
```

---

## Types

- **feat**: Tính năng mới
- **fix**: Sửa bug
- **docs**: Thay đổi documentation
- **style**: Formatting, missing semi colons, etc (no code change)
- **refactor**: Code refactoring (không thêm feature, không fix bug)
- **perf**: Performance improvements
- **test**: Thêm hoặc sửa tests
- **chore**: Maintenance tasks, build scripts, etc
- **revert**: Revert một commit trước đó

---

## Scopes

- **core**: Rust core engine
- **macos**: macOS platform layer (Swift)
- **windows**: Windows platform layer
- **ffi**: FFI interface giữa Rust và platform layers
- **docs**: Documentation
- **build**: Build system, dependencies
- **ci**: CI/CD configuration

---

## Subject (Tiêu đề)

- Sử dụng imperative mood ("add", "fix", không phải "added", "fixed")
- Không viết hoa chữ cái đầu
- Không dấu chấm ở cuối
- Giới hạn 50 ký tự
- Viết bằng tiếng Anh

**Good:**
```
fix(macos): pass through arrow keys when IME enabled
```

**Bad:**
```
Fixed the arrow keys. (dấu chấm, quá khứ)
Arrow key fix (không rõ)
```

---

## Body (Nội dung)

- Giải thích **what** và **why**, không phải **how**
- Wrap at 72 characters
- Tách khỏi subject bằng một dòng trống
- Có thể có nhiều paragraphs

**Example:**
```
Arrow keys (←, →, ↑, ↓) were blocked when IME was enabled because
Swift layer was manually injecting characters when engine returned
action == 0.

This fix makes Swift layer pass through events when action == 0
instead of intercepting them. This allows navigation keys to work
naturally while Vietnamese input still works correctly.
```

---

## Footer (Thông tin bổ sung)

- References to issues: `Fixes #123`, `Closes #456`
- Breaking changes: `BREAKING CHANGE: <description>`
- Co-authors: `Co-authored-by: Name <email>`

**Example:**
```
Fixes #42
Closes #53

BREAKING CHANGE: Removed composition length tracking from Swift layer.
Platform implementations must now rely on engine for buffer state.

Co-authored-by: John Doe <john@example.com>
```

---

## Example: Arrow Key Fix

```
fix(macos): pass through arrow keys when IME enabled

Arrow keys (←, →, ↑, ↓) were blocked when IME was enabled because
InputManager was manually injecting characters when engine returned
action == 0 (no transformation).

Changes:
- Pass through events when action == 0 instead of manual injection
- Remove composition length tracking (engine manages buffer state)
- Simplify backspace handling (100+ lines removed)
- Establish clear event routing pattern (0=Pass, 1=Inject, 2=Restore)

This fix restores natural keyboard navigation while preserving
accurate Vietnamese input. Code is simpler and more maintainable.

Based on gonhanh.org reference implementation pattern.

Fixes #42
Closes #53

Documentation:
- docs/ARROW_KEY_FIX.md
- docs/ARROW_KEY_FIX_SUMMARY.md
- docs/BUILD_AND_TEST_ARROW_FIX.md
- docs/ARROW_KEY_FIX_CHECKLIST.md
```

---

## Example: Performance Optimization

```
perf(macos): add instant injection method for modern editors

VSCode and Zed had 14ms deletion latency due to slow backspace
injection with delays. Modern editors have fast text buffers and
don't need delays between events.

Changes:
- Add .instant injection method (zero delays)
- Route VSCode/Zed to instant method
- Keep .slow method for terminals
- Add app-specific detection logic

Performance:
- Before: 14ms per backspace
- After: < 1ms per backspace
- Improvement: 47× faster

Tested on VSCode, Zed, Sublime Text, Terminal, iTerm2.
Zero regressions.

Documentation:
- docs/PERFORMANCE_OPTIMIZATION_GUIDE.md
- docs/EDITOR_PERFORMANCE_OPTIMIZATION.md
```

---

## Example: Bug Fix

```
fix(core): backspace not removing tone marks correctly

Backspace was not restoring previous syllable state when removing
tone marks (e.g., "hoá" + backspace should give "hoa" not "ho").

Root cause: Engine was clearing entire buffer instead of rebuilding
from syllable boundary.

Changes:
- Add syllable boundary detection
- Rebuild from last syllable start on backspace
- Preserve previous state in raw_input buffer

Fixes #38

Tests:
- Added test_backspace_removes_tone_marks
- Added test_backspace_restores_previous_state
- All 50+ existing tests still pass
```

---

## Example: Documentation

```
docs: add arrow key fix documentation suite

Added comprehensive documentation for arrow key fix:
- ARROW_KEY_FIX.md (202 lines) - detailed explanation
- ARROW_KEY_FIX_SUMMARY.md (102 lines) - quick summary
- BUILD_AND_TEST_ARROW_FIX.md (297 lines) - build and test guide
- ARROW_KEY_FIX_CHECKLIST.md (119 lines) - quick checklist

Also updated:
- RUST_CORE_ROADMAP.md with recent achievements
- PROJECT_STATUS.md with current status
- CHANGELOG.md with version history
- docs/README.md with navigation

Total: 1,869 lines of new documentation
```

---

## Example: Refactor

```
refactor(macos): simplify InputManager event routing

Simplified event routing logic to follow clear pattern:
- action == 0: Pass through
- action == 1: Inject transformation
- action == 2: Restore (ESC key)

Changes:
- Remove composition length tracking (100+ lines)
- Remove manual character injection logic
- Remove complex backspace special-case handling
- Establish single source of truth (Rust engine)

No functional changes. All tests pass.
Code is now easier to understand and maintain.
```

---

## Tips

### DO:
✅ Write clear, descriptive subjects  
✅ Explain the "why" in body  
✅ Reference issues and PRs  
✅ List breaking changes explicitly  
✅ Add test information  
✅ Mention documentation updates

### DON'T:
❌ Write vague subjects ("fix bug", "update code")  
❌ Mix multiple unrelated changes  
❌ Forget to mention breaking changes  
❌ Skip the body for non-trivial changes  
❌ Use past tense ("fixed", "added")

---

## Quick Templates

### Feature
```
feat(<scope>): add <feature name>

<Why this feature is needed>
<How it works (brief)>
<Any important details>

Closes #<issue>
```

### Bug Fix
```
fix(<scope>): <what was broken>

<Why it was broken>
<How the fix works>

Fixes #<issue>
```

### Documentation
```
docs: <what documentation was added/updated>

<List of files changed>
<Summary of content>
```

### Performance
```
perf(<scope>): <what was optimized>

Before: <metric>
After: <metric>
Improvement: <percentage>

<Brief explanation>
```

---

## Commit Message Checklist

Before committing, ask yourself:

- [ ] Subject is clear and under 50 characters?
- [ ] Body explains WHY not just WHAT?
- [ ] Breaking changes are mentioned?
- [ ] Related issues are referenced?
- [ ] Tests added/updated are mentioned?
- [ ] Documentation updates are noted?
- [ ] Type and scope are correct?

---

## Git Command

```bash
# Use template
git commit -t docs/COMMIT_MESSAGE_TEMPLATE.md

# Or create commit
git commit -m "type(scope): subject" -m "body" -m "footer"

# View last commit
git log -1 --pretty=format:"%h %s%n%n%b%n%n%N"
```

---

**Last Updated:** 2024  
**Reference:** Conventional Commits (https://www.conventionalcommits.org/)