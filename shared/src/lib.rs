//! Shared library for common components and utilities across all Dioxus apps

pub mod components;
pub mod utils;

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
