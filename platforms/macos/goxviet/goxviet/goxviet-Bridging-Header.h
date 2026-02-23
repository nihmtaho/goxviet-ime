#ifndef GoxViet_Bridging_Header_h
#define GoxViet_Bridging_Header_h

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

// ============================================================
// FFI API v2 - Clean Architecture
// ============================================================
//
// ALL FFI types and functions are declared in Swift (RustBridgeV2.swift)
// using @_silgen_name for direct symbol binding to the Rust library.
//
// This header is kept EMPTY to avoid dual declarations that cause:
// 1. Function overload ambiguity (C-imported vs @_silgen_name)
// 2. Type shadowing issues (C struct vs Swift struct)
//
// See RustBridgeV2.swift for the canonical type definitions.
//
// ============================================================
// REFERENCE: Rust FFI Types (DO NOT UNCOMMENT)
// ============================================================
//
// typedef enum { FFI_INPUT_METHOD_TELEX=0, FFI_INPUT_METHOD_VNI=1 } FfiInputMethod;
// typedef enum { FFI_TONE_STYLE_TRADITIONAL=0, FFI_TONE_STYLE_MODERN=1 } FfiToneStyle;
// typedef struct { FfiInputMethod input_method; FfiToneStyle tone_style; bool smart_mode; } FfiConfig_v2;
// typedef struct { char *text; uint8_t backspace_count; bool consumed; } FfiProcessResult_v2;
// typedef struct { uint32_t major,minor,patch,api_version; } FfiVersionInfo;
// typedef void* FfiEnginePtr;
//
// FfiEnginePtr ime_create_engine_v2(const FfiConfig_v2 *config);
// void ime_destroy_engine_v2(FfiEnginePtr engine);
// int32_t ime_process_key_v2(FfiEnginePtr engine, char key_char, FfiProcessResult_v2 *out);
// int32_t ime_get_config_v2(FfiEnginePtr engine, FfiConfig_v2 *out);
// int32_t ime_set_config_v2(FfiEnginePtr engine, const FfiConfig_v2 *config);
// int32_t ime_get_version_v2(FfiVersionInfo *out);
// void ime_free_string_v2(char *ptr);
// int32_t ime_add_shortcut_v2(FfiEnginePtr engine, const char *trigger, const char *expansion);
// int32_t ime_remove_shortcut_v2(FfiEnginePtr engine, const char *trigger);
// int32_t ime_clear_shortcuts_v2(FfiEnginePtr engine);
// int32_t ime_shortcuts_count_v2(FfiEnginePtr engine);
// int32_t ime_set_shortcuts_enabled_v2(FfiEnginePtr engine, bool enabled);
//
// ============================================================
// LEGACY v1 API (REMOVED in v3.0.0)
// ============================================================
// All v1 functions (ime_init, ime_key, ime_free, etc.) have been removed.
// Use RustEngineV2 wrapper functions (ime_init_v2, ime_key_v2, etc.) instead.

#endif /* GoxViet_Bridging_Header_h */
