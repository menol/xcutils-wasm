#!/usr/bin/env pwsh
# Build script for xcutils-wasm
# This script builds the WASM package and copies it to the webapp directory

Write-Host "=== xcutils-wasm Build Script ===" -ForegroundColor Cyan
Write-Host "Current directory: $(Get-Location)" -ForegroundColor Yellow

# Check if wasm-pack is installed
Write-Host "Checking wasm-pack installation..." -ForegroundColor Green
try {
    $wasmPackVersion = wasm-pack --version 2>$null
    Write-Host "Found wasm-pack: $wasmPackVersion" -ForegroundColor Green
} catch {
    Write-Host "Error: wasm-pack is not installed or not in PATH" -ForegroundColor Red
    Write-Host "Please install wasm-pack: https://rustwasm.github.io/wasm-pack/installer/" -ForegroundColor Yellow
    exit 1
}

# Build the WASM package
Write-Host "Building WASM package..." -ForegroundColor Green
wasm-pack build --target web --out-dir pkg --verbose

if ($LASTEXITCODE -ne 0) {
    Write-Host "WASM build failed!" -ForegroundColor Red
    Write-Host "Please check the error messages above." -ForegroundColor Yellow
    exit 1
}

Write-Host "WASM build successful!" -ForegroundColor Green

# Check if pkg directory was created
if (!(Test-Path "pkg")) {
    Write-Host "Error: pkg directory was not created" -ForegroundColor Red
    exit 1
}

Write-Host "Generated files in pkg directory:" -ForegroundColor Yellow
Get-ChildItem "pkg" | ForEach-Object { Write-Host "  - $($_.Name)" -ForegroundColor Gray }

# Create webapp/src/wasm directory if it doesn't exist
$wasmDir = "webapp/src/wasm"
if (!(Test-Path $wasmDir)) {
    New-Item -ItemType Directory -Path $wasmDir -Force
    Write-Host "Created directory: $wasmDir" -ForegroundColor Yellow
}

# Copy WASM files to webapp directory
Write-Host "Copying WASM files to webapp..." -ForegroundColor Green

# Copy all necessary files
Copy-Item "pkg/xcutils_wasm.js" "$wasmDir/" -Force
Copy-Item "pkg/xcutils_wasm_bg.wasm" "$wasmDir/" -Force
Copy-Item "pkg/xcutils_wasm.d.ts" "$wasmDir/" -Force
Copy-Item "pkg/package.json" "$wasmDir/" -Force

if (Test-Path "pkg/xcutils_wasm_bg.wasm.d.ts") {
    Copy-Item "pkg/xcutils_wasm_bg.wasm.d.ts" "$wasmDir/" -Force
}

Write-Host "Files copied successfully!" -ForegroundColor Green
Write-Host "You can now run 'npm run dev' in the webapp directory" -ForegroundColor Cyan