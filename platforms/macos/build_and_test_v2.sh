#!/bin/bash
#
# Build and Test FFI API v2
#
# This script:
# 1. Builds the Rust core library
# 2. Compiles C test
# 3. Compiles Swift standalone test
# 4. Runs both tests
# 5. Compares results

set -e  # Exit on error

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
MACOS_DIR="$PROJECT_ROOT/platforms/macos"
CORE_DIR="$PROJECT_ROOT/core"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║           FFI API v2 Build & Test Script                  ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# ============================================================================
# Step 1: Build Rust Core Library
# ============================================================================

echo "📦 Step 1: Building Rust core library..."
echo "─────────────────────────────────────────────────────────────"

cd "$CORE_DIR"

# Build for current architecture first (faster for testing)
echo "Building for current architecture..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Cargo build failed!"
    exit 1
fi

# Copy library to macos directory
LIB_SRC="$CORE_DIR/target/release/libgoxviet_core.a"
LIB_DST="$MACOS_DIR/libgoxviet_core.a"

if [ ! -f "$LIB_SRC" ]; then
    echo "❌ Library not found: $LIB_SRC"
    exit 1
fi

cp "$LIB_SRC" "$LIB_DST"
echo "✅ Library copied to: $LIB_DST"
echo "📊 Library size: $(du -h "$LIB_DST" | cut -f1)"
echo ""

# ============================================================================
# Step 2: Compile C Test
# ============================================================================

echo "🔧 Step 2: Compiling C test..."
echo "─────────────────────────────────────────────────────────────"

cd "$MACOS_DIR"

gcc -o test_ffi_v2 test_ffi_v2.c \
    -L. -lgoxviet_core \
    -Wl,-rpath,@loader_path \
    -framework Foundation

if [ $? -ne 0 ]; then
    echo "❌ C compilation failed!"
    exit 1
fi

echo "✅ C test compiled: test_ffi_v2"
echo ""

# ============================================================================
# Step 3: Compile Swift Standalone Test
# ============================================================================

echo "🔧 Step 3: Compiling Swift standalone test..."
echo "─────────────────────────────────────────────────────────────"

swiftc test_ffi_v2.swift \
    -L. -lgoxviet_core \
    -Xlinker -rpath -Xlinker @loader_path \
    -o test_ffi_v2_swift

if [ $? -ne 0 ]; then
    echo "❌ Swift compilation failed!"
    exit 1
fi

echo "✅ Swift test compiled: test_ffi_v2_swift"
echo ""

# ============================================================================
# Step 4: Run C Test
# ============================================================================

echo "🧪 Step 4: Running C test..."
echo "─────────────────────────────────────────────────────────────"

./test_ffi_v2
C_EXIT_CODE=$?

echo ""
if [ $C_EXIT_CODE -eq 0 ]; then
    echo "✅ C test passed"
else
    echo "❌ C test failed (exit code: $C_EXIT_CODE)"
fi
echo ""

# ============================================================================
# Step 5: Run Swift Standalone Test (CRITICAL!)
# ============================================================================

echo "🧪 Step 5: Running Swift standalone test (CRITICAL TEST!)..."
echo "─────────────────────────────────────────────────────────────"
echo "⚠️  This test failed with v1 API due to ABI issue"
echo "✨  Should now work with v2 API out parameter pattern"
echo ""

./test_ffi_v2_swift
SWIFT_EXIT_CODE=$?

echo ""
if [ $SWIFT_EXIT_CODE -eq 0 ]; then
    echo "✅ Swift test passed"
    echo "🎉 OUT PARAMETER PATTERN FIXES THE ABI ISSUE!"
else
    echo "❌ Swift test failed (exit code: $SWIFT_EXIT_CODE)"
    echo "⚠️  ABI issue may still exist"
fi
echo ""

# ============================================================================
# Summary
# ============================================================================

echo "╔════════════════════════════════════════════════════════════╗"
echo "║                      TEST SUMMARY                          ║"
echo "╠════════════════════════════════════════════════════════════╣"

if [ $C_EXIT_CODE -eq 0 ]; then
    echo "║  C Test:     ✅ PASSED                                    ║"
else
    echo "║  C Test:     ❌ FAILED                                    ║"
fi

if [ $SWIFT_EXIT_CODE -eq 0 ]; then
    echo "║  Swift Test: ✅ PASSED (ABI ISSUE FIXED!)                 ║"
else
    echo "║  Swift Test: ❌ FAILED (ABI issue persists)               ║"
fi

echo "╚════════════════════════════════════════════════════════════╝"
echo ""

if [ $C_EXIT_CODE -eq 0 ] && [ $SWIFT_EXIT_CODE -eq 0 ]; then
    echo "🎉 SUCCESS! FFI API v2 works correctly in both C and Swift!"
    echo "✨ Out parameter pattern resolves the ABI struct-return issue!"
    echo ""
    echo "Next steps:"
    echo "  1. Update documentation"
    echo "  2. Mark v1 API as deprecated"
    echo "  3. Create migration guide"
    echo "  4. Prepare v2.0.0 release"
    exit 0
else
    echo "❌ Tests failed. Please investigate."
    echo ""
    echo "Debugging tips:"
    echo "  - Check compilation warnings"
    echo "  - Verify library symbols: nm -g libgoxviet_core.a | grep ime_"
    echo "  - Run with verbose output: RUST_LOG=debug ./test_ffi_v2"
    exit 1
fi
