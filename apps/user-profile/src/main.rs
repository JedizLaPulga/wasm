use dioxus::prelude::*;
use wasm_shared::components::*;

fn main() {
    dioxus::LaunchBuilder::new().launch(app);
}

#[component]
fn app() -> Element {
    let mut name = use_signal(|| "John Doe");
    let mut email = use_signal(|| "john@example.com");
    let mut notifications_enabled = use_signal(|| true);

    rsx! {
        div {
            style: "font-family: Arial, sans-serif;",
            Header {}
            
            div {
                style: "padding: 40px; max-width: 800px; margin: 0 auto;",
                h2 { "👤 User Profile Application" }
                p { "This is the User Profile app - runs independently with its own state." }
                
                div {
                    style: "background: #f9f9f9; padding: 20px; border-radius: 8px; margin: 20px 0;",
                    h3 { "Profile Information" }
                    
                    div {
                        style: "margin: 15px 0;",
                        p { "Name: {name}" }
                        p { "Email: {email}" }
                        p {
                            "Notifications: "
                            if notifications_enabled() {
                                "Enabled"
                            } else {
                                "Disabled"
                            }
                        }
                    }
                    
                    div {
                        Button {
                            label: "Change Name".to_string(),
                            onclick: move |_| name.set("Jane Smith"),
                        }
                        Button {
                            label: "Change Email".to_string(),
                            onclick: move |_| email.set("jane@example.com"),
                        }
                        Button {
                            label: "Toggle Notifications".to_string(),
                            onclick: move |_| notifications_enabled.toggle(),
                        }
                    }
                }
            }
            
            Footer {}
        }
    }
}
