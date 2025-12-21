# BACKSPACE OPTIMIZATION - QUICK TEST GUIDE

## Mục đích
Hướng dẫn test nhanh để verify backspace optimization đã được apply thành công.

## Điều kiện tiên quyết
- ✅ Code đã được update theo `BACKSPACE_OPTIMIZATION_APPLIED.md`
- ✅ Project compile thành công
- ✅ IME đã được install và enabled trong System Preferences

## Quick Test Cases

### Test 1: VSCode - Zero Delay Test ⚡
**Mục tiêu:** Verify instant method với zero delays

**Steps:**
1. Mở VSCode
2. Tạo file mới
3. Gõ các từ sau và quan sát:
   ```
   hoaf → hòa     (expect: instant, no lag)
   truong → trường (expect: smooth, fast backspaces)
   vieejt → việt   (expect: zero delay between backspace & text)
   ```

**Expected behavior:**
- ✅ Không có độ trễ nhìn thấy được
- ✅ Backspace + text replacement < 16ms (60fps)
- ✅ Gõ nhanh 10 từ liên tiếp không bị lag

**Check logs:**
```bash
tail -f ~/Library/Logs/VietnameseIME/keyboard.log | grep "instant:editor"
```
Phải thấy: `method: instant:editor` cho VSCode

---

### Test 2: Zed - Modern Editor Test 🚀
**Mục tiêu:** Verify Zed cũng được detect đúng

**Steps:**
1. Mở Zed editor
2. Gõ test:
   ```
   hoaf → hòa
   naym → năm
   tuooi → tuổi
   ```

**Expected behavior:**
- ✅ Same instant feedback như VSCode
- ✅ No delays between events

**Check logs:**
```bash
tail -f ~/Library/Logs/VietnameseIME/keyboard.log | grep "instant:editor"
```

---

### Test 3: Terminal - Slow Method Test 🐌
**Mục tiêu:** Verify terminals vẫn dùng slow method (có delays)

**Steps:**
1. Mở iTerm2 hoặc Terminal.app
2. Gõ test (trong bash/zsh prompt):
   ```
   hoaf → hòa
   vieejt → việt
   ```

**Expected behavior:**
- ✅ Có delays nhẹ giữa backspaces (bình thường)
- ✅ Không bị lost characters
- ✅ Stable, không flicker

**Check logs:**
```bash
tail -f ~/Library/Logs/VietnameseIME/keyboard.log | grep "slow:term"
```
Phải thấy: `method: slow:term`

---

### Test 4: Chrome Address Bar - Selection Method Test 🌐
**Mục tiêu:** Verify browser address bars dùng selection method

**Steps:**
1. Mở Google Chrome
2. Click vào address bar
3. Gõ test:
   ```
   ha noi → hà nội
   viet nam → việt nam
   ```

**Expected behavior:**
- ✅ Autocomplete không bị conflict
- ✅ Text replacement hoạt động đúng
- ✅ Không mất ký tự đầu tiên

**Check logs:**
```bash
tail -f ~/Library/Logs/VietnameseIME/keyboard.log | grep "sel:browser"
```

---

## Performance Verification

### Method 1: Manual Timing Test
Gõ test trên VSCode và đếm:
```
Count: Gõ "hoaf" 10 lần liên tiếp
Feel: Có lag không? Có delay nhìn thấy được không?
Result: Phải feel "instant" như gõ tiếng Anh
```

### Method 2: Log Analysis
```bash
# Filter instant method logs
grep "instant:editor" ~/Library/Logs/VietnameseIME/keyboard.log

# Expected output:
# method: instant:editor
# send: instant 3 hòa
# send: instant 5 trường
```

### Method 3: Compare Before/After
Nếu có video recording của version cũ:
- So sánh frame-by-frame
- Đo timing từ lúc nhấn phím đến lúc text thay đổi
- Target: < 16ms (1 frame @ 60fps)

---

## Troubleshooting

