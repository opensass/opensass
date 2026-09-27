pub(crate) mod card;

use crate::components::project::card::Project;
use crate::components::project::card::ProjectCard;
use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[component]
pub fn Projects() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);
    let title_color = if is_light {
        "text-gray-800"
    } else {
        "text-white"
    };
    let subtitle_color = if is_light {
        "text-gray-500"
    } else {
        "text-gray-400"
    };
    let projects = vec![
        Project {
            title: "ELDFLOW: Effortless ELD Logging & AI Insights",
            description: "Automate your ELD logs, track trips, and get AI-powered summaries. All in one platform.",
            link: Some("/eldflow"),
            tech_stack: vec!["Rust", "Dioxus", "Axum", "MongoDB", "Gemini AI", "Unsplash API", "Google Maps", "ELD"],
            image: "/assets/eldflow.gif",
        },
        Project {
            title: "AIBook: AI-Powered Book Generation Platform",
            description: "A robust and scalable platform enabling users to generate complete books using Gemini AI models.",
            link: Some("/aibook"),
            tech_stack: vec!["Rust", "Dioxus", "Axum", "MongoDB", "Gemini AI", "Unsplash API"],
            image: "https://github.com/user-attachments/assets/68a4951b-57de-4563-9939-d664c604b85d",
        },
        Project {
            title: "Tripper: Smart Travel Assistant",
            description: "An intelligent travel planning platform powered by AWS Bedrock, making your trips seamless and personalized.",
            link: Some("/tripper"),
            tech_stack: vec!["Rust", "Dioxus", "MongoDB", "AWS Bedrock", "Unsplash API"],
            image: "https://github.com/user-attachments/assets/28dc576c-40b6-4548-a0ee-3390eda988f0",
        },
        Project {
            title: "Nano OG: AI-Generated Open Graph Images",
            description: "Create stunning OG images for your websites with Gemini Nano AI, tailored for seamless integration.",
            link: Some("/nanoog"),
            tech_stack: vec!["Rust", "Dioxus", "Gemini Nano AI"],
            image: "https://github.com/user-attachments/assets/a9888b6e-c3b5-4e5e-a041-67ca1498137e",
        },
    ];

    rsx! {
        section { id: "projects", class: if is_light { "bg-white py-16 flex items-center justify-center min-h-screen" } else { "py-16 flex items-center justify-center min-h-screen" }, style: if is_light { "" } else { "background: var(--bg-primary);" },
            div { class: "container mx-auto px-4",
                div { class: "flex flex-col items-center",
                    div { class: "w-full mb-12",
                        div { class: "text-center", "data-aos": "fade-up", "data-aos-duration": "800",
                            h2 { class: format!("text-5xl font-bold mb-4 tracking-wide {}", title_color), "Our Innovative Projects" }
                            p { class: format!("text-lg {}", subtitle_color), "Explore a selection of our projects, each crafted to push boundaries and deliver value." }
                        }
                    }
                    div { class: "w-full",
                        div { class: "grid md:grid-cols-3 grid-cols-1 gap-10",
                            for project in projects.iter() {
                                ProjectCard { project: project.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}
