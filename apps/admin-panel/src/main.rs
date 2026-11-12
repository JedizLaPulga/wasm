use dioxus::prelude::*;
use wasm_shared::components::*;

fn main() {
    dioxus_web::launch(app);
}

#[component]
fn app() -> Element {
    let mut selected_user = use_signal(|| "User 1");
    let mut is_active = use_signal(|| true);

    rsx! {
        div {
            style: "font-family: Arial, sans-serif;",
            Header {}
            
            div {
                style: "padding: 40px; max-width: 800px; margin: 0 auto;",
                h2 { "🔐 Admin Panel Application" }
                p { "This is the Admin Panel app - completely independent from other apps." }
                
                div {
                    style: "background: #f9f9f9; padding: 20px; border-radius: 8px; margin: 20px 0;",
                    h3 { "User Management" }
                    p { "Selected User: {selected_user}" }
                    p { "Status: {if is_active() { \"Active\" } else { \"Inactive\" }}" }
                    
                    div {
                        Button {
                            label: "Select User 1".to_string(),
                            onclick: move |_| selected_user.set("User 1"),
                        }
                        Button {
                            label: "Select User 2".to_string(),
                            onclick: move |_| selected_user.set("User 2"),
                        }
                        Button {
                            label: "Toggle Status".to_string(),
                            onclick: move |_| is_active.toggle(),
                        }
                    }
                }
            }
            
            Footer {}
        }
    }
}
