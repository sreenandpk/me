use dioxus::prelude::*;
use crate::data::SKILL_CATEGORIES;

struct CoreSkill {
    name: &'static str,
    desc1: &'static str,
    desc2: &'static str,
}

const CORE_SKILLS: &[CoreSkill] = &[
    CoreSkill { name: "PYTHON", desc1: "Core language", desc2: "Backend systems" },
    CoreSkill { name: "DJANGO", desc1: "Web framework", desc2: "APIs / apps" },
    CoreSkill { name: "FASTAPI", desc1: "High-perf APIs", desc2: "Async services" },
    CoreSkill { name: "REACT / NEXT", desc1: "Frontend UI", desc2: "Component arch" },
    CoreSkill { name: "POSTGRESQL", desc1: "Relational DB", desc2: "Data modeling" },
    CoreSkill { name: "REDIS", desc1: "In-memory cache", desc2: "Session mgmt" },
    CoreSkill { name: "DOCKER", desc1: "Containerization", desc2: "Deployment" },
    CoreSkill { name: "AWS", desc1: "Cloud computing", desc2: "Infrastructure" },
];

#[component]
pub fn Skills() -> Element {
    // Inject the isolated, GPU-accelerated scroll engine
    use_effect(move || {
        let _ = document::eval(
            r#"
            const initDino = () => {
                const track = document.getElementById('dino-scroll-track');
                if (!track) return;
                
                // Respect prefers-reduced-motion
                if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
                    track.classList.add('reduced-motion-fallback');
                    return;
                }

                const terminal = document.getElementById('dino-terminal');
                const finalTxt = document.getElementById('dino-final');
                const dino = document.getElementById('dino-char');
                const obsContainer = document.getElementById('dino-obstacles');
                const obstacles = Array.from(document.querySelectorAll('.dino-obstacle')).map(el => ({
                    el,
                    box: el.querySelector('.dino-obstacle-box')
                }));

                let ticking = false;

                const update = () => {
                    const rect = track.getBoundingClientRect();
                    const vh = window.innerHeight;
                    
                    const maxScroll = rect.height - vh;
                    let p = -rect.top / maxScroll;
                    
                    if (p < 0) p = 0;
                    if (p > 1) p = 1;

                    // 1. Terminal fade out (0 to 0.1)
                    if (p < 0.1) {
                        const tOp = 1 - (p * 10);
                        terminal.style.opacity = tOp.toFixed(3);
                        terminal.style.transform = `translate3d(0, -${p * 200}px, 0)`;
                        terminal.style.pointerEvents = 'auto';
                    } else {
                        terminal.style.opacity = '0';
                        terminal.style.pointerEvents = 'none';
                    }

                    // 2. Final text fade in (0.9 to 1.0)
                    if (p > 0.9) {
                        const finalP = (p - 0.9) * 10;
                        finalTxt.style.opacity = finalP.toFixed(3);
                        finalTxt.style.transform = `translate3d(0, ${20 - finalP * 20}px, 0)`;
                    } else {
                        finalTxt.style.opacity = '0';
                    }

                    // 3. Move obstacles (0.1 to 0.9)
                    let moveP = (p - 0.1) / 0.8;
                    if (moveP < 0) moveP = 0;
                    if (moveP > 1) moveP = 1;

                    // Obstacles container moves left across the screen
                    const isMobile = window.innerWidth <= 768;
                    const padding = isMobile ? 200 : 600;
                    const totalTravel = obsContainer.scrollWidth - window.innerWidth + padding;
                    obsContainer.style.transform = `translate3d(${-moveP * totalTravel}px, 0, 0)`;

                    // 4. Calculate Dino Jump via relative positions
                    const dinoRect = dino.getBoundingClientRect();
                    const dinoCenter = dinoRect.left + dinoRect.width / 2;
                    let jumpY = 0;

                    const jumpRadius = isMobile ? 120 : 180;
                    const maxJump = isMobile ? 90 : 130;

                    obstacles.forEach(obs => {
                        const boxRect = obs.box.getBoundingClientRect();
                        const boxCenter = boxRect.left + boxRect.width / 2;
                        const dist = boxCenter - dinoCenter;
                        
                        // Jump if near
                        if (Math.abs(dist) < jumpRadius) {
                            const norm = dist / jumpRadius; 
                            const currentJump = -maxJump * (1 - norm * norm);
                            if (currentJump < jumpY) {
                                jumpY = currentJump;
                            }
                        }

                        // Clear if passed
                        if (dist < -20) {
                            obs.el.classList.add('cleared');
                        } else {
                            obs.el.classList.remove('cleared');
                        }
                    });

                    dino.style.transform = `translate3d(0, ${jumpY}px, 0)`;
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

                window.addEventListener('scroll', onScroll, { passive: true });
                window.addEventListener('resize', update, { passive: true });
                update();
            };
            setTimeout(initDino, 100);
            setTimeout(initDino, 500);
            "#
        );
    });

    rsx! {
        section { id: "skills", class: "dino-skills-section blur-on-enter",
            // The Scroll Track
            div { class: "dino-scroll-track", id: "dino-scroll-track",
                div { class: "dino-sticky-view",
                    
                    // Terminal Screen
                    div { class: "dino-terminal-text", id: "dino-terminal",
                        p { "CONNECTION LOST" }
                        p { "NETWORK OFFLINE" }
                        p { "DEVELOPER STILL RUNNING" }
                        span { class: "blinking-cursor", "_" }
                    }

                    // The Ground
                    div { class: "dino-ground" }

                    // The Dino (Minimal geometric runner)
                    div { class: "dino-character", id: "dino-char",
                        svg {
                            view_box: "0 0 100 100",
                            class: "dino-svg",
                            // An original, sleek geometric shape
                            polygon { points: "30,80 30,50 50,30 70,30 70,50 90,50 90,70 70,70 70,80 50,80", fill: "currentColor" }
                            // Eye
                            rect { x: "75", y: "35", width: "5", height: "5", fill: "#ffffff" }
                            // Leg 1
                            rect { x: "40", y: "80", width: "8", height: "20", fill: "currentColor" }
                            // Leg 2
                            rect { x: "60", y: "80", width: "8", height: "15", fill: "currentColor" }
                        }
                    }

                    // Obstacles Container
                    div { class: "dino-obstacles", id: "dino-obstacles",
                        // Initial spacer to give dino running room
                        div { class: "dino-spacer" }
                        
                        for (i, skill) in CORE_SKILLS.iter().enumerate() {
                            div { class: "dino-obstacle", "data-index": "{i}",
                                div { class: "dino-obstacle-box",
                                    // Blocky aesthetic obstacle
                                    div { class: "box-top" }
                                    div { class: "box-bottom" }
                                }
                                div { class: "dino-obstacle-reveal",
                                    h4 { class: "reveal-title", "{skill.name}" }
                                    div { class: "reveal-line" }
                                    p { class: "reveal-desc", "{skill.desc1}" }
                                    p { class: "reveal-desc", "{skill.desc2}" }
                                }
                            }
                        }
                        
                        // End spacer
                        div { class: "dino-spacer" }
                    }

                    // Final Screen
                    div { class: "dino-final-text", id: "dino-final",
                        p { class: "green-text", "CONNECTION RESTORED" }
                        br {}
                        p { "NETWORK       ONLINE" }
                        p { "SERVICES      ONLINE" }
                        p { "DEVELOPER     STILL RUNNING" }
                        br {}
                        h3 { "FULL STACK DEVELOPER" }
                    }
                }
            }

            // Secondary Skills Grid (Normal flow)
            div { class: "dino-secondary-skills container",
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
