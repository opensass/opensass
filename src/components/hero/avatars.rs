use dioxus::prelude::*;

#[component]
pub fn AvatarGroup() -> Element {
    rsx! {
        div {
            class: "flex -space-x-3",
            img {
                class: "w-8 h-8 rounded-full object-cover border-2 border-white",
                src: asset!("/assets/cuddlyferris.svg"),
                alt: "Ferris avatar 1",
                loading: "eager",
            }
            img {
                class: "w-8 h-8 rounded-full object-cover border-2 border-white",
                src: asset!("/assets/rustacean-flat-happy.svg"),
                alt: "Ferris avatar 2",
                loading: "eager",
            }
            img {
                class: "w-8 h-8 rounded-full object-cover border-2 border-white",
                src: asset!("/assets/rustacean-flat-gesture.svg"),
                alt: "Ferris avatar 3",
                loading: "eager",
            }
        }
    }
}
