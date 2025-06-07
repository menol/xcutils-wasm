# XCUtils WASM

A cross-platform utility for converting data structures between different programming languages (Dart, Kotlin, Swift, TypeScript) with WebAssembly support.

## Features

- 🚀 **Multi-language Support**: Convert between Dart, Kotlin, Swift, and TypeScript
- 🌐 **WebAssembly**: High-performance conversion in the browser
- 📱 **Web Interface**: User-friendly web application for easy conversion
- 🛠️ **Protocol Buffers**: Support for protobuf schema conversion

## Live Demo

🌐 **[Try it online](https://menol.github.io/xcutils-wasm/)**

## Project Structure

```
├── src/                    # Rust source code
│   ├── converter/         # Language converters
│   │   ├── dart.rs       # Dart converter
│   │   ├── kt.rs         # Kotlin converter
│   │   ├── swift.rs      # Swift converter
│   │   └── typescript.rs # TypeScript converter
│   └── lib.rs            # Main library
├── webapp/               # Web application
│   ├── src/             # React/Preact source
│   ├── index.html       # Main HTML file
│   └── package.json     # Node.js dependencies
├── build.ps1            # Windows build script
├── build.sh             # Unix build script
├── Makefile             # Make build targets
└── BUILD.md             # Build instructions
```

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/) (v16 or later)

### Build and Run

1. **Clone the repository**:
   ```bash
   git clone https://github.com/menol/xcutils-wasm.git
   cd xcutils-wasm
   ```

2. **Build WASM and start development server**:
   
   **Windows (PowerShell)**:
   ```powershell
   .\build.ps1
   ```
   
   **Unix/Linux/macOS**:
   ```bash
   chmod +x build.sh
   ./build.sh
   ```
   
   **Using Make**:
   ```bash
   make dev
   ```

3. **Open your browser** and navigate to `http://localhost:5173`

## Usage

1. Select the source language from the dropdown
2. Select the target language
3. Paste your code or data structure
4. Click "Convert" to see the result
5. Copy the converted code

## Supported Conversions

| From → To | Dart | Kotlin | Swift | TypeScript |
|-----------|------|--------|-------|------------|
| **Dart** | ✅ | ✅ | ✅ | ✅ |
| **Kotlin** | ✅ | ✅ | ✅ | ✅ |
| **Swift** | ✅ | ✅ | ✅ | ✅ |
| **TypeScript** | ✅ | ✅ | ✅ | ✅ |

## Development

### Manual Build Steps

1. **Build WASM package**:
   ```bash
   wasm-pack build --target web --out-dir pkg
   ```

2. **Copy WASM files to webapp**:
   ```bash
   cp pkg/*.{js,wasm,ts} webapp/src/wasm/
   cp pkg/package.json webapp/src/wasm/
   ```

3. **Install webapp dependencies**:
   ```bash
   cd webapp
   npm install
   ```

4. **Start development server**:
   ```bash
   npm run dev
   ```

### Build for Production

```bash
# Build WASM
make build

# Build webapp
cd webapp
npm run build
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and [WebAssembly](https://webassembly.org/)
- Web interface powered by [Preact](https://preactjs.com/) and [Vite](https://vitejs.dev/)
- UI components from [Material-UI](https://mui.com/)