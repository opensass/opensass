use dioxus::prelude::*;

#[component]
pub fn Heading() -> Element {
    rsx! {
        div {
            class: "flex flex-col text-center md:text-left",
            h1 {
                class: "text-4xl md:text-5xl lg:text-6xl font-bold text-black mb-6 leading-tight",
                "Welcome to Open SASS"
            }
        }
    }
}
