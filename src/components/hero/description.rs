use dioxus::prelude::*;

#[component]
pub fn Description() -> Element {
    rsx! {
        p {
            class: "text-lg md:text-xl text-black mb-8 leading-relaxed",
            "Join the Rust SASS community and explore enhanced tools, projects, and collaboration spaces."
        }
    }
}
