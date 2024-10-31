use crate::components::roadmap::card::RoadmapItem;
use crate::components::roadmap::card::RoadmapItemCard;
use dioxus::prelude::*;

#[component]
pub fn RoadmapSection(roadmap_data: Vec<RoadmapItem>) -> Element {
    rsx! {
        div {
            class: "flex flex-wrap justify-center sm:border-t-2 border-gray-600 px-4 py-0 relative",
            for item in roadmap_data.iter() {
                RoadmapItemCard { roadmap_item: item.clone() }
            }
        }
    }
}
