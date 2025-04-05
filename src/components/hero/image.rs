use dioxus::prelude::*;

#[component]
pub fn HeroImage() -> Element {
    rsx! {
        div {
            class: "w-full md:w-1/2 flex justify-center",
            img {
                class: "w-full max-w-[560px] h-[320px] md:h-[408px] object-cover rounded-[20px]",
                src: asset!("/assets/banner.webp"),
                alt: "Hero image showcasing Rust and SASS community members collaborating",
                width: "661px",
                height: "377px",
                loading: "eager",
            }
        }
    }
}
