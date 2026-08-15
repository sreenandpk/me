use dioxus::prelude::*;
use crate::data::PROJECTS;

#[component]
pub fn Projects() -> Element {
    rsx! {
        section { id: "projects", class: "section reveal-on-scroll",
            div { class: "container projects-container",
                div { class: "skills-header",
                    span { class: "skills-pretitle", "WHAT I'VE BUILT" }
                    h2 { class: "skills-headline", "FEATURED PROJECTS" }
                    div { class: "skills-divider" }
                }
                div { class: "experience-rows projects-wide-list",
                    for project in PROJECTS.iter() {
                        {
                            let subtitle = match project.name {
                                "CareStream" => "Healthcare / Real-Time ICU Telemetry Engine",
                                "Just Listen" => "FastAPI / Asynchronous Audio Processing Platform",
                                _ => "Go & C++ / High-Frequency Trading Microservices",
                            };
                            rsx! {
                                div { class: "experience-row project-wide-card",
                                    div { class: "project-info-main",
                                        div { class: "exp-header",
                                            h3 { class: "exp-title", "{project.name}" }
                                            span { class: "exp-company", "{subtitle}" }
                                        }
                                        p { class: "project-entry-desc", "{project.description}" }
                                        
                                        h4 { class: "project-highlights-title", "Challenges & Solutions" }
                                        ul { class: "exp-achievements project-highlights",
                                            for highlight in project.highlights {
                                                li { "{highlight}" }
                                            }
                                        }

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
