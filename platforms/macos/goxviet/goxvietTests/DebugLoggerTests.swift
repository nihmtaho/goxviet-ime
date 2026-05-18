//
//  DebugLoggerTests.swift
//  GoxVietTests
//
//  Unit tests for DebugLogger
//

import XCTest
@testable import goxviet

final class DebugLoggerTests: XCTestCase {

    var logger: DebugLogger!

    override func setUp() {
        super.setUp()
        if DebugLogger.shared == nil {
            DebugLogger.shared = DebugLogger()
        }
        logger = DebugLogger.shared
        logger.isEnabled = true
        logger.clear()
    }

    override func tearDown() {
        logger.clear()
        logger.isEnabled = false
        super.tearDown()
    }

    func testLogWritesToFile() {
        logger.log("test message unique 12345")
        let content = logger.readLog()
        XCTAssertTrue(content.contains("test message unique 12345"),
                      "Log should contain the message we wrote, got: \(content.prefix(200))")
    }

    func testClearEmptiesLog() {
        logger.log("will be cleared")
        logger.clear()
        let content = logger.readLog()
        XCTAssertTrue(content.isEmpty, "Log should be empty after clear()")
    }

    func testDisabledLogDoesNotWrite() {
        logger.isEnabled = false
        logger.log("should not appear")
        let content = logger.readLog()
        XCTAssertFalse(content.contains("should not appear"))
    }

    func testLogFileURL() {
        let url = logger.logFileURL
        XCTAssertTrue(url.path.contains("Logs/GoxViet"))
        XCTAssertEqual(url.lastPathComponent, "debug.log")
    }
}
