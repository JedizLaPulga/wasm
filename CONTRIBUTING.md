# Contributing to WASM

Thank you for your interest in contributing to this project! Here are some guidelines to help you get started.

## Getting Started

1. Fork the repository
2. Clone your fork locally:
   ```bash
   git clone https://github.com/<your-username>/wasm.git
   cd wasm
   ```

3. Create a new branch for your feature or fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install Dioxus CLI:
   ```bash
   cargo install dioxus-cli
   ```

3. Run the development server:
   ```bash
   dx serve
   ```

## Code Guidelines

- Follow Rust naming conventions and style
- Run `cargo fmt` to format your code
- Run `cargo clippy` to check for common mistakes
- Add tests for new features
- Update documentation as needed

## Making Changes

1. Make your changes in your feature branch
2. Test your changes:
   ```bash
   cargo test
   dx build --release
   ```

3. Commit your changes with clear, descriptive messages:
   ```bash
   git commit -m "Add feature: description of your changes"
   ```

4. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

5. Create a Pull Request on GitHub

## Pull Request Guidelines

- Provide a clear description of what your PR does
- Reference any related issues
- Ensure all tests pass
- Keep commits clean and logically separated
- Be open to feedback and suggestions

## Code Review Process

- At least one maintainer will review your PR
- We may ask for changes or improvements
- Once approved, your PR will be merged

## Reporting Issues

If you find a bug or have a suggestion, please:

1. Check if the issue already exists
2. Provide a clear description
3. Include steps to reproduce (for bugs)
4. Include your environment details (OS, Rust version, etc.)

## Questions?

Feel free to open an issue or reach out to the maintainers.

Thank you for contributing! 🎉
