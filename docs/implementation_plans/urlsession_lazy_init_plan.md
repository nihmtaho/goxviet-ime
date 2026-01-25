# Kế hoạch triển khai: Lazy URLSession Initialization

## Mô tả
Optimize UpdateManager để sử dụng lazy initialization cho URLSession objects (apiSession, downloadSession), chỉ tạo khi thực sự cần thiết. Hiện tại, apiSession được khởi tạo eager trong init(), gây tốn ~100-500KB memory ngay cả khi không check update.

## Problem Issue
### Current Issues
1. **Eager URLSession Allocation**: `apiSession` được tạo ngay trong `init()`, chiếm memory ngay cả khi:
   - Auto-update bị tắt (autoUpdateCheckEnabled = false)
   - Không có update nào được kiểm tra
   - App chỉ dùng để gõ tiếng Việt không cần update check
2. **Memory Footprint**: URLSession object + internal buffers có thể chiếm 100KB-500KB RAM
3. **No Cleanup**: `apiSession` không được release trong `stop()` method (hiện tại chỉ release downloadSession)

### Root Causes
- UpdateManager được init eager tại app startup (singleton pattern)
- URLSession được init eager trong UpdateManager's init
- Không có lazy initialization cho apiSession
- `stop()` method không clean up apiSession

## Các bước triển khai

### Step 1: Convert apiSession to lazy var
- Change `private let apiSession: URLSession` → `private var apiSession: URLSession?`
- Create helper method `makeAPISession() -> URLSession` with lazy check
- Add logging để track khi nào session được tạo

### Step 2: Update all apiSession usages
- Find all places using `apiSession` → call `makeAPISession()` instead
- Locations:
  - `checkForUpdates()` method
  - `urlSession(:downloadTask:didFinishDownloadingTo:)` delegate
  - Any other API calls

### Step 3: Add cleanup in stop()
- Call `apiSession?.invalidateAndCancel()` in `stop()`
- Set `apiSession = nil` to release memory
- Add log message: "API URLSession released"

### Step 4: Test validation
- Verify UpdateManager starts without creating URLSession immediately
- Verify URLSession created only when `checkForUpdates()` called
- Verify URLSession released when `stop()` called
- Verify no crashes/leaks after repeated start/stop cycles

## Proposed Changes

```swift
// OLD:
private let apiSession: URLSession

private override init() {
    let apiConfig = URLSessionConfiguration.default
    apiConfig.waitsForConnectivity = true
    apiConfig.timeoutIntervalForRequest = 12
    apiConfig.timeoutIntervalForResource = 12
    apiSession = URLSession(configuration: apiConfig)
    super.init()
}

func stop() {
    // ... no apiSession cleanup
    downloadSession?.invalidateAndCancel()
    downloadSession = nil
}

// NEW:
private var apiSession: URLSession?

private override init() {
    super.init()
}

private func makeAPISession() -> URLSession {
    if let existing = apiSession { return existing }
    
    let apiConfig = URLSessionConfiguration.default
    apiConfig.waitsForConnectivity = true
    apiConfig.timeoutIntervalForRequest = 12
    apiConfig.timeoutIntervalForResource = 12
    
    let session = URLSession(configuration: apiConfig)
    apiSession = session
    Log.info("API URLSession created (lazy)")
    return session
}

func stop() {
    // ...
    downloadSession?.invalidateAndCancel()
    downloadSession = nil
    apiSession?.invalidateAndCancel()
    apiSession = nil
    Log.info("UpdateManager stopped, sessions released")
}
```

## Thời gian dự kiến
- Step 1: 10 phút (change declaration + add helper)
- Step 2: 15 phút (update usages + search all references)
- Step 3: 5 phút (add cleanup)
- Step 4: 15 phút (testing)
- **Total: ~45 phút**

## Tài nguyên cần thiết
- Access to UpdateManager.swift
- Ability to build and test macOS app
- Instruments (Allocations) để verify memory reduction

## Implementation Order
1. Step 1 (convert to lazy)
2. Step 2 (update usages)
3. Step 3 (cleanup)
4. Step 4 (testing)

## Expected Memory Savings
- **Immediate savings**: ~100-500KB khi UpdateManager không chạy
- **Long-term**: ~200-700KB nếu cả apiSession và downloadSession đều không được dùng
- **Target contribution**: ~0.3-0.5MB toward <10MB goal (current 28.4MB → ~27.9-28.1MB)
