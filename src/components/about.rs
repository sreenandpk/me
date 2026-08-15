use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        section { id: "about", class: "section reveal-on-scroll",
            div { class: "container",
                div { class: "about-hero",
                    // Pre-label (matches hero "HELLO, I AM")
                    span { class: "about-pretitle", "ABOUT ME" }

                    // Big statement (matches hero name style)
                    h2 { class: "about-headline",
                        "I BUILD SOFTWARE"
                        br {}
                        "DESIGNED TO LAST."
                    }

                    // Subtitle (matches hero role style)
                    p { class: "about-role",
                        "Python Developer & Backend Engineer"
                    }

                    // Divider line
                    div { class: "about-divider" }

                    // Bio paragraphs
                    div { class: "about-bio",
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
