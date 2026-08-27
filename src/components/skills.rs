use dioxus::prelude::*;
use crate::data::SKILL_CATEGORIES;

const TOTAL_FRAMES: usize = 50;

#[component]
pub fn Skills() -> Element {
    use_effect(move || {
        let _ = document::eval(
            r#"
            (function initCharacterScroll() {
                const TOTAL_FRAMES = 50;
                const track = document.getElementById('char-scroll-track');
                if (!track) return;

                // ── Reduced motion fallback ──────────────────────────────────
                if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
                    const imgA = document.getElementById('char-frame-a');
                    if (imgA) imgA.src = '/assets/character/skills/frame-025.png';
                    return;
                }

                const imgA = document.getElementById('char-frame-a');
                const imgB = document.getElementById('char-frame-b');
                if (!imgA || !imgB) return;

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

                let lastExact = -1;
                let ticking   = false;

                // ── Smooth cross-fade between consecutive frames ──────────────
                // imgA shows floor frame, imgB overlays ceil frame at fractional opacity
                const setBlendedFrame = (exactFrame) => {
                    if (Math.abs(exactFrame - lastExact) < 0.001) return;
                    lastExact = exactFrame;

                    const idxA = Math.max(0, Math.min(TOTAL_FRAMES - 1, Math.floor(exactFrame)));
                    const idxB = Math.max(0, Math.min(TOTAL_FRAMES - 1, idxA + 1));
                    const blend = exactFrame - idxA;  // 0..1 fractional part

                    if (frames[idxA]?.complete) imgA.src = frames[idxA].src;
                    if (frames[idxB]?.complete) {
                        imgB.src     = frames[idxB].src;
                        imgB.style.opacity = String(blend.toFixed(3));
                    } else {
                        imgB.style.opacity = '0';
                    }
                };

                const update = () => {
                    const rect     = track.getBoundingClientRect();
                    const vh       = window.innerHeight;
                    const maxScroll = rect.height - vh;

                    let p = -rect.top / maxScroll;
                    if (p < 0) p = 0;
                    if (p > 1) p = 1;

                    // Smooth ease: slow-in + slow-out so the run cycle feels natural
                    // Using cubic ease-in-out on p
                    const ep = p < 0.5
                        ? 4 * p * p * p
                        : 1 - Math.pow(-2 * p + 2, 3) / 2;

                    // Map eased progress → exact frame position (continuous)
                    const exactFrame = ep * (TOTAL_FRAMES - 1);
                    setBlendedFrame(exactFrame);

                    // ── Intro text: fade out first 10% ───────────────────────
                    const termEl = document.getElementById('char-terminal');
                    if (termEl) {
                        if (p < 0.10) {
                            const t = 1 - (p / 0.10);
                            termEl.style.opacity   = String(t);
                            termEl.style.transform = `translate3d(0,${-p * 80}px,0)`;
                        } else {
                            termEl.style.opacity   = '0';
                            termEl.style.transform = `translate3d(0,-80px,0)`;
                        }
                    }

                    // ── Final text: fade in last 10% ─────────────────────────
                    const finalEl = document.getElementById('char-final');
                    if (finalEl) {
                        if (p > 0.90) {
                            const fp = (p - 0.90) / 0.10;
                            finalEl.style.opacity   = String(fp);
                            finalEl.style.transform = `translate3d(0,${20 - fp * 20}px,0)`;
                        } else {
                            finalEl.style.opacity   = '0';
                            finalEl.style.transform = 'translate3d(0,20px,0)';
                        }
                    }

                    ticking = false;
                };

                const onScroll = () => {
                    const rect = track.getBoundingClientRect();
                    if (rect.top <= window.innerHeight && rect.bottom >= 0) {
                        if (!ticking) {
                            requestAnimationFrame(update);
                            ticking = true;
                        }
                    }
                };

                // Show first frame immediately
                imgA.src = '/assets/character/skills/frame-001.png';
                imgB.style.opacity = '0';

                allLoaded.then(() => {
                    setBlendedFrame(0);
                    update();
                });

                window.addEventListener('scroll', onScroll, { passive: true });
                window.addEventListener('resize', update,   { passive: true });

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

            // ── SCROLL TRACK — 700vh gives a leisurely animation pace ──────────
            div { id: "char-scroll-track", class: "char-scroll-track",
                div { class: "char-sticky-view",

                    // ── Terminal intro text ──────────────────────────────────────
                    div { id: "char-terminal", class: "char-terminal-text",
                        p { "CONNECTION LOST" }
                        p { "NETWORK OFFLINE" }
                        p { "DEVELOPER STILL RUNNING" }
                        span { class: "char-cursor", "_" }
                    }

                    // ── Skills content (left column) ─────────────────────────────
                    div { class: "char-content-col",
                        h2 { class: "char-section-heading", "Skills" }
                        p { class: "char-section-sub",
                            "Technologies I work with every day — from systems design to shipping products."
                        }

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

                    // ── Anime character (right column) — two images for blending ──
                    div { class: "char-frame-col",
                        div { class: "char-frame-container",
                            // Layer A: base frame (opacity always 1)
                            img {
                                id: "char-frame-a",
                                class: "char-frame-img",
                                src: "/assets/character/skills/frame-001.png",
                                alt: "Developer character animation",
                                draggable: "false",
                            }
                            // Layer B: next frame (opacity 0→1 for smooth blend)
                            img {
                                id: "char-frame-b",
                                class: "char-frame-img char-frame-blend",
                                src: "/assets/character/skills/frame-001.png",
                                alt: "",
                                draggable: "false",
                                aria_hidden: "true",
                            }
                        }
                    }

                    // ── Final text ───────────────────────────────────────────────
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

            // ── Secondary Skills Grid (normal page flow, unchanged) ────────────
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
