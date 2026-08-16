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
        role: "Full-Stack Developer Intern",
        company: "Bridgeon",
        location: "Kozhikode, Kerala, India · On-site",
        period: "Jun 2025 - Present",
        achievements: &[
            "Building real-world websites and apps from scratch — handling both what users see on screen and the behind-the-scenes logic that makes everything work.",
            "Creating secure login systems and user access controls so that only the right people can see and do the right things in an application.",
            "Working as part of a team using professional planning tools to organize tasks, track progress, and ship features on schedule.",
            "Following industry-standard coding practices — writing clean, well-organized code that is easy to maintain and update over time.",
            "Using automated tools to catch mistakes in code early, saving time and reducing the chance of bugs reaching users.",
            "Keeping all team members — developers, reviewers, and managers — in sync automatically by connecting the tools they use every day.",
        ],
    },
];
