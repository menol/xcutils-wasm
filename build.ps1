# PowerShell Build script for xcutils-wasm
# This script builds the WASM package and copies it to the webapp directory

Write-Host "Checking wasm-pack installation..." -ForegroundColor Yellow

# Check if wasm-pack is installed
try {
    $wasmPackVersion = wasm-pack --version
    Write-Host "wasm-pack found: $wasmPackVersion" -ForegroundColor Green
} catch {
    Write-Host "Error: wasm-pack is not installed or not in PATH" -ForegroundColor Red
    Write-Host "Please install wasm-pack from: https://rustwasm.github.io/wasm-pack/installer/" -ForegroundColor Yellow
    exit 1
}

Write-Host "Building WASM package..." -ForegroundColor Yellow

# Build the WASM package with verbose output
wasm-pack build --target web --out-dir pkg --verbose

if ($LASTEXITCODE -ne 0) {
    Write-Host "WASM build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "WASM build successful!" -ForegroundColor Green

# Check if pkg directory was created
if (-not (Test-Path "pkg")) {
    Write-Host "Error: pkg directory was not created" -ForegroundColor Red
    exit 1
}

Write-Host "pkg directory contents:" -ForegroundColor Yellow
Get-ChildItem "pkg" | ForEach-Object { Write-Host "  $($_.Name)" }

# Create webapp/src/wasm directory if it doesn't exist
$wasmDir = "webapp/src/wasm"
if (-not (Test-Path $wasmDir)) {
    New-Item -ItemType Directory -Path $wasmDir -Force | Out-Null
    Write-Host "Created directory: $wasmDir" -ForegroundColor Green
}

# Copy WASM files to webapp directory
Write-Host "Copying WASM files to webapp..." -ForegroundColor Yellow

# Copy all necessary files
$filesToCopy = @(
    "xcutils_wasm.js",
    "xcutils_wasm_bg.wasm",
    "xcutils_wasm.d.ts",
    "package.json"
)

foreach ($file in $filesToCopy) {
    $sourcePath = "pkg/$file"
    $destPath = "$wasmDir/$file"
    
    if (Test-Path $sourcePath) {
        Copy-Item $sourcePath $destPath -Force
        Write-Host "Copied: $file" -ForegroundColor Green
    } else {
        Write-Host "Warning: $file not found in pkg directory" -ForegroundColor Yellow
    }
}

# Copy optional files
if (Test-Path "pkg/xcutils_wasm_bg.wasm.d.ts") {
    Copy-Item "pkg/xcutils_wasm_bg.wasm.d.ts" "$wasmDir/" -Force
    Write-Host "Copied: xcutils_wasm_bg.wasm.d.ts" -ForegroundColor Green
}

Write-Host "Files copied successfully!" -ForegroundColor Green
Write-Host "You can now run 'npm run dev' in the webapp directory" -ForegroundColor Cyan