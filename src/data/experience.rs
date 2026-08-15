#[derive(Debug, Clone, PartialEq)]
pub struct Experience {
    pub role: &'static str,
    pub company: &'static str,
    pub location: &'static str,
    pub period: &'static str,
    pub achievements: &'static [&'static str],
}

pub const EXPERIENCE_ITEMS: &[Experience] = &[
    Experience {
        role: "Backend Engineer Intern / Trainee (Placeholder)",
        company: "Company / Organization Name Placeholder",
        location: "City, Country (or Remote)",
        period: "MM/YYYY - MM/YYYY",
        achievements: &[
            "Collaborated on designing and documenting RESTful APIs using FastAPI/Django.",
            "Wrote test suites with pytest to achieve higher code coverage and automate API checks.",
            "Configured PostgreSQL databases and performed data migrations using Alembic/Django Migrations.",
            "Helped containerize local developer services using Docker and Docker Compose."
        ],
    },
    Experience {
        role: "Software Engineering Trainee (Placeholder)",
        company: "Technology Solutions Agency Placeholder",
        location: "City, Country",
        period: "MM/YYYY - MM/YYYY",
        achievements: &[
            "Learned and applied clean architecture principles to Python backend modules.",
            "Assisted in maintaining and deploying microservices to testing environments.",
            "Integrated background task queues with Redis and Celery to process CPU-intensive jobs asynchronously."
        ],
    },
];
