use dioxus::prelude::*;

#[component]
pub fn BannerImage() -> Element {
    rsx! {
        img {
            src: "./banner.png",
            alt: "Rust and SASS community members collaborating",
            class: "max-w-sm lg:max-w-xl h-auto animate-bounce ease-in-out",
        }
    }
}
