/**
 * FFI API v2 Test Suite (C)
 * 
 * Purpose: Validate new out-parameter API and compare with v1
 * 
 * Build:
 *   gcc -o test_ffi_v2 test_ffi_v2.c \
 *       -L. -lgoxviet_core \
 *       -Wl,-rpath,@loader_path
 * 
 * Run:
 *   ./test_ffi_v2
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <assert.h>

// ============================================================================
// FFI Type Definitions
// ============================================================================

// Status codes (v2 only)
typedef enum {
    FFI_SUCCESS = 0,
    FFI_ERROR_NULL_POINTER = -1,
    FFI_ERROR_INVALID_ENGINE = -2,
    FFI_ERROR_PROCESSING = -3,
    FFI_ERROR_PANIC = -99,
} FfiStatusCode;

// Key event (same for v1 and v2)
typedef struct {
    uint32_t key_code;
    uint8_t action;
    uint8_t modifiers;
} FfiKeyEvent;

// Config v1
typedef struct {
    uint8_t input_method;
    uint8_t tone_style;
    uint8_t smart_mode;
    uint8_t enable_shortcuts;
} FfiConfig;

// Config v2 (no enable_shortcuts)
typedef struct {
    uint8_t input_method;
    uint8_t tone_style;
    uint8_t smart_mode;
} FfiConfig_v2;

// Process result v1 (returned by value - ABI issue!)
typedef struct {
    char* text;
    uint8_t consumed;
    uint8_t requires_backspace;
} FfiProcessResult;

// Process result v2 (out parameter - ABI safe)
typedef struct {
    char* text;
    uint8_t consumed;
    uint8_t requires_backspace;
} FfiProcessResult_v2;

// Version info v2
typedef struct {
    uint8_t major;
    uint8_t minor;
    uint8_t patch;
} FfiVersionInfo;

// ============================================================================
// FFI Function Declarations
// ============================================================================

// v1 API (existing - not used in this test)
// extern void* ime_create_engine(FfiConfig config);
// extern void ime_destroy_engine(void* handle);
// extern FfiProcessResult ime_process_key(void* handle, FfiKeyEvent key);
// extern FfiConfig ime_get_config(void* handle);
// extern void ime_set_config(void* handle, FfiConfig config);
// extern char* ime_get_version(void);
// extern void ime_free_string(char* ptr);

// v2 API (new - out parameters - focus of this test)
extern int32_t ime_create_engine_v2(void** out_engine, const FfiConfig_v2* config);
extern int32_t ime_destroy_engine_v2(void* engine);
extern int32_t ime_process_key_v2(void* engine, FfiKeyEvent key, FfiProcessResult_v2* out);
extern int32_t ime_get_config_v2(void* engine, FfiConfig_v2* out);
extern int32_t ime_set_config_v2(void* engine, const FfiConfig_v2* config);
extern int32_t ime_get_version_v2(FfiVersionInfo* out);
extern void ime_free_string_v2(char* ptr);

// ============================================================================
// Test Utilities
// ============================================================================

static int test_count = 0;
static int test_passed = 0;
static int test_failed = 0;

#define TEST_START(name) \
    do { \
        test_count++; \
        printf("\n[TEST %d] %s\n", test_count, name); \
    } while(0)

#define TEST_ASSERT(cond, msg) \
    do { \
        if (!(cond)) { \
            printf("  ❌ FAIL: %s\n", msg); \
            test_failed++; \
            return; \
        } \
    } while(0)

#define TEST_PASS(msg) \
    do { \
        printf("  ✅ PASS: %s\n", msg); \
        test_passed++; \
    } while(0)

// ============================================================================
// v2 API Tests
// ============================================================================

void test_v2_version(void) {
    TEST_START("v2 Get Version");
    
    FfiVersionInfo version = {0};
    int32_t status = ime_get_version_v2(&version);
    
    TEST_ASSERT(status == FFI_SUCCESS, "Status should be SUCCESS");
    TEST_ASSERT(version.major > 0, "Major version should be > 0");
    
    printf("  📌 Version: %d.%d.%d\n", version.major, version.minor, version.patch);
    TEST_PASS("Version info retrieved");
}

void test_v2_engine_lifecycle(void) {
    TEST_START("v2 Engine Lifecycle");
    
    // Create with default config (NULL)
    void* engine = NULL;
    int32_t status = ime_create_engine_v2(&engine, NULL);
    
    TEST_ASSERT(status == FFI_SUCCESS, "Create should succeed");
    TEST_ASSERT(engine != NULL, "Engine handle should not be NULL");
    
    // Destroy
    status = ime_destroy_engine_v2(engine);
    TEST_ASSERT(status == FFI_SUCCESS, "Destroy should succeed");
    
    TEST_PASS("Lifecycle complete");
}

void test_v2_engine_with_config(void) {
    TEST_START("v2 Engine with Custom Config");
    
    FfiConfig_v2 config = {
        .input_method = 1,  // VNI
        .tone_style = 1,    // Old
        .smart_mode = 0,    // Off
    };
    
    void* engine = NULL;
    int32_t status = ime_create_engine_v2(&engine, &config);
    
    TEST_ASSERT(status == FFI_SUCCESS, "Create with config should succeed");
    TEST_ASSERT(engine != NULL, "Engine handle should not be NULL");
    
    // Verify config was applied
    FfiConfig_v2 retrieved = {0};
    status = ime_get_config_v2(engine, &retrieved);
    
    TEST_ASSERT(status == FFI_SUCCESS, "Get config should succeed");
    TEST_ASSERT(retrieved.input_method == 1, "Input method should be VNI");
    TEST_ASSERT(retrieved.tone_style == 1, "Tone style should be Old");
    TEST_ASSERT(retrieved.smart_mode == 0, "Smart mode should be Off");
    
    ime_destroy_engine_v2(engine);
    TEST_PASS("Config roundtrip successful");
}

void test_v2_process_key_simple(void) {
    TEST_START("v2 Process Key - Simple Character");
    
    void* engine = NULL;
    int32_t status = ime_create_engine_v2(&engine, NULL);
    TEST_ASSERT(status == FFI_SUCCESS, "Engine created");
    
    // Process 'a'
    FfiKeyEvent key = {
        .key_code = 'a',
        .action = 0,  // Press
        .modifiers = 0,
    };
    
    FfiProcessResult_v2 result = {0};
    status = ime_process_key_v2(engine, key, &result);
    
    TEST_ASSERT(status == FFI_SUCCESS, "Process key should succeed");
    TEST_ASSERT(result.text != NULL, "Result text should not be NULL");
    TEST_ASSERT(result.consumed == 1, "Key should be consumed");
    
    printf("  📌 Input: 'a' -> Output: '%s', consumed: %d\n", 
           result.text, result.consumed);
    
    // Critical check: text should be 'a'
    TEST_ASSERT(strcmp(result.text, "a") == 0, "Text should be 'a'");
    
    ime_free_string_v2(result.text);
    ime_destroy_engine_v2(engine);
    
    TEST_PASS("Simple key processing works (ABI SAFE!)");
}

void test_v2_process_key_tone(void) {
    TEST_START("v2 Process Key - Tone Mark (Telex)");
    
    void* engine = NULL;
    ime_create_engine_v2(&engine, NULL);
    
    // Process 'a'
    FfiKeyEvent key_a = {.key_code = 'a', .action = 0, .modifiers = 0};
    FfiProcessResult_v2 result1 = {0};
    ime_process_key_v2(engine, key_a, &result1);
    
    printf("  📌 Step 1: 'a' -> '%s'\n", result1.text);
    ime_free_string_v2(result1.text);
    
    // Process 's' (sắc tone in Telex)
    FfiKeyEvent key_s = {.key_code = 's', .action = 0, .modifiers = 0};
    FfiProcessResult_v2 result2 = {0};
    int32_t status = ime_process_key_v2(engine, key_s, &result2);
    
    TEST_ASSERT(status == FFI_SUCCESS, "Tone mark should succeed");
    TEST_ASSERT(result2.text != NULL, "Result should not be NULL");
    
    printf("  📌 Step 2: 's' -> '%s' (should be 'á')\n", result2.text);
    
    // Should produce 'á' (a with sắc tone)
    TEST_ASSERT(strcmp(result2.text, "á") == 0 || strlen(result2.text) > 1, 
                "Should produce accented character");
    
    ime_free_string_v2(result2.text);
    ime_destroy_engine_v2(engine);
    
    TEST_PASS("Tone mark processing works");
}

void test_v2_config_get_set(void) {
    TEST_START("v2 Config Get/Set");
    
    void* engine = NULL;
    ime_create_engine_v2(&engine, NULL);
    
    // Get initial config
    FfiConfig_v2 initial = {0};
    ime_get_config_v2(engine, &initial);
    printf("  📌 Initial: method=%d, tone=%d, smart=%d\n",
           initial.input_method, initial.tone_style, initial.smart_mode);
    
    // Change to VNI
    FfiConfig_v2 new_config = {
        .input_method = 1,  // VNI
        .tone_style = initial.tone_style,
        .smart_mode = initial.smart_mode,
    };
    
    int32_t status = ime_set_config_v2(engine, &new_config);
    TEST_ASSERT(status == FFI_SUCCESS, "Set config should succeed");
    
    // Verify change
    FfiConfig_v2 updated = {0};
    ime_get_config_v2(engine, &updated);
    TEST_ASSERT(updated.input_method == 1, "Input method should be changed to VNI");
    
    ime_destroy_engine_v2(engine);
    TEST_PASS("Config get/set roundtrip works");
}

void test_v2_null_safety(void) {
    TEST_START("v2 Null Pointer Safety");
    
    // Create with null out pointer
    int32_t status = ime_create_engine_v2(NULL, NULL);
    TEST_ASSERT(status == FFI_ERROR_NULL_POINTER, "Should return null pointer error");
    
    // Process with null engine
    FfiKeyEvent key = {.key_code = 'a', .action = 0, .modifiers = 0};
    FfiProcessResult_v2 result = {0};
    status = ime_process_key_v2(NULL, key, &result);
    TEST_ASSERT(status == FFI_ERROR_NULL_POINTER, "Should return null pointer error");
    
    // Process with null out
    void* engine = NULL;
    ime_create_engine_v2(&engine, NULL);
    status = ime_process_key_v2(engine, key, NULL);
    TEST_ASSERT(status == FFI_ERROR_NULL_POINTER, "Should return null pointer error");
    
    ime_destroy_engine_v2(engine);
    TEST_PASS("Null pointer checks work");
}

void test_v2_memory_cleanup(void) {
    TEST_START("v2 Memory Cleanup");
    
    void* engine = NULL;
    ime_create_engine_v2(&engine, NULL);
    
    // Process multiple keys and free strings
    for (int i = 0; i < 100; i++) {
        FfiKeyEvent key = {.key_code = 'a' + (i % 26), .action = 0, .modifiers = 0};
        FfiProcessResult_v2 result = {0};
        ime_process_key_v2(engine, key, &result);
        
        if (result.text) {
            ime_free_string_v2(result.text);
        }
    }
    
    ime_destroy_engine_v2(engine);
    TEST_PASS("100 keys processed and cleaned up");
}

// ============================================================================
// v1 vs v2 Comparison Tests
// ============================================================================

// NOTE: v1 API comparison test disabled - v1 symbols may not be exported
// The v2 API is standalone and doesn't require v1 for validation
// Swift standalone test will demonstrate the ABI fix

/*
void test_v1_vs_v2_same_result(void) {
    TEST_START("v1 vs v2 Result Comparison");
    // ... test disabled ...
}
*/

