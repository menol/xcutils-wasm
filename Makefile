# Makefile for xcutils-wasm project

.PHONY: build clean install dev test help

# Default target
all: build

# Build the WASM package
build:
	@echo "Building WASM package..."
	wasm-pack build --target web --out-dir pkg
	@echo "Copying WASM files to webapp..."
	@mkdir -p webapp/src/wasm
	cp pkg/xcutils_wasm.js webapp/src/wasm/
	cp pkg/xcutils_wasm_bg.wasm webapp/src/wasm/
	cp pkg/xcutils_wasm.d.ts webapp/src/wasm/
	cp pkg/package.json webapp/src/wasm/
	@if [ -f pkg/xcutils_wasm_bg.wasm.d.ts ]; then cp pkg/xcutils_wasm_bg.wasm.d.ts webapp/src/wasm/; fi
	@echo "Build complete!"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	rm -rf pkg/
	rm -rf webapp/src/wasm/
	rm -rf webapp/node_modules/
	rm -rf webapp/dist/
	@echo "Clean complete!"

# Install dependencies
install:
	@echo "Installing webapp dependencies..."
	cd webapp && npm install

# Start development server
dev: build install
	@echo "Starting development server..."
	cd webapp && npm run dev

# Run tests
test:
	@echo "Running Rust tests..."
	cargo test

# Build for production
prod: build
	@echo "Building for production..."
	cd webapp && npm install && npm run build

# Help
help:
	@echo "Available targets:"
	@echo "  build    - Build WASM package and copy to webapp"
	@echo "  clean    - Clean all build artifacts"
	@echo "  install  - Install webapp dependencies"
	@echo "  dev      - Build and start development server"
	@echo "  test     - Run Rust tests"
	@echo "  prod     - Build for production"
	@echo "  help     - Show this help message"