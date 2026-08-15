pub mod experience;
pub mod projects;
pub mod skills;

pub use experience::EXPERIENCE_ITEMS;
pub use projects::PROJECTS;
pub use skills::SKILL_CATEGORIES;

#[derive(Debug, Clone, PartialEq)]
pub struct PersonalInfo {
    pub name: &'static str,
    pub title: &'static str,
    pub github_url: &'static str,
    pub linkedin_url: &'static str,
    pub email: &'static str,
    pub summary: &'static str,
}

pub const PERSONAL_INFO: PersonalInfo = PersonalInfo {
    name: "Sreenand P K",
    title: "Python Developer & Backend Engineer",
    github_url: "https://github.com/sreenandpk",
    linkedin_url: "https://linkedin.com/in/sreenandpk-placeholder",
    email: "sreenandpk@example.com", // clearly marked placeholder email
    summary: "Dedicated Backend Engineer specializing in Python, FastAPI, and Django. I build scalable backend architectures, high-performance REST APIs, containerized microservices, and automated testing suites. Passionate about clean code, robust database designs, and writing software that is maintainable, highly observable, and secure.",
};

#[derive(Debug, Clone, PartialEq)]
pub struct PhilosophyItem {
    pub title: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
}

pub const PHILOSOPHY_ITEMS: &[PhilosophyItem] = &[
    PhilosophyItem {
        title: "Clean Architecture",
        description: "Enforcing clear separation of concerns, decoupling core business logic from frameworks, databases, and third-party API integrations.",
        icon: "🏗️",
    },
    PhilosophyItem {
        title: "Robust Testing",
        description: "Treating automated tests as documentation and a safety net—leveraging pytest, mock objects, and async integration testing to guarantee reliability.",
        icon: "🧪",
    },
    PhilosophyItem {
        title: "Security by Design",
        description: "Implementing strict authentication, authorization controls, secure data handling, JWT validation, and encryption at every service layer.",
        icon: "🔒",
    },
    PhilosophyItem {
        title: "High Performance",
        description: "Optimizing database queries, using asynchronous tasks for long-running workflows, and caching hot paths to achieve low-latency execution.",
        icon: "⚡",
    },
    PhilosophyItem {
        title: "Infrastructure Automation",
        description: "Embracing Docker-centric local and production setups alongside CI/CD pipelines to ensure deployments are predictable and repeatable.",
        icon: "🤖",
    },
    PhilosophyItem {
        title: "Continuous Learning",
        description: "Actively exploring emerging languages and paradigms—such as Rust, Dioxus, and WebAssembly—to expand my technical problem-solving capabilities.",
        icon: "🦀",
    },
];
