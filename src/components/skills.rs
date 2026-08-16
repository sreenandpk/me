use dioxus::prelude::*;
use crate::data::SKILL_CATEGORIES;

#[component]
pub fn Skills() -> Element {
    rsx! {
        section { id: "skills", class: "section reveal-on-scroll",
            div { class: "container",
                div { class: "skills-grid-layout",
                    // Left Column: Heading
                    div { class: "skills-heading-col",
                        span { class: "skills-pretitle", "WHAT I WORK WITH" }
                        h2 { class: "skills-headline", "Skills & tools" }
                        p { class: "skills-subtitle", "Technologies I use to build robust applications." }
                        div { class: "skills-divider" }
                    }

                    // Right Column: Skills Matrix
                    div { class: "skills-matrix-col",
                        div { class: "skills-matrix",
                            for category in SKILL_CATEGORIES {
                                div { class: "skills-row",
                                    // Category Header
                                    h3 { class: "skills-category-title", "{category.name}" }

                                    // Active details list
                                    div { class: "skills-matrix-list",
                                        for skill in category.skills {
                                            {
                                                let skill_str = *skill;
                                                rsx! {
                                                    span { class: "skill-tag", "{skill_str}" }
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
