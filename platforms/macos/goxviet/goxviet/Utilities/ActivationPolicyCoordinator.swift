//
//  ActivationPolicyCoordinator.swift
//  GoxViet
//
//  Coalesces activation policy changes to avoid layout recursion warnings.
//

import AppKit

final class ActivationPolicyCoordinator {
    static let shared = ActivationPolicyCoordinator()

    private var pendingPolicy: NSApplication.ActivationPolicy?
    private var scheduled = false
    private let applyDelay: TimeInterval = 0.05

    private init() {}

    func request(_ policy: NSApplication.ActivationPolicy) {
        // Skip if already at target policy
        guard NSApp.activationPolicy() != policy else { return }
        
        // Skip if already requested with same value to reduce churn
        if pendingPolicy == policy { return }
        pendingPolicy = policy
        scheduleApply()
    }

    private func scheduleApply() {
        guard !scheduled else { return }
        scheduled = true
        DispatchQueue.main.asyncAfter(deadline: .now() + applyDelay) { [weak self] in
            self?.applyIfNeeded()
        }
    }

    private func applyIfNeeded() {
        scheduled = false
        guard let policy = pendingPolicy else { return }
        pendingPolicy = nil

        // Apply only if different to avoid redundant layout triggers
        if NSApp.activationPolicy() != policy {
            NSApp.setActivationPolicy(policy)
        }
    }
}
