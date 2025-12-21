# BACKSPACE OPTIMIZATION GUIDE

## Mục tiêu
Tối ưu hóa hiệu suất xử lý backspace khi gõ Telex trên các editor hiện đại (VSCode, Zed, Sublime Text) để đạt độ trễ < 16ms.

## Vấn đề hiện tại

### 1. Delay không cần thiết
Code hiện tại sử dụng delays giữa các backspace events ngay cả với modern editors:
- VSCode, Zed, Sublime Text có text buffer nhanh
- Không cần delay giữa các keystrokes
- Delay gây lag nhìn thấy rõ khi gõ nhanh

### 2. Batch Backspace chưa tối ưu
```swift
// Current implementation
for _ in 0..<count {
    postKey(KeyCode.backspace, source: src, proxy: proxy)
    usleep(delays.0)  // ❌ Delay không cần thiết cho modern editors
}
```

## Giải pháp tối ưu (Based on reference implementation)

### 1. Zero-Delay Batch Backspace
**Nguyên tắc:** Modern editors có fast text buffers → Gửi tất cả backspace events liên tiếp không cần delay.

```swift
private func postBackspaces(_ count: Int, source: CGEventSource, proxy: CGEventTapProxy) {
    guard count > 0 else { return }
    
    // Gửi liên tiếp tất cả backspace events (no delays)
    for _ in 0..<count {
        guard let dn = CGEvent(keyboardEventSource: source, virtualKey: KeyCode.backspace, keyDown: true),
              let up = CGEvent(keyboardEventSource: source, virtualKey: KeyCode.backspace, keyDown: false) 
        else { continue }
        
        dn.setIntegerValueField(.eventSourceUserData, value: Int64(kEventMarker))
        up.setIntegerValueField(.eventSourceUserData, value: Int64(kEventMarker))
        
        // Post ngay lập tức, không delay
        dn.tapPostEvent(proxy)
        up.tapPostEvent(proxy)
    }
}
```

**Lợi ích:**
- Giảm event loop overhead
- Zero latency giữa các backspace events
- Editors xử lý được vì có buffer nhanh

### 2. Instant Method cho Modern Editors

```swift
private func injectViaInstant(bs: Int, text: String, proxy: CGEventTapProxy) {
    guard let src = CGEventSource(stateID: .privateState) else { return }
    
    // 1. Batch backspace - no delays
    postBackspaces(bs, source: src, proxy: proxy)
    
    // 2. Type replacement text immediately - zero delay
    postText(text, source: src, delay: 0, proxy: proxy)
    
    Log.send("instant", bs, text)
}
```

**Đặc điểm:**
- `delay: 0` cho tất cả operations
- Không có `usleep()` calls
- Maximum throughput

### 3. App Detection Logic

```swift
func detectMethod() -> (InjectionMethod, (UInt32, UInt32, UInt32)) {
    // ... get bundleId ...
    
    // Modern editors - instant method with ZERO delays
    let modernEditors = [
        "com.microsoft.VSCode",
        "com.microsoft.VSCodeInsiders",
        "com.vscodium",
        "dev.zed.Zed",
        "dev.zed.preview",
        "com.sublimetext.4",
        "com.sublimetext.3",
        "com.panic.Nova",
        "com.github.atom",
        "com.coteditor.CotEditor"
    ]
    
    if modernEditors.contains(bundleId) {
        Log.method("instant:editor")
        return (.instant, (0, 0, 0))  // ✅ All zeros!
    }
    
    // Terminals - conservative delays for stability
    let terminals = [
        "com.apple.Terminal",
        "com.googlecode.iterm2",
        "io.alacritty",
        "net.kovidgoyal.kitty"
    ]
    
    if terminals.contains(bundleId) {
        Log.method("slow:term")
        return (.slow, (3000, 8000, 3000))  // ✅ Delays needed
    }
    
    // Default
    return (.fast, (1000, 3000, 1500))
}
```

## Performance Metrics

### Before Optimization
```
VSCode:
- Single keystroke: ~25-35ms (có delay giữa backspaces)
- Backspace + text: ~40-50ms
- Người dùng cảm nhận: Lag nhẹ khi gõ nhanh
```

### After Optimization (Target)
```
VSCode:
- Single keystroke: < 16ms (60fps threshold)
- Backspace + text: < 20ms
- Người dùng cảm nhận: Instant, như gõ native
```

## Implementation Checklist

### Phase 1: Core Changes ✅
- [x] Implement zero-delay `postBackspaces()` method
- [x] Update `injectViaInstant()` to use batch backspace
- [x] Ensure no `usleep()` calls in instant path

### Phase 2: App Detection ✅
- [x] Add comprehensive modern editors list
- [x] Separate terminals (need delays) from editors
- [x] Add logging for debugging

### Phase 3: Testing
- [ ] Test VSCode: gõ "hoaf" → "hòa" (backspace + tone)
- [ ] Test Zed: gõ "truong" → "trường" (multiple backspaces)
- [ ] Test Sublime Text: gõ nhanh nhiều từ liên tiếp
- [ ] Verify terminals still work (iTerm2, Terminal.app)

### Phase 4: Verification
- [ ] Measure latency với `test-performance.sh`
- [ ] Confirm < 16ms trên VSCode/Zed
- [ ] Kiểm tra không có lost characters
- [ ] User testing: gõ thực tế 5-10 phút

## Key Differences vs Current Implementation

| Aspect | Current | Optimized (Reference) |
|--------|---------|----------------------|
| Backspace delay | `delays.0` (1000-3000µs) | `0µs` cho modern editors |
| Text injection delay | `delays.2` (1500-3000µs) | `0µs` cho modern editors |
| Method detection | Generic "fast/slow" | App-specific instant/slow |
| Batch backspace | ✅ Có nhưng vẫn có delay | ✅ True batch, zero delay |

## Code Locations

### Files cần update:
1. `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/RustBridge.swift`
   - Line 101-109: `injectViaInstant()` 
   - Line 113-127: `postBackspaces()`
   - Line 537-670: `detectMethod()`

### Test files:
1. `test-performance.sh` - Measure actual latency
2. Manual testing trong VSCode/Zed

## Notes

### ⚠️ Quan trọng
- **Terminals VẪN CẦN delays:** Terminal emulators render slower, batch events gây lost characters
- **Browsers cần Selection method:** Address bars có autocomplete conflict với backspace method
- **Microsoft Office cần Slow method:** Office apps có suggestion features phức tạp

### 🎯 Target Apps cho Instant Method
Chỉ apply instant method cho apps có:
1. Fast native text buffer
2. Direct text manipulation API
3. No autocomplete interference
4. Known to handle rapid events

### 📊 Monitoring
Sử dụng `Log.send()` để track:
- Method được chọn cho mỗi app
- Số lượng backspaces
- Text replacement
- Latency measurements

## References

- Reference implementation: `example-project/gonhanh.org-main/platforms/macos/RustBridge.swift`
- Performance docs: `docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- Testing guide: `docs/TESTING_GUIDE.md`

---

**Status:** Ready for implementation
**Priority:** HIGH - Ảnh hưởng trực tiếp đến user experience
**Estimated Impact:** 40-60% latency reduction trên modern editors