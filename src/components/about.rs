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
                            "I build software"
                            br {}
                            "designed to last."
                        }
                        p { class: "about-role", "Python Developer & Backend Engineer" }
                        div { class: "about-divider" }
                    }

                    // Right Column: Bio paragraphs
                    div { class: "about-bio-col",
                        p {
                            "I am a software engineer focused on building clean, stable, and high-performance backend systems. I specialize in the Python ecosystem—specifically FastAPI and Django—and extend my curiosity into Rust and cloud deployment platforms."
                        }
                        p {
                            "My engineering philosophy centers on architectural reliability: code that is not only functional, but also maintainable, thoroughly tested, and secure from day one."
                        }
                    }
                }
            }
        }
    }
}
