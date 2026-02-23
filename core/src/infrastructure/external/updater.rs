//! Updater Adapter
//!
//! A small wrapper around crate::updater::Version for external integrations.

use crate::updater::Version;

/// Adapter for version comparison operations
#[derive(Debug, Clone, Copy, Default)]
pub struct UpdaterAdapter;

impl UpdaterAdapter {
    /// Create a new UpdaterAdapter
    pub fn new() -> Self {
        UpdaterAdapter
    }

    /// Parse a version string
    ///
    /// Returns Some(Version) if parsing succeeds, None otherwise.
    pub fn parse_version(&self, s: &str) -> Option<Version> {
        Version::parse(s)
    }

    /// Compare two version strings
    ///
    /// Returns:
    /// - Some(-1) if v1 < v2
    /// - Some(0) if v1 == v2
    /// - Some(1) if v1 > v2
    /// - None if either version string is invalid
    pub fn compare_versions(&self, v1: &str, v2: &str) -> Option<i32> {
        let ver1 = Version::parse(v1)?;
        let ver2 = Version::parse(v2)?;
        Some(ver1.compare(&ver2))
    }

    /// Check if an update is available
    ///
    /// Returns:
    /// - Some(true) if latest > current (update available)
    /// - Some(false) if latest <= current (no update)
    /// - None if either version string is invalid
    pub fn has_update(&self, current: &str, latest: &str) -> Option<bool> {
        let curr_ver = Version::parse(current)?;
        let latest_ver = Version::parse(latest)?;
        Some(curr_ver.has_update(&latest_ver))
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let adapter = UpdaterAdapter::new();
        let version = adapter.parse_version("1.2.3");
        assert!(version.is_some());
        let ver = version.unwrap();
        assert_eq!(ver.major, 1);
        assert_eq!(ver.minor, 2);
        assert_eq!(ver.patch, 3);
    }

    #[test]
    fn test_parse_version_invalid() {
        let adapter = UpdaterAdapter::new();
        assert!(adapter.parse_version("invalid").is_none());
    }

    #[test]
    fn test_compare_versions() {
        let adapter = UpdaterAdapter::new();
        assert_eq!(adapter.compare_versions("1.0.0", "1.0.1"), Some(-1));
        assert_eq!(adapter.compare_versions("1.0.1", "1.0.0"), Some(1));
        assert_eq!(adapter.compare_versions("1.0.0", "1.0.0"), Some(0));
    }

    #[test]
    fn test_compare_versions_invalid() {
        let adapter = UpdaterAdapter::new();
        assert!(adapter.compare_versions("invalid", "1.0.0").is_none());
        assert!(adapter.compare_versions("1.0.0", "invalid").is_none());
    }

    #[test]
    fn test_has_update() {
        let adapter = UpdaterAdapter::new();
        assert_eq!(adapter.has_update("1.0.9", "1.0.10"), Some(true));
        assert_eq!(adapter.has_update("1.0.10", "1.0.9"), Some(false));
        assert_eq!(adapter.has_update("1.0.0", "1.0.0"), Some(false));
    }

    #[test]
    fn test_has_update_invalid() {
        let adapter = UpdaterAdapter::new();
        assert!(adapter.has_update("invalid", "1.0.0").is_none());
        assert!(adapter.has_update("1.0.0", "invalid").is_none());
    }

    #[test]
    fn test_default() {
        let _adapter = UpdaterAdapter::default();
    }

    #[test]
    fn test_clone_copy() {
        let adapter = UpdaterAdapter::new();
        let _cloned = adapter.clone();
        let _copied = adapter;
    }
}
