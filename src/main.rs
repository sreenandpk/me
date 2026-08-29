use dioxus::prelude::*;

mod components;
mod data;

use components::*;

fn main() {
    // Init logger
    dioxus::logger::init(dioxus::logger::tracing::Level::INFO).expect("failed to init logger");
    
    // Launch the app
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let is_light = use_signal(|| false);

    // Pure Scroll-Driven Sticky Depth Progress Engine (Zero Layout Thrashing, 60FPS GPU Composite)
    use_effect(move || {
        let _ = document::eval(
            r#"
            const initScrollDrivenSticky = () => {
                let sections = Array.from(document.querySelectorAll('section.section, footer.footer-section, .blur-on-enter'));
                if (!sections.length) return;

                let sectionData = [];
                let cachedWidth = window.innerWidth;

                const measure = () => {
                    const scrollY = window.pageYOffset || document.documentElement.scrollTop;
                    sectionData = sections.map((sec, idx) => {
                        const rect = sec.getBoundingClientRect();
                        return {
                            sec,
                            idx,
                            isBlurOnly: sec.classList.contains('blur-on-enter'),
                            top: rect.top + scrollY,
                            height: rect.height || window.innerHeight
                        };
                    });
                };

                measure();
                window.addEventListener('resize', () => {
                    if (window.innerWidth !== cachedWidth) {
                        cachedWidth = window.innerWidth;
                        measure();
                    }
                }, { passive: true });
                window.addEventListener('load', measure, { passive: true });
                setTimeout(measure, 300);
                setTimeout(measure, 1000);

                let ticking = false;

                const updateScrollProgress = () => {
                    const scrollY = window.pageYOffset || document.documentElement.scrollTop;
                    const viewportHeight = window.innerHeight;
                    const isMobile = window.innerWidth <= 768;

                    const recedeScale = isMobile ? 0.985 : 0.96;
                    const travelMaxY = isMobile ? 25 : 90;
                    const maxBlur = isMobile ? 4 : 16;
                    const minScale = isMobile ? 0.97 : 0.94;

                    sectionData.forEach(item => {
                        const sec = item.sec;
                        const top = item.top - scrollY;
                        const height = item.height;

                        if (top > 0) {
                            // Entering phase: smooth GPU entrance tailored for mobile and desktop
                            let p = (viewportHeight - top) / (viewportHeight * 0.35);
                            p = Math.min(Math.max(p, 0), 1);
                            
                            const blur = (1 - p) * maxBlur;
                            sec.style.filter = `blur(${blur.toFixed(1)}px)`;
                            
                            if (!item.isBlurOnly) {
                                const translateY = (1 - p) * travelMaxY;
                                const scale = minScale + p * (1 - minScale);
                                const opacity = 0.2 + p * 0.8;
                                sec.style.transform = `translate3d(0, ${translateY.toFixed(1)}px, 0) scale(${scale.toFixed(4)})`;
                                sec.style.opacity = opacity.toFixed(2);
                            }
                        } else {
                            // Receding phase: active section recedes into depth as user scrolls past top
                            sec.style.filter = 'blur(0px)';
                            
                            if (!item.isBlurOnly) {
                                const recedeProgress = Math.min(Math.abs(top) / (height * 0.8), 1);
                                const scale = 1 - recedeProgress * (1 - recedeScale);
                                const opacity = 1 - recedeProgress * 0.15;
                                const translateY = -recedeProgress * (isMobile ? 8 : 15);
                                sec.style.transform = `scale(${scale.toFixed(4)}) translate3d(0, ${translateY.toFixed(1)}px, 0)`;
                                sec.style.opacity = opacity.toFixed(2);
                            }
                        }
                    });

                    ticking = false;
                };

                const onScroll = () => {
                    if (!ticking) {
                        requestAnimationFrame(updateScrollProgress);
                        ticking = true;
                    }
                };

                window.addEventListener('scroll', onScroll, { passive: true });
                window.addEventListener('touchmove', onScroll, { passive: true });
                updateScrollProgress();
            };

            setTimeout(initScrollDrivenSticky, 100);
            setTimeout(initScrollDrivenSticky, 400);
            "#
        );
    });

    // Remove skeleton loader once app has mounted
    use_effect(move || {
        let _ = document::eval(
            r#"
            const sk = document.getElementById('sk-loader');
            if (sk) {
                sk.classList.add('sk-hidden');
                setTimeout(() => { if (sk.parentNode) sk.remove(); }, 600);
            }
            "#
        );
    });

    rsx! {
        // Link the main stylesheet via Dioxus asset management
        document::Stylesheet { href: asset!("/assets/main.css") }

        main {
            // Header Navbar (Sticky)
            Navbar { is_light }

            // Hero section
            Hero {}

            div { class: "about-projects-wrapper",
                // About section
                About {}

                // Services section (Formerly Projects)
                Services {}
            }

            // Real Projects Section (Normal scroll initially)
            Projects {}

            // Skills section
            Skills {}

            // Experience timeline
            ExperienceSection {}

            // Contact details
            Contact {}

            // Page Footer Container (Pure Depth & Blur Transition)
            Footer {}

            // Floating RAG AI indicator
            RagAiButton {}
        }
    }
}
