use dioxus::prelude::*;
use crate::data::PROJECTS;

#[component]
pub fn Projects() -> Element {
    rsx! {
        section { id: "projects", class: "section reveal-on-scroll",
            div { class: "container",
                div { class: "projects-heading-col",
                    span { class: "projects-pretitle", "CASE STUDIES" }
                    h2 { class: "projects-headline", "Selected projects" }
                    p { class: "projects-subtitle", "Interactive gallery showcase of full-stack systems and cloud applications." }
                    div { class: "projects-divider" }
                }

                // Album Gallery 2-Column Grid
                div { class: "projects-album-grid",
                    for (project_idx, project) in PROJECTS.iter().enumerate() {
                        ProjectAlbumCard {
                            key: "{project_idx}",
                            project: project,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectAlbumCard(project: &'static crate::data::projects::Project) -> Element {
    let mut selected_slide = use_signal(|| 0usize);

    let slide_labels = ["01 OVERVIEW", "02 ARCHITECTURE", "03 LIVE DEMO"];

    rsx! {
        div { class: "project-album-card",
            // Aesthetic Glass Browser Mockup Frame (No Broken Image Icons!)
            div { class: "project-mockup-frame",
                // Browser Window Bar
                div { class: "mockup-mac-header",
                    div { class: "mockup-dots",
                        span { class: "mac-dot dot-red" }
                        span { class: "mac-dot dot-yellow" }
                        span { class: "mac-dot dot-green" }
                    }
                    div { class: "mockup-url-bar",
                        span { class: "url-protocol", "https://" }
                        span { class: "url-domain", "sreenand.dev/{project.name.to_lowercase()}" }
                    }
                    div { class: "mockup-status-tag",
                        span { class: "status-pulse-dot" }
                        span { "ACTIVE" }
                    }
                }

                // Main Gallery Display Area
                div { class: "mockup-display-canvas",
                    // Decorative Animated Glow Aura
                    div { class: "canvas-ambient-glow" }

                    // Slide 0: High-Tech Project Preview
                    if *selected_slide.read() == 0 {
                        div { class: "canvas-slide-content slide-fade-in",
                            div { class: "canvas-project-badge", "{project.category}" }
                            h4 { class: "canvas-project-title", "{project.name}" }
                            p { class: "canvas-project-tagline", "{project.subtitle}" }
                            
                            // Visual Code Matrix Graphic
                            div { class: "canvas-code-matrix",
                                div { class: "code-line",
                                    span { class: "code-kw", "const " }
                                    span { class: "code-var", "system " }
                                    span { class: "code-op", "= " }
                                    span { class: "code-str", "\"{project.name}\"" }
                                    span { ";" }
                                }
                                div { class: "code-line",
                                    span { class: "code-kw", "async function " }
                                    span { class: "code-fn", "streamData" }
                                    span { "() {{" }
                                }
                                div { class: "code-line indent",
                                    span { class: "code-kw", "await " }
                                    span { class: "code-var", "connectSockets" }
                                    span { "({{ status: " }
                                    span { class: "code-str", "\"ONLINE\"" }
                                    span { " }});" }
                                }
                                div { class: "code-line", span { "}}" } }
                            }
                        }
                    } else if *selected_slide.read() == 1 {
                        // Slide 1: Architectural Nodes Visual
                        div { class: "canvas-slide-content slide-fade-in",
                            div { class: "canvas-arch-grid",
                                div { class: "arch-node",
                                    span { class: "node-icon", "🌐" }
                                    span { class: "node-title", "Client UI" }
                                }
                                div { class: "arch-connector", "⚡" }
                                div { class: "arch-node arch-node--highlight",
                                    span { class: "node-icon", "⚡" }
                                    span { class: "node-title", "WebSockets" }
                                }
                                div { class: "arch-connector", "⚡" }
                                div { class: "arch-node",
                                    span { class: "node-icon", "🗄️" }
                                    span { class: "node-title", "Database" }
                                }
                            }
                        }
                    } else {
                        // Slide 2: Real-time Metrics & Impact
                        div { class: "canvas-slide-content slide-fade-in",
                            div { class: "canvas-metrics-row",
                                div { class: "metric-box",
                                    span { class: "metric-val", "< 50ms" }
                                    span { class: "metric-lbl", "LATENCY" }
                                }
                                div { class: "metric-box",
                                    span { class: "metric-val", "99.9%" }
                                    span { class: "metric-lbl", "UPTIME" }
                                }
                                div { class: "metric-box",
                                    span { class: "metric-val", "SECURE" }
                                    span { class: "metric-lbl", "AUTH" }
                                }
                            }
                        }
                    }
                }

                // Gallery Slide Selector Tabs (01 OVERVIEW / 02 ARCHITECTURE / 03 LIVE DEMO)
                div { class: "mockup-slide-selector",
                    for (idx, label) in slide_labels.iter().enumerate() {
                        button {
                            key: "{idx}",
                            class: if *selected_slide.read() == idx { "slide-tab-btn slide-tab-btn--active" } else { "slide-tab-btn" },
                            onclick: move |_| selected_slide.set(idx),
                            "{label}"
                        }
                    }
                }
            }

            // Project Info Details Body
            div { class: "project-album-body",
                div { class: "project-album-category-pill", "{project.category}" }
                h3 { class: "project-album-title", "{project.name}" }
                div { class: "project-album-subtitle", "{project.subtitle}" }
                p { class: "project-album-overview", "{project.overview}" }

                // Technical Challenge & Solution Cards
                div { class: "project-album-insights",
                    div { class: "insight-card",
                        span { class: "insight-label", "THE CHALLENGE" }
                        p { class: "insight-text", "{project.problem_faced}" }
                    }
                    div { class: "insight-card",
                        span { class: "insight-label", "THE SOLUTION" }
                        p { class: "insight-text", "{project.solution_implemented}" }
                    }
                }

                // Tech Stack Badges
                div { class: "project-album-tags",
                    for tech in project.tech_badges {
                        {
                            let tech_str = *tech;
                            rsx! { span { class: "album-tag-badge", "{tech_str}" } }
                        }
                    }
                }

                // Action Links
                div { class: "project-album-actions",
                    a {
                        class: "project-album-btn project-album-btn--primary",
                        href: "{project.github_url}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "14", height: "14",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            path { d: "M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22" }
                        }
                        span { "SOURCE CODE" }
                    }
                    if let Some(live_url) = project.live_url {
                        a {
                            class: "project-album-btn project-album-btn--secondary",
                            href: "{live_url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "14", height: "14",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6m4-3h6v6m-11 5L21 3" }
                            }
                            span { "LIVE DEMO" }
                        }
                    }
                }
            }
        }
    }
}

