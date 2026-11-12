# WASM Project - Multiple Independent Dioxus Applications

A workspace of multiple independent WebAssembly applications built with [Dioxus](https://dioxusjs.com/), a reactive UI framework for Rust.

## 🎯 Overview

This project demonstrates a **workspace-based architecture** for building multiple independent WASM applications using Rust and Dioxus. Each app runs independently with its own bundle, deployment cycle, and state management.

### Applications Included

- **Dashboard** (`apps/dashboard/`) - Analytics and metrics display
- **Admin Panel** (`apps/admin-panel/`) - User and system management
- **User Profile** (`apps/user-profile/`) - User profile and settings

### Shared Library

- **wasm-shared** (`shared/`) - Reusable components and utilities across all apps

## 🚀 Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (optional, for tooling)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/JedizLaPulga/wasm.git
cd wasm
```

2. Install Dioxus CLI:
```bash
cargo install dioxus-cli
```

## 🏗️ Project Structure

```
wasm/
├── Cargo.toml                 # Workspace root configuration
├── apps/
│   ├── dashboard/
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   └── Dioxus.toml
│   ├── admin-panel/
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   └── Dioxus.toml
│   └── user-profile/
│       ├── Cargo.toml
│       ├── src/main.rs
│       └── Dioxus.toml
├── shared/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── components.rs
│       └── utils.rs
├── README.md
├── CHANGELOG.md
└── CONTRIBUTING.md
```

## 🎮 Running Applications

### Run a specific app

```bash
# Dashboard
cd apps/dashboard
dx serve

# Admin Panel
cd apps/admin-panel
dx serve

# User Profile
cd apps/user-profile
dx serve
```

### Build specific app for production

```bash
cd apps/dashboard
dx build --release
```

The built artifacts will be in `dist/` within each app directory.

### Build all apps

```bash
cargo build --release -p dashboard -p admin-panel -p user-profile
```

## 📚 Documentation

- [Dioxus Official Docs](https://dioxusjs.com/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [WebAssembly](https://webassembly.org/)
- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)

## 🧪 Testing

Run tests for all workspace members:
```bash
cargo test --workspace
```

Run tests for a specific app:
```bash
cargo test -p dashboard
```

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## 💡 Architecture Benefits

✅ **Independent Deployment** - Deploy each app on its own schedule  
✅ **Smaller Bundles** - Only code needed for each app is compiled  
✅ **Clear Separation** - Each app has distinct concerns and boundaries  
✅ **Shared Components** - Common UI elements and utilities in `shared/`  
✅ **Scalability** - Easy to add new apps to the workspace  
✅ **Unified Tooling** - Single Cargo workspace for dependency management  

## 📝 Adding New Applications

To add a new app to the workspace:

1. Create app directory: `apps/my-new-app`
2. Add Cargo.toml with workspace dependency references
3. Add src/main.rs with Dioxus app
4. Update root Cargo.toml members list
5. Create Dioxus.toml for CLI configuration

## 🔗 Resources

- [Dioxus GitHub](https://github.com/DioxusLabs/dioxus)
- [Dioxus Discord Community](https://discord.gg/XgGxMSkvzm)
- [Cargo Workspaces Guide](https://doc.rust-lang.org/cargo/reference/workspaces.html)

---

Made with ❤️ using Rust, Dioxus, and WASM
