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
                h2 { "📄 Document Application" }
                p { "A Dioxus WASM application for document management and editing." }
                
                div {
                    style: "background: #f9f9f9; padding: 20px; border-radius: 8px; margin: 20px 0;",
                    h3 { "Documents" }
                    p { "Build your document features here." }
                }
            }
            
            Footer {}
        }
    }
}
