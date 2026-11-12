use dioxus::prelude::*;
use wasm_shared::components::*;

fn main() {
    dioxus_web::launch(app);
}

#[component]
fn app() -> Element {
    let mut counter = use_signal(|| 0);

    rsx! {
        div {
            style: "font-family: Arial, sans-serif;",
            Header {}
            
            div {
                style: "padding: 40px; max-width: 800px; margin: 0 auto;",
                h2 { "📊 Dashboard Application" }
                p { "This is the Dashboard app - one of multiple independent Dioxus WASM applications." }
                
                div {
                    style: "background: #f9f9f9; padding: 20px; border-radius: 8px; margin: 20px 0;",
                    h3 { "Dashboard Stats" }
                    p { "Counter Value: {counter}" }
                    
                    div {
                        Button {
                            label: "Increment".to_string(),
                            onclick: move |_| counter += 1,
                        }
                        Button {
                            label: "Decrement".to_string(),
                            onclick: move |_| counter -= 1,
                        }
                        Button {
                            label: "Reset".to_string(),
                            onclick: move |_| counter.set(0),
                        }
                    }
                }
            }
            
            Footer {}
        }
    }
}
