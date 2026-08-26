#[derive(Debug, Clone, PartialEq)]
pub struct RealProject {
    pub id: &'static str,
    pub title: &'static str,
    pub client: &'static str,
    pub images: &'static [&'static str],
    pub live_link: &'static str,
}

pub const PROJECTS: &[RealProject] = &[
    RealProject {
        id: "01",
        client: "CLIENT",
        title: "Skyline Studios",
        images: &[
            "/assets/projects/placeholder_1.jpg",
            "/assets/projects/placeholder_2.jpg",
            "/assets/projects/placeholder_3.jpg",
        ],
        live_link: "#",
    },
    RealProject {
        id: "02",
        client: "CLIENT",
        title: "Nexus Dynamics",
        images: &[
            "/assets/projects/placeholder_3.jpg",
            "/assets/projects/placeholder_1.jpg",
            "/assets/projects/placeholder_2.jpg",
        ],
        live_link: "#",
    },
    RealProject {
        id: "03",
        client: "CLIENT",
        title: "Aura Creative",
        images: &[
            "/assets/projects/placeholder_2.jpg",
            "/assets/projects/placeholder_3.jpg",
            "/assets/projects/placeholder_1.jpg",
        ],
        live_link: "#",
    },
];
