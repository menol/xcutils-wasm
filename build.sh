#!/bin/bash
# Build script for xcutils-wasm
# This script builds the WASM package and copies it to the webapp directory

echo "Building WASM package..."

# Build the WASM package
wasm-pack build --target web --out-dir pkg

if [ $? -ne 0 ]; then
    echo "WASM build failed!"
    exit 1
fi

echo "WASM build successful!"

# Create webapp/src/wasm directory if it doesn't exist
WASM_DIR="webapp/src/wasm"
if [ ! -d "$WASM_DIR" ]; then
    mkdir -p "$WASM_DIR"
    echo "Created directory: $WASM_DIR"
fi

# Copy WASM files to webapp directory
echo "Copying WASM files to webapp..."

# Copy all necessary files
cp pkg/xcutils_wasm.js "$WASM_DIR/"
cp pkg/xcutils_wasm_bg.wasm "$WASM_DIR/"
cp pkg/xcutils_wasm.d.ts "$WASM_DIR/"
cp pkg/package.json "$WASM_DIR/"

if [ -f "pkg/xcutils_wasm_bg.wasm.d.ts" ]; then
    cp pkg/xcutils_wasm_bg.wasm.d.ts "$WASM_DIR/"
fi

echo "Files copied successfully!"
echo "You can now run 'npm run dev' in the webapp directory"