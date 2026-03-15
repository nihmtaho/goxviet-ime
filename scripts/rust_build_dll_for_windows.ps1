# rust_build_dll_for_windows.ps1
# Build goxviet_core.dll for Windows x64 using the MSVC toolchain.
# Run from the repository root on a Windows machine with:
#   - Rust toolchain (cargo) with target x86_64-pc-windows-msvc
#   - Visual Studio Build Tools (MSVC linker)
#
# Cross-compile from macOS/Linux (using mingw-w64):
#   rustup target add x86_64-pc-windows-gnu
#   brew install mingw-w64          # macOS
#   cargo build --target x86_64-pc-windows-gnu
#
# Usage:
#   .\scripts\rust_build_dll_for_windows.ps1
#   .\scripts\rust_build_dll_for_windows.ps1 -Target x86_64-pc-windows-msvc -Profile release

param(
    [string]$Target  = "x86_64-pc-windows-msvc",
    [string]$Profile = "release"
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$coreDir  = Join-Path $repoRoot "core"
$destDir  = Join-Path $repoRoot "platforms\windows\src\GoxViet"

Write-Host "==> Building goxviet_core.dll" -ForegroundColor Cyan
Write-Host "    Target:  $Target"
Write-Host "    Profile: $Profile"
Write-Host "    Core:    $coreDir"

# Ensure target is installed
rustup target add $Target

Push-Location $coreDir
try {
    if ($Profile -eq "release") {
        cargo build --release --target $Target
    } else {
        cargo build --target $Target
    }
    if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }
} finally {
    Pop-Location
}

$dllSrc = Join-Path $coreDir "target\$Target\$Profile\goxviet_core.dll"

if (-not (Test-Path $dllSrc)) {
    Write-Error "DLL not found at: $dllSrc"
    exit 1
}

# Verify exported symbols
if (Get-Command dumpbin -ErrorAction SilentlyContinue) {
    Write-Host "`n==> Verifying DLL exports..." -ForegroundColor Cyan
    $exports = & dumpbin /exports $dllSrc 2>&1
    $required = @(
        "ime_create_engine_v2",
        "ime_process_key_v2",
        "ime_process_key_ext_v2",
        "ime_free_string_v2",
        "ime_destroy_engine_v2"
    )
    foreach ($sym in $required) {
        if ($exports -match $sym) {
            Write-Host "    [OK] $sym" -ForegroundColor Green
        } else {
            Write-Warning "    [MISSING] $sym"
        }
    }
}

# Copy alongside the .csproj so the <None> item picks it up
if (-not (Test-Path $destDir)) { New-Item -ItemType Directory -Path $destDir | Out-Null }
Copy-Item $dllSrc (Join-Path $destDir "goxviet_core.dll") -Force
Write-Host "`n==> Done. DLL copied to $destDir\goxviet_core.dll" -ForegroundColor Green
