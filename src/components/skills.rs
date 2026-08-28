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
                let targetX = -15;  // start offscreen left
                let currentX = -15;
                let targetY = 0;
                let currentY = 0;
                let currentV = 0;
                let lastP = 0;
                let lastT = Date.now();
                let lastIdx = -1;
                let isLoopRunning = false;

                // Create dust particles pool dynamically
                const dustContainer = document.getElementById('char-dust-container');
                const dustParticles = [];
                if (dustContainer) {
                    for (let i = 0; i < 6; i++) {
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
                    for (let i = 0; i < 10; i++) {
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
                    p.vx = -(1.0 + Math.random() * 1.5);
                    p.vy = -(Math.random() * 1.0);
                    p.el.style.opacity = '0.3';
                    p.el.style.transform = 'translate3d(0, 0, 0) scale(1)';
                };

                const spawnNeonDot = () => {
                    if (isMobile()) return;
                    const d = neonParticles.find(part => !part.active);
                    if (!d) return;
                    d.active = true;
                    d.life = 1.0;
                    d.x = (Math.random() - 0.5) * 40; // horizontal scatter
                    d.y = 0;
                    d.vx = (Math.random() - 0.5) * 1.0;
                    d.vy = 2.0 + Math.random() * 3.0; // fall down fast
                    d.el.style.opacity = '0.7';
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

                    // Lerp horizontal and vertical positions
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

                    // Velocity calculation
                    const now = Date.now();
                    const dt = Math.max(1, now - lastT);
                    const currentP = -rect.top / (rect.height - window.innerHeight);
                    const instantV = Math.abs(currentP - lastP) / dt;
                    currentV += (instantV * 300 - currentV) * 0.1;
                    lastP = currentP;
                    lastT = now;

                    // Animate speed lines
                    const speedLines = document.querySelectorAll('.char-speed-line');
                    const time = now * 0.003;
                    speedLines.forEach((line, index) => {
                        if (currentV > 0.12) {
                            const offset = ((time * (120 + index * 50)) % window.innerWidth);
                            line.style.transform = `translate3d(${-offset}px, 0, 0) scaleX(${1 + currentV * 0.4})`;
                            line.style.opacity = String(Math.min(0.3, (currentV - 0.12) * 0.6));
                        } else {
                            line.style.opacity = '0';
                        }
                    });

                    // Dust particles emission
                    if (currentV > 0.15 && Math.random() < 0.25) {
                        spawnDust();
                    }

                    // Neon particles emission (triggered when character is high in the air during jump)
                    if (currentY < -30 && Math.random() < 0.4) {
                        spawnNeonDot();
                    }

                    // Update dust particles
                    dustParticles.forEach(p => {
                        if (!p.active) return;
                        p.x += p.vx;
                        p.y += p.vy;
                        p.life -= 0.04;
                        if (p.life <= 0) {
                            p.active = false;
                            p.el.style.opacity = '0';
                        } else {
                            p.el.style.opacity = String(p.life * 0.3);
                            p.el.style.transform = `translate3d(${p.x}px, ${p.y}px, 0) scale(${0.5 + p.life * 0.7})`;
                        }
                    });

                    // Update neon particles
                    neonParticles.forEach(d => {
                        if (!d.active) return;
                        d.x += d.vx;
                        d.y += d.vy;
                        d.life -= 0.03;
                        if (d.life <= 0) {
                            d.active = false;
                            d.el.style.opacity = '0';
                        } else {
                            d.el.style.opacity = String(d.life * 0.7);
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

                    // ── 4-Stage Horizontal Pathing ──
                    if (p < 0.22) {
                        const t = p / 0.22;
                        targetX = -15 + t * 35; // enter offscreen left -> 20%
                    } else if (p < 0.48) {
                        const t = (p - 0.22) / 0.26;
                        targetX = 20 + t * 30; // run from 20% -> 50%
                    } else if (p < 0.74) {
                        const t = (p - 0.48) / 0.26;
                        targetX = 50 + t * 30; // run from 50% -> 80%
                    } else {
                        const t = Math.min(1.0, (p - 0.74) / 0.26);
                        targetX = 80 + t * 8; // run from 80% -> 88%
                    }

                    // ── Vertical Jumping Curve (Stage 4 Climax) ──
                    if (p >= 0.76 && p <= 0.94) {
                        const jp = (p - 0.76) / 0.18;
                        targetY = -Math.sin(jp * Math.PI) * 280; // Peak jump height 280px
                    } else {
                        targetY = 0;
                    }

                    // ── Frame state coordination based on movement speed & pauses ──
                    let frameSelection = 0;

                    if (p < 0.16) {
                        const t = p / 0.16;
                        frameSelection = Math.round(t * 3 * 24) % 24; // running
                    } else if (p >= 0.16 && p < 0.22) {
                        frameSelection = 0; // idle standing
                    } else if (p >= 0.22 && p < 0.42) {
                        const t = (p - 0.22) / 0.20;
                        frameSelection = Math.round(t * 3 * 24) % 24; // running
                    } else if (p >= 0.42 && p < 0.48) {
                        frameSelection = 0; // idle standing
                    } else if (p >= 0.48 && p < 0.68) {
                        const t = (p - 0.48) / 0.20;
                        frameSelection = Math.round(t * 3 * 24) % 24; // running
                    } else if (p >= 0.68 && p < 0.74) {
                        frameSelection = 0; // idle standing
                    } else if (p >= 0.74 && p < 0.76) {
                        const t = (p - 0.74) / 0.02;
                        frameSelection = Math.round(25 + t * 5); // crouch prep
                    } else if (p >= 0.76 && p < 0.94) {
                        const t = (p - 0.76) / 0.18;
                        frameSelection = Math.round(31 + t * 14); // rising/falling jump frames
                    } else {
                        const t = Math.min(1.0, (p - 0.94) / 0.06);
                        frameSelection = Math.round(46 + t * 3); // landing & final stand
                    }

                    targetFrame = frameSelection;

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

                    setCategoryActive(0, p >= 0.12);
                    setCategoryActive(1, p >= 0.38);
                    setCategoryActive(2, p >= 0.64);
                    setCategoryActive(3, p >= 0.64);
                    setCategoryActive(4, p >= 0.82);
                    setCategoryActive(5, p >= 0.82);
                    setCategoryActive(6, p >= 0.82);
                    setCategoryActive(7, p >= 0.82);

                    // ── Parallax Background Typography ──
                    const bgTypography = document.getElementById('char-bg-text');
                    if (bgTypography) {
                        const pxOffset = (p - 0.5) * -120;
                        bgTypography.style.transform = `translate3d(calc(-50% + ${pxOffset}px), -50%, 0)`;
                    }

                    // ── Character visibility near exit ──
                    const mover = document.getElementById('char-mover');
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
                            stickyView.style.transform = `scale(${0.97 + dissolveP * 0.03})`;
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
                
                // Initialize speed lines top heights
                const speedLines = document.querySelectorAll('.char-speed-line');
                const positions = [18, 38, 58, 78];
                speedLines.forEach((line, index) => {
                    line.style.top = positions[index] + '%';
                    line.style.right = '-180px';
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

    // Helper closure to render category lists cardless
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
                            style: "transition-delay: {t_idx as f32 * 0.06}s;",
                            "{skill}"
                        }
                    }
                }
            }
        }
    };

    rsx! {
        section { id: "skills", class: "char-skills-section blur-on-enter",

            // ── SCROLL TRACK (Cinematic 3.5vh Height) ──
            div { id: "char-scroll-track", class: "char-scroll-track",
                div { class: "char-sticky-view",

                    // ── Background massive outline typography ──
                    div { id: "char-bg-text", class: "char-bg-typography", "SKILLS" }

                    // ── Environmental speed lines ──
                    div { class: "char-speed-lines-container",
                        div { class: "char-speed-line" }
                        div { class: "char-speed-line" }
                        div { class: "char-speed-line" }
                        div { class: "char-speed-line" }
                    }

                    // ── Editorial Columns (NO CARDS) ──
                    div { class: "char-editorial-container",
                        div { class: "char-grid-header",
                            h2 { class: "real-projects-3d-title", "SKILLS" }
                            p { "Technologies I work with every day — from systems design to shipping products." }
                        }
                        
                        div { class: "char-editorial-columns",
                            // Left Column (Languages & Frontend)
                            div { class: "char-editorial-column",
                                {render_cat(0)},
                                {render_cat(1)},
                            }
                            // Center Column (Backend & Database)
                            div { class: "char-editorial-column",
                                {render_cat(2)},
                                {render_cat(3)},
                            }
                            // Right Column (DevOps, Tools, Collaboration, Concepts)
                            div { class: "char-editorial-column",
                                {render_cat(4)},
                                {render_cat(5)},
                                {render_cat(6)},
                                {render_cat(7)},
                            }
                        }
                    }

                    // ── Character mover with backlight neon aura ──
                    div { id: "char-mover", class: "char-mover",
                        div { class: "char-neon-aura" }
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
