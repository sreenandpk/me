use dioxus::prelude::*;
use crate::data::EXPERIENCE_ITEMS;

#[component]
pub fn ExperienceSection() -> Element {
    rsx! {
        section { id: "experience", class: "projects-white-section blur-on-enter",
            div { class: "container projects-container",
                h2 { class: "projects-massive-title", "EXPERIENCE" }
                
                div { class: "projects-list",
                    for (idx, exp) in EXPERIENCE_ITEMS.iter().enumerate() {
                        {
                            let formatted_num = format!("{:02}", idx + 1);
                            rsx! {
                                div { key: "{idx}", class: "projects-list-item",
                                    div { class: "project-list-num", "{formatted_num}" }
                                    div { class: "project-list-details",
                                        h3 { class: "project-list-title", "{exp.role}" }
                                        p { class: "project-list-desc",
                                            span { style: "font-weight: 700; color: #1e293b;", "{exp.company}" }
                                            " • {exp.period} • {exp.location}"
                                        }
                                        ul {
                                            style: "margin-top: 1rem; padding-left: 1.2rem; list-style-type: disc; color: #475569; display: flex; flex-direction: column; gap: 0.5rem;",
                                            for ach in exp.achievements {
                                                li {
                                                    style: "font-family: var(--font-body); font-size: 0.95rem; line-height: 1.55;",
                                                    "{ach}"
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
