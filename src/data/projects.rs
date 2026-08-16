#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub name: &'static str,
    pub subtitle: &'static str,
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
        subtitle: "Healthcare / Real-Time ICU Telemetry Engine",
        tech_badges: &["Django", "FastAPI", "PostgreSQL", "WebSockets", "Redis"],
        overview: "A modern healthcare platform designed to aggregate, process, and display real-time vitals and device data from ICU monitoring equipment with strict authentication protocols and low latency.",
        problem_faced: "The legacy system polled the database for patient vitals every 5 seconds, resulting in massive database load and unacceptable latency for critical alerts when scaling beyond 50 concurrent patients.",
        solution_implemented: "I re-architected the data ingestion layer using FastAPI and WebSockets, backed by Redis for pub/sub message brokering. Vitals are now pushed to connected clients instantly, bypassing the database for real-time views, which reduced database write load by 85% and cut latency down to sub-100ms.",
        github_url: "https://github.com/sreenandpk/carestream-placeholder",
        live_url: None,
    },
    Project {
        name: "Just Listen",
        subtitle: "FastAPI / Asynchronous Audio Processing Platform",
        tech_badges: &["FastAPI", "Celery", "PostgreSQL", "Docker", "AWS S3"],
        overview: "A high-performance asynchronous API designed to support disciplined decision-making by processing and analyzing large audio files and generating transcriptions and summaries in the background.",
        problem_faced: "Audio processing tasks (like transcription and NLP summarization) are heavily CPU-bound and took minutes to complete. Serving these requests synchronously was blocking the ASGI event loop, causing the entire API to hang for other users during uploads.",
        solution_implemented: "I decoupled the heavy processing from the API by implementing an asynchronous task queue using Celery and Redis. The API now instantly returns a polling task ID, while background workers pull from AWS S3 to process the audio. This improved API throughput by 10x and prevented any blocking of the main thread.",
        github_url: "https://github.com/sreenandpk/just-listen-placeholder",
        live_url: None,
    },
    Project {
        name: "Trading Platform",
        subtitle: "Go & Python / High-Frequency Trading Microservices",
        tech_badges: &["Microservices", "Docker", "PostgreSQL", "JWT", "API Gateway"],
        overview: "A multi-repository trading microservices ecosystem designed with strict service boundary separation, shared message brokers, and highly available databases to execute simulated trades.",
        problem_faced: "As the platform grew into separate services (Authentication, Market Feed, Order Execution), managing shared configurations, database schemas, and JWT validation across multiple repositories became a nightmare, leading to code duplication and drift.",
        solution_implemented: "I designed a centralized 'shared infrastructure' Python package that contains base models, authentication middleware, and database utility classes. This package is published privately and installed in all microservices, ensuring a single source of truth and drastically reducing the time required to spin up new services.",
        github_url: "https://github.com/sreenandpk/trading-platform-placeholder",
        live_url: None,
    },
];

