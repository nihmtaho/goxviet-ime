#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

typedef struct {
    bool success;
    int32_t error_code;
} FfiResult;

typedef struct {
    char *text;
    int32_t backspace_count;
    bool consumed;
    FfiResult result;
} FfiProcessResult;

int main() {
    printf("=== C Struct Layout (GCC) ===\n");
    printf("FfiResult:\n");
    printf("  sizeof = %zu\n", sizeof(FfiResult));
    printf("  success offset = %zu\n", offsetof(FfiResult, success));
    printf("  error_code offset = %zu\n", offsetof(FfiResult, error_code));
    printf("\n");
    
    printf("FfiProcessResult:\n");
    printf("  sizeof = %zu\n", sizeof(FfiProcessResult));
    printf("  text offset = %zu\n", offsetof(FfiProcessResult, text));
    printf("  backspace_count offset = %zu\n", offsetof(FfiProcessResult, backspace_count));
    printf("  consumed offset = %zu\n", offsetof(FfiProcessResult, consumed));
    printf("  result offset = %zu\n", offsetof(FfiProcessResult, result));
    
    return 0;
}
