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
                        src: "/avatar.jpg",
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
                    // Theme Toggle Button
                    button {
                        class: "theme-toggle-btn",
                        onclick: toggle_theme,
                        title: "Toggle Theme",
                        if is_light() {
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "16",
                                height: "16",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                view_box: "0 0 24 24",
                                path { d: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" }
                            }
                        } else {
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "16",
                                height: "16",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                view_box: "0 0 24 24",
                                circle { cx: "12", cy: "12", r: "5" }
                                line { x1: "12", y1: "1", x2: "12", y2: "3" }
                                line { x1: "12", y1: "21", x2: "12", y2: "23" }
                                line { x1: "4.22", y1: "4.22", x2: "5.64", y2: "5.64" }
                                line { x1: "18.36", y1: "18.36", x2: "19.78", y2: "19.78" }
                                line { x1: "1", y1: "12", x2: "3", y2: "12" }
                                line { x1: "21", y1: "12", x2: "23", y2: "12" }
                                line { x1: "4.22", y1: "19.78", x2: "5.64", y2: "18.36" }
                                line { x1: "18.36", y1: "5.64", x2: "19.78", y2: "4.22" }
                            }
                        }
                    }

                    // Mobile Menu Toggle
                    button {
                        class: "mobile-nav-toggle",
                        onclick: toggle_mobile,
                        aria_label: "Toggle Menu",
                        if mobile_open() {
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "20",
                                height: "20",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                view_box: "0 0 24 24",
                                line { x1: "18", y1: "6", x2: "6", y2: "18" }
                                line { x1: "6", y1: "6", x2: "18", y2: "18" }
                            }
                        } else {
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "20",
                                height: "20",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                view_box: "0 0 24 24",
                                line { x1: "3", y1: "12", x2: "21", y2: "12" }
                                line { x1: "3", y1: "6", x2: "21", y2: "6" }
                                line { x1: "3", y1: "18", x2: "21", y2: "18" }
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
