use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer-section",
            div { class: "container",
                div { class: "footer-spacer" }
            }
        }
    }
}
