use dioxus::prelude::*;
use crate::data::PROJECTS;

#[component]
pub fn Projects() -> Element {
    rsx! {
        section { id: "projects", class: "section reveal-on-scroll",
            div { class: "container",
                h2 { "Projects" }
                div { class: "projects-list",
                    for project in PROJECTS.iter() {
                        {
                            let subtitle = match project.name {
                                "CareStream" => "Healthcare / Real-Time ICU Telemetry Engine",
                                "Just Listen" => "FastAPI / Asynchronous Audio Processing Platform",
                                _ => "Go & C++ / High-Frequency Trading Microservices",
                            };
                            rsx! {
                                div { class: "project-entry",
                                    div { class: "project-info-main",
                                        h3 { class: "project-entry-title", "{project.name}" }
                                        div { class: "project-entry-subtitle", "{subtitle}" }
                                        p { class: "project-entry-desc", "{project.description}" }

                                        div { class: "project-tech-list",
                                            for tech in project.tech_badges {
                                                span { class: "project-tech-badge", "{tech}" }
                                            }
                                        }

                                        div { class: "project-entry-links",
                                            a {
                                                class: "project-link",
                                                href: "{project.github_url}",
                                                target: "_blank",
                                                rel: "noopener noreferrer",
                                                "Source Code"
                                            }
                                            if let Some(live_url) = project.live_url {
                                                a {
                                                    class: "project-link",
                                                    href: "{live_url}",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    "Live Demo"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
