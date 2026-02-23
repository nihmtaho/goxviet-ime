#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

typedef struct {
    bool success;
    int32_t error_code;
} FfiResult;

typedef struct {
    char *text;                // 0: *mut c_char (8 bytes)
    int32_t backspace_count;   // 8: i32 (4 bytes)
    bool consumed;             // 12: bool (1 byte + 3 padding)
    FfiResult result;          // 16: FfiResult (5 bytes -> 8 with padding)
} FfiProcessResult;

extern void* ime_engine_new(void);
extern FfiProcessResult ime_process_key(void* handle, const char* key_char, int32_t action);
extern void ime_engine_free(void* handle);
extern void ime_free_string(char* str);

int main() {
    printf("=== C FFI Test ===\n");
    printf("sizeof(FfiResult) = %zu\n", sizeof(FfiResult));
    printf("sizeof(FfiProcessResult) = %zu\n\n", sizeof(FfiProcessResult));
    
    void* handle = ime_engine_new();
    printf("Engine created: %p\n", handle);
    
    FfiProcessResult result = ime_process_key(handle, "a", 0);
    
    printf("\nResult:\n");
    printf("  text ptr: %p\n", result.text);
    printf("  backspace_count: %d\n", result.backspace_count);
    printf("  consumed: %d\n", result.consumed);
    printf("  result.success: %d\n", result.result.success);
    printf("  result.error_code: %d\n", result.result.error_code);
    
    if (result.text) {
        printf("  text value: '%s'\n", result.text);
        ime_free_string(result.text);
    }
    
    ime_engine_free(handle);
    printf("\nEngine freed\n");
    
    return 0;
}
