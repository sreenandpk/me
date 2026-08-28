use dioxus::prelude::*;
use crate::data::SKILL_CATEGORIES;

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

                // Check mobile layout (we disable sticky timeline animation on mobile for UX/accessibility)
                const isMobile = () => window.innerWidth <= 768;

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

                // Easing, position, and velocity variables
                let targetFrame = 0;
                let currentFrame = 0;
                let targetX = 10;  // starting left %
                let currentX = 10;
                let targetY = 0;   // jumping Y offset px
                let currentY = 0;
                let currentV = 0;  // scroll velocity
                let lastP = 0;
                let lastT = Date.now();
                let lastIdx = -1;
                let isLoopRunning = false;

                // Create dust particles pool dynamically
                const dustContainer = document.getElementById('char-dust-container');
                const dustParticles = [];
                if (dustContainer) {
                    for (let i = 0; i < 8; i++) {
                        const p = document.createElement('div');
                        p.className = 'char-dust-particle';
                        dustContainer.appendChild(p);
                        dustParticles.push({
                            el: p,
                            x: 0,
                            y: 0,
                            vx: 0,
                            vy: 0,
                            life: 0,
                            active: false
                        });
                    }
                }

                const setFrame = (idx) => {
                    const clamped = Math.max(0, Math.min(TOTAL_FRAMES - 1, idx));
                    if (clamped !== lastIdx) {
                        lastIdx = clamped;
                        if (frames[clamped]?.complete) {
                            imgA.src = frames[clamped].src;
                        }
                    }
                };

                // Spawn a dust particle behind the character
                const spawnDust = () => {
                    if (isMobile()) return;
                    const p = dustParticles.find(part => !part.active);
                    if (!p) return;
                    p.active = true;
                    p.life = 1.0;
                    p.x = 0; // relative to container
                    p.y = 0;
                    p.vx = -(1.5 + Math.random() * 2); // blow backward
                    p.vy = -(Math.random() * 1.5);     // drift up
                    p.el.style.opacity = '0.35';
                    p.el.style.transform = 'translate3d(0, 0, 0) scale(1)';
                };

                // Continuous loop for inertial interpolation, speed streaks, and dust
                const loop = () => {
                    if (isMobile()) {
                        isLoopRunning = false;
                        return;
                    }

                    const rect = track.getBoundingClientRect();
                    const inView = rect.top <= window.innerHeight && rect.bottom >= 0;
                    if (!inView) {
                        isLoopRunning = false;
                        return;
                    }

                    // Lerp position
                    currentX += (targetX - currentX) * 0.12;
                    currentY += (targetY - currentY) * 0.12;
                    
                    // Lerp frame indices
                    const diff = targetFrame - currentFrame;
                    if (Math.abs(diff) > 0.01) {
                        currentFrame += diff * 0.12;
                        setFrame(Math.round(currentFrame));
                    } else {
                        currentFrame = targetFrame;
                        setFrame(Math.round(currentFrame));
                    }

                    // Apply character mover styles
                    const mover = document.getElementById('char-mover');
                    if (mover) {
                        mover.style.left = `${currentX}%`;
                        mover.style.transform = `translate3d(-50%, ${currentY}px, 0)`;
                    }

                    // Velocity calculation for environmental effects
                    const now = Date.now();
                    const dt = Math.max(1, now - lastT);
                    // Smoothing velocity
                    const currentP = -rect.top / (rect.height - window.innerHeight);
                    const instantV = Math.abs(currentP - lastP) / dt;
                    currentV += (instantV * 300 - currentV) * 0.1;
                    lastP = currentP;
                    lastT = now;

                    // Animate speed lines based on scroll velocity
                    const speedLines = document.querySelectorAll('.char-speed-line');
                    const time = now * 0.003;
                    speedLines.forEach((line, index) => {
                        if (currentV > 0.12) {
                            const offset = ((time * (100 + index * 40)) % window.innerWidth);
                            line.style.transform = `translate3d(${-offset}px, 0, 0) scaleX(${1 + currentV * 0.5})`;
                            line.style.opacity = String(Math.min(0.4, (currentV - 0.12) * 0.8));
                        } else {
                            line.style.opacity = '0';
                        }
                    });

                    // Occasionally spawn foot dust when moving
                    if (currentV > 0.2 && Math.random() < 0.3) {
                        spawnDust();
                    }

                    // Update dust particles physics
                    dustParticles.forEach(p => {
                        if (!p.active) return;
                        p.x += p.vx;
                        p.y += p.vy;
                        p.life -= 0.04;
                        if (p.life <= 0) {
                            p.active = false;
                            p.el.style.opacity = '0';
                        } else {
                            p.el.style.opacity = String(p.life * 0.35);
                            p.el.style.transform = `translate3d(${p.x}px, ${p.y}px, 0) scale(${0.5 + p.life * 0.8})`;
                        }
                    });

                    requestAnimationFrame(loop);
                };

                const updateTarget = () => {
                    if (isMobile()) return;

                    const rect     = track.getBoundingClientRect();
                    const vh       = window.innerHeight;
                    const maxScroll = rect.height - vh;

                    let p = -rect.top / maxScroll;
                    if (p < 0) p = 0;
                    if (p > 1) p = 1;

                    // Character horizontal coordinate (travels from 10% on left to 88% on right)
                    targetX = 10 + p * 78;

                    // Character Y coordinate logic for dynamic jumps
                    // Jump range is p from 0.78 to 0.92
                    if (p >= 0.78 && p <= 0.92) {
                        const jp = (p - 0.78) / 0.14; // normalized jump progress [0..1]
                        targetY = -Math.sin(jp * Math.PI) * 220; // 220px peak jump height
                    } else {
                        targetY = 0;
                    }

                    // Coordinate frame indices with scrolling phases
                    if (p < 0.75) {
                        // Phase 1 - 3: continuous running loop
                        const runCycleLoops = 5;
                        targetFrame = Math.round((p / 0.75) * runCycleLoops * 24) % 24;
                    } else if (p < 0.78) {
                        // Phase 4: slow down & prepare to jump (frames 25 to 30)
                        const t = (p - 0.75) / 0.03;
                        targetFrame = Math.round(25 + t * 5);
                    } else if (p < 0.92) {
                        // Phase 5: final jump rise and fall (frames 31 to 45)
                        const t = (p - 0.78) / 0.14;
                        targetFrame = Math.round(31 + t * 14);
                    } else {
                        // Phase 6: landing & settled accent state (frames 46 to 49)
                        const t = Math.min(1.0, (p - 0.92) / 0.08);
                        targetFrame = Math.round(46 + t * 3);
                    }

                    if (!isLoopRunning) {
                        isLoopRunning = true;
                        requestAnimationFrame(loop);
                    }

                    // ── Parallax Background Typography ───────────────────────
                    const bgTypography = document.getElementById('char-bg-text');
                    if (bgTypography) {
                        const pxOffset = (p - 0.5) * -150;
                        bgTypography.style.transform = `translate3d(calc(-50% + ${pxOffset}px), -50%, 0)`;
                    }

                    // ── Floating cards reveal timeline (p < 0.82) ────────────
                    const centers = [0.10, 0.20, 0.30, 0.40, 0.50, 0.60, 0.70, 0.78];
                    centers.forEach((center, idx) => {
                        const card = document.getElementById(`float-card-${idx}`);
                        if (card) {
                            if (p < 0.82) {
                                const dist = Math.abs(p - center);
                                const opacity = Math.max(0, 1 - dist / 0.10);
                                card.style.opacity = String(opacity);
                                const driftX = (p - center) * -120;
                                const driftY = (p - center) * -20;
                                card.style.transform = `translate3d(calc(-50% + ${driftX}px), calc(-50% + ${driftY}px), 0) scale(${0.9 + opacity * 0.1})`;
                                card.style.filter = `blur(${(1 - opacity) * 4}px)`;
                            } else {
                                card.style.opacity = '0';
                            }
                        }
                    });

                    // ── Final Settled Grid Showcase (fades in on jump, p >= 0.82) ──
                    const showcase = document.getElementById('char-final-showcase');
                    const floatingContainer = document.getElementById('char-floating-container');
                    
                    if (showcase) {
                        if (p >= 0.82) {
                            const showP = (p - 0.82) / 0.18; // normalized [0..1]
                            showcase.style.opacity = String(showP);
                            showcase.style.transform = `translate3d(0, ${(1 - showP) * 30}px, 0)`;
                            showcase.style.pointerEvents = showP > 0.5 ? 'auto' : 'none';
                        } else {
                            showcase.style.opacity = '0';
                            showcase.style.transform = 'translate3d(0, 30px, 0)';
                            showcase.style.pointerEvents = 'none';
                        }
                    }

                    if (floatingContainer) {
                        if (p >= 0.82) {
                            floatingContainer.style.opacity = '0';
                        } else {
                            floatingContainer.style.opacity = '1';
                        }
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

                    // ── Final Terminal Card: fade in during jump, fade out as grid settles ──
                    const finalEl = document.getElementById('char-final-status');
                    if (finalEl) {
                        if (p > 0.78 && p < 0.88) {
                            const fp = (p - 0.78) / 0.10;
                            finalEl.style.opacity   = String(fp);
                            finalEl.style.transform = `translate3d(0,${15 - fp * 15}px,0)`;
                        } else if (p >= 0.88 && p < 0.95) {
                            const fp = 1 - (p - 0.88) / 0.07;
                            finalEl.style.opacity   = String(fp);
                            finalEl.style.transform = `translate3d(0,${(1 - fp) * -15}px,0)`;
                        } else {
                            finalEl.style.opacity   = '0';
                            finalEl.style.transform = 'translate3d(0,-15px,0)';
                        }
                    }
                };

                const onScroll = () => {
                    updateTarget();
                };

                // Set initial properties
                imgA.src = '/character/skills/frame-001.png';
                
                // Set up speed lines initial coordinates
                const speedLines = document.querySelectorAll('.char-speed-line');
                const positions = [18, 38, 58, 78];
                speedLines.forEach((line, index) => {
                    line.style.top = positions[index] + '%';
                    line.style.right = '-150px';
                });

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

                    // ── Background typography ──────────────────────────────────
                    div { id: "char-bg-text", class: "char-bg-typography", "SKILLS" }

                    // ── Speed Lines ───────────────────────────────────────────
                    div { class: "char-speed-lines-container",
                        div { class: "char-speed-line" }
                        div { class: "char-speed-line" }
                        div { class: "char-speed-line" }
                        div { class: "char-speed-line" }
                    }

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

                    // ── Floating dynamic cards (Phase 1-3 reveals) ─────────────
                    div { id: "char-floating-container", class: "char-floating-container",
                        for (idx, cat) in SKILL_CATEGORIES.iter().enumerate() {
                            div {
                                id: "float-card-{idx}",
                                class: "char-float-card",
                                h3 { "{cat.name}" }
                                div { class: "char-float-tags",
                                    for skill in cat.skills.iter().take(4) {
                                        span { "{skill}" }
                                    }
                                }
                            }
                        }
                    }

                    // ── Character mover wrapper ────────────────────────────────
                    div { id: "char-mover", class: "char-mover",
                        div { id: "char-frame-container", class: "char-frame-container",
                            img {
                                id: "char-frame-a",
                                class: "char-frame-img",
                                src: "/character/skills/frame-001.png",
                                alt: "Developer character animation",
                                draggable: "false",
                            }
                        }
                        // Dust particles container
                        div { id: "char-dust-container", class: "char-dust-emitter" }
                    }

                    // ── Final Settled Grid Showcase (Phase 5 reveal) ──────────
                    div { id: "char-final-showcase", class: "char-final-showcase",
                        div { class: "char-grid-header",
                            h2 { class: "real-projects-3d-title", "SKILLS" }
                            p { "Technologies I work with every day — from systems design to shipping products." }
                        }
                        div { class: "char-grid-layout",
                            for cat in SKILL_CATEGORIES.iter() {
                                div { class: "char-grid-card",
                                    h4 { 
                                        span { class: "char-card-icon", "{cat.icon}" }
                                        span { "{cat.name}" }
                                    }
                                    div { class: "char-grid-tags",
                                        for skill in cat.skills.iter() {
                                            span { class: "char-grid-tag", "{skill}" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // ── Final Status Terminal Card ──────────────────────────────
                    div { id: "char-final-status", class: "char-terminal-card char-final-card",
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

            // ── Secondary Skills Grid (statically visible on Mobile ONLY) ────
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
