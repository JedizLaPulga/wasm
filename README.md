# WASM Project with Dioxus

A WebAssembly project built with [Dioxus](https://dioxusjs.com/), a reactive UI framework for Rust.

## 🎯 Overview

This project demonstrates building interactive web applications using Rust and Dioxus, compiled to WebAssembly (WASM). Dioxus provides a React-like experience with Rust's performance and safety guarantees.

## 🚀 Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (optional, for tooling)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/<your-username>/wasm.git
cd wasm
```

2. Install Dioxus CLI:
```bash
cargo install dioxus-cli
```

3. Run the development server:
```bash
dx serve
```

The application will be available at `http://localhost:8080`

## 📦 Building

### Development Build
```bash
dx serve
```

### Production Build
```bash
dx build --release
```

The built artifacts will be in the `dist/` directory.

## 🏗️ Project Structure

```
wasm/
├── src/
│   ├── main.rs          # Application entry point
│   └── lib.rs           # Library code
├── Cargo.toml           # Project manifest
├── Dioxus.toml          # Dioxus configuration
├── README.md            # This file
├── CHANGELOG.md         # Version history
└── .gitignore           # Git ignore rules
```

## 📚 Documentation

- [Dioxus Official Docs](https://dioxusjs.com/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [WebAssembly](https://webassembly.org/)

## 🧪 Testing

Run tests with:
```bash
cargo test
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 📝 Notes

- WASM is optimal for computationally intensive tasks and interactive UIs
- Dioxus provides hot-reload during development via `dx serve`
- For production, ensure you optimize bundle size and enable minification

## 🔗 Resources

- [Dioxus GitHub](https://github.com/DioxusLabs/dioxus)
- [Dioxus Discord Community](https://discord.gg/XgGxMSkvzm)

---

Made with ❤️ using Rust and Dioxus
