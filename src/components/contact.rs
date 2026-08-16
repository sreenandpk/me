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
                                href: "{PERSONAL_INFO.leetcode_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "LeetCode"
                            }
                            a {
                                class: "contact-brutalist-link",
                                href: "{PERSONAL_INFO.linkedin_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "LinkedIn"
                            }
                            a {
                                class: "contact-brutalist-link",
                                href: "{PERSONAL_INFO.github_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "GitHub"
                            }
                        }
                    }
                }
            }
        }
    }
}
