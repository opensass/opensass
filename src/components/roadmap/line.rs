use crate::components::roadmap::card::RoadmapStatus;
use dioxus::prelude::*;

#[component]
pub fn VerticalLine(status: RoadmapStatus) -> Element {
    rsx! {
        div {
            class: format!("absolute top-0 h-full w-0.5 {}", match status {
                RoadmapStatus::Active => "bg-black",
                RoadmapStatus::Inactive => "bg-gray-400",
            }),
            style: "height: 1.75rem;"
        }
    }
}
