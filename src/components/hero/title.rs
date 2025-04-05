use dioxus::prelude::*;

#[component]
pub fn HeroTitle() -> Element {
    rsx! {
        h1 {
            class: "text-3xl md:text-4xl font-bold text-[#333333] leading-snug",
            "Build Full-Stack Rusty Apps, Blazingly Fast"
        }
    }
}
