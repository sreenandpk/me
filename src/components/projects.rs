use dioxus::prelude::*;
use crate::data::PROJECTS;

#[component]
pub fn Projects() -> Element {
    rsx! {
        section { id: "projects", class: "section reveal-on-scroll",
            div { class: "container",
                
                // Centered hero-matched heading
                div { class: "projects-header",
                    span { class: "projects-pretitle", "CASE STUDIES" }
                    h2 { class: "projects-headline", "SELECTED WORK" }
                    div { class: "projects-divider" }
                }

                div { class: "projects-showcase-list",
                    for (index, project) in PROJECTS.iter().enumerate() {
                        div { class: "project-showcase-row",
                            // Left side: Big Typography and Tech Stack
                            div { class: "project-showcase-header",
                                h3 { class: "project-title", "{project.name}" }
                                p { class: "project-subtitle", "{project.subtitle}" }
                                
                                div { class: "project-tech-inline",
                                    for tech in project.tech_badges {
                                        {
                                            let tech_str = *tech;
                                            rsx! { span { class: "tech-item", "{tech_str}" } }
                                        }
                                    }
                                }

                                div { class: "project-links-minimal",
                                    a {
                                        class: "project-link-minimal",
                                        href: "{project.github_url}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        "VIEW SOURCE"
                                    }
                                    if let Some(live_url) = project.live_url {
                                        a {
                                            class: "project-link-minimal",
                                            href: "{live_url}",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            "LIVE DEMO"
                                        }
                                    }
                                }
                            }

                            // Right side: Editorial text explaining the project
                            div { class: "project-showcase-story",
                                div { class: "story-block",
                                    p { class: "story-text overview-text", "{project.overview}" }
                                }
                                div { class: "story-block",
                                    span { class: "story-label", "THE CHALLENGE" }
                                    p { class: "story-text", "{project.problem_faced}" }
                                }
                                div { class: "story-block",
                                    span { class: "story-label", "THE SOLUTION" }
                                    p { class: "story-text", "{project.solution_implemented}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

