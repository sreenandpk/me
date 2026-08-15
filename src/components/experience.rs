use dioxus::prelude::*;
use crate::data::EXPERIENCE_ITEMS;

#[component]
pub fn ExperienceSection() -> Element {
    rsx! {
        section { id: "experience", class: "section reveal-on-scroll",
            div { class: "container",
                h2 { "Experience" }
                div { class: "experience-rows",
                    for exp in EXPERIENCE_ITEMS {
                        div { class: "experience-row",
                            // Left side: Period/Timeframe
                            div { class: "exp-year-col mono-text", "{exp.period}" }

                            // Right side: Title & Achievements
                            div { class: "exp-details-col",
                                div { class: "exp-header",
                                    h3 { class: "exp-title", "{exp.role}" }
                                    span { class: "exp-company", "{exp.company} • {exp.location}" }
                                }
                                ul { class: "exp-achievements",
                                    for ach in exp.achievements {
                                        li { "{ach}" }
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
