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
                    if (imgA) imgA.src = '/character/skills/frame-025.png';
                    return;
                }

                const imgA = document.getElementById('char-frame-a');
                if (!imgA) return;

                // ── Preload all frames ───────────────────────────────────────
                const frames = [];
                let loadedCount = 0;
                const allLoaded = new Promise((resolve) => {
                    for (let i = 1; i <= TOTAL_FRAMES; i++) {
                        const img = new Image();
                        const n   = String(i).padStart(3, '0');
                        img.src   = `/character/skills/frame-${n}.png`;
                        img.onload  = () => { loadedCount++; if (loadedCount === TOTAL_FRAMES) resolve(); };
                        img.onerror = () => { loadedCount++; if (loadedCount === TOTAL_FRAMES) resolve(); };
                        frames.push(img);
                    }
                });

                let targetFrame = 0;
                let currentFrame = 0;
                let lastIdx = -1;
                let isLoopRunning = false;

                const setFrame = (idx) => {
                    const clamped = Math.max(0, Math.min(TOTAL_FRAMES - 1, idx));
                    if (clamped !== lastIdx) {
                        lastIdx = clamped;
                        if (frames[clamped]?.complete) {
                            imgA.src = frames[clamped].src;
                        }
                    }
                };

                // Continuous loop for butter-smooth inertial interpolation (lerp)
                const loop = () => {
                    const rect = track.getBoundingClientRect();
                    const inView = rect.top <= window.innerHeight && rect.bottom >= 0;

                    if (!inView) {
                        isLoopRunning = false;
                        return;
                    }

                    // Smooth lerping to targets
                    const diff = targetFrame - currentFrame;
                    if (Math.abs(diff) > 0.01) {
                        currentFrame += diff * 0.12; // Damping constant for fluid feel
                        setFrame(Math.round(currentFrame));
                    } else {
                        currentFrame = targetFrame;
                        setFrame(Math.round(currentFrame));
                    }

                    requestAnimationFrame(loop);
                };

                const updateTarget = () => {
                    const rect     = track.getBoundingClientRect();
                    const vh       = window.innerHeight;
                    const maxScroll = rect.height - vh;

                    let p = -rect.top / maxScroll;
                    if (p < 0) p = 0;
                    if (p > 1) p = 1;

                    // Cubic ease-in-out for responsive scroll physics
                    const ep = p < 0.5
                        ? 4 * p * p * p
                        : 1 - Math.pow(-2 * p + 2, 3) / 2;

                    targetFrame = ep * (TOTAL_FRAMES - 1);

                    if (!isLoopRunning) {
                        isLoopRunning = true;
                        requestAnimationFrame(loop);
                    }

                    // ── Intro Terminal Card: fade out first 12% ──────────────
                    const termEl = document.getElementById('char-terminal');
                    if (termEl) {
                        if (p < 0.12) {
                            const t = 1 - (p / 0.12);
                            termEl.style.opacity   = String(t);
                            termEl.style.transform = `translate3d(0,${-p * 60}px,0)`;
                        } else {
                            termEl.style.opacity   = '0';
                            termEl.style.transform = `translate3d(0,-60px,0)`;
                        }
                    }

                    // ── Final Terminal Card: fade in last 12% ───────────────
                    const finalEl = document.getElementById('char-final');
                    if (finalEl) {
                        if (p > 0.88) {
                            const fp = (p - 0.88) / 0.12;
                            finalEl.style.opacity   = String(fp);
                            finalEl.style.transform = `translate3d(0,${15 - fp * 15}px,0)`;
                        } else {
                            finalEl.style.opacity   = '0';
                            finalEl.style.transform = 'translate3d(0,15px,0)';
                        }
                    }
                };

                const onScroll = () => {
                    updateTarget();
                };

                imgA.src = '/character/skills/frame-001.png';

                allLoaded.then(() => {
                    updateTarget();
                });

                window.addEventListener('scroll', onScroll, { passive: true });
                window.addEventListener('resize', updateTarget,   { passive: true });

                return () => {
                    window.removeEventListener('scroll', onScroll);
                    window.removeEventListener('resize', updateTarget);
                };
            })();
            "#
        );
    });

    rsx! {
        section { id: "skills", class: "char-skills-section blur-on-enter",

            // ── SCROLL TRACK ──
            div { id: "char-scroll-track", class: "char-scroll-track",
                div { class: "char-sticky-view",

                    // ── Intro Terminal Card ──────────────────────────────────────
                    div { id: "char-terminal", class: "char-terminal-card",
                        div { class: "char-terminal-header",
                            div { class: "char-terminal-dot red" }
                            div { class: "char-terminal-dot yellow" }
                            div { class: "char-terminal-dot green" }
                            span { class: "char-terminal-title", "status_monitor.sh" }
                        }
                        div { class: "char-terminal-body",
                            p { class: "char-red", "► STATUS: CONNECTION LOST" }
                            p { class: "char-red", "► NETWORK: OFFLINE" }
                            p { class: "char-blink-text", "► SEARCHING FOR HOST..." }
                            span { class: "char-cursor", "_" }
                        }
                    }

                    // ── Skills content (left column) ─────────────────────────────
                    div { class: "char-content-col",
                        h2 { class: "real-projects-3d-title", "SKILLS" }
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

                    // ── Anime character (right column) ──
                    div { class: "char-frame-col",
                        div { class: "char-frame-container",
                            img {
                                id: "char-frame-a",
                                class: "char-frame-img",
                                src: "/character/skills/frame-001.png",
                                alt: "Developer character animation",
                                draggable: "false",
                            }
                        }
                    }

                    // ── Final Terminal Card ──────────────────────────────────────
                    div { id: "char-final", class: "char-terminal-card char-final-card",
                        div { class: "char-terminal-header",
                            div { class: "char-terminal-dot red" }
                            div { class: "char-terminal-dot yellow" }
                            div { class: "char-terminal-dot green" }
                            span { class: "char-terminal-title", "system_status.sh" }
                        }
                        div { class: "char-terminal-body",
                            p { class: "char-green", "► STATUS: CONNECTION RESTORED" }
                            p { class: "char-green", "► NETWORK: ONLINE" }
                            p { class: "char-green", "► SERVICES: ACTIVE" }
                            br {}
                            h3 { class: "char-final-h3", "► DEVELOPER ONLINE" }
                        }
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
