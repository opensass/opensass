use dioxus::prelude::*;

#[component]
pub fn StatusDot() -> Element {
    rsx! {
        div {
            class: "flex justify-center items-center mb-4",
            div {
                class: "w-full h-0.5 bg-gray-400 relative",
                div {
                    class: "absolute inset-x-1/2 -top-4 transform -translate-x-1/2 w-4 h-4 rounded-full bg-gray-600"
                }
            }
        }
    }
}
