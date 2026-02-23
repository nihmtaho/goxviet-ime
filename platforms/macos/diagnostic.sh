#!/bin/bash

# GoxViet Diagnostic Script
# Checks all common issues with the app

echo "╔════════════════════════════════════════════════════════════╗"
echo "║           GoxViet Diagnostic Tool v1.0                     ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# 1. Check if process is running
echo "1️⃣  Checking if app is running..."
if ps aux | grep -i "goxviet\.app" | grep -v grep > /dev/null; then
    echo "✅ GoxViet is running"
    ps aux | grep -i "goxviet\.app" | grep -v grep | awk '{print "   PID:", $2, "| Memory:", $6/1024"MB"}'
else
    echo "❌ GoxViet is NOT running"
    echo "   → Launch the app from Xcode or Applications"
fi
echo ""

# 2. Check bundle ID
echo "2️⃣  Checking bundle identifier..."
DERIVED_DATA=$(find ~/Library/Developer/Xcode/DerivedData -name "goxviet-*" -type d 2>/dev/null | head -1)
if [ -n "$DERIVED_DATA" ]; then
    APP_PATH="$DERIVED_DATA/Build/Products/Debug/goxviet.app"
    if [ -d "$APP_PATH" ]; then
        BUNDLE_ID=$(defaults read "$APP_PATH/Contents/Info.plist" CFBundleIdentifier 2>/dev/null)
        if [ "$BUNDLE_ID" = "com.goxviet.ime" ]; then
            echo "✅ Bundle ID: $BUNDLE_ID (correct)"
        else
            echo "⚠️  Bundle ID: $BUNDLE_ID (expected: com.goxviet.ime)"
        fi
    else
        echo "⚠️  App not found at: $APP_PATH"
    fi
else
    echo "⚠️  Cannot find DerivedData folder"
    echo "   → Build the app in Xcode first"
fi
echo ""

# 3. Check settings
echo "3️⃣  Checking UserDefaults settings..."
IS_ENABLED=$(defaults read com.goxviet.ime isEnabled 2>/dev/null)
if [ "$IS_ENABLED" = "1" ]; then
    echo "✅ isEnabled = true (Vietnamese input ON)"
elif [ "$IS_ENABLED" = "0" ]; then
    echo "⚠️  isEnabled = false (Vietnamese input OFF)"
    echo "   → Toggle ON via menu bar or Ctrl+Space"
else
    echo "⚠️  isEnabled not set (first launch)"
    echo "   → Will default to true on first launch"
fi

INPUT_METHOD=$(defaults read com.goxviet.ime inputMethod 2>/dev/null)
if [ "$INPUT_METHOD" = "0" ]; then
    echo "   Input method: Telex"
elif [ "$INPUT_METHOD" = "1" ]; then
    echo "   Input method: VNI"
else
    echo "   Input method: Not set (will default to Telex)"
fi
echo ""

# 4. Check logging
echo "4️⃣  Checking logging status..."
LOGGING=$(defaults read com.goxviet.ime loggingEnabled 2>/dev/null)
if [ "$LOGGING" = "1" ]; then
    echo "✅ Logging: ENABLED"
    
    if [ -f ~/Library/Logs/GoxViet/keyboard.log ]; then
        LOG_SIZE=$(du -h ~/Library/Logs/GoxViet/keyboard.log | awk '{print $1}')
        LOG_LINES=$(wc -l < ~/Library/Logs/GoxViet/keyboard.log)
        echo "   Log file: $LOG_SIZE ($LOG_LINES lines)"
        echo ""
        echo "   Last 5 log entries:"
        tail -5 ~/Library/Logs/GoxViet/keyboard.log 2>/dev/null | sed 's/^/   /'
    else
        echo "   ⚠️  Log file doesn't exist yet"
    fi
else
    echo "⚠️  Logging: DISABLED"
    echo "   → Enable with: defaults write com.goxviet.ime loggingEnabled -bool true"
    echo "   → Then restart the app"
fi
echo ""

# 5. Check system logs for InputManager
echo "5️⃣  Checking system logs (last 2 minutes)..."
echo "   Looking for: InputManager, Permission, Toggle, Error..."
LOGS=$(log show --predicate 'process == "goxviet"' --last 2m --style compact 2>/dev/null | grep -i -E "(inputmanager|permission|accessibility|toggle|error|failed)" | tail -10)

if [ -n "$LOGS" ]; then
    echo "$LOGS" | sed 's/^/   /'
else
    echo "   ⚠️  No relevant system logs found"
    echo "   → App might not be running or not logging"
fi
echo ""

# 6. Check Accessibility permission
echo "6️⃣  Checking Accessibility permission..."
echo "   NOTE: This check is approximate - System Settings has final authority"

# Try to check if ANY app has accessibility
if osascript -e 'tell application "System Events" to get name of every process' >/dev/null 2>&1; then
    echo "✅ System has accessibility access enabled"
    echo "   → But specific app might still need permission"
    echo "   → Open System Settings → Privacy & Security → Accessibility"
    echo "   → Look for 'goxviet' and toggle it ON"
else
    echo "❌ No accessibility access detected"
    echo "   → Grant permission in System Settings"
fi
echo ""

# 7. Check for crashes
echo "7️⃣  Checking for recent crashes..."
CRASHES=$(log show --predicate 'eventMessage CONTAINS "goxviet" AND messageType == fault' --last 5m 2>/dev/null | head -5)
if [ -n "$CRASHES" ]; then
    echo "❌ Crashes detected:"
    echo "$CRASHES" | sed 's/^/   /'
else
    echo "✅ No crashes in last 5 minutes"
fi
echo ""

# 8. Summary and recommendations
echo "╔════════════════════════════════════════════════════════════╗"
echo "║                         SUMMARY                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Determine status
ISSUES=0
WARNINGS=0

if ! ps aux | grep -i "goxviet\.app" | grep -v grep > /dev/null; then
    ISSUES=$((ISSUES + 1))
    echo "❌ CRITICAL: App not running"
fi

if [ "$LOGGING" != "1" ]; then
    WARNINGS=$((WARNINGS + 1))
    echo "⚠️  WARNING: Logging disabled (recommended to enable)"
fi

if [ "$IS_ENABLED" = "0" ]; then
    WARNINGS=$((WARNINGS + 1))
    echo "⚠️  WARNING: Vietnamese input disabled"
fi

if [ $ISSUES -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo "✅ All checks passed!"
    echo ""
    echo "Next steps:"
    echo "1. Test keyboard shortcut: Press Ctrl+Space"
    echo "2. Test Vietnamese typing: Type 'viet' → should become 'việt'"
    echo "3. Test menu bar: Click the 🇻🇳 or ✏️ icon"
elif [ $ISSUES -gt 0 ]; then
    echo ""
    echo "🔧 FIXES NEEDED:"
    echo "1. Launch the app from Xcode or Applications"
    echo "2. Grant Accessibility permission in System Settings"
    echo "3. Check system logs for errors"
elif [ $WARNINGS -gt 0 ]; then
    echo ""
    echo "💡 RECOMMENDATIONS:"
    echo "1. Enable logging for debugging:"
    echo "   defaults write com.goxviet.ime loggingEnabled -bool true"
    echo "2. Restart the app"
    echo "3. Check ~/Library/Logs/GoxViet/keyboard.log"
fi

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║          For more help, see DEBUGGING_RUNTIME_ISSUES.md   ║"
echo "╚════════════════════════════════════════════════════════════╝"
