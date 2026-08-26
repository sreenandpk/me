use dioxus::prelude::*;
use crate::data::SERVICES;

#[component]
pub fn Services() -> Element {
    rsx! {
        section { id: "projects", class: "projects-white-section section",
            div { class: "container projects-container",
                h2 { class: "projects-massive-title", "SERVICES" }
                
                div { class: "projects-list",
                    for (project_idx, project) in SERVICES.iter().enumerate() {
                        {
                            let formatted_num = format!("{:02}", project_idx + 1);
                            rsx! {
                                div { key: "{project_idx}", class: "projects-list-item",
                                    div { class: "project-list-num", "{formatted_num}" }
                                    div { class: "project-list-details",
                                        h3 { class: "project-list-title", "{project.name}" }
                                        p { class: "project-list-desc", "{project.subtitle}" }
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

