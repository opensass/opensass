use dioxus::prelude::*;

#[component]
pub fn Logo() -> Element {
    rsx! {
        div {
            class: "mb-6 lg:mb-0",
            div {
                class: "flex items-center space-x-2 mb-4",
                img { src: "/server-icon.jpg", alt: "Logo", class: "h-10" },
            }
            p { class: "text-sm text-gray-400", "Your Gateway to Secure Open-Sourc Rusty SaaS Solutions." }
        }
    }
}