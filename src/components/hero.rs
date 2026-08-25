use dioxus::prelude::*;
use crate::data::PERSONAL_INFO;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { id: "home", class: "hero-section",
            div { class: "container",
                div { class: "hero-grid",
                    div { class: "hero-content reveal-on-scroll revealed",
                        h1 { class: "hero-name", "HI, I'M SREENAND P K" }
                        h2 { class: "hero-headline",
                            span { style: "position: relative; display: inline-block;",
                                "{PERSONAL_INFO.title}"
                                svg {
                                    class: "hero-headline-underline",
                                    view_box: "0 0 300 12",
                                    preserve_aspect_ratio: "none",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "3",
                                    stroke_linecap: "round",
                                    path { d: "M5 9 C 90 12, 180 5, 295 8" }
                                }
                            }
                        }
                        p { class: "hero-description",
                            "I’m a Full Stack Developer working on both sides of the screen — from the experience people see to everything working behind it. I turn ideas into simple, reliable applications built to last."
                        }

                        // CTAs
                        div { class: "hero-ctas",
                            a { class: "btn btn-primary", href: "#projects", "View Projects" }
                            a { class: "btn btn-secondary", href: "#contact", "Get In Touch" }
                        }

                        // Monospace status footer details
                        div { class: "hero-footer-info",
                            span { "Based in Kerala, India" }
                            span { class: "hero-bullet", "•" }
                            span { "Open to Full-Time Roles" }
                            span { class: "hero-bullet", "•" }
                            a {
                                href: "{PERSONAL_INFO.github_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "GitHub"
                            }
                            span { class: "hero-bullet", "•" }
                            a {
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
