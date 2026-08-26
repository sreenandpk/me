use dioxus::prelude::*;
use crate::data::PROJECTS;

#[component]
pub fn Projects() -> Element {
    rsx! {
        section { id: "real-projects", class: "real-projects-section",
            div { class: "container projects-container",
                h2 { class: "real-projects-3d-title", "PROJECTS" }
                
                div { class: "real-projects-list",
                    for project in PROJECTS.iter() {
                        div { key: "{project.id}", class: "real-project-card",
                            // Card Header
                            div { class: "real-project-card-header",
                                div { class: "real-project-card-title-group",
                                    span { class: "real-project-id", "{project.id}" }
                                    span { class: "real-project-client", "{project.title} - {project.client}" }
                                }
                                a { class: "real-project-live-btn", href: "{project.live_link}", target: "_blank", "LIVE PROJECT" }
                            }
                            
                            // Image Gallery
                            div { class: "real-project-gallery",
                                for (img_idx, img_src) in project.images.iter().enumerate() {
                                    img { key: "{img_idx}", class: "real-project-img", src: "{img_src}", alt: "{project.title} screenshot" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
