use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

#[component]
fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            style: "text-align: center; font-family: Arial, sans-serif;",
            h1 { "Welcome to Dioxus WASM" }
            p { "This is a simple counter app built with Dioxus and Rust" }
            
            div {
                style: "margin: 20px; padding: 20px; background: #f0f0f0; border-radius: 8px;",
                p { "Counter: {count}" }
                button {
                    onclick: move |_| count += 1,
                    style: "padding: 10px 20px; font-size: 16px; cursor: pointer; margin: 5px;",
                    "Increment"
                }
                button {
                    onclick: move |_| count -= 1,
                    style: "padding: 10px 20px; font-size: 16px; cursor: pointer; margin: 5px;",
                    "Decrement"
                }
                button {
                    onclick: move |_| count.set(0),
                    style: "padding: 10px 20px; font-size: 16px; cursor: pointer; margin: 5px;",
                    "Reset"
                }
            }
        }
    }
}