### Issue 1: VSCode vẫn bị lag
**Possible causes:**
- ❌ Code chưa compile đúng
- ❌ IME cũ vẫn đang chạy
- ❌ Cache chưa clear

**Solutions:**
```bash
# 1. Rebuild
cd platforms/macos/VietnameseIMEFast
xcodebuild clean
xcodebuild build

# 2. Kill old process
killall VietnameseIMEFast

# 3. Clear logs & restart
rm ~/Library/Logs/VietnameseIME/keyboard.log
# Restart IME
```

### Issue 2: Terminal bị lost characters
**Possible causes:**
- ❌ Terminal đang dùng instant method (sai)
- ❌ Bundle ID không match

**Solutions:**
```bash
# Check terminal detection
tail -f ~/Library/Logs/VietnameseIME/keyboard.log

# Should see: "slow:term" NOT "instant:editor"
```

### Issue 3: Browser address bar bị conflict
**Possible causes:**
- ❌ Đang dùng backspace method thay vì selection
- ❌ Role detection không đúng

**Solutions:**
```bash
# Check detection
tail -f ~/Library/Logs/VietnameseIME/keyboard.log

# Should see: "sel:browser" NOT "instant:editor"
```

---

## Success Criteria

### ✅ PASS nếu:
1. **VSCode/Zed:** Instant feedback, no lag, < 16ms latency
2. **Terminals:** Stable với delays, no lost chars
3. **Browsers:** Selection method works, no autocomplete conflict
4. **Logs:** Correct method cho mỗi app type

### ❌ FAIL nếu:
1. VSCode vẫn lag như cũ
2. Terminal bị lost characters
3. Browser address bar bị conflict
4. Logs shows wrong method

---

## Quick Commands Reference

```bash
# Watch logs real-time
tail -f ~/Library/Logs/VietnameseIME/keyboard.log

# Filter by method
grep "instant:editor" ~/Library/Logs/VietnameseIME/keyboard.log
grep "slow:term" ~/Library/Logs/VietnameseIME/keyboard.log
grep "sel:browser" ~/Library/Logs/VietnameseIME/keyboard.log

# Count methods used
grep "method:" ~/Library/Logs/VietnameseIME/keyboard.log | sort | uniq -c

# Clear logs
rm ~/Library/Logs/VietnameseIME/keyboard.log

# Restart IME
killall VietnameseIMEFast
# Then manually relaunch from Xcode or Applications
```

---

## Test Results Template

```markdown
## Test Date: [DATE]
## Tester: [NAME]

### Test 1: VSCode
- Status: [ ] PASS / [ ] FAIL
- Notes: 
- Feel: [ ] Instant [ ] Slight lag [ ] Noticeable lag

### Test 2: Zed
- Status: [ ] PASS / [ ] FAIL
- Notes:
- Feel: [ ] Instant [ ] Slight lag [ ] Noticeable lag

### Test 3: Terminal
- Status: [ ] PASS / [ ] FAIL
- Notes:
- Lost chars: [ ] No [ ] Yes (describe)

### Test 4: Chrome
- Status: [ ] PASS / [ ] FAIL
- Notes:
- Autocomplete: [ ] OK [ ] Conflict

### Overall Result
- [ ] All tests passed - Ready for release
- [ ] Some tests failed - Need investigation
- [ ] Major issues - Need rework

### Recommendations
[Your feedback here]
```

---

## Next Steps After Testing

### If PASS:
1. ✅ Update IMPLEMENTATION_COMPLETE.md
2. ✅ Create release notes
3. ✅ Tag version
4. ✅ Beta test với users

### If FAIL:
1. ❌ Document specific failures
2. ❌ Check code vs reference implementation
3. ❌ Debug với Instruments
4. ❌ Retest after fixes

---

**Document version:** 1.0
**Last updated:** 2024
**Related docs:**
- `BACKSPACE_OPTIMIZATION_GUIDE.md` - Strategy
- `BACKSPACE_OPTIMIZATION_APPLIED.md` - Implementation details
- `TESTING_GUIDE.md` - Comprehensive testing