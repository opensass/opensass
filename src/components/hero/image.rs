use dioxus::prelude::*;

#[component]
pub fn BannerImage() -> Element {
    rsx! {
        img {
            src: asset!("/assets/banner.webp"),
            alt: "Rust and SASS community members collaborating",
            width: "661px",
            height: "377px",
            class: "max-w-sm lg:max-w-xl h-auto animate-bounce ease-in-out",
            loading: "eager",

        }
    }
}
