pub(crate) mod card;

use crate::components::project::card::Project;
use crate::components::project::card::ProjectCard;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    let projects = vec![Project {
        title: "AIBook: AI-Powered Book Generation Platform",
        description: "A robust and scalable platform enabling users to generate complete books using Gemini AI models.",
        link: Some("/aibook"),
        tech_stack: vec!["Rust", "Dioxus", "Axum", "MongoDB", "Gemini AI"],
    },];

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
                    div { class: "w-full max-w-sm",
                        div { class: "grid grid-cols-1 gap-10",
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