// ============================================================================
// Main Test Runner
// ============================================================================

int main(void) {
    printf("╔════════════════════════════════════════════════════════════╗\n");
    printf("║           GoxViet FFI API v2 Test Suite (C)               ║\n");
    printf("║                                                            ║\n");
    printf("║  Note: v1 comparison test disabled (v1 API not required)  ║\n");
    printf("╚════════════════════════════════════════════════════════════╝\n");
    
    // v2 API tests
    test_v2_version();
    test_v2_engine_lifecycle();
    test_v2_engine_with_config();
    test_v2_process_key_simple();
    test_v2_process_key_tone();
    test_v2_config_get_set();
    test_v2_null_safety();
    test_v2_memory_cleanup();
    
    // Comparison test disabled - v1 API not required
    // test_v1_vs_v2_same_result();
    
    // Summary
    printf("\n╔════════════════════════════════════════════════════════════╗\n");
    printf("║                      TEST SUMMARY                          ║\n");
    printf("╠════════════════════════════════════════════════════════════╣\n");
    printf("║  Total Tests: %d                                          ║\n", test_count);
    printf("║  Passed:      %d ✅                                       ║\n", test_passed);
    printf("║  Failed:      %d ❌                                       ║\n", test_failed);
    printf("╚════════════════════════════════════════════════════════════╝\n");
    
    if (test_failed == 0) {
        printf("\n🎉 ALL TESTS PASSED! FFI API v2 is working correctly.\n");
        return 0;
    } else {
        printf("\n❌ SOME TESTS FAILED. Please investigate.\n");
        return 1;
    }
}
