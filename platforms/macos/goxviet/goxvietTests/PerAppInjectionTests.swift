//
//  PerAppInjectionTests.swift
//  GoxVietTests
//
//  Unit tests for PerAppInjectionManager and related model types
//

import XCTest
@testable import goxviet

final class PerAppInjectionTests: XCTestCase {

    var manager: PerAppInjectionManager!

    override func setUp() {
        super.setUp()
        UserDefaults.standard.removeObject(forKey: "perAppInjectionProfiles")
        if PerAppInjectionManager.shared == nil {
            PerAppInjectionManager.shared = PerAppInjectionManager()
        }
        manager = PerAppInjectionManager.shared
        // Clear any persisted profiles for a clean slate
        for profile in manager.allProfiles {
            manager.removeProfile(for: profile.bundleId)
        }
    }

    func testDefaultProfileIsAuto() {
        let profile = manager.profile(for: "com.example.test.auto")
        XCTAssertEqual(profile.injectionMethod, .auto)
        XCTAssertEqual(profile.delayPreset, .none)
        XCTAssertTrue(profile.isEnabled)
    }

    func testSetAndGetProfile() {
        var p = PerAppInjectionProfile(bundleId: "com.test.app")
        p.injectionMethod = .slow
        p.delayPreset = .medium
        manager.setProfile(p)

        let retrieved = manager.profile(for: "com.test.app")
        XCTAssertEqual(retrieved.injectionMethod, .slow)
        XCTAssertEqual(retrieved.delayPreset, .medium)
    }

    func testDelayPresetValues() {
        XCTAssertEqual(DelayPreset.none.delays.0, 200)
        XCTAssertEqual(DelayPreset.none.delays.1, 800)
        XCTAssertEqual(DelayPreset.medium.delays.1, 8000)
        XCTAssertEqual(DelayPreset.veryHigh.delays.0, 12000)
    }

    func testDelayPresetClosest() {
        let preset = DelayPreset.closest(to: (3000, 8000, 3000))
        XCTAssertEqual(preset, .medium)
    }

    func testRemoveProfile() {
        manager.setProfile(PerAppInjectionProfile(bundleId: "com.remove.me"))
        manager.removeProfile(for: "com.remove.me")
        let all = manager.allProfiles
        XCTAssertFalse(all.contains(where: { $0.bundleId == "com.remove.me" }))
    }
}
