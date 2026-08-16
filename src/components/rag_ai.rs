use dioxus::prelude::*;

/// Floating RAG AI chat button — orbit animation, zap icon.
#[component]
pub fn RagAiButton() -> Element {
    let mut is_open = use_signal(|| false);
    let open = *is_open.read();

    rsx! {
        div { class: "rag-ai-wrapper",

            // ── Chat panel ──────────────────────────────────────────────────
            div {
                class: if open { "rag-chat-panel rag-chat-panel--open" } else { "rag-chat-panel" },

                // Header
                div { class: "rag-chat-header",
                    div { class: "rag-chat-header-left",
                        div { class: "rag-chat-status-dot" }
                        span { class: "rag-chat-title", "AI Assistant" }
                    }
                    button {
                        class: "rag-chat-close",
                        onclick: move |_| is_open.set(false),
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "14", height: "14",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            path { d: "M18 6 6 18" }
                            path { d: "m6 6 12 12" }
                        }
                    }
                }

                // Messages
                div { class: "rag-chat-messages",
                    div { class: "rag-chat-msg",
                        div { class: "rag-chat-avatar",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "13", height: "13",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "4 17 10 11 4 5" }
                                line { x1: "12", y1: "19", x2: "20", y2: "19" }
                            }
                        }
                        div { class: "rag-chat-bubble",
                            p { "Hi! I'm Sreenand's AI assistant." }
                            p { "Ask me about his skills, projects, or experience." }
                        }
                    }

                    div { class: "rag-chat-chips",
                        span { class: "rag-chat-chip", "Skills & stack" }
                        span { class: "rag-chat-chip", "Projects" }
                        span { class: "rag-chat-chip", "Experience" }
                        span { class: "rag-chat-chip", "Contact" }
                    }
                }

                // Input row
                div { class: "rag-chat-input-row",
                    input {
                        class: "rag-chat-input",
                        r#type: "text",
                        placeholder: "Ask me anything…",
                    }
                    button { class: "rag-chat-send",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "15", height: "15",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            path { d: "m22 2-7 20-4-9-9-4Z" }
                            path { d: "M22 2 11 13" }
                        }
                    }
                }
            }

            // ── FAB ────────────────────────────────────────────────────────
            div {
                class: "rag-ai-fab",
                onclick: move |_| is_open.set(!open),

                // Glass icon — Zap / AI lightning
                div { class: "rag-ai-icon",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "21", height: "21",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.7",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        // Zap / lightning bolt
                        polygon { points: "13 2 3 14 12 14 11 22 21 10 12 10 13 2" }
                    }
                }

                // Tooltip
                if !open {
                    div { class: "rag-ai-tooltip",
                        span { class: "rag-ai-dot" }
                        span { class: "rag-ai-label", "Ask me anything" }
                    }
                }
            }
        }
    }
}
