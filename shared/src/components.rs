//! Shared reusable components

use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            style: "background: #333; color: white; padding: 20px; text-align: center;",
            h1 { "WASM Application" }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            style: "background: #f0f0f0; padding: 20px; text-align: center; margin-top: 40px;",
            p { "© 2026 WASM Project. Built with Dioxus & Rust" }
        }
    }
}

#[component]
pub fn Button(
    label: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            style: "padding: 10px 20px; font-size: 16px; cursor: pointer; margin: 5px; border-radius: 4px; border: 1px solid #ccc;",
            onclick: onclick,
            "{label}"
        }
    }
}
