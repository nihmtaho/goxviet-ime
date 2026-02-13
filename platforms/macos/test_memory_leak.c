#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

typedef struct {
    bool success;
    int error_code;
} FfiResult;

typedef struct {
    char *text;
    int backspace_count;
    bool consumed;
    FfiResult result;
} FfiProcessResult;

extern void* ime_engine_new(void);
extern FfiProcessResult ime_process_key(void* handle, const char* key_char, int action);
extern void ime_engine_free(void* handle);
extern void ime_free_string(char* str);

// Test 1: Engine lifecycle stress test
void test_engine_lifecycle(int iterations) {
    printf("Test 1: Engine Lifecycle (%d iterations)\n", iterations);
    for (int i = 0; i < iterations; i++) {
        void* handle = ime_engine_new();
        if (!handle) {
            printf("  ❌ Failed to create engine at iteration %d\n", i);
            return;
        }
        ime_engine_free(handle);
    }
    printf("  ✅ %d engine create/destroy cycles completed\n", iterations);
}

// Test 2: String allocation/deallocation stress
void test_string_lifecycle(int iterations) {
    printf("\nTest 2: String Lifecycle (%d iterations)\n", iterations);
    void* handle = ime_engine_new();
    if (!handle) {
        printf("  ❌ Failed to create engine\n");
        return;
    }
    
    int leaked = 0;
    for (int i = 0; i < iterations; i++) {
        FfiProcessResult result = ime_process_key(handle, "a", 0);
        if (result.text) {
            ime_free_string(result.text);
        } else {
            leaked++;
        }
    }
    
    ime_engine_free(handle);
    printf("  ✅ %d process_key calls completed\n", iterations);
    if (leaked > 0) {
        printf("  ⚠️  %d iterations returned null text\n", leaked);
    }
}

// Test 3: Mixed operations stress
void test_mixed_operations(int iterations) {
    printf("\nTest 3: Mixed Operations (%d iterations)\n", iterations);
    void* handle = ime_engine_new();
    if (!handle) {
        printf("  ❌ Failed to create engine\n");
        return;
    }
    
    const char* keys[] = {"a", "b", "c", "d", "e", "f", "s", "r", "x"};
    int key_count = sizeof(keys) / sizeof(keys[0]);
    
    for (int i = 0; i < iterations; i++) {
        const char* key = keys[i % key_count];
        FfiProcessResult result = ime_process_key(handle, key, 0);
        if (result.text) {
            ime_free_string(result.text);
        }
    }
    
    ime_engine_free(handle);
    printf("  ✅ %d mixed key operations completed\n", iterations);
}

// Test 4: Rapid create/destroy with processing
void test_rapid_lifecycle(int iterations) {
    printf("\nTest 4: Rapid Lifecycle (%d iterations)\n", iterations);
    for (int i = 0; i < iterations; i++) {
        void* handle = ime_engine_new();
        if (!handle) {
            printf("  ❌ Failed at iteration %d\n", i);
            return;
        }
        
        // Process a few keys
        FfiProcessResult r1 = ime_process_key(handle, "a", 0);
        if (r1.text) ime_free_string(r1.text);
        
        FfiProcessResult r2 = ime_process_key(handle, "s", 0);
        if (r2.text) ime_free_string(r2.text);
        
        ime_engine_free(handle);
    }
    printf("  ✅ %d rapid lifecycle cycles completed\n", iterations);
}

// Test 5: Long-running session
void test_long_session(int keystrokes) {
    printf("\nTest 5: Long Session (%d keystrokes)\n", keystrokes);
    void* handle = ime_engine_new();
    if (!handle) {
        printf("  ❌ Failed to create engine\n");
        return;
    }
    
    const char* alphabet = "abcdefghijklmnopqrstuvwxyz";
    int alpha_len = strlen(alphabet);
    
    for (int i = 0; i < keystrokes; i++) {
        char key[2] = {alphabet[i % alpha_len], '\0'};
        FfiProcessResult result = ime_process_key(handle, key, 0);
        if (result.text) {
            ime_free_string(result.text);
        }
        
        // Progress indicator
        if ((i + 1) % 1000 == 0) {
            printf("  Progress: %d/%d keystrokes\n", i + 1, keystrokes);
        }
    }
    
    ime_engine_free(handle);
    printf("  ✅ Long session completed\n");
}

int main() {
    printf("========================================\n");
    printf("GoxViet Memory Leak Detection Tests\n");
    printf("========================================\n");
    printf("\n🔍 Run with Instruments/Valgrind for leak detection\n");
    printf("   macOS: leaks test_memory_leak\n");
    printf("   Linux: valgrind --leak-check=full ./test_memory_leak\n\n");
    
    // Warm-up
    printf("Warm-up...\n");
    void* handle = ime_engine_new();
    FfiProcessResult r = ime_process_key(handle, "test", 0);
    if (r.text) ime_free_string(r.text);
    ime_engine_free(handle);
    printf("Warm-up complete.\n\n");
    
    // Main tests
    test_engine_lifecycle(1000);
    test_string_lifecycle(5000);
    test_mixed_operations(5000);
    test_rapid_lifecycle(500);
    test_long_session(10000);
    
    printf("\n========================================\n");
    printf("All tests completed!\n");
    printf("========================================\n");
    printf("\n📊 Check memory usage:\n");
    printf("   - Run 'leaks <PID>' during execution\n");
    printf("   - Use Instruments Leaks template\n");
    printf("   - Check Activity Monitor for memory growth\n\n");
    
    return 0;
}
