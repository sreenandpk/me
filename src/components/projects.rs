use dioxus::prelude::*;
use crate::data::PROJECTS;

#[component]
pub fn Projects() -> Element {
    let mut lightbox_image = use_signal(|| None::<(&'static str, &'static str)>);

    rsx! {
        section { id: "projects", class: "section reveal-on-scroll",
            div { class: "container",
                div { class: "projects-heading-col",
                    span { class: "projects-pretitle", "CASE STUDIES" }
                    h2 { class: "projects-headline", "Selected projects" }
                    p { class: "projects-subtitle", "Interactive gallery showcase of full-stack systems and cloud applications." }
                    div { class: "projects-divider" }
                }

                // Album Gallery Grid
                div { class: "projects-album-grid",
                    for (project_idx, project) in PROJECTS.iter().enumerate() {
                        ProjectAlbumCard {
                            key: "{project_idx}",
                            project: project,
                            on_open_lightbox: move |(img_url, proj_name): (&'static str, &'static str)| {
                                lightbox_image.set(Some((img_url, proj_name)));
                            }
                        }
                    }
                }
            }

            // Lightbox Modal for Fullscreen Screenshot Preview
            if let Some((img_url, proj_name)) = *lightbox_image.read() {
                div {
                    class: "album-lightbox-overlay",
                    onclick: move |_| lightbox_image.set(None),
                    div {
                        class: "album-lightbox-content",
                        onclick: move |evt| evt.stop_propagation(),
                        div { class: "album-lightbox-header",
                            span { class: "album-lightbox-title", "{proj_name}" }
                            button {
                                class: "album-lightbox-close",
                                onclick: move |_| lightbox_image.set(None),
                                "✕"
                            }
                        }
                        div { class: "album-lightbox-img-wrapper",
                            img {
                                class: "album-lightbox-img",
                                src: "{img_url}",
                                alt: "{proj_name}",
                            }
                            div { class: "album-lightbox-fallback",
                                span { class: "fallback-text", "Image Preview for {proj_name}" }
                                span { class: "fallback-sub", "Upload screenshots to {img_url} to display here." }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectAlbumCard(project: &'static crate::data::projects::Project, on_open_lightbox: EventHandler<(&'static str, &'static str)>) -> Element {
    let mut selected_img_idx = use_signal(|| 0usize);
    let active_img = project.gallery_images.get(*selected_img_idx.read()).copied().unwrap_or(project.cover_image);

    rsx! {
        div { class: "project-album-card",
            // Album Gallery Header Banner / Main Image View
            div { class: "project-album-media",
                div {
                    class: "project-album-cover-box",
                    onclick: move |_| on_open_lightbox.call((active_img, project.name)),
                    img {
                        class: "project-album-cover-img",
                        src: "{active_img}",
                        alt: "{project.name}",
                    }
                    // Sleek Glass Gradient Fallback Banner (displays if images aren't uploaded yet)
                    div { class: "project-album-fallback-banner",
                        div { class: "album-banner-badge", "{project.category}" }
                        div { class: "album-banner-title", "{project.name}" }
                        div { class: "album-banner-hint", "Click to view full preview" }
                    }
                    div { class: "project-album-zoom-badge",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "14", height: "14",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            path { d: "M15 3h6v6M9 21H3v-6M21 3l-7 7M3 21l7-7" }
                        }
                    }
                }

                // Gallery Thumbnails Strip (Switch active image on click!)
                if project.gallery_images.len() > 1 {
                    div { class: "project-album-thumbs-strip",
                        for (idx, img_path) in project.gallery_images.iter().enumerate() {
                            button {
                                key: "{idx}",
                                class: if *selected_img_idx.read() == idx { "album-thumb-btn album-thumb-btn--active" } else { "album-thumb-btn" },
                                onclick: move |_| selected_img_idx.set(idx),
                                img {
                                    class: "album-thumb-img",
                                    src: "{img_path}",
                                    alt: "Thumbnail {idx + 1}",
                                }
                                div { class: "album-thumb-fallback", "Shot {idx + 1}" }
                            }
                        }
                    }
                }
            }

            // Project Info Details
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
                            width: "15", height: "15",
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
                                width: "15", height: "15",
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

