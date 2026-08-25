use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        section { id: "about", class: "section reveal-on-scroll",
            div { class: "container",
                div { class: "about-split-layout",
                    div { class: "about-heading-left",
                        h2 { class: "about-huge-title-left", "ABOUT ME" }
                    }
                    
                    div { class: "about-content-right",
                        div { class: "about-bio-text",
                            p {
                                "I’m a Full Stack Developer working on both sides of the screen — from the experience people see to everything working behind it. I turn ideas into simple, reliable applications built to last."
                            }
                            p {
                                "My approach is simple: understand the problem, plan the work, build it, and improve it as I go. I pay attention to the details, and try to keep the final solution clear and easy to maintain."
                            }
                        }
                        
                        div { class: "about-cta-left",
                            a { class: "btn btn-primary", href: "#contact", "CONTACT ME" }
                        }
                    }
                }
            }
        }
    }
}
