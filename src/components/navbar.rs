use dioxus::prelude::*;

#[component]
pub fn Navbar(is_light: Signal<bool>) -> Element {
    let mut mobile_open = use_signal(|| false);

    // Effect to track scroll and update active section and scroll progress
    use_effect(move || {
        let _ = document::eval(
            r#"
            // Scroll progress bar
            const updateScrollProgress = () => {
                const scrollProgress = document.querySelector('.scroll-progress');
                if (scrollProgress) {
                    const totalHeight = document.documentElement.scrollHeight - window.innerHeight;
                    const progress = totalHeight > 0 ? (window.pageYOffset / totalHeight) * 100 : 0;
                    scrollProgress.style.width = progress + '%';
                }
            };
            window.addEventListener('scroll', updateScrollProgress);
            updateScrollProgress();

            // Intersection Observer to highlight navbar links
            const observer = new IntersectionObserver((entries) => {
                entries.forEach(entry => {
                    if (entry.isIntersecting) {
                        const id = entry.target.id;
                        const navLinks = document.querySelectorAll('.nav-link');
                        navLinks.forEach(link => {
                            if (link.getAttribute('href') === '#' + id) {
                                link.classList.add('active');
                            } else {
                                link.classList.remove('active');
                            }
                        });
                    }
                });
            }, { threshold: 0.2, rootMargin: "-80px 0px -40% 0px" });

            document.querySelectorAll('section').forEach(section => {
                observer.observe(section);
            });
            "#
        );
    });

    let toggle_theme = move |_| {
        let mut curr = is_light;
        curr.toggle();
        let _ = document::eval(&format!(
            "document.documentElement.classList.toggle('light-theme', {});",
            curr()
        ));
    };

    let toggle_mobile = move |_| {
        mobile_open.toggle();
    };

    let mut close_mobile = move || {
        mobile_open.set(false);
    };

    rsx! {
        // Scroll Progress Bar
        div { 
            class: "scroll-progress", 
            style: "width: 0%;" 
        }

        header { class: "navbar-header",
            div { class: "container nav-container",

                // Avatar icon + caption on the left
                a { class: "nav-avatar-link", href: "#home",
                    img {
                        class: "nav-avatar",
                        src: "/assets/avatar.jpg",
                        alt: "Sreenand P K"
                    }
                    span { class: "nav-avatar-caption", "Portfolio" }
                }

                // Desktop Minimal Navigation
                ul { class: "nav-links",
                    li { a { class: "nav-link active", href: "#home", "Home" } }
                    li { a { class: "nav-link", href: "#about", "About" } }
                    li { a { class: "nav-link", href: "#skills", "Skills" } }
                    li { a { class: "nav-link", href: "#projects", "Projects" } }
                    li { a { class: "nav-link", href: "#experience", "Experience" } }
                    li { a { class: "nav-link", href: "#philosophy", "Principles" } }
                    li { a { class: "nav-link", href: "#contact", "Contact" } }
                }

                // Actions
                div { class: "nav-actions",
                    // Theme Toggle Button — Artistic Cosmic Sparkle / Solar Starburst
                    button {
                        class: "theme-toggle-btn",
                        onclick: toggle_theme,
                        title: "Toggle Theme",
                        if is_light() {
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "18",
                                height: "18",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.7",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                view_box: "0 0 24 24",
                                path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                                path { d: "M19 3v4m-2-2h4" }
                            }
                        } else {
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "18",
                                height: "18",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.7",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                view_box: "0 0 24 24",
                                circle { cx: "12", cy: "12", r: "4" }
                                path { d: "M12 2v2m0 16v2M4.93 4.93l1.41 1.41m11.32 11.32l1.41 1.41M2 12h2m16 0h2M4.93 19.07l1.41-1.41m11.32-11.32l1.41-1.41" }
                            }
                        }
                    }

                    // Mobile Menu Toggle — Aesthetic Asymmetric 2-Line & Accent Dot
                    button {
                        class: if mobile_open() { "mobile-nav-toggle mobile-nav-toggle--open" } else { "mobile-nav-toggle" },
                        onclick: toggle_mobile,
                        aria_label: "Toggle Menu",
                        if mobile_open() {
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "18",
                                height: "18",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.8",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                view_box: "0 0 24 24",
                                line { x1: "18", y1: "6", x2: "6", y2: "18" }
                                line { x1: "6", y1: "6", x2: "18", y2: "18" }
                            }
                        } else {
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
                                line { x1: "4", y1: "8", x2: "20", y2: "8" }
                                line { x1: "10", y1: "16", x2: "20", y2: "16" }
                                circle { cx: "5.5", cy: "16", r: "1.5", fill: "currentColor" }
                            }
                        }
                    }
                }
            }
        }

        // Mobile Nav Drawer
        ul { class: if mobile_open() { "mobile-nav-menu open" } else { "mobile-nav-menu" },
            li { a { class: "nav-link", href: "#home", onclick: move |_| close_mobile(), "Home" } }
            li { a { class: "nav-link", href: "#about", onclick: move |_| close_mobile(), "About" } }
            li { a { class: "nav-link", href: "#skills", onclick: move |_| close_mobile(), "Skills" } }
            li { a { class: "nav-link", href: "#projects", onclick: move |_| close_mobile(), "Projects" } }
            li { a { class: "nav-link", href: "#experience", onclick: move |_| close_mobile(), "Experience" } }
            li { a { class: "nav-link", href: "#philosophy", onclick: move |_| close_mobile(), "Principles" } }
            li { a { class: "nav-link", href: "#contact", onclick: move |_| close_mobile(), "Contact" } }
        }
    }
}
