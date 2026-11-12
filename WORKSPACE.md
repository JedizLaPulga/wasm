# Workspace Guide

This guide explains how to work with the WASM workspace containing multiple independent Dioxus applications.

## 📋 Table of Contents

- [Workspace Structure](#workspace-structure)
- [Running Applications](#running-applications)
- [Building Applications](#building-applications)
- [Adding New Applications](#adding-new-applications)
- [Shared Code](#shared-code)
- [Deployment](#deployment)
- [Troubleshooting](#troubleshooting)

## 🏗️ Workspace Structure

```
wasm/
├── Cargo.toml              # Workspace root - manages shared dependencies
├── Dioxus.toml             # Workspace Dioxus config (optional)
├── apps/                   # All independent applications
│   ├── dashboard/          # First independent app
│   ├── admin-panel/        # Second independent app
│   └── user-profile/       # Third independent app
├── shared/                 # Shared library used by all apps
│   └── src/
│       ├── lib.rs          # Main library exports
│       ├── components.rs   # Reusable components
│       └── utils.rs        # Utility functions
└── README.md               # Main project documentation
```

## 🎮 Running Applications

### Run a specific application

Each app runs independently with its own development server:

```bash
# Terminal 1: Dashboard
cd apps/dashboard
dx serve

# Terminal 2: Admin Panel (in another terminal)
cd apps/admin-panel
dx serve

# Terminal 3: User Profile (in another terminal)
cd apps/user-profile
dx serve
```

Each app will start on a different port (default: 8080, 8081, 8082, etc.).

### Run all apps simultaneously (using Cargo)

```bash
# Build all apps
cargo build --workspace

# Check/lint all code
cargo check --workspace

# Run tests for all apps
cargo test --workspace
```

## 📦 Building Applications

### Production build for a specific app

```bash
cd apps/dashboard
dx build --release
```

Output will be in `apps/dashboard/dist/`

### Production build for all apps

```bash
# Build each app
for app in apps/*/; do
  cd "$app"
  dx build --release
  cd ../../
done
```

### Using Cargo to build

```bash
# Build specific app with Cargo
cargo build --release -p dashboard

# Build all apps
cargo build --release -p dashboard -p admin-panel -p user-profile

# Build shared library
cargo build --release -p wasm-shared
```

## ➕ Adding New Applications

### Step 1: Create the app directory

```bash
mkdir -p apps/my-new-app/src
```

### Step 2: Create Cargo.toml

```toml
[package]
name = "my-new-app"
version.workspace = true
edition.workspace = true
authors.workspace = true
description = "My new application"
repository.workspace = true
license.workspace = true

[dependencies]
dioxus.workspace = true
dioxus-web.workspace = true
log.workspace = true
wasm-bindgen.workspace = true
wasm-shared.workspace = true

[dev-dependencies]
wasm-bindgen-test.workspace = true
```

### Step 3: Create src/main.rs

```rust
use dioxus::prelude::*;
use wasm_shared::components::*;

fn main() {
    dioxus_web::launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        div {
            Header {}
            
            div {
                style: "padding: 40px;",
                h1 { "My New App" }
                p { "Welcome to my application!" }
            }
            
            Footer {}
        }
    }
}
```

### Step 4: Create Dioxus.toml

```toml
[application]
name = "MyNewApp"
default_platform = "web"

[web.app]
title = "WASM - My New App"
```

### Step 5: Update root Cargo.toml

Add the new app to the workspace members:

```toml
[workspace]
members = [
    "apps/dashboard",
    "apps/admin-panel",
    "apps/user-profile",
    "apps/my-new-app",  # Add this line
]
```

### Step 6: Test the new app

```bash
cd apps/my-new-app
dx serve
```

## 🔗 Shared Code

### Using shared components

In any app's `src/main.rs`:

```rust
use wasm_shared::components::{Header, Footer, Button};

#[component]
fn app() -> Element {
    rsx! {
        div {
            Header {}
            
            Button {
                label: "Click me".to_string(),
                onclick: move |_| { /* handle click */ }
            }
            
            Footer {}
        }
    }
}
```

### Using shared utilities

```rust
use wasm_shared::utils::{format_version, get_author};

fn some_function() {
    println!("Version: {}", format_version());
    println!("Author: {}", get_author());
}
```

### Adding to shared library

Edit `shared/src/lib.rs`:

```rust
pub mod components;
pub mod utils;
pub mod new_module;  // Add new modules here
```

Create new files in `shared/src/`:

```bash
touch shared/src/new_module.rs
```

## 🚀 Deployment

### Deploy a single app

Each app in `dist/` is ready to be served:

```bash
# Build dashboard for production
cd apps/dashboard
dx build --release

# Copy dist/ to your server
scp -r dist/* user@server:/var/www/dashboard/
```

### Deploy all apps

```bash
# Build all
for app in apps/*/; do
  (cd "$app" && dx build --release)
done

# Deploy all
for app in apps/*/; do
  app_name=$(basename "$app")
  scp -r "$app/dist/*" user@server:/var/www/"$app_name"/
done
```

### Using Docker (optional)

Create `Dockerfile`:

```dockerfile
FROM rust:latest

WORKDIR /app

# Install Dioxus CLI
RUN cargo install dioxus-cli

COPY . .

# Build all apps
RUN for app in apps/*/; do (cd "$app" && dx build --release); done

# Serve with nginx or another server
```

## 🐛 Troubleshooting

### App won't start with `dx serve`

**Problem**: Port already in use

**Solution**: Specify a different port
```bash
dx serve --port 3000
```

### Compilation errors with shared library

**Problem**: `can't find crate for 'wasm_shared'`

**Solution**: Make sure the shared library is in the workspace members and dependencies are correct:
```bash
cargo build -p wasm-shared
```

### Changes to shared library not reflected

**Problem**: Cached build

**Solution**: Clean and rebuild
```bash
cargo clean
cargo build --workspace
```

### Module not found in shared library

**Problem**: `pub mod` added but not in `lib.rs`

**Solution**: Add the module to `shared/src/lib.rs`:
```rust
pub mod my_new_module;
```

### Different apps have conflicting versions

**Problem**: Dependency version conflicts

**Solution**: Update `[workspace.dependencies]` in root `Cargo.toml` to use same versions

## 📚 Best Practices

1. **Keep apps independent** - Each app should be deployable separately
2. **Use shared library wisely** - Only put truly shared code in `shared/`
3. **Avoid circular dependencies** - Apps should use shared, never the reverse
4. **Document your apps** - Add README.md in each app directory
5. **Test regularly** - Run `cargo test --workspace` frequently
6. **Update dependencies together** - Use workspace to keep versions in sync

## 🔗 Resources

- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Dioxus Documentation](https://dioxusjs.com/)
- [Dioxus CLI](https://github.com/DioxusLabs/cli)

---

For more information, see [README.md](./README.md)
