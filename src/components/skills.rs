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

                // Check mobile layout
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
                let targetX = 15;  // horizontal starting left %
                let currentX = 15;
                let targetY = window.innerHeight * 0.22; // vertical starting px
                let currentY = window.innerHeight * 0.22;
                let targetYOffset = 0; // jump offset
                let currentYOffset = 0;
                let lastP = 0;
                let lastIdx = -1;
                let isLoopRunning = false;

                // Create dust particles pool dynamically
                const dustContainer = document.getElementById('char-dust-container');
                const dustParticles = [];
                if (dustContainer) {
                    for (let i = 0; i < 5; i++) {
                        const p = document.createElement('div');
                        p.className = 'char-dust-particle';
                        dustContainer.appendChild(p);
                        dustParticles.push({
                            el: p, x: 0, y: 0, vx: 0, vy: 0, life: 0, active: false
                        });
                    }
                }

                // Create neon scatter particles for jump
                const neonContainer = document.getElementById('char-neon-dots');
                const neonParticles = [];
                if (neonContainer) {
                    for (let i = 0; i < 8; i++) {
                        const d = document.createElement('div');
                        d.className = 'char-neon-dot';
                        neonContainer.appendChild(d);
                        neonParticles.push({
                            el: d, x: 0, y: 0, vx: 0, vy: 0, life: 0, active: false
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

                const spawnDust = () => {
                    if (isMobile()) return;
                    const p = dustParticles.find(part => !part.active);
                    if (!p) return;
                    p.active = true;
                    p.life = 1.0;
                    p.x = 0;
                    p.y = 0;
                    p.vx = -(0.8 + Math.random() * 1.2);
                    p.vy = -(Math.random() * 0.8);
                    p.el.style.opacity = '0.25';
                    p.el.style.transform = 'translate3d(0, 0, 0) scale(1)';
                };

                const spawnNeonDot = () => {
                    if (isMobile()) return;
                    const d = neonParticles.find(part => !part.active);
                    if (!d) return;
                    d.active = true;
                    d.life = 1.0;
                    d.x = (Math.random() - 0.5) * 30;
                    d.y = 0;
                    d.vx = (Math.random() - 0.5) * 0.8;
                    d.vy = 1.5 + Math.random() * 2.0; // fall down trail
                    d.el.style.opacity = '0.6';
                    d.el.style.transform = `translate3d(${d.x}px, ${d.y}px, 0) scale(1)`;
                };

                // Continuous loop for physics & particles
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

                    // Lerp coordinates
                    currentX += (targetX - currentX) * 0.12;
                    currentY += (targetY - currentY) * 0.10; // slightly slower vertical alignment
                    currentYOffset += (targetYOffset - currentYOffset) * 0.12;
                    
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
                        mover.style.top = `${currentY}px`;
                        mover.style.transform = `translate3d(-50%, ${currentYOffset}px, 0)`;
                    }

                    // Spawn dust particles during movement
                    const moverEl = document.getElementById('char-mover');
                    if (moverEl && moverEl.classList.contains('moving') && Math.random() < 0.2) {
                        spawnDust();
                    }

                    // Neon particles trail during jump
                    if (currentYOffset < -20 && Math.random() < 0.3) {
                        spawnNeonDot();
                    }

                    // Update dust particles
                    dustParticles.forEach(p => {
                        if (!p.active) return;
                        p.x += p.vx;
                        p.y += p.vy;
                        p.life -= 0.05;
                        if (p.life <= 0) {
                            p.active = false;
                            p.el.style.opacity = '0';
                        } else {
                            p.el.style.opacity = String(p.life * 0.25);
                            p.el.style.transform = `translate3d(${p.x}px, ${p.y}px, 0) scale(${0.5 + p.life * 0.5})`;
                        }
                    });

                    // Update neon particles
                    neonParticles.forEach(d => {
                        if (!d.active) return;
                        d.x += d.vx;
                        d.y += d.vy;
                        d.life -= 0.04;
                        if (d.life <= 0) {
                            d.active = false;
                            d.el.style.opacity = '0';
                        } else {
                            d.el.style.opacity = String(d.life * 0.6);
                            d.el.style.transform = `translate3d(${d.x}px, ${d.y}px, 0) scale(${0.6 + d.life * 0.6})`;
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

                    // ── 3-Row Vertical Alignment (Grounded under row underlines) ──
                    if (p < 0.28) {
                        targetY = vh * 0.24; // Row 1 Y
                    } else if (p < 0.58) {
                        targetY = vh * 0.52; // Row 2 Y
                    } else {
                        targetY = vh * 0.80; // Row 3 Y
                    }

                    // ── 3-Stage Horizontal Burst Movement (Short Runs) ──
                    if (p < 0.28) {
                        const t = Math.min(1.0, p / 0.20);
                        targetX = 18 + t * 20; // Row 1: short run 18% -> 38%
                    } else if (p < 0.58) {
                        const t = Math.min(1.0, (p - 0.28) / 0.20);
                        targetX = 26 + t * 20; // Row 2: short run 26% -> 46%
                    } else if (p < 0.80) {
                        const t = Math.min(1.0, (p - 0.58) / 0.20);
                        targetX = 35 + t * 20; // Row 3: short run 35% -> 55%
                    } else {
                        const t = Math.min(1.0, (p - 0.80) / 0.12);
                        targetX = 55 + t * 8;  // Climax / Jump prep: move slightly to 63%
                    }

                    // ── Vertical Jumping Curve (Stage 4 Climax, Short/Tasteful Peak) ──
                    if (p >= 0.80 && p <= 0.92) {
                        const jp = (p - 0.80) / 0.12;
                        targetYOffset = -Math.sin(jp * Math.PI) * 140; // 140px peak height
                    } else {
                        targetYOffset = 0;
                    }

                    // ── Moving state detection for speed lines & dust ──
                    let isMoving = false;
                    let frameSelection = 0;

                    if (p < 0.20) {
                        const t = p / 0.20;
                        frameSelection = Math.round(t * 2 * 24) % 24;
                        isMoving = true;
                    } else if (p >= 0.20 && p < 0.28) {
                        frameSelection = 0; // idle
                    } else if (p >= 0.28 && p < 0.48) {
                        const t = (p - 0.28) / 0.20;
                        frameSelection = Math.round(t * 2 * 24) % 24;
                        isMoving = true;
                    } else if (p >= 0.48 && p < 0.58) {
                        frameSelection = 0; // idle
                    } else if (p >= 0.58 && p < 0.78) {
                        const t = (p - 0.58) / 0.20;
                        frameSelection = Math.round(t * 2 * 24) % 24;
                        isMoving = true;
                    } else if (p >= 0.78 && p < 0.80) {
                        frameSelection = 0; // idle
                    } else if (p >= 0.80 && p < 0.82) {
                        const t = (p - 0.80) / 0.02;
                        frameSelection = Math.round(25 + t * 5); // crouch
                    } else if (p >= 0.82 && p < 0.92) {
                        const t = (p - 0.82) / 0.10;
                        frameSelection = Math.round(31 + t * 14); // jump frames
                        isMoving = true;
                    } else {
                        const t = Math.min(1.0, (p - 0.92) / 0.08);
                        frameSelection = Math.round(46 + t * 3); // land
                    }

                    targetFrame = frameSelection;

                    const mover = document.getElementById('char-mover');
                    if (mover) {
                        if (isMoving) {
                            mover.classList.add('moving');
                        } else {
                            mover.classList.remove('moving');
                        }
                    }

                    if (!isLoopRunning) {
                        isLoopRunning = true;
                        requestAnimationFrame(loop);
                    }

                    // ── Category Reveals ──
                    const setCategoryActive = (idx, active) => {
                        const group = document.getElementById(`cat-group-${idx}`);
                        if (group) {
                            if (active) {
                                group.classList.add('active');
                            } else {
                                group.classList.remove('active');
                            }
                        }
                    };

                    setCategoryActive(0, p >= 0.14); // Languages
                    setCategoryActive(3, p >= 0.14); // Database
                    setCategoryActive(1, p >= 0.42); // Frontend
                    setCategoryActive(5, p >= 0.42); // Tools
                    setCategoryActive(2, p >= 0.70); // Backend
                    setCategoryActive(6, p >= 0.70); // Collaboration
                    setCategoryActive(4, p >= 0.78); // DevOps
                    setCategoryActive(7, p >= 0.78); // Concepts

                    // ── Parallax Background Typography ──
                    const bgTypography = document.getElementById('char-bg-text');
                    if (bgTypography) {
                        const pxOffset = (p - 0.5) * -80;
                        bgTypography.style.transform = `translate3d(calc(-50% + ${pxOffset}px), -50%, 0)`;
                    }

                    // ── Character visibility near exit ──
                    if (mover) {
                        if (p >= 0.90) {
                            const charOpacity = 1 - (p - 0.90) / 0.08;
                            mover.style.opacity = String(Math.max(0, charOpacity));
                        } else {
                            mover.style.opacity = '1';
                        }
                    }

                    // ── Cinematic Dissolve Transition at the End (p >= 0.94) ──
                    const stickyView = document.querySelector('.char-sticky-view');
                    if (stickyView) {
                        if (p >= 0.94) {
                            const dissolveP = 1 - (p - 0.94) / 0.06;
                            stickyView.style.opacity = String(dissolveP);
                            stickyView.style.transform = `scale(${0.98 + dissolveP * 0.02})`;
                        } else {
                            stickyView.style.opacity = '1';
                            stickyView.style.transform = 'scale(1)';
                        }
                    }
                };

                const onScroll = () => {
                    updateTarget();
                };

                // Initialize properties
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

    // Helper closure to render categories cardless with inline stagger transition-delay
    let render_cat = |idx: usize| {
        let cat = &SKILL_CATEGORIES[idx];
        rsx! {
            div {
                id: "cat-group-{idx}",
                class: "char-category-group",
                h4 { class: "char-category-title",
                    span { class: "char-card-icon", "{cat.icon}" }
                    span { "{cat.name}" }
                }
                div { class: "char-category-line" }
                div { class: "char-tech-tags",
                    for (t_idx, skill) in cat.skills.iter().enumerate() {
                        span {
                            class: "char-tech-tag",
                            style: "transition-delay: {t_idx as f32 * 0.04}s;",
                            "{skill}"
                        }
                    }
                }
            }
        }
    };

    rsx! {
        section { id: "skills", class: "char-skills-section blur-on-enter",

            // ── SCROLL TRACK (Shorter 2.0vh Height for fast unpinning) ──
            div { id: "char-scroll-track", class: "char-scroll-track",
                div { class: "char-sticky-view",

                    // ── Background typography ──
                    div { id: "char-bg-text", class: "char-bg-typography", "SKILLS" }

                    // ── Editorial Container (NO CARDS) ──
                    div { class: "char-editorial-container",
                        div { class: "char-grid-header",
                            h2 { class: "real-projects-3d-title", "SKILLS" }
                            p { "Technologies I work with every day — from systems design to shipping products." }
                        }
                        
                        div { class: "char-editorial-columns",
                            // Row 1: Languages & Databases
                            div { id: "skills-row-1", class: "char-skills-row",
                                div { class: "char-row-left", {render_cat(0)} }
                                div { class: "char-row-right", {render_cat(3)} }
                            }
                            // Row 2: Frontend & Tools
                            div { id: "skills-row-2", class: "char-skills-row",
                                div { class: "char-row-left", {render_cat(1)} }
                                div { class: "char-row-right", {render_cat(5)} }
                            }
                            // Row 3: Backend & DevOps
                            div { id: "skills-row-3", class: "char-skills-row",
                                div { class: "char-row-left", 
                                    {render_cat(2)},
                                    {render_cat(6)} 
                                }
                                div { class: "char-row-right", 
                                    {render_cat(4)},
                                    {render_cat(7)} 
                                }
                            }
                        }
                    }

                    // ── Character mover wrapper ──
                    div { id: "char-mover", class: "char-mover",
                        // Subtle speed lines behind character when moving
                        div { class: "char-mover-speed-lines",
                            div { class: "char-mover-streak" }
                            div { class: "char-mover-streak" }
                            div { class: "char-mover-streak" }
                        }
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
                        // Neon scatter dots for jump
                        div { id: "char-neon-dots", class: "char-neon-dots" }
                    }
                }
            }

            // ── Static Mobile Fallback (Normal Flow, Mobile-Only) ──
            div { class: "char-mobile-fallback container",
                div { class: "char-grid-header",
                    h2 { class: "real-projects-3d-title", "SKILLS" }
                    p { "Technologies I work with every day — from systems design to shipping products." }
                }
                div { class: "char-mobile-list",
                    for cat in SKILL_CATEGORIES {
                        div { class: "char-mobile-group",
                            h4 { 
                                span { class: "char-card-icon", "{cat.icon}" }
                                span { "{cat.name}" }
                            }
                            div { class: "char-category-line active" }
                            div { class: "char-mobile-tags",
                                for skill in cat.skills {
                                    span { class: "char-tech-tag active", "{skill}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
