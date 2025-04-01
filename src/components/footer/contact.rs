use crate::components::footer::links::ContactLink;
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        div {
            class: "mb-6 lg:mb-0",
            h5 { class: "text-lg font-semibold mb-4", "Contact us" }
            ul {
                class: "space-y-2 text-gray-400",
                ContactLink { label: "Address", href: "#", text: "The Cosmos" },
                ContactLink { label: "Email", href: "mail:oss@opensass.org", text: "oss@opensass.org" },
            }
        }
    }
}
