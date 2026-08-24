#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub name: &'static str,
    pub subtitle: &'static str,
    pub category: &'static str,
    pub cover_image: &'static str,
    pub gallery_images: &'static [&'static str],
    pub tech_badges: &'static [&'static str],
    pub overview: &'static str,
    pub problem_faced: &'static str,
    pub solution_implemented: &'static str,
    pub github_url: &'static str,
    pub live_url: Option<&'static str>,
}

pub const PROJECTS: &[Project] = &[
    Project {
        name: "CareStream",
        subtitle: "Real-Time Patient Health Monitor",
        category: "Healthcare & IoT",
        cover_image: "/assets/projects/carestream-1.jpg",
        gallery_images: &[
            "/assets/projects/carestream-1.jpg",
            "/assets/projects/carestream-2.jpg",
            "/assets/projects/carestream-3.jpg",
        ],
        tech_badges: &[
            "Next.js",
            "Django REST",
            "PostgreSQL",
            "Redis",
            "WebSockets",
            "AWS",
        ],
        overview: "Real-time vitals monitoring system that alerts doctors instantly when patient readings become critical.",
        problem_faced: "Manual vital checks caused dangerous delays during patient emergencies.",
        solution_implemented: "Built WebSocket sensor streaming to live dashboards with instant emergency alerts.",
        github_url: "https://github.com/sreenandpk/carestream",
        live_url: Some("https://care-stream.vercel.app/docs"),
    },
    Project {
        name: "E-Commerce Platform",
        subtitle: "Full-Stack Online Shopping Store",
        category: "Web & Retail",
        cover_image: "/assets/projects/ecommerce-1.jpg",
        gallery_images: &[
            "/assets/projects/ecommerce-1.jpg",
            "/assets/projects/ecommerce-2.jpg",
            "/assets/projects/ecommerce-3.jpg",
        ],
        tech_badges: &[
            "React",
            "Django REST",
            "PostgreSQL",
            "JWT Auth",
            "Vercel",
        ],
        overview: "Modern online store featuring product catalog, cart management, checkout, and admin portal.",
        problem_faced: "Handling product inventory, orders, and customer accounts securely.",
        solution_implemented: "Built fast responsive storefront with JWT security and admin dashboard.",
        github_url: "https://github.com/sreenandpk/ecommerce",
        live_url: Some("https://ecommerce-django-frontend-lhvj.vercel.app"),
    },
];
