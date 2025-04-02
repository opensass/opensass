pub(crate) mod card;
pub(crate) mod dot;
pub(crate) mod line;
pub(crate) mod section;

use crate::components::common::header::Header;
use crate::components::roadmap::card::RoadmapItem;
use crate::components::roadmap::card::RoadmapStatus;
use crate::components::roadmap::section::RoadmapSection;
use dioxus::prelude::*;

#[component]
pub fn Roadmap() -> Element {
    let roadmap_data = vec![
        RoadmapItem {
            date: "2025 Q1",
            items: vec![
                "Initial release with full Dioxus support",
                "Enable seamless integration of Dioxus components",
                "Provide comprehensive documentation and starter templates for Dioxus users",
            ],
            status: RoadmapStatus::Active,
        },
        RoadmapItem {
            date: "2025 Q2",
            items: vec![
                "Expand support to include Yew framework",
                "Develop compatibility tools to help Yew developers transition projects to Open SASS",
                "Add community-driven examples and tutorials for Yew",
            ],
            status: RoadmapStatus::Inactive,
        },
        RoadmapItem {
            date: "2025 Q3",
            items: vec![
                "Introduce support for Leptos framework",
                "Integrate optimized Leptos components and templates for Open SASS",
                "Enhance documentation to cover multi-framework project",
            ],
            status: RoadmapStatus::Inactive,
        },
    ];

    rsx! {
        section {
            class: "flex flex-col items-center justify-center min-h-screen space-x-4 mt-10", id: "roadmap",
            div { class: "container mx-auto",
                Header {
                    title: "Our Roadmap Ahead",
                    subtitle: "Discover the journey we're on and the milestones we aim to achieve."
                }
                RoadmapSection { roadmap_data: roadmap_data.clone() }
            }
        }
    }
}
