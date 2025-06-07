# Makefile for xcutils-wasm project

.PHONY: build dev clean install help

# Default target
all: build

# Build WASM and copy to webapp
build:
	@echo "Building WASM package..."
	@wasm-pack build --target web --out-dir pkg
	@echo "Copying files to webapp..."
	@mkdir -p webapp/src/wasm
	@cp pkg/xcutils_wasm.js webapp/src/wasm/
	@cp pkg/xcutils_wasm_bg.wasm webapp/src/wasm/
	@cp pkg/xcutils_wasm.d.ts webapp/src/wasm/
	@cp pkg/package.json webapp/src/wasm/
	@if [ -f pkg/xcutils_wasm_bg.wasm.d.ts ]; then cp pkg/xcutils_wasm_bg.wasm.d.ts webapp/src/wasm/; fi
	@echo "Build complete! You can now run 'make dev' to start the development server."

# Start development server
dev: build
	@echo "Starting development server..."
	@cd webapp && npm run dev

# Install dependencies
install:
	@echo "Installing webapp dependencies..."
	@cd webapp && npm install

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@rm -rf pkg
	@rm -rf webapp/src/wasm
	@echo "Clean complete!"

# Show help
help:
	@echo "Available commands:"
	@echo "  make build    - Build WASM package and copy to webapp"
	@echo "  make dev      - Build and start development server"
	@echo "  make install  - Install webapp dependencies"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make help     - Show this help message"