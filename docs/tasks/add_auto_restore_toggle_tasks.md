# Tasks cho Add Auto-Restore Toggle

- [ ] 1. Thêm FFI function `ime_instant_restore()` vào core/src/lib.rs
- [ ] 2. Thêm wrapper function `setInstantRestore()` vào RustBridge.swift
- [ ] 3. Thêm `@AppStorage("instantRestoreEnabled")` binding vào SettingsRootView
- [ ] 4. Thêm binding parameter vào GeneralSettingsView
- [ ] 5. Thêm Toggle UI trong Smart Features section
- [ ] 6. Thêm property `instantRestoreEnabled` vào AppState.swift
- [ ] 7. Sync config khi initialize trong RustBridge
- [ ] 8. Test tính năng bật/tắt auto-restore
- [ ] 9. Verify persistence sau khi restart app
