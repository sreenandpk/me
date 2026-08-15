use dioxus::prelude::*;
use crate::data::SKILL_CATEGORIES;

#[component]
pub fn Skills() -> Element {
    rsx! {
        section { id: "skills", class: "section reveal-on-scroll",
            div { class: "container",
                // Centered hero-matched heading
                div { class: "skills-header",
                    span { class: "skills-pretitle", "WHAT I WORK WITH" }
                    h2 { class: "skills-headline", "SKILLS & TOOLS" }
                    div { class: "skills-divider" }
                }
                div { class: "skills-matrix",
                    for category in SKILL_CATEGORIES {
                        div { class: "skills-row",
                            // Left side: Category Header
                            h3 { class: "skills-category-title", "{category.name}" }

                            // Right side: Active details list
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
