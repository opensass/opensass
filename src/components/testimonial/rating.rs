use dioxus::prelude::*;

#[component]
pub fn StarRating(star_images: Vec<&'static str>) -> Element {
    rsx! {
        div { class: "flex justify-center mb-4",
            for star in &star_images {
                img { src: "{star}", class: "w-5 h-5" }
            }
        }
    }
}
