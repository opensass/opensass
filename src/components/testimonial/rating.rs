use dioxus::prelude::*;

#[component]
pub fn StarRating(star_images: Vec<Element>) -> Element {
    rsx! {
        div { class: "flex justify-center mb-4",
            for star in &star_images {
                div { class: "w-5 h-5", {star} }
            }
        }
    }
}
