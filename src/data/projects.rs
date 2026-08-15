#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub tech_badges: &'static [&'static str],
    pub highlights: &'static [&'static str],
    pub github_url: &'static str,
    pub live_url: Option<&'static str>,
}

pub const PROJECTS: &[Project] = &[
    Project {
        name: "CareStream",
        description: "A modern healthcare and ICU patient monitoring platform that collects, processes, and displays real-time vitals and device data with strict authentication protocols.",
        tech_badges: &["Django", "Django REST Framework", "PostgreSQL", "REST APIs", "Token Auth"],
        highlights: &[
            "Architected a modular system to scale device monitoring feeds independently.",
            "Designed schema and optimized queries for patient vitals history in PostgreSQL.",
            "Implemented secure RESTful API endpoints for medical devices data ingestion.",
            "Engineered secure medical staff authentication and role-based access control."
        ],
        github_url: "https://github.com/sreenandpk/carestream-placeholder",
        live_url: None,
    },
    Project {
        name: "Just Listen",
        description: "A high-performance asynchronous API designed to support disciplined decision-making and analysis. Integrates background workers and caching for fast response times.",
        tech_badges: &["FastAPI", "Async Python", "PostgreSQL", "SQLAlchemy", "Alembic", "Redis", "Celery", "Docker", "pytest"],
        highlights: &[
            "Leveraged FastAPI's async/await paradigm for concurrent connection handling.",
            "Set up robust database migrations with Alembic and ORM models via SQLAlchemy.",
            "Integrated Redis and Celery for async background jobs, task queuing, and caching.",
            "Developed strict JWT authentication and authorization middleware.",
            "Achieved high test coverage using pytest, mock testing, and async test runners."
        ],
        github_url: "https://github.com/sreenandpk/just-listen-placeholder",
        live_url: None,
    },
    Project {
        name: "Trading Platform & Microservices",
        description: "A multi-repository trading microservices ecosystem designed with strict service boundary separation, shared message brokers, and highly available databases.",
        tech_badges: &["Microservices", "FastAPI", "PostgreSQL", "Redis", "Celery", "Docker", "JWT", "Shared Infrastructure", "API Gateway"],
        highlights: &[
            "Separated concerns into discrete Authentication, Market Feed, and Order execution services.",
            "Managed asynchronous worker coordination with Celery and Redis broker.",
            "Containerized the entire infrastructure using multi-stage Docker builds.",
            "Designed a unified API routing architecture ensuring high availability and load handling.",
            "Shared core utilities across repositories via centralized base infrastructure packages."
        ],
        github_url: "https://github.com/sreenandpk/trading-platform-placeholder",
        live_url: None,
    },
];
