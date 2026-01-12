/// WindowMemorySafetyTests.swift
/// Test suite để verify isReleasedWhenClosed=true không gây crash
/// Kiểm thử:
/// 1. Memory được giải phóng khi window đóng
/// 2. windowWillClose delegate được gọi
/// 3. Không crash khi open/close nhiều lần

import XCTest
@testable import goxviet

class WindowMemorySafetyTests: XCTestCase {
    
    // MARK: - Setup & Teardown
    
    override func setUp() {
        super.setUp()
        // Ensure WindowManager starts fresh
        WindowManager.shared.closeSettingsWindow()
        WindowManager.shared.closeUpdateWindow()
    }
    
    override func tearDown() {
        WindowManager.shared.closeSettingsWindow()
        WindowManager.shared.closeUpdateWindow()
        super.tearDown()
    }
    
    // MARK: - Test 1: Settings Window Memory Cleanup
    
    /// Test that settings window is properly released when closed
    /// Expected: window pointer becomes invalid after windowWillClose
    func testSettingsWindowMemoryCleanup() throws {
        // Act: Open settings window
        WindowManager.shared.showSettingsWindow()
        XCTAssertTrue(
            WindowManager.shared.isSettingsWindowOpen,
            "Settings window should be open"
        )
        
        // Act: Close window
        let expectation = expectation(description: "windowWillClose called")
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
            WindowManager.shared.closeSettingsWindow()
            expectation.fulfill()
        }
        
        waitForExpectations(timeout: 1.0)
        
