use crate::components::features::item::FeatureItem;
use crate::components::features::Feature;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FeatureGridProps {
    features: Vec<Feature>,
}

#[component]
pub fn Grid(props: FeatureGridProps) -> Element {
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-3 gap-12",

            for feature in &props.features {
                div {
                    class: "border border-themed shadow-md p-6 rounded-lg bg-themed-card cursor-pointer hover:bg-themed-secondary hover:shadow-lg transition-all duration-300",

                    FeatureItem {
                        icon: feature.icon.clone(),
                        title: feature.title,
                        description: feature.description,
                    }
                }
            }
        }
    }
}
