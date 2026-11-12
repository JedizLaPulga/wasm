use dioxus::prelude::*;
use wasm_shared::components::*;

fn main() {
    dioxus::LaunchBuilder::new().launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        div {
            style: "font-family: Arial, sans-serif;",
            Header {}
            
            div {
                style: "padding: 40px; max-width: 800px; margin: 0 auto;",
                h2 { "✓ Task Application" }
                p { "A Dioxus WASM application for task management and tracking." }
                
                div {
                    style: "background: #f9f9f9; padding: 20px; border-radius: 8px; margin: 20px 0;",
                    h3 { "Tasks" }
                    p { "Build your task features here." }
                }
            }
            
            Footer {}
        }
    }
}
