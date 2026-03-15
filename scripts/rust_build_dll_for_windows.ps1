# rust_build_dll_for_windows.ps1
# Build goxviet_core.dll for Windows (x64 and/or arm64) using the MSVC toolchain.
# Run from the repository root on a Windows machine with:
#   - Rust toolchain (cargo) with the desired target installed
#   - Visual Studio Build Tools (MSVC linker) including ARM64 cross-compile tools
#
# Usage:
#   .\scripts\rust_build_dll_for_windows.ps1                          # x64 only
#   .\scripts\rust_build_dll_for_windows.ps1 -Arch arm64              # arm64 only
#   .\scripts\rust_build_dll_for_windows.ps1 -Arch all                # x64 + arm64
#   .\scripts\rust_build_dll_for_windows.ps1 -Arch x64 -Profile debug

param(
    [ValidateSet("x64", "arm64", "all")]
    [string]$Arch    = "x64",
    [string]$Profile = "release"
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$coreDir  = Join-Path $repoRoot "core"
$destDir  = Join-Path $repoRoot "platforms\windows\src\GoxViet"

$targets = @{
    "x64"   = "x86_64-pc-windows-msvc"
    "arm64" = "aarch64-pc-windows-msvc"
}

$buildList = if ($Arch -eq "all") { @("x64", "arm64") } else { @($Arch) }

foreach ($arch in $buildList) {
    $target = $targets[$arch]

    Write-Host "`n==> Building goxviet_core.dll [$arch]" -ForegroundColor Cyan
    Write-Host "    Target:  $target"
    Write-Host "    Profile: $Profile"

    # Ensure Rust target is installed
    rustup target add $target

    Push-Location $coreDir
    try {
        if ($Profile -eq "release") {
            cargo build --release --target $target
        } else {
            cargo build --target $target
        }
        if ($LASTEXITCODE -ne 0) { throw "cargo build failed for $target" }
    } finally {
        Pop-Location
    }

    $dllSrc = Join-Path $coreDir "target\$target\$Profile\goxviet_core.dll"

    if (-not (Test-Path $dllSrc)) {
        Write-Error "DLL not found at: $dllSrc"
        exit 1
    }

    # Verify exported symbols (only if dumpbin is available)
    $dumpbin = Get-Command dumpbin -ErrorAction SilentlyContinue
    if ($dumpbin) {
        Write-Host "`n    Verifying DLL exports..." -ForegroundColor DarkCyan
        $exports  = & dumpbin /exports $dllSrc 2>&1
        $required = @(
            "ime_create_engine_v2",
            "ime_process_key_v2",
            "ime_process_key_ext_v2",
            "ime_free_string_v2",
            "ime_destroy_engine_v2"
        )
        foreach ($sym in $required) {
            if (-not ($exports -match $sym)) {
                Write-Warning "    [MISSING] $sym"
            } else {
                Write-Host "    [OK] $sym" -ForegroundColor Green
            }
        }
    }

    # Copy into the project directory so the .csproj <None> item picks it up
    $archDestDir = Join-Path $destDir $arch
    if (-not (Test-Path $archDestDir)) { New-Item -ItemType Directory -Path $archDestDir | Out-Null }
    Copy-Item $dllSrc (Join-Path $archDestDir "goxviet_core.dll") -Force
    Write-Host "    Copied to $archDestDir\goxviet_core.dll" -ForegroundColor Green
}

Write-Host "`n==> Done." -ForegroundColor Cyan
