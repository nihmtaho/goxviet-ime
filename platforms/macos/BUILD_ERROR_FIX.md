# Build Error Fix - Type Redeclaration

## Problem

Build errors due to duplicate type definitions:
```
Invalid redeclaration of 'FfiInputMethod'
Invalid redeclaration of 'FfiToneStyle'
'FfiInputMethod' is ambiguous for type lookup
'FfiToneStyle' is ambiguous for type lookup
```

## Root Cause

Two files defining the same FFI types:
1. **RustBridgeV2.swift** (v2 API, ACTIVE)
   - `enum FfiInputMethod: UInt8`
   - `enum FfiToneStyle: UInt8`
   - `enum FfiStatusCode: Int32`

2. **CleanArchitectureFFIBridge.swift** (OBSOLETE)
   - `enum FfiInputMethod: Int32` ← Different base type!
   - `enum FfiToneStyle: Int32` ← Different base type!
   - Calls non-existent FFI functions (ime_engine_new, etc.)

## Solution

**Disabled CleanArchitectureFFIBridge.swift:**

```bash
# Renamed to prevent compilation
mv CleanArchitectureFFIBridge.swift CleanArchitectureFFIBridge.swift.unused
```

**Why safe:**
- ✅ File is not imported anywhere
- ✅ No code references it
- ✅ Defines wrong FFI functions (ime_engine_new vs ime_create_engine_v2)
- ✅ Uses wrong types (Int32 vs UInt8)
- ✅ Was never part of v2 migration

## Result

✅ **Build should now succeed**  
✅ Only RustBridgeV2.swift defines FFI types  
✅ No more ambiguous type lookups  
✅ Clean type system

## Verification

```bash
# Check for remaining conflicts
cd platforms/macos/goxviet
grep -r "enum FfiInputMethod" --include="*.swift" . | grep -v ".unused"
# Output: Only RustBridgeV2.swift
```

## Next Steps

1. Build in Xcode (Cmd+B) - should work now
2. If successful, can delete .unused file permanently
3. Continue with functional testing

---

*Fixed: 2026-02-12*  
*Status: Ready for build*