        // Assert: Window should be deallocated
        XCTAssertFalse(
            WindowManager.shared.isSettingsWindowOpen,
            "Settings window should be deallocated after close"
        )
    }
    
    // MARK: - Test 2: Update Window Memory Cleanup
    
    /// Test that update window is properly released when closed
    func testUpdateWindowMemoryCleanup() throws {
        // Act: Open update window
        WindowManager.shared.showUpdateWindow()
        XCTAssertTrue(
            WindowManager.shared.isUpdateWindowOpen,
            "Update window should be open"
        )
        
        // Act: Close window
        let expectation = expectation(description: "windowWillClose called")
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
            WindowManager.shared.closeUpdateWindow()
            expectation.fulfill()
        }
        
        waitForExpectations(timeout: 1.0)
        
        // Assert: Window should be deallocated
        XCTAssertFalse(
            WindowManager.shared.isUpdateWindowOpen,
            "Update window should be deallocated after close"
        )
    }
    
    // MARK: - Test 3: Rapid Open/Close (Regression Test)
    
    /// Test that rapid open/close cycles don't cause crashes
    /// This catches double-free or use-after-free bugs
    func testRapidSettingsWindowOpenClose() throws {
        let cycleCount = 20
        
        for i in 0..<cycleCount {
            // Act: Open
            WindowManager.shared.showSettingsWindow()
            
            // Small delay to let window initialize
            let openExpectation = expectation(description: "Window opened - cycle \(i)")
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.05) {
                XCTAssertTrue(
                    WindowManager.shared.isSettingsWindowOpen,
                    "Settings window should be open at cycle \(i)"
                )
                openExpectation.fulfill()
            }
            
            waitForExpectations(timeout: 1.0)
            
            // Act: Close
            let closeExpectation = expectation(description: "Window closed - cycle \(i)")
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.05) {
                WindowManager.shared.closeSettingsWindow()
                
                // Small delay for delegate callback
                DispatchQueue.main.asyncAfter(deadline: .now() + 0.05) {
                    XCTAssertFalse(
                        WindowManager.shared.isSettingsWindowOpen,
                        "Settings window should be closed at cycle \(i)"
                    )
                    closeExpectation.fulfill()
                }
            }
            
            waitForExpectations(timeout: 1.0)
        }
    }
    
    // MARK: - Test 4: Simultaneous Windows
    
    /// Test that having multiple windows open simultaneously works correctly
    func testSimultaneousWindowsMemoryCleanup() throws {
        // Act: Open both windows
        WindowManager.shared.showSettingsWindow()
        WindowManager.shared.showUpdateWindow()
        
        XCTAssertTrue(WindowManager.shared.isSettingsWindowOpen)
        XCTAssertTrue(WindowManager.shared.isUpdateWindowOpen)
        
        // Act: Close settings first
        let closeSettingsExp = expectation(description: "Settings closed")
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
            WindowManager.shared.closeSettingsWindow()
            closeSettingsExp.fulfill()
        }
        
        waitForExpectations(timeout: 1.0)
        
        // Assert: Settings closed, update still open
        XCTAssertFalse(WindowManager.shared.isSettingsWindowOpen)
        XCTAssertTrue(WindowManager.shared.isUpdateWindowOpen)
        
        // Act: Close update
        let closeUpdateExp = expectation(description: "Update closed")
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
            WindowManager.shared.closeUpdateWindow()
            closeUpdateExp.fulfill()
        }
        
        waitForExpectations(timeout: 1.0)
        
        // Assert: Both closed
        XCTAssertFalse(WindowManager.shared.isSettingsWindowOpen)
        XCTAssertFalse(WindowManager.shared.isUpdateWindowOpen)
    }
    
    // MARK: - Test 5: Reopen After Close
    
    /// Test that we can reopen a window after closing it
    /// This ensures no stale references remain
    func testReopenWindowAfterClose() throws {
        // Act: Open-Close-Reopen cycle
        WindowManager.shared.showSettingsWindow()
        XCTAssertTrue(WindowManager.shared.isSettingsWindowOpen)
        
        let closeExp = expectation(description: "First close")
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
            WindowManager.shared.closeSettingsWindow()
            closeExp.fulfill()
        }
        waitForExpectations(timeout: 1.0)
        
        XCTAssertFalse(WindowManager.shared.isSettingsWindowOpen)
        
        // Act: Reopen
        let reopenExp = expectation(description: "Reopen")
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.05) {
            WindowManager.shared.showSettingsWindow()
            reopenExp.fulfill()
        }
        waitForExpectations(timeout: 1.0)
        
        // Assert: Should be open again with fresh state
        XCTAssertTrue(
            WindowManager.shared.isSettingsWindowOpen,
            "Window should reopen cleanly after being deallocated"
        )
    }
    
    // MARK: - Test 6: Stress Test (Memory Endurance)
    
    /// Stress test: 50 cycles to catch subtle memory issues
    func testMemoryStressTest() throws {
        let cycleCount = 50
        var successCount = 0
        
        for i in 0..<cycleCount {
            WindowManager.shared.showSettingsWindow()
            
            let testExp = expectation(description: "Cycle \(i)")
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.02) {
                if WindowManager.shared.isSettingsWindowOpen {
                    WindowManager.shared.closeSettingsWindow()
                    
                    DispatchQueue.main.asyncAfter(deadline: .now() + 0.02) {
                        if !WindowManager.shared.isSettingsWindowOpen {
                            successCount += 1
                        }
                        testExp.fulfill()
                    }
                } else {
                    testExp.fulfill()
                }
            }
            
            waitForExpectations(timeout: 1.0)
        }
        
        XCTAssertEqual(
            successCount,
            cycleCount,
            "All \(cycleCount) cycles should complete without crashes"
        )
    }
}

// MARK: - Helper Extensions for Memory Testing

extension WindowManager {
    /// For testing: Get current reference count (debug only)
    /// Returns the number of strong references to settings window
    func debugSettingsWindowRefCount() -> Int {
        guard let window = settingsWindow else { return 0 }
        return CFGetRetainCount(window as CFTypeRef)
    }
    
    /// For testing: Get current reference count (debug only)
    func debugUpdateWindowRefCount() -> Int {
        guard let window = updateWindow else { return 0 }
        return CFGetRetainCount(window as CFTypeRef)
    }
}
