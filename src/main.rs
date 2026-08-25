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
                const sections = Array.from(document.querySelectorAll('section.section, footer.footer-section'));
                if (sections.length === 0) {
                    setTimeout(initScrollDrivenSticky, 100);
                    return;
                }

                let sectionData = [];

                const measure = () => {
                    const scrollY = window.pageYOffset || document.documentElement.scrollTop;
                    sectionData = sections.map((sec, idx) => {
                        const rect = sec.getBoundingClientRect();
                        return {
                            sec,
                            idx,
                            top: rect.top + scrollY,
                            height: rect.height || window.innerHeight,
                            isFooterOrLast: sec.tagName === 'FOOTER' || idx >= sections.length - 2
                        };
                    });
                };

                measure();
                window.addEventListener('resize', measure, { passive: true });
                window.addEventListener('load', measure, { passive: true });
                setTimeout(measure, 300);
                setTimeout(measure, 1000);

                let ticking = false;

                const updateScrollProgress = () => {
                    const scrollY = window.pageYOffset || document.documentElement.scrollTop;
                    const viewportHeight = window.innerHeight;
                    const isMobile = window.innerWidth <= 768;

                    const recedeScale = isMobile ? 0.985 : 0.96;
                    const travelMaxY = isMobile ? 45 : 90;

                    sectionData.forEach(item => {
                        const sec = item.sec;
                        const idx = item.idx;
                        const top = item.top - scrollY;
                        const height = item.height;

                        const isLastContainer = sec.id === 'contact' || sec.tagName === 'FOOTER' || idx >= sectionData.length - 2;

                        // Last container stops cleanly in 100% unblurred focus when visible near bottom
                        if (isLastContainer && top < viewportHeight * 0.85) {
                            sec.style.transform = 'translate3d(0, 0, 0) scale(1)';
                            sec.style.opacity = '1';
                            sec.style.filter = 'blur(0px)';
                            return;
                        }

                        if (top > 0) {
                            // Entering phase: section rises from below, unblurs 16px -> 0px, scales 0.94 -> 1.0, opacity 0.15 -> 1.0
                            let p = (viewportHeight - top) / (viewportHeight * 0.70);
                            p = Math.min(Math.max(p, 0), 1);
                            const translateY = (1 - p) * travelMaxY;
                            const scale = 0.94 + p * 0.06;
                            const opacity = 0.15 + p * 0.85;
                            const blur = (1 - p) * 16;
                            sec.style.transform = `translate3d(0, ${translateY.toFixed(1)}px, 0) scale(${scale.toFixed(4)})`;
                            sec.style.opacity = opacity.toFixed(2);
                            sec.style.filter = `blur(${blur.toFixed(1)}px)`;
                        } else {
                            // Receding phase: active section recedes into depth as user scrolls past top
                            sec.style.transform = 'translate3d(0, 0, 0) scale(1)';
                            sec.style.opacity = '1';
                            sec.style.filter = 'blur(0px)';
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

            // About section
            About {}

            // Skills section
            Skills {}

            // Projects section
            Projects {}

            // Experience timeline
            ExperienceSection {}

            // Philosophy card grid
            Philosophy {}

            // Contact details
            Contact {}

            // Page Footer Container (Pure Depth & Blur Transition)
            Footer {}

            // Floating RAG AI indicator
            RagAiButton {}
        }
    }
}
