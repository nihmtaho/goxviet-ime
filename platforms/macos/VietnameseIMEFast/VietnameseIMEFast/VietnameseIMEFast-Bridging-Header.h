#ifndef VietnameseIME_Bridging_Header_h
#define VietnameseIME_Bridging_Header_h

#include <stdint.h>
#include <stdbool.h>

// ============================================================
// Core FFI Functions
// ============================================================

/// Initialize the IME engine (must be called once at startup)
void ime_init(void);

/// Process a key event
/// Returns pointer to Result struct (must be freed with ime_free)
/// Returns NULL if engine not initialized
typedef struct {
    uint32_t chars[64];  // UTF-32 codepoints (MUST match Rust MAX constant)
    uint8_t action;      // 0=None, 1=Send, 2=Restore
    uint8_t backspace;   // Number of chars to delete
    uint8_t count;       // Number of valid chars in array
    uint8_t _pad;        // Padding
} ImeResult;

ImeResult* ime_key(uint16_t key, bool caps, bool ctrl);

/// Process key event with extended parameters (for Shift handling)
ImeResult* ime_key_ext(uint16_t key, bool caps, bool ctrl, bool shift);

/// Free a result pointer
void ime_free(ImeResult* result);

/// Set input method (0=Telex, 1=VNI)
void ime_method(uint8_t method);

/// Enable or disable the engine
void ime_enabled(bool enabled);

/// Clear the input buffer (call on word boundaries)
void ime_clear(void);

// ============================================================
// Configuration Functions
// ============================================================

/// Skip w→ư shortcut in Telex mode
void ime_skip_w_shortcut(bool skip);

/// Enable ESC key to restore raw ASCII
void ime_esc_restore(bool enabled);

/// Enable free tone placement (skip validation)
void ime_free_tone(bool enabled);

/// Use modern orthography for tone placement
void ime_modern(bool modern);

// ============================================================
// Shortcut Management
// ============================================================

/// Add a text expansion shortcut
void ime_add_shortcut(const char* trigger, const char* replacement);

/// Remove a shortcut
void ime_remove_shortcut(const char* trigger);

/// Clear all shortcuts
void ime_clear_shortcuts(void);

// ============================================================
// Word Restore
// ============================================================

/// Restore buffer from a Vietnamese word string
void ime_restore_word(const char* word);

#endif /* VietnameseIME_Bridging_Header_h */