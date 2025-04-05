use dioxus::prelude::*;

#[component]
pub fn HeroDescription() -> Element {
    rsx! {
        p {
            class: "text-gray-600 text-base leading-relaxed",
            "OpenSass gives you everything you need to create, deploy, and scale full-stack applications using Rust and WebAssembly. Developer-friendly tools, production-ready out of the box."
        }
    }
}
