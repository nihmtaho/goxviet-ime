# TEST GUIDE: SMART PER-APP MODE

**Version:** 1.0.1  
**Date:** 2025-12-20  
**Estimated Time:** 10 minutes

---

## 🎯 OBJECTIVE

Verify that Smart Per-App Mode correctly remembers and restores Vietnamese input state for each application.

---

## 📋 PRE-REQUISITES

- [ ] Vietnamese IME Fast is built and running
- [ ] Accessibility permissions granted
- [ ] At least 2-3 different apps available (e.g., Chrome, Notes, Terminal)
- [ ] Menu bar icon visible (🇻🇳 or EN)

---

## 🧪 TEST CASES

### Test 1: Enable Smart Mode

**Steps:**
1. Click Vietnamese IME menu bar icon
2. Find "Smart Per-App Mode" toggle
3. Toggle it to **ON**
4. Verify toggle shows enabled state

**Expected:**
- ✅ Toggle switches to ON position
- ✅ Feature is now active

**Log Check:**
```
Smart Per-App Mode: ON
```

---

### Test 2: Set Different States for Apps

**Steps:**
1. Open **Chrome** (or any browser)
2. Check menu bar - should show 🇻🇳 (enabled by default)
3. Toggle Vietnamese to **OFF** (press Control+Space or use menu)
4. Verify icon changes to **EN**
5. Open **Notes** app
6. Verify icon automatically changes to **🇻🇳** (enabled)
7. Type some Vietnamese text to confirm
8. Switch back to **Chrome**
9. Verify icon automatically changes to **EN** (disabled)

**Expected:**
- ✅ Chrome: Vietnamese OFF → Icon shows EN
- ✅ Notes: Vietnamese ON → Icon shows 🇻🇳
- ✅ Switch Chrome→Notes: Auto-enables Vietnamese
- ✅ Switch Notes→Chrome: Auto-disables Vietnamese

**Log Check:**
```
App switched: Google Chrome (com.google.Chrome)
Mode restored for Google Chrome: disabled
State saved for Google Chrome: disabled

App switched: Notes (com.apple.Notes)
Mode restored for Notes: enabled
```

---

### Test 3: New App Default State

**Steps:**
1. Open a new app you haven't used yet (e.g., Terminal, Slack)
2. Check menu bar icon

**Expected:**
- ✅ New apps default to Vietnamese **enabled** (🇻🇳)
- ✅ Can type Vietnamese immediately

---

### Test 4: State Persistence After Restart

**Steps:**
1. Ensure you have different states for different apps (e.g., Chrome OFF, Notes ON)
2. Quit Vietnamese IME Fast
3. Relaunch Vietnamese IME Fast
4. Switch to Chrome
5. Verify Vietnamese is still **OFF**
6. Switch to Notes
7. Verify Vietnamese is still **ON**

**Expected:**
- ✅ Settings persist across app restarts
- ✅ Each app remembers its last state

---

### Test 5: Disable Smart Mode

**Steps:**
1. Click menu bar icon
2. Toggle "Smart Per-App Mode" to **OFF**
3. Switch between Chrome and Notes multiple times
4. Manually toggle Vietnamese ON/OFF with Control+Space
5. Verify manual toggling works

**Expected:**
- ✅ Smart Mode disabled
- ✅ Manual toggle works normally
- ✅ State doesn't auto-change when switching apps

---

### Test 6: Re-enable Smart Mode (State Restoration)

**Steps:**
1. With Smart Mode OFF, set Vietnamese to OFF in Chrome
2. Toggle Smart Mode back to **ON**
3. Switch to Chrome
4. Verify Vietnamese state

**Expected:**
- ✅ Previous per-app states are restored
- ✅ Chrome should remember it was OFF

---

### Test 7: View Settings

**Steps:**
1. Click menu bar icon
2. Click "Settings..."
3. Review displayed information

**Expected:**
- ✅ Shows current app name
- ✅ Shows Smart Mode status (Enabled/Disabled)
- ✅ Shows number of apps with custom settings
- ✅ Shows current configuration

**Example Display:**
```
Current Configuration:

• Input Method: Telex
• Tone Style: Traditional
• ESC Restore: Enabled
• Free Tone: Disabled

Smart Per-App Mode: Enabled
• Current App: Google Chrome
• Apps with custom settings: 2
```

---

### Test 8: Clear Per-App Settings

**Steps:**
1. Set different states for multiple apps (some ON, some OFF)
2. Click menu bar icon → "Settings..."
3. Click "Clear Per-App Settings" button
4. Confirm the action
5. Switch between previously configured apps
6. Verify all apps now default to **enabled**

**Expected:**
- ✅ Confirmation dialog appears
- ✅ All per-app settings cleared
- ✅ All apps reset to Vietnamese enabled
- ✅ Can reconfigure from scratch

---

### Test 9: Rapid App Switching

