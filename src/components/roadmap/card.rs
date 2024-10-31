use crate::components::roadmap::dot::StatusDot;
use crate::components::roadmap::line::VerticalLine;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum RoadmapStatus {
    Active,
    Inactive,
}

#[derive(Clone, PartialEq)]
pub struct RoadmapItem {
    pub date: &'static str,
    pub items: Vec<&'static str>,
    pub status: RoadmapStatus,
}

#[component]
pub fn RoadmapItemCard(roadmap_item: RoadmapItem) -> Element {
    rsx! {
        div {
            class: "relative flex flex-col items-center w-full sm:w-1/3 md:w-1/4 p-2",
            VerticalLine { status: roadmap_item.status },
            StatusDot {},
            div {
                class: format!("flex-none p-4 rounded-lg shadow-lg border-2 {}", match roadmap_item.status {
                    RoadmapStatus::Active => "text-black border-black",
                    RoadmapStatus::Inactive => "text-gray-400 border-gray-300",
                }),
                div {
                    class: "flex justify-center mb-4",
                    img {
                        src: "./down_arrow.svg",
                        alt: "roadmap-icon",
                        class: "w-8 h-8",
                    }
                }
                div {
                    class: "text-left",
                    h6 { class: "text-lg font-semibold", "{roadmap_item.date}" }
                    ul {
                        class: "text-sm list-disc list-inside space-y-1",
                        for text in roadmap_item.items.iter() {
                            li { "{text}" }
                        }
                    }
                }
            }
        }
    }
}
