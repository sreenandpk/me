use dioxus::prelude::*;
use crate::data::PERSONAL_INFO;

#[component]
pub fn Contact() -> Element {
    rsx! {
        section { id: "contact", class: "section reveal-on-scroll",
            div { class: "container",
                div { class: "contact-grid",
                    div { class: "contact-left-col",
                        div { class: "contact-giant-text",
                            span { "Let's build" }
                            br {}
                            span { "something together." }
                        }
                        div { class: "contact-subtext", "Available for engineering opportunities." }

                        div { class: "contact-links-grid",
                            a {
                                class: "contact-brutalist-link",
                                href: "mailto:{PERSONAL_INFO.email}",
                                "Email"
                            }
                            a {
                                class: "contact-brutalist-link",
                                href: "{PERSONAL_INFO.github_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "GitHub"
                            }
                            a {
                                class: "contact-brutalist-link",
                                href: "{PERSONAL_INFO.linkedin_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "LinkedIn"
                            }
                        }
                    }
                }
            }
        }
    }
}
