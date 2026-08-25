use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        section { id: "about", class: "section reveal-on-scroll",
            div { class: "container",
                div { class: "about-centered-layout",
                    h2 { class: "about-huge-title", "ABOUT ME" }
                    
                    div { class: "about-centered-bio",
                        p {
                            "I’m a Full Stack Developer working on both sides of the screen —"
                            br {}
                            "from the experience people see to everything working behind it."
                            br {}
                            "I turn ideas into simple, reliable applications built to last."
                        }
                        br {}
                        p {
                            "My approach is simple: understand the problem, plan the work,"
                            br {}
                            "build it, and improve it as I go. I pay attention to the details,"
                            br {}
                            "and try to keep the final solution clear and easy to maintain."
                        }
                    }
                    
                    div { class: "about-cta-container",
                        a { class: "btn btn-primary", href: "#contact", "CONTACT ME" }
                    }
                }
            }
        }
    }
}
