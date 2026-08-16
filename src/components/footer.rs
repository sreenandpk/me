use dioxus::prelude::*;
use crate::data::PERSONAL_INFO;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer-section",
            div { class: "container",
                div { class: "footer-grid",
                    // Left Column: Credentials
                    div { class: "footer-col-left",
                        div { class: "footer-name", "{PERSONAL_INFO.name}" }
                        div { class: "footer-title", "{PERSONAL_INFO.title}" }
                    }

                    // Right Column: Links & Build stamp
                    div { class: "footer-col-right",
                        div { class: "footer-links",

                            a {
                                class: "footer-link",
                                href: "{PERSONAL_INFO.instagram_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                title: "Instagram",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "18",
                                    height: "18",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    view_box: "0 0 24 24",
                                    rect { x: "2", y: "2", width: "20", height: "20", rx: "5", ry: "5" }
                                    path { d: "M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" }
                                    line { x1: "17.5", y1: "6.5", x2: "17.51", y2: "6.5" }
                                }
                            }
                            a {
                                class: "footer-link",
                                href: "{PERSONAL_INFO.whatsapp_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                title: "WhatsApp",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "18",
                                    height: "18",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    view_box: "0 0 24 24",
                                    path { d: "M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z" }
                                }
                            }
                            a {
                                class: "footer-link",
                                href: "{PERSONAL_INFO.phone}",
                                title: "Phone",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "18",
                                    height: "18",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    view_box: "0 0 24 24",
                                    path { d: "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" }
                                }
                            }
                        }
                        div { class: "footer-quote", "Always learning. Always building." }
                    }
                }
            }
        }
    }
}
