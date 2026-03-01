# FFI API v2 (`presentation/ffi/`)

`lib.rs` re-exports the public C-compatible API from `presentation/ffi/api.rs`. All functions use `catch_unwind` and return explicit status codes — **no panics cross the FFI boundary**.

> **v1 API removed in v3.0.0.** All `ime_init`, `ime_key`, `ime_key_ext`, `ime_free`, `ime_method`, `ime_clear`, etc. have been removed. See the migration note at the bottom.

---

## C Usage Pattern

```c
// 1. Create engine (NULL = default config)
void *engine = ime_create_engine_v2(NULL);

// 2. Process keystrokes
FfiProcessResult_v2 result = {0};
int status = ime_process_key_v2(engine, 'a', &result);

if (status == 0 /* FFI_STATUS_OK */ && result.consumed) {
    apply_backspaces(result.backspace_count);
    insert_text(result.text);        // UTF-8, null-terminated
}
ime_free_string_v2(result.text);     // ALWAYS free, even if NULL

// 3. Destroy
ime_destroy_engine_v2(engine);
```

---

## Lifecycle Functions

### `ime_create_engine_v2`
```c
void *ime_create_engine_v2(const FfiConfig_v2 *config);
```
- Creates and returns an opaque engine pointer.
- `config = NULL` uses `FfiConfig_v2` defaults (Telex, Modern tone, Smart Mode on).
- **Caller must destroy** with `ime_destroy_engine_v2`.

### `ime_destroy_engine_v2`
```c
void ime_destroy_engine_v2(void *engine_ptr);
```
- Frees engine memory. Safe to call with NULL.

---

## Key Processing

### `ime_process_key_v2`
```c
int ime_process_key_v2(void *engine_ptr, char key, FfiProcessResult_v2 *out);
```
- Processes a single ASCII character keystroke.
- Writes result via **out parameter** (avoids Swift ABI struct-return issues).
- Returns `FfiStatusCode` as `int`.

**`FfiProcessResult_v2` fields:**

| Field | Type | Description |
|---|---|---|
| `text` | `char *` | UTF-8 replacement text (may be NULL if no output). **Caller must free** with `ime_free_string_v2`. |
| `backspace_count` | `uint8_t` | How many chars to delete before inserting `text`. |
| `consumed` | `bool` | `true` = IME handled the key; `false` = pass through to OS. |

---

## Configuration

### `ime_get_config_v2`
```c
int ime_get_config_v2(void *engine_ptr, FfiConfig_v2 *out);
```

### `ime_set_config_v2`
```c
int ime_set_config_v2(void *engine_ptr, const FfiConfig_v2 *config);
```

**`FfiConfig_v2` fields:**

| Field | Type | Default | Description |
|---|---|---|---|
| `input_method` | `FfiInputMethod` | `Telex (0)` | `Telex=0`, `Vni=1` |
| `tone_style` | `FfiToneStyle` | `New (1)` | `Old=0` (hòa), `New=1` (hoà) |
| `smart_mode` | `bool` | `true` | Enable per-app Smart Mode |
| `instant_restore_enabled` | `bool` | `true` | Auto-restore English words |
| `esc_restore_enabled` | `bool` | `false` | ESC restores raw ASCII |
| `enable_shortcuts` | `bool` | `true` | Text expansion shortcuts |

---

## Shortcut Management

```c
int ime_add_shortcut_v2(void *engine, const char *trigger, const char *replacement);
int ime_remove_shortcut_v2(void *engine, const char *trigger);
int ime_clear_shortcuts_v2(void *engine);
int ime_shortcuts_count_v2(void *engine);
int ime_set_shortcuts_enabled_v2(void *engine, bool enabled);
```

---

## Memory Management

### `ime_free_string_v2`
```c
void ime_free_string_v2(char *s);
```
- Frees any `char *` returned inside `FfiProcessResult_v2.text`.
- **Must be called after every `ime_process_key_v2` call**, even if `text` is NULL (the function is NULL-safe).

---

## Status Codes (`FfiStatusCode`)

| Code | Value | Meaning |
|---|---|---|
| `Success` | 0 | OK |
| `ErrorNullEngine` | -1 | Engine pointer is NULL |
| `ErrorNullOutput` | -2 | Out-parameter pointer is NULL |
| `ErrorNullConfig` | -3 | Config pointer is NULL |
| `ErrorInvalidKey` | -4 | Invalid key character |
| `ErrorProcessingFailed` | -10 | Internal processing error |
| `ErrorAlreadyExists` | -30 | Shortcut already exists |
| `ErrorNotFound` | -31 | Shortcut not found |
| `ErrorPanic` | -99 | Rust panic caught at FFI boundary |

---

## Version Info

```c
int ime_get_version_v2(void *engine_ptr, FfiVersionInfo *out);
// out: { major, minor, patch, api_version=2 }
```

---

## v1 → v2 Migration Reference

| v1 (removed) | v2 equivalent |
|---|---|
| `ime_init()` | `ime_create_engine_v2(NULL)` |
| `ime_key(key, caps, ctrl)` | `ime_process_key_v2(engine, key, &result)` |
| `ime_key_ext(key, caps, ctrl, shift)` | `ime_process_key_v2(engine, key, &result)` |
| `ime_free(result)` | `ime_free_string_v2(result.text)` |
| `ime_method(0/1)` | `ime_set_config_v2(engine, &config)` |
| `ime_modern(bool)` | `config.tone_style = FfiToneStyle::New/Old` |
| `ime_instant_restore(bool)` | `config.instant_restore_enabled` |
| `ime_clear()` / `ime_clear_all()` | No explicit clear needed (per-engine state) |
| `ime_add_shortcut(t, r)` | `ime_add_shortcut_v2(engine, t, r)` |

**Key differences in v2:**
- Out-parameter result (no struct-return ABI issues with Swift).
- Per-engine config — no global state.
- Explicit status code returns instead of panics.
