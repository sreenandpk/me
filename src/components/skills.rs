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

                // Coordinates and easing physics
                let targetFrame = 0;
                let currentFrame = 0;
                let targetX = 12;
                let currentX = -15;
                let targetYOffset = 0; // jump vertical displacement
                let currentYOffset = 0;
                let lastP = 0;
                let isLoopRunning = false;

                // Emitters and particle systems removed

                const setFrame = (idx) => {
                    const clamped = Math.max(0, Math.min(TOTAL_FRAMES - 1, idx));
                    if (clamped !== lastIdx) {
                        lastIdx = clamped;
                        if (frames[clamped]?.complete) {
                            imgA.src = frames[clamped].src;
                        }
                    }
                };

                let lastIdx = -1;



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
                    currentYOffset += (targetYOffset - currentYOffset) * 0.12;

                    // Sync frame advancement with actual horizontal delta distance
                    const dx = targetX - currentX;
                    const isMoving = Math.abs(dx) > 0.08;

                    const mover = document.getElementById('char-mover');
                    if (mover) {
                        mover.style.left = `${currentX}%`;
                        mover.style.transform = `translate3d(-50%, ${currentYOffset}px, 0)`;

                        if (isMoving) {
                            mover.classList.add('moving');
                            // Advance running frames proportional to speed
                            currentFrame += Math.abs(dx) * 0.6 + 0.08;
                            if (currentFrame >= 24) currentFrame = 0;
                            setFrame(Math.round(currentFrame));
                        } else {
                            mover.classList.remove('moving');
                            setFrame(0); // Standing frame when stationary
                        }
                    }

                    // Emitters update logic removed

                    requestAnimationFrame(loop);
                };

                const updateTarget = () => {
                    if (isMobile()) return;

                    const rect     = track.getBoundingClientRect();
                    const vh       = window.innerHeight;
                    const startOffset = vh * 0.6; // Start animation when container is 60% up the screen
                    const maxScroll = rect.height - vh + startOffset;

                    let p = (startOffset - rect.top) / maxScroll;
                    if (p < 0) p = 0;
                    if (p > 1) p = 1;

                    // ── Cinematic Header Fade & Translate ──
                    const headerEl = document.getElementById('char-skills-header');
                    if (headerEl) {
                        const ho = 1 - Math.min(1.0, p / 0.08);
                        headerEl.style.opacity = String(ho);
                        headerEl.style.transform = `translate3d(0, ${p * -30}px, 0)`;
                    }

                    // ── Main Neon Ground Line Extension ──
                    const lineEl = document.querySelector('.char-neon-ground-line');
                    if (lineEl) {
                        const ls = Math.min(1.0, p / 0.08);
                        lineEl.style.transform = `scaleX(${ls})`;
                    }

                    // ── Horizontal running path with controlled pauses ──
                    if (p <= 0.02) {
                        targetX = 12; // start
                    } else if (p < 0.12) {
                        const t = (p - 0.02) / 0.10;
                        targetX = 12 + t * 20; // 12% to 32% (run)
                    } else if (p <= 0.28) {
                        targetX = 32; // pause at languages
                    } else if (p < 0.38) {
                        const t = (p - 0.28) / 0.10;
                        targetX = 32 + t * 25; // 32% to 57% (run)
                    } else if (p <= 0.54) {
                        targetX = 57; // pause at frontend
                    } else if (p < 0.63) {
                        const t = (p - 0.54) / 0.09;
                        targetX = 57 + t * 23; // 57% to 80% (run)
                    } else {
                        targetX = 80; // stays on right side
                    }

                    // ── Vertical jumping disabled (always grounded) ──
                    targetYOffset = 0;

                    // ── Neon outline city parallax growth ──
                    const backBuildings = [
                        { id: 'b-back-1', start: 0.02, end: 0.22, parallax: -20 },
                        { id: 'b-back-2', start: 0.06, end: 0.28, parallax: -20 },
                        { id: 'b-back-3', start: 0.12, end: 0.36, parallax: -20 },
                        { id: 'b-back-4', start: 0.20, end: 0.44, parallax: -20 },
                        { id: 'b-back-5', start: 0.30, end: 0.54, parallax: -20 },
                        { id: 'b-back-6', start: 0.40, end: 0.64, parallax: -20 },
                    ];
                    const foreBuildings = [
                        { id: 'b-fore-1', start: 0.04, end: 0.26, parallax: -45 },
                        { id: 'b-fore-2', start: 0.10, end: 0.32, parallax: -45 },
                        { id: 'b-fore-3', start: 0.18, end: 0.40, parallax: -45 },
                        { id: 'b-fore-4', start: 0.28, end: 0.50, parallax: -45 },
                        { id: 'b-fore-5', start: 0.38, end: 0.60, parallax: -45 },
                    ];
                    const updateBuildings = (bList) => {
                        bList.forEach(b => {
                            const el = document.getElementById(b.id);
                            if (!el) return;
                            let bp = (p - b.start) / (b.end - b.start);
                            bp = Math.min(Math.max(bp, 0), 1);
                            const easeScaleY = 1 - Math.pow(1 - bp, 3);
                            const easeOpacity = bp;
                            const tx = p * b.parallax;
                            el.style.transform = `translate3d(${tx}px, 0, 0) scaleY(${easeScaleY})`;
                            el.style.opacity = String(easeOpacity);
                        });
                    };
                    updateBuildings(backBuildings);
                    updateBuildings(foreBuildings);

                    // ── Typographic Editorial Skill columns reveal ──
                    const groups = [
                        {
                            id: 'skill-group-0',
                            start: 0.08,
                            items: ['title-0', 'item-0-0', 'item-0-1', 'item-0-2']
                        },
                        {
                            id: 'skill-group-1',
                            start: 0.30,
                            items: ['title-1', 'item-1-0', 'item-1-1', 'item-1-2', 'item-1-3']
                        },
                        {
                            id: 'skill-group-2',
                            start: 0.52,
                            items: ['title-2', 'item-2-0', 'item-2-1', 'item-2-2', 'item-2-3']
                        }
                    ];
                    groups.forEach(g => {
                        g.items.forEach((itemId, idx) => {
                            const el = document.getElementById(itemId);
                            if (!el) return;
                            const itemStart = g.start + idx * 0.04;
                            let itemP = (p - itemStart) / 0.10;
                            itemP = Math.min(Math.max(itemP, 0), 1);
                            const easeY = (1 - itemP) * 12;
                            const easeBlur = (1 - itemP) * 6;
                            el.style.opacity = String(itemP);
                            el.style.transform = `translate3d(0, ${easeY}px, 0)`;
                            el.style.filter = `blur(${easeBlur}px)`;
                        });
                    });

                    if (!isLoopRunning) {
                        isLoopRunning = true;
                        requestAnimationFrame(loop);
                    }

                    // ── Character visibility near exit (completely invisible at p=0.90) ──
                    const mover = document.getElementById('char-mover');
                    if (mover) {
                        if (p >= 0.70) {
                            const charOpacity = 1 - (p - 0.70) / 0.10;
                            mover.style.opacity = String(Math.max(0, charOpacity));
                        } else {
                            mover.style.opacity = '1';
                        }
                    }

                    // ── Cinematic Dissolve Transition at the End (starts at p=0.78) ──
                    const stickyView = document.querySelector('.char-sticky-view');
                    if (stickyView) {
                        if (p >= 0.78) {
                            const dissolveP = 1 - (p - 0.78) / 0.22;
                            stickyView.style.opacity = String(Math.max(0, dissolveP));
                            stickyView.style.transform = `scale(${0.98 + Math.max(0, dissolveP) * 0.02})`;
                        } else if (rect.top > 0) {
                            // Entering phase animation (Scale & Translate)
                            let entryP = (vh - rect.top) / (vh * 0.35);
                            entryP = Math.min(Math.max(entryP, 0), 1);
                            
                            const isMobile = window.innerWidth <= 768;
                            const travelMaxY = isMobile ? 25 : 90;
                            const minScale = isMobile ? 0.97 : 0.94;
                            
                            const translateY = (1 - entryP) * travelMaxY;
                            const scale = minScale + entryP * (1 - minScale);
                            const opacity = 0.2 + entryP * 0.8;
                            
                            stickyView.style.transform = `translate3d(0, ${translateY.toFixed(1)}px, 0) scale(${scale.toFixed(4)})`;
                            stickyView.style.opacity = String(opacity.toFixed(2));
                        } else {
                            stickyView.style.opacity = '1';
                            stickyView.style.transform = 'scale(1) translate3d(0, 0, 0)';
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

    rsx! {
        section { id: "skills", class: "char-skills-section blur-on-enter",

            // ── SCROLL TRACK (Shorter 1.5vh Height for fast unpinning) ──
            div { id: "char-scroll-track", class: "char-scroll-track",
                div { class: "char-sticky-view",



                    // ── Holographic Neon City Skyline ──
                    div { class: "char-neon-skyline",
                        // Background layer (slower parallax)
                        div { id: "b-back-1", class: "char-building b-back b-blue", style: "left: 10%; width: 60px; height: 180px;" }
                        div { id: "b-back-2", class: "char-building b-back b-purple", style: "left: 22%; width: 80px; height: 260px;" }
                        div { id: "b-back-3", class: "char-building b-back b-blue", style: "left: 38%; width: 70px; height: 220px;" }
                        div { id: "b-back-4", class: "char-building b-back b-cyan", style: "left: 55%; width: 90px; height: 300px;" }
                        div { id: "b-back-5", class: "char-building b-back b-purple", style: "left: 70%; width: 60px; height: 240px;" }
                        div { id: "b-back-6", class: "char-building b-back b-blue", style: "left: 85%; width: 80px; height: 190px;" }

                        // Foreground layer (faster parallax)
                        div { id: "b-fore-1", class: "char-building b-fore b-cyan", style: "left: 18%; width: 50px; height: 140px;" }
                        div { id: "b-fore-2", class: "char-building b-fore b-purple", style: "left: 30%; width: 65px; height: 190px;" }
                        div { id: "b-fore-3", class: "char-building b-fore b-blue", style: "left: 48%; width: 85px; height: 160px;" }
                        div { id: "b-fore-4", class: "char-building b-fore b-purple", style: "left: 65%; width: 55px; height: 220px;" }
                        div { id: "b-fore-5", class: "char-building b-fore b-cyan", style: "left: 78%; width: 70px; height: 150px;" }
                    }

                    // ── Neon Ground Line ──
                    div { class: "char-neon-ground-line" }

                    // ── Typographic Editorial Skill Columns ──
                    // Languages Group
                    div { id: "skill-group-0", class: "char-editorial-group", style: "left: 10%; top: 15%;",
                        h3 { id: "title-0", class: "char-group-title", "LANGUAGES" }
                        div { class: "char-group-items",
                            span { id: "item-0-0", class: "char-tech-item", "Python" }
                            span { id: "item-0-1", class: "char-tech-item", "JavaScript" }
                            span { id: "item-0-2", class: "char-tech-item", "SQL" }
                        }
                    }
                    // Frontend Group
                    div { id: "skill-group-1", class: "char-editorial-group", style: "left: 42%; top: 10%;",
                        h3 { id: "title-1", class: "char-group-title", "FRONTEND DEVELOPMENT" }
                        div { class: "char-group-items",
                            span { id: "item-1-0", class: "char-tech-item", "React.js" }
                            span { id: "item-1-1", class: "char-tech-item", "Next.js" }
                            span { id: "item-1-2", class: "char-tech-item", "HTML5" }
                            span { id: "item-1-3", class: "char-tech-item", "CSS3" }
                        }
                    }
                    // Backend Group
                    div { id: "skill-group-2", class: "char-editorial-group", style: "left: 74%; top: 15%;",
                        h3 { id: "title-2", class: "char-group-title", "BACKEND DEVELOPMENT" }
                        div { class: "char-group-items",
                            span { id: "item-2-0", class: "char-tech-item", "FastAPI" }
                            span { id: "item-2-1", class: "char-tech-item", "Django" }
                            span { id: "item-2-2", class: "char-tech-item", "Django REST Framework" }
                            span { id: "item-2-3", class: "char-tech-item", "REST API Development" }
                        }
                    }

                    // ── Character mover wrapper ──
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
                    }
                }
            }

            // ── Static Mobile Fallback (Normal Flow, Mobile-Only) ──
            div { class: "char-mobile-fallback container",
                div { class: "char-mobile-list",
                    for cat in SKILL_CATEGORIES {
                        div { class: "char-mobile-group",
                            h4 { 
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

