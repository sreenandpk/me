use dioxus::prelude::*;
use crate::data::SKILL_CATEGORIES;

#[component]
pub fn Skills() -> Element {
    rsx! {
        section { id: "skills", class: "skills-white-section blur-on-enter",
            div { class: "container projects-container",
                h2 { class: "skills-massive-title", "SKILLS" }
                
                div { class: "skills-grid-modern",
                    for category in SKILL_CATEGORIES {
                        div { class: "skill-category-card",
                            h3 { class: "skill-card-title",
                                span { class: "skill-icon-emoji", "{category.icon}" }
                                "{category.name}"
                            }
                            div { class: "skill-pill-container",
                                for skill in category.skills {
                                    div { class: "skill-pill",
                                        i { class: "{skill.icon_class}" }
                                        span { "{skill.name}" }
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
