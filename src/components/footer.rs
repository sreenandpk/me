use dioxus::prelude::*;
use crate::data::PERSONAL_INFO;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer-section",
            div { class: "container",
                div { class: "footer-grid",
                    // Left Column: Credentials
                    div { class: "footer-col-left",
                        div { class: "footer-name", "{PERSONAL_INFO.name}" }
                        div { class: "footer-title", "{PERSONAL_INFO.title}" }
                    }

                    // Right Column: Links & Build stamp
                    div { class: "footer-col-right",
                        div { class: "footer-links",
                            a {
                                class: "footer-link",
                                href: "{PERSONAL_INFO.github_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "GitHub"
                            }
                            a {
                                class: "footer-link",
                                href: "{PERSONAL_INFO.linkedin_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "LinkedIn"
                            }
                        }
                        div { class: "footer-built", "© 2026. Built with Rust, Dioxus, and WebAssembly." }
                    }
                }
            }
        }
    }
}
