#[derive(Debug, Clone, PartialEq)]
pub struct SkillItem {
    pub name: &'static str,
    pub icon_class: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SkillCategory {
    pub name: &'static str,
    pub icon: &'static str,
    pub skills: &'static [SkillItem],
}

pub const SKILL_CATEGORIES: &[SkillCategory] = &[
    SkillCategory {
        name: "Languages",
        icon: "💻",
        skills: &[
            SkillItem { name: "Python", icon_class: "devicon-python-plain" },
            SkillItem { name: "JavaScript", icon_class: "devicon-javascript-plain" },
            SkillItem { name: "SQL", icon_class: "devicon-azuresqldatabase-plain" }, // generic SQL
        ],
    },
    SkillCategory {
        name: "Frontend Development",
        icon: "🎨",
        skills: &[
            SkillItem { name: "React.js", icon_class: "devicon-react-original" },
            SkillItem { name: "Next.js", icon_class: "devicon-nextjs-plain" },
            SkillItem { name: "HTML5", icon_class: "devicon-html5-plain" },
            SkillItem { name: "CSS3", icon_class: "devicon-css3-plain" },
            SkillItem { name: "Bootstrap 5", icon_class: "devicon-bootstrap-plain" },
            SkillItem { name: "Tailwind CSS", icon_class: "devicon-tailwindcss-original" },
            SkillItem { name: "Framer Motion", icon_class: "devicon-framermotion-original" },
        ],
    },
    SkillCategory {
        name: "Backend Development",
        icon: "⚙️",
        skills: &[
            SkillItem { name: "FastAPI", icon_class: "devicon-fastapi-plain" },
            SkillItem { name: "Django", icon_class: "devicon-django-plain" },
            SkillItem { name: "REST API", icon_class: "devicon-nestjs-plain" }, // Generic API looking icon
            SkillItem { name: "Swagger", icon_class: "devicon-swagger-plain" },
        ],
    },
    SkillCategory {
        name: "Database & Caching",
        icon: "🗄️",
        skills: &[
            SkillItem { name: "PostgreSQL", icon_class: "devicon-postgresql-plain" },
            SkillItem { name: "Redis", icon_class: "devicon-redis-plain" },
        ],
    },
    SkillCategory {
        name: "DevOps & Cloud",
        icon: "☁️",
        skills: &[
            SkillItem { name: "Docker", icon_class: "devicon-docker-plain" },
            SkillItem { name: "AWS", icon_class: "devicon-amazonwebservices-original" },
            SkillItem { name: "Vercel", icon_class: "devicon-vercel-original" },
            SkillItem { name: "Terraform", icon_class: "devicon-terraform-plain" },
            SkillItem { name: "Nginx", icon_class: "devicon-nginx-original" },
            SkillItem { name: "Linux", icon_class: "devicon-linux-plain" },
        ],
    },
    SkillCategory {
        name: "Developer Tools",
        icon: "🛠️",
        skills: &[
            SkillItem { name: "Git", icon_class: "devicon-git-plain" },
            SkillItem { name: "GitHub", icon_class: "devicon-github-original" },
            SkillItem { name: "VS Code", icon_class: "devicon-vscode-plain" },
            SkillItem { name: "Postman", icon_class: "devicon-postman-plain" },
        ],
    },
    SkillCategory {
        name: "Collaboration",
        icon: "🤝",
        skills: &[
            SkillItem { name: "Slack", icon_class: "devicon-slack-plain" },
            SkillItem { name: "Linear", icon_class: "devicon-figma-plain" }, // Generic UI tool substitute
            SkillItem { name: "Agile / Scrum", icon_class: "devicon-jira-plain" }, // Generic agile
        ],
    },
];
