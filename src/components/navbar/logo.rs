use dioxus::prelude::*;

#[component]
pub fn Logo() -> Element {
    rsx! {
        div { class: "flex items-center",
            img {
                src: "./logo.webp",
                alt: "Open SASS Logo",
                class: "w-16 h-16 object-contain"
            }
        }
    }
}
