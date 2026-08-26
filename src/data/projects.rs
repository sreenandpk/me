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
    // We can add more projects here later, but for now we'll just implement the layout for one
];
