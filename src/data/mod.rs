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
    pub leetcode_url: &'static str,
    pub instagram_url: &'static str,
    pub whatsapp_url: &'static str,
    pub phone: &'static str,
}

pub const PERSONAL_INFO: PersonalInfo = PersonalInfo {
    name: "Sreenand P K",
    title: "Full Stack Developer",
    github_url: "https://github.com/sreenandpk",
    linkedin_url: "https://www.linkedin.com/in/sreenand-p-k-3842b936b/",
    email: "sreenandpk3@gmail.com",
    summary: "I build web applications — from the interface users interact with to the systems running quietly behind the scenes. I enjoy solving real problems with technology, writing clean code that is easy to understand and maintain, and shipping software that actually works reliably in the real world.",
    leetcode_url: "https://leetcode.com/sreenandpk-placeholder",
    instagram_url: "https://instagram.com/sreenandpk-placeholder",
    whatsapp_url: "https://wa.me/919539379577",
    phone: "tel:+919539379577",
};

#[derive(Debug, Clone, PartialEq)]
pub struct PhilosophyItem {
    pub title: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
}

pub const PHILOSOPHY_ITEMS: &[PhilosophyItem] = &[
    PhilosophyItem {
        title: "Code That Lasts",
        description: "I write code the way I would want to find it — organized, easy to read, and simple to change. Good structure saves everyone time down the road.",
        icon: "🏗️",
    },
    PhilosophyItem {
        title: "Security First",
        description: "Every application I build is protected from the ground up — so only the right people can access the right things, and user data stays private and safe.",
        icon: "🔒",
    },
    PhilosophyItem {
        title: "Built for the Real World",
        description: "I deploy applications to the cloud so they are always online, handle real users, and keep running reliably — even as they grow.",
        icon: "☁️",
    },
    PhilosophyItem {
        title: "Live & Responsive",
        description: "Some problems need instant answers. I build systems that react to events in real time — whether that is a live dashboard, instant notifications, or streaming data from physical devices.",
        icon: "⚡",
    },
    PhilosophyItem {
        title: "Team-First Mindset",
        description: "Great software is built by great teams. I follow professional workflows, communicate clearly, and make sure my work fits seamlessly into what everyone else is building.",
        icon: "🤝",
    },
    PhilosophyItem {
        title: "Always Learning",
        description: "Technology never stops evolving and neither do I. I am constantly exploring new tools, languages, and ideas to become a better engineer every day.",
        icon: "🦀",
    },
];
