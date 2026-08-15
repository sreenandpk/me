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

    // Effect to handle cinematic scroll reveal using IntersectionObserver
    use_effect(move || {
        let _ = document::eval(
            r#"
            const initReveal = () => {
                const elements = document.querySelectorAll('.reveal-on-scroll');
                if (elements.length === 0) {
                    // Elements not rendered yet, retry
                    setTimeout(initReveal, 150);
                    return;
                }
                const observer = new IntersectionObserver((entries) => {
                    entries.forEach(entry => {
                        if (entry.isIntersecting) {
                            entry.target.classList.add('revealed');
                        }
                    });
                }, {
                    threshold: 0.08,
                    rootMargin: '0px 0px -60px 0px'
                });
                elements.forEach(el => observer.observe(el));
            };
            setTimeout(initReveal, 300);
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
        }
    }
}
