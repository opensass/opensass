pub(crate) mod card;

use crate::components::project::card::Project;
use crate::components::project::card::ProjectCard;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    let projects = vec![
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
        section { id: "projects", class: "bg-white py-16 flex items-center justify-center min-h-screen",
            div { class: "container mx-auto px-4",
                div { class: "flex flex-col items-center",
                    div { class: "w-full mb-12",
                        div { class: "text-center", "data-aos": "fade-up", "data-aos-duration": "800",
                            h2 { class: "text-5xl font-bold text-gray-800 mb-4 tracking-wide", "Our Innovative Projects" }
                            p { class: "text-gray-500 text-lg", "Explore a selection of our projects, each crafted to push boundaries and deliver value." }
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
