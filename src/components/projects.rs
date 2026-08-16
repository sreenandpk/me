use dioxus::prelude::*;
use crate::data::PROJECTS;

#[component]
pub fn Projects() -> Element {
    rsx! {
        section { id: "projects", class: "section reveal-on-scroll",
            div { class: "container",
                div { class: "projects-grid-layout",
                    // Left Column: Heading
                    div { class: "projects-heading-col",
                        span { class: "projects-pretitle", "CASE STUDIES" }
                        h2 { class: "projects-headline", "Selected projects" }
                        p { class: "projects-subtitle", "Handcrafted backend architectures and microservices." }
                        div { class: "projects-divider" }
                    }

                    // Right Column: Projects List
                    div { class: "projects-list-col",
                        div { class: "projects-minimal-list",
                            for project in PROJECTS.iter() {
                                div { class: "project-minimal-card",
                                    div { class: "project-info-main",
                                        h3 { class: "project-minimal-title", "{project.name}" }
                                        div { class: "project-minimal-subtitle", "{project.subtitle}" }
                                        p { class: "project-minimal-desc", "{project.overview}" }
                                        
                                        // Challenge & Solution details
                                        div { class: "project-minimal-details",
                                            div { class: "detail-minimal-block",
                                                span { class: "detail-minimal-label", "THE CHALLENGE" }
                                                p { class: "detail-minimal-text", "{project.problem_faced}" }
                                            }
                                            div { class: "detail-minimal-block",
                                                span { class: "detail-minimal-label", "THE SOLUTION" }
                                                p { class: "detail-minimal-text", "{project.solution_implemented}" }
                                            }
                                        }

                                        // Tech tags
                                        div { class: "project-minimal-tech",
                                            for tech in project.tech_badges {
                                                {
                                                    let tech_str = *tech;
                                                    rsx! { span { class: "tech-minimal-item", "{tech_str}" } }
                                                }
                                            }
                                        }

                                        // Links
                                        div { class: "project-minimal-links",
                                            a {
                                                class: "project-link-minimal-btn",
                                                href: "{project.github_url}",
                                                target: "_blank",
                                                rel: "noopener noreferrer",
                                                "VIEW SOURCE"
                                            }
                                            if let Some(live_url) = project.live_url {
                                                a {
                                                    class: "project-link-minimal-btn",
                                                    href: "{live_url}",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    "LIVE DEMO"
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

