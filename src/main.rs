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

    // Pure Scroll-Driven Sticky Depth Progress Engine
    use_effect(move || {
        let _ = document::eval(
            r#"
            const initScrollDrivenSticky = () => {
                const sections = Array.from(document.querySelectorAll('section.section, footer.footer-section'));
                if (sections.length === 0) {
                    setTimeout(initScrollDrivenSticky, 100);
                    return;
                }

                let ticking = false;

                const updateScrollProgress = () => {
                    const viewportHeight = window.innerHeight;
                    const isMobile = window.innerWidth <= 768;

                    const recedeScale = isMobile ? 0.985 : 0.96;
                    const travelMaxY = isMobile ? 45 : 90;

                    sections.forEach((sec, idx) => {
                        const rect = sec.getBoundingClientRect();
                        const top = rect.top;
                        const height = rect.height || viewportHeight;

                        const isFooterOrLast = sec.tagName === 'FOOTER' || idx >= sections.length - 2;

                        if (idx === 0) {
                            if (top >= 0) {
                                sec.style.transform = 'scale(1) translateY(0px)';
                                sec.style.opacity = '1';
                                sec.style.filter = 'blur(0px)';
                            } else {
                                const p = Math.min(Math.abs(top) / (height * 0.7), 1);
                                const scale = 1 - p * (1 - recedeScale);
                                const opacity = 1 - p * 0.15;
                                sec.style.transform = `scale(${scale.toFixed(4)}) translateY(${(-p * 15).toFixed(1)}px)`;
                                sec.style.opacity = opacity.toFixed(2);
                                sec.style.filter = 'blur(0px)';
                            }
                            return;
                        }

                        // Always keep footer and bottom contact section 100% crisp and unblurred once visible
                        if (isFooterOrLast && top < viewportHeight * 0.95) {
                            sec.style.transform = 'scale(1) translateY(0px)';
                            sec.style.opacity = '1';
                            sec.style.filter = 'blur(0px)';
                            return;
                        }

                        const focusStart = viewportHeight * 0.85;
                        const focusEnd = viewportHeight * 0.20;

                        if (top > focusStart) {
                            sec.style.transform = `translateY(${travelMaxY}px) scale(1)`;
                            sec.style.opacity = '0.25';
                            sec.style.filter = 'blur(12px)';
                        } else if (top >= 0) {
                            let enterProgress = 1 - ((top - focusEnd) / (focusStart - focusEnd));
                            enterProgress = Math.min(Math.max(enterProgress, 0), 1);
                            const translateY = (1 - enterProgress) * travelMaxY;
                            const opacity = 0.25 + enterProgress * 0.75;
                            const blur = (1 - enterProgress) * 12;
                            sec.style.transform = `translateY(${translateY.toFixed(1)}px) scale(1)`;
                            sec.style.opacity = opacity.toFixed(2);
                            sec.style.filter = `blur(${blur.toFixed(1)}px)`;
                        } else {
                            if (idx === sections.length - 1) {
                                sec.style.transform = 'scale(1) translateY(0px)';
                                sec.style.opacity = '1';
                                sec.style.filter = 'blur(0px)';
                            } else {
                                const recedeProgress = Math.min(Math.abs(top) / (height * 0.8), 1);
                                const scale = 1 - recedeProgress * (1 - recedeScale);
                                const opacity = 1 - recedeProgress * 0.15;
                                sec.style.transform = `scale(${scale.toFixed(4)}) translateY(${(-recedeProgress * 15).toFixed(1)}px)`;
                                sec.style.opacity = opacity.toFixed(2);
                                sec.style.filter = 'blur(0px)';
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
                window.addEventListener('resize', onScroll, { passive: true });
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

            // Page Footer
            Footer {}

            // Floating RAG AI indicator
            RagAiButton {}
        }
    }
}
