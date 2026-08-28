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
            "/projects/placeholder_1.jpg",
            "/projects/placeholder_2.jpg",
            "/projects/placeholder_3.jpg",
        ],
        live_link: "#",
    },
    RealProject {
        id: "02",
        client: "CLIENT",
        title: "Nexus Dynamics",
        images: &[
            "/projects/placeholder_3.jpg",
            "/projects/placeholder_1.jpg",
            "/projects/placeholder_2.jpg",
        ],
        live_link: "#",
    },
    RealProject {
        id: "03",
        client: "CLIENT",
        title: "Aura Creative",
        images: &[
            "/projects/placeholder_2.jpg",
            "/projects/placeholder_3.jpg",
            "/projects/placeholder_1.jpg",
        ],
        live_link: "#",
    },
];
