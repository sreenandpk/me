use dioxus::prelude::*;
use crate::data::SKILL_CATEGORIES;

const TOTAL_FRAMES: usize = 50;

#[component]
pub fn Skills() -> Element {
    // Inject the scroll-controlled frame animation engine (isolated to Skills section)
    use_effect(move || {
        let _ = document::eval(
            r#"
            (function initCharacterScroll() {
                const TOTAL_FRAMES = 50;
                const track   = document.getElementById('char-scroll-track');
                if (!track) return;

                // ── Reduced motion fallback ──────────────────────────────────
                if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
                    const img = document.getElementById('char-frame-img');
                    if (img) img.src = '/assets/character/skills/frame-050.png';
                    return;
                }

                const frameImg = document.getElementById('char-frame-img');
                if (!frameImg) return;

                // ── Preload all frames ───────────────────────────────────────
                const frames = [];
                let loadedCount = 0;
                const allLoaded = new Promise((resolve) => {
                    for (let i = 1; i <= TOTAL_FRAMES; i++) {
                        const img = new Image();
                        const n   = String(i).padStart(3, '0');
                        img.src   = `/assets/character/skills/frame-${n}.png`;
                        img.onload  = () => { loadedCount++; if (loadedCount === TOTAL_FRAMES) resolve(); };
                        img.onerror = () => { loadedCount++; if (loadedCount === TOTAL_FRAMES) resolve(); };
                        frames.push(img);
                    }
                });

                let currentFrame = 0;
                let ticking = false;

                const setFrame = (idx) => {
                    const clamped = Math.max(0, Math.min(TOTAL_FRAMES - 1, idx));
                    if (clamped !== currentFrame || currentFrame === 0) {
                        currentFrame = clamped;
                        frameImg.src = frames[clamped]?.src || frameImg.src;
                    }
                };

                const update = () => {
                    const rect    = track.getBoundingClientRect();
                    const vh      = window.innerHeight;
                    const maxScroll = rect.height - vh;

                    let p = -rect.top / maxScroll;
                    if (p < 0) p = 0;
                    if (p > 1) p = 1;

                    // Map scroll progress to frame index (0→0, 1→49)
                    const frameIdx = Math.round(p * (TOTAL_FRAMES - 1));
                    setFrame(frameIdx);

                    // Fade in the terminal text at start, fade out as we scroll
                    const termEl = document.getElementById('char-terminal');
                    if (termEl) {
                        if (p < 0.12) {
                            termEl.style.opacity = String(1 - (p / 0.12));
                            termEl.style.transform = `translate3d(0, ${-p * 120}px, 0)`;
                            termEl.style.pointerEvents = 'auto';
                        } else {
                            termEl.style.opacity = '0';
                            termEl.style.pointerEvents = 'none';
                        }
                    }

                    // Fade in the final text near end
                    const finalEl = document.getElementById('char-final');
                    if (finalEl) {
                        if (p > 0.88) {
                            const fp = (p - 0.88) / 0.12;
                            finalEl.style.opacity = String(fp);
                            finalEl.style.transform = `translate3d(0, ${20 - fp * 20}px, 0)`;
                        } else {
                            finalEl.style.opacity = '0';
                        }
                    }

                    ticking = false;
                };

                const onScroll = () => {
                    const rect = track.getBoundingClientRect();
                    // Only update when section is in viewport
                    if (rect.top <= window.innerHeight && rect.bottom >= 0) {
                        if (!ticking) {
                            requestAnimationFrame(update);
                            ticking = true;
                        }
                    }
                };

                // Start showing frame 1 as soon as first frame loads
                frames[0] && (frames[0].onload = () => setFrame(0));

                allLoaded.then(() => {
                    setFrame(0);
                    update();
                });

                window.addEventListener('scroll', onScroll, { passive: true });
                window.addEventListener('resize', update,   { passive: true });

                // Cleanup on hot-reload
                return () => {
                    window.removeEventListener('scroll', onScroll);
                    window.removeEventListener('resize', update);
                };
            })();
            "#
        );
    });

    rsx! {
        section { id: "skills", class: "char-skills-section blur-on-enter",

            // ── SCROLL TRACK (600vh gives enough scroll room for 50 frames) ──
            div { id: "char-scroll-track", class: "char-scroll-track",
                div { class: "char-sticky-view",

                    // ── Terminal intro text ──────────────────────────────────
                    div { id: "char-terminal", class: "char-terminal-text",
                        p { "CONNECTION LOST" }
                        p { "NETWORK OFFLINE" }
                        p { "DEVELOPER STILL RUNNING" }
                        span { class: "char-cursor", "_" }
                    }

                    // ── Skills content (left column) ─────────────────────────
                    div { class: "char-content-col",
                        h2 { class: "char-section-heading", "Skills" }
                        p { class: "char-section-sub",
                            "Technologies I work with every day — from systems design to shipping products."
                        }

                        // Core skill badges
                        div { class: "char-core-badges",
                            for cat in SKILL_CATEGORIES.iter().take(3) {
                                div { class: "char-badge-group",
                                    span { class: "char-badge-label", "{cat.name}" }
                                    div { class: "char-badge-tags",
                                        for skill in cat.skills.iter().take(4) {
                                            span { class: "char-badge-tag", "{skill}" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // ── Anime character frame (right column) ─────────────────
                    div { class: "char-frame-col",
                        div { class: "char-frame-container",
                            img {
                                id: "char-frame-img",
                                class: "char-frame-img",
                                src: "/assets/character/skills/frame-001.png",
                                alt: "Developer character animation",
                                draggable: "false",
                            }
                        }
                    }

                    // ── Final restored text ──────────────────────────────────
                    div { id: "char-final", class: "char-final-text",
                        p { class: "char-green", "CONNECTION RESTORED" }
                        br {}
                        p { "NETWORK       ONLINE" }
                        p { "SERVICES      ONLINE" }
                        p { "DEVELOPER     STILL RUNNING" }
                        br {}
                        h3 { "FULL STACK DEVELOPER" }
                    }
                }
            }

            // ── Secondary Skills Grid (unchanged, normal page flow) ──────────
            div { class: "char-secondary-skills container",
                div { class: "secondary-grid",
                    for category in SKILL_CATEGORIES {
                        div { class: "secondary-card",
                            h4 { class: "secondary-title", "{category.name}" }
                            div { class: "secondary-tags",
                                for skill in category.skills {
                                    span { class: "secondary-tag", "{skill}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
