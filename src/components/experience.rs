use dioxus::prelude::*;
use crate::data::EXPERIENCE_ITEMS;

#[component]
pub fn ExperienceSection() -> Element {
    rsx! {
        section { id: "experience", class: "section reveal-on-scroll",
            div { class: "container",
                div { class: "experience-grid-layout",
                    // Left Column: Heading
                    div { class: "experience-heading-col",
                        span { class: "experience-pretitle", "MY JOURNEY" }
                        h2 { class: "experience-headline", "Experience" }
                        p { class: "experience-subtitle", "Professional timeline and key milestones." }
                        div { class: "experience-divider" }
                    }

                    // Right Column: Experience Items
                    div { class: "experience-list-col",
                        div { class: "experience-minimal-list",
                            for exp in EXPERIENCE_ITEMS {
                                div { class: "experience-minimal-item",
                                    div { class: "exp-minimal-timeframe", "{exp.period}" }
                                    h3 { class: "exp-minimal-role", "{exp.role}" }
                                    div { class: "exp-minimal-company", "{exp.company} • {exp.location}" }
                                    ul { class: "exp-minimal-achievements",
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
}
