use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        section { id: "about", class: "section reveal-on-scroll",
            div { class: "container",
                div { class: "about-grid-layout",
                    // Left Column: Heading
                    div { class: "about-heading-col",
                        span { class: "about-pretitle", "ABOUT ME" }
                        h2 { class: "about-headline",
                            "I build things"
                            br {}
                            "people actually use."
                        }
                        p { class: "about-role", "Full Stack Developer" }
                        div { class: "about-divider" }
                    }

                    // Right Column: Bio paragraphs
                    div { class: "about-bio-col",
                        p {
                            "I’m a Full Stack Developer working on both sides of the screen — from the experience people see to everything working behind it. I turn ideas into simple, reliable applications built to last."
                        }
                        p {
                            "My approach is simple: understand the problem, plan the work, build it, and improve it as I go. I pay attention to the details, test what I build, and try to keep the final solution clear and easy to maintain."
                        }
                    }
                }
            }
        }
    }
}
