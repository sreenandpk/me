use dioxus::prelude::*;
use crate::data::PHILOSOPHY_ITEMS;

#[component]
pub fn Philosophy() -> Element {
    let get_desc = |title: &str| -> &'static str {
        match title {
            "Maintainability" => "Code is read far more often than it is written. I prioritize explicit structures over clever hacks, keeping dependency footprints small, and designing decoupled architectural layers so that system expansion is a low-friction operation.",
            "Testing" => "Test suites are the core validation safety net for distributed web services. I write exhaustive unit checks for model inputs and mock API networks, aiming to catch errors before they reach git branches.",
            "Fault Tolerance" => "Remote services drop connections; database queries queue up; APIs rate limit. I design system components that fail gracefully, incorporating retry backoffs, connection timeouts, and backup log pools.",
            "Automation" => "If an operation must be performed twice, automate it. I write container configurations, target checkers, and build commands to eliminate manual deploy mistakes and speed up release feedback loops.",
            // Observability
            _ => "A running backend is a black box without instrumentation. I prioritize structured JSON logs, database transaction tracing, and performance profiling indicators to locate bugs and bottlenecks."
        }
    };

    rsx! {
        section { id: "philosophy", class: "section reveal-on-scroll",
            div { class: "container",
                div { class: "principles-grid-layout",
                    // Left Column: Heading
                    div { class: "principles-heading-col",
                        span { class: "principles-pretitle", "HOW I WORK" }
                        h2 { class: "principles-headline", "Principles" }
                        p { class: "principles-subtitle", "Core guidelines I follow in engineering." }
                        div { class: "principles-divider" }
                    }

                    // Right Column: Principles list
                    div { class: "principles-list-col",
                        div { class: "principles-minimal-list",
                            for (i, item) in PHILOSOPHY_ITEMS.iter().enumerate() {
                                {
                                    let num = format!("{:02}", i + 1);
                                    let desc = get_desc(item.title);
                                    rsx! {
                                        div { class: "principles-minimal-item",
                                            div { class: "principle-minimal-header",
                                                span { class: "principle-minimal-num", "{num}" }
                                                span { class: "principle-minimal-divider", "/" }
                                                h3 { class: "principle-minimal-title", "{item.title}" }
                                            }
                                            p { class: "principle-minimal-desc", "{desc}" }
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
