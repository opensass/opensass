use dioxus::prelude::*;

#[component]
pub fn HeroBadge() -> Element {
    rsx! {
        span {
            class: "text-sm font-medium text-gray-700 tracking-wide",
            "Backed by Developers Worldwide"
        }
    }
}
