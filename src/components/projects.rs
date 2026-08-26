use dioxus::prelude::*;
use crate::data::PROJECTS;

#[component]
pub fn Projects() -> Element {
    rsx! {
        section { id: "real-projects-wrapper", class: "real-projects-wrapper section",
            div { class: "real-projects-inner",
                div { class: "container projects-container",
                    h2 { class: "real-projects-3d-title", "PROJECTS" }
                
                div { class: "real-projects-list",
                    for project in PROJECTS.iter() {
                        div { key: "{project.id}", class: "real-project-card",
                            // Card Header
                            div { class: "real-project-card-header",
                                div { class: "real-project-title-left",
                                    span { class: "real-project-id", "{project.id}" }
                                    div { class: "real-project-client-group",
                                        span { class: "real-project-client-label", "CLIENT" }
                                        span { class: "real-project-client-name", "{project.title}" }
                                    }
                                }
                                a { class: "real-project-live-btn", href: "{project.live_link}", target: "_blank", "LIVE PROJECT" }
                            }
                            
                            // Image Gallery
                            div { class: "real-project-gallery",
                                if project.images.len() >= 3 {
                                    img { class: "real-project-img img-main", src: "{project.images[0]}", alt: "Main Project View" }
                                    div { class: "real-project-subgrid",
                                        img { class: "real-project-img img-sub", src: "{project.images[1]}", alt: "Secondary Project View" }
                                        img { class: "real-project-img img-sub", src: "{project.images[2]}", alt: "Tertiary Project View" }
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