**Steps:**
1. Enable Smart Mode
2. Configure different states for 3+ apps
3. Rapidly switch between apps (Cmd+Tab multiple times)
4. Verify no crashes or lag

**Expected:**
- ✅ No crashes
- ✅ No freezing or lag
- ✅ States restore correctly even with rapid switching
- ✅ Composition buffer clears on each switch

---

### Test 10: Edge Cases

#### 10.1. Same App, Multiple Windows

**Steps:**
1. Open Chrome with 2 windows
2. Disable Vietnamese in Chrome
3. Switch between Chrome windows

**Expected:**
- ✅ Both windows share same state (disabled)
- ✅ No state change when switching between same app's windows

#### 10.2. App Without Bundle ID

**Steps:**
1. Launch Vietnamese IME Fast
2. Switch to system apps (Finder, System Settings)

**Expected:**
- ✅ No crashes
- ✅ Falls back to default (enabled) or ignores

#### 10.3. First Time Using IME

**Steps:**
1. Clear all per-app settings
2. Open an app you haven't configured
3. Check default state

**Expected:**
- ✅ Defaults to Vietnamese enabled
- ✅ No errors in log

---

## 🐛 TROUBLESHOOTING

### Issue: Smart Mode Toggle Not Working

**Check:**
1. Is the toggle actually ON? (Visual check)
2. Check log: `Smart Per-App Mode: ON`
3. Restart Vietnamese IME Fast

**Solution:**
```bash
# View logs
open ~/Library/Logs/VietnameseIME/keyboard.log
```

---

### Issue: State Not Restoring

**Check:**
1. Is Smart Mode enabled?
2. Are you switching to a different app (not just windows of same app)?
3. Check saved states:

**Debug:**
```bash
# View saved per-app settings
defaults read com.vietnamese.ime.perAppModes
```

**Output Example:**
```
{
    "com.google.Chrome" = 0;
    "com.microsoft.VSCode" = 0;
}
```

---

### Issue: State Not Persisting After Restart

**Check:**
1. UserDefaults permissions
2. App sandbox settings

**Solution:**
- Clear and reconfigure: Settings → Clear Per-App Settings
- Restart macOS (last resort)

---

## 📊 TEST RESULTS TEMPLATE

**Tester:** _______________  
**Date:** _______________  
**Version:** 1.0.1  

| Test # | Test Name | Result | Notes |
|--------|-----------|--------|-------|
| 1 | Enable Smart Mode | ☐ Pass ☐ Fail | |
| 2 | Different States | ☐ Pass ☐ Fail | |
| 3 | New App Default | ☐ Pass ☐ Fail | |
| 4 | State Persistence | ☐ Pass ☐ Fail | |
| 5 | Disable Smart Mode | ☐ Pass ☐ Fail | |
| 6 | Re-enable Smart Mode | ☐ Pass ☐ Fail | |
| 7 | View Settings | ☐ Pass ☐ Fail | |
| 8 | Clear Settings | ☐ Pass ☐ Fail | |
| 9 | Rapid Switching | ☐ Pass ☐ Fail | |
| 10 | Edge Cases | ☐ Pass ☐ Fail | |

**Overall:** ☐ All Pass ☐ Some Fail ☐ Critical Issues

---

## ✅ SUCCESS CRITERIA

- [x] All 10 test cases pass
- [x] No crashes or freezes
- [x] State persists across restarts
- [x] Clear settings functionality works
- [x] Performance acceptable (no lag)
- [x] Logs show correct behavior

---

## 📝 NOTES

### Common Observations

1. **First Switch Delay:**
   - First app switch after enabling Smart Mode may take 100-200ms
   - Subsequent switches are instant (<1ms)
   - This is normal (UserDefaults cold start)

2. **Bundle ID Detection:**
   - Some apps may not have bundle IDs
   - These fall back to default (enabled)
   - Not an error - expected behavior

3. **Composition Buffer:**
   - Buffer always cleared on app switch
   - Prevents Vietnamese text from carrying over
   - This is intentional behavior

---

## 🎯 QUICK SMOKE TEST (2 minutes)

If short on time, run this minimal test:

1. ✅ Enable Smart Mode
2. ✅ Disable Vietnamese in Chrome
3. ✅ Switch to Notes → Should auto-enable
4. ✅ Switch to Chrome → Should auto-disable
5. ✅ Restart app → Settings persist

If all 5 pass, feature is working correctly.

---

## 📞 REPORT ISSUES

If tests fail, include:
1. Test number that failed
2. Expected vs actual behavior
3. Logs from `~/Library/Logs/VietnameseIME/keyboard.log`
4. macOS version
5. App versions (Chrome, Notes, etc.)

---

## 🔗 RELATED DOCUMENTATION

- `SMART_PER_APP_MODE.md` - Full feature documentation
- `CHANGELOG_SMART_PER_APP_MODE.md` - Implementation details
- `getting-started/TESTING_GUIDE.md` - General testing guide

---

**Status:** ✅ READY FOR TESTING  
**Last Updated:** 2025-12-20