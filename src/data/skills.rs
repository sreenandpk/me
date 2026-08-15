#[derive(Debug, Clone, PartialEq)]
pub struct SkillCategory {
    pub name: &'static str,
    pub icon: &'static str, // Simple emoji representing the category
    pub skills: &'static [&'static str],
}

pub const SKILL_CATEGORIES: &[SkillCategory] = &[
    SkillCategory {
        name: "Backend Development",
        icon: "⚡",
        skills: &["Python", "FastAPI", "Django", "Django REST Framework", "REST APIs"],
    },
    SkillCategory {
        name: "Databases & Caching",
        icon: "🗄️",
        skills: &["PostgreSQL", "MySQL", "MongoDB", "Redis"],
    },
    SkillCategory {
        name: "Architecture & Engineering",
        icon: "🏗️",
        skills: &[
            "Microservices",
            "Clean Architecture",
            "Async Programming",
            "JWT Authentication",
            "OAuth",
            "WebSockets",
            "API Design",
        ],
    },
    SkillCategory {
        name: "DevOps & Infrastructure",
        icon: "☁️",
        skills: &["Docker", "Git", "GitHub", "CI/CD", "Linux", "Nginx"],
    },
    SkillCategory {
        name: "Testing & Automation",
        icon: "🧪",
        skills: &["pytest", "Playwright", "pytest-asyncio", "Test Automation"],
    },
    SkillCategory {
        name: "Frontend Development",
        icon: "💻",
        skills: &["React", "JavaScript", "HTML", "CSS", "Tailwind CSS"],
    },
    SkillCategory {
        name: "Currently Exploring",
        icon: "🦀",
        skills: &["Rust", "Dioxus", "WebAssembly"],
    },
];
