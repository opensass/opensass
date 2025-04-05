use crate::components::hero::avatars::AvatarGroup;
use dioxus::prelude::*;

#[component]
pub fn TrustBox() -> Element {
    rsx! {
        div {
            class: "relative flex items-center gap-3 px-4 py-2 border border-gray-300 rounded-full bg-white",

            div {
                class: "relative w-full h-full rounded-full avatars-container",

                div {
                    class: "avatars-container-inner",
                    AvatarGroup {},
                }
            }

            span {
                class: "text-sm text-gray-700 ml-3",
                "Entrusted by ",
                strong {
                    class: "font-semibold",
                    "3.5B+ Rustaceans"
                }
            }
        }
    }
}
