use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

/// Floating RAG AI chat button — orbit animation, zap icon.
#[component]
pub fn RagAiButton() -> Element {
    let mut is_open = use_signal(|| false);
    let mut messages = use_signal(|| vec![
        ChatMessage {
            role: MessageRole::Assistant,
            content: "Hi! I'm Sreenand's AI assistant. Ask me about his skills, projects, or experience.".to_string(),
        }
    ]);
    let mut input_text = use_signal(String::new);
    let mut is_loading = use_signal(|| false);

    let open = *is_open.read();

    let mut send_question = move |q: String| {
        let question = q.trim().to_string();
        if question.is_empty() || *is_loading.read() {
            return;
        }

        // Add user message to history
        messages.write().push(ChatMessage {
            role: MessageRole::User,
            content: question.clone(),
        });

        // Clear input text & set loading state
        input_text.set(String::new());
        is_loading.set(true);

        // Spawn async task to call serverless API (/api/chat with local 3001 fallback)
        spawn(async move {
            let mut eval = document::eval(
                r#"
                const question = await dioxus.recv();
                try {
                    let res = await fetch('/api/chat', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ question: question })
                    });
                    if (!res.ok && res.status === 404) {
                        res = await fetch('http://localhost:3001/api/chat', {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify({ question: question })
                        });
                    }
                    const data = await res.json();
                    if (res.ok && data.answer) {
                        dioxus.send({ ok: true, answer: data.answer });
                    } else {
                        dioxus.send({ ok: false, error: data.error || 'Unable to get response.' });
                    }
                } catch (err) {
                    try {
                        const res2 = await fetch('http://localhost:3001/api/chat', {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify({ question: question })
                        });
                        const data2 = await res2.json();
                        if (res2.ok && data2.answer) {
                            dioxus.send({ ok: true, answer: data2.answer });
                            return;
                        }
                    } catch (e) {}
                    dioxus.send({ ok: false, error: 'Network error. Please try again.' });
                }
                "#
            );

            let _ = eval.send(question.clone());

            let answer = match eval.recv::<serde_json::Value>().await {
                Ok(val) => {
                    let ok = val.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
                    if ok {
                        val.get("answer")
                            .and_then(|v| v.as_str())
                            .unwrap_or("No response received.")
                            .to_string()
                    } else {
                        val.get("error")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Failed to process request.")
                            .to_string()
                    }
                }
                Err(_) => "Failed to communicate with AI assistant.".to_string(),
            };

            messages.write().push(ChatMessage {
                role: MessageRole::Assistant,
                content: answer,
            });
            is_loading.set(false);

            // Auto-scroll messages container to bottom
            let _ = document::eval(
                r#"
                setTimeout(() => {
                    const msgBox = document.querySelector('.rag-chat-messages');
                    if (msgBox) {
                        msgBox.scrollTop = msgBox.scrollHeight;
                    }
                }, 50);
                "#
            );
        });
    };

    rsx! {
        div { class: "rag-ai-wrapper",

            // ── Chat panel ──────────────────────────────────────────────────
            div {
                class: if open { "rag-chat-panel rag-chat-panel--open" } else { "rag-chat-panel" },

                // Header
                div { class: "rag-chat-header",
                    div { class: "rag-chat-header-left",
                        div { class: "rag-chat-status-dot" }
                        div { class: "rag-chat-header-text",
                            span { class: "rag-chat-title", "AI Assistant" }
                        }
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

                // Messages list
                div { class: "rag-chat-messages",
                    for msg in messages.read().iter() {
                        div {
                            class: match msg.role {
                                MessageRole::User => "rag-chat-msg rag-chat-msg--user",
                                MessageRole::Assistant => "rag-chat-msg",
                            },
                            div {
                                class: match msg.role {
                                    MessageRole::User => "rag-chat-avatar rag-chat-avatar--user",
                                    MessageRole::Assistant => "rag-chat-avatar",
                                },
                                if msg.role == MessageRole::User {
                                    // Modern User icon
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "13", height: "13",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        circle { cx: "12", cy: "8", r: "5" }
                                        path { d: "M20 21a8 8 0 1 0-16 0" }
                                    }
                                } else {
                                    // Modern AI Sparkles icon
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "13", height: "13",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z" }
                                    }
                                }
                            }
                            div { class: "rag-chat-bubble",
                                p {
                                    { render_formatted_content(&msg.content) }
                                }
                            }
                        }
                    }

                    // Loading indicator
                    if *is_loading.read() {
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
                                    path { d: "m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z" }
                                }
                            }
                            div { class: "rag-chat-bubble",
                                p { style: "font-style: italic; opacity: 0.7;", "Thinking…" }
                            }
                        }
                    }
                }

                // Suggestion chips (hide while loading or when conversation has progressed)
                if messages.read().len() <= 2 && !*is_loading.read() {
                    div { class: "rag-chat-chips",
                        span {
                            class: "rag-chat-chip",
                            onclick: move |_| send_question("What are Sreenand's technical skills and tech stack?".to_string()),
                            "Skills & stack"
                        }
                        span {
                            class: "rag-chat-chip",
                            onclick: move |_| send_question("Tell me about the projects Sreenand has built.".to_string()),
                            "Projects"
                        }
                        span {
                            class: "rag-chat-chip",
                            onclick: move |_| send_question("What is Sreenand's work and internship experience?".to_string()),
                            "Experience"
                        }
                        span {
                            class: "rag-chat-chip",
                            onclick: move |_| send_question("How can I contact Sreenand?".to_string()),
                            "Contact"
                        }
                    }
                }

                // Input row — wrapped in form for 100% reliable Enter key and submit button clicks
                form {
                    class: "rag-chat-input-row",
                    onsubmit: move |evt| {
                        evt.prevent_default();
                        let text = input_text.cloned();
                        send_question(text);
                    },
                    input {
                        class: "rag-chat-input",
                        r#type: "text",
                        placeholder: "Ask me anything…",
                        value: "{input_text}",
                        oninput: move |evt| input_text.set(evt.value()),
                    }
                    button {
                        class: "rag-chat-send",
                        r#type: "submit",
                        disabled: *is_loading.read(),
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

fn render_formatted_content(content: &str) -> Element {
    let lines = content.lines().collect::<Vec<_>>();
    rsx! {
        for (line_idx, line) in lines.iter().enumerate() {
            if line_idx > 0 { br {} }
            { render_formatted_line(line) }
        }
    }
}

fn render_formatted_line(line: &str) -> Element {
    // Clean up raw Markdown symbols: **, *, #, [, ]
    let cleaned = line
        .replace("**", "")
        .replace('*', "")
        .replace('#', "")
        .replace('[', "")
        .replace(']', "");

    let words = cleaned.split_whitespace().collect::<Vec<_>>();

    rsx! {
        for (w_idx, word) in words.iter().enumerate() {
            if w_idx > 0 { " " }
            { render_word(word) }
        }
    }
}

fn render_word(word: &str) -> Element {
    let clean = word.trim_matches(|c| c == '(' || c == ')' || c == '[' || c == ']');
    if clean.starts_with("https://") || clean.starts_with("http://") || clean.starts_with("mailto:") {
        let url = clean.trim_end_matches(|c| c == '.' || c == ',' || c == ')' || c == ']');
        rsx! {
            a {
                href: "{url}",
                target: "_blank",
                rel: "noopener noreferrer",
                style: "color: #a5b4fc; text-decoration: underline; word-break: break-all;",
                "{url}"
            }
        }
    } else {
        rsx! { "{word}" }
    }
}

