pub(crate) mod accordion;
pub(crate) mod item;

use crate::components::common::header::Header;
use crate::components::faq::accordion::Accordion;
use dioxus::prelude::*;

#[component]
pub fn Faq() -> Element {
    rsx! {
        section { id: "faq",class: "py-16 bg-gray-100 min-h-screen flex items-center justify-center",
            div { class: "container mx-auto px-4",
                Header {
                    title: "Got Questions?",
                    subtitle: "Explore our FAQs for insights and guidance on all things Open SASS."
                }
                div { class: "max-w-2xl mx-auto border-2 rounded",
                    Accordion {}
                }
                div { class: "text-center mt-8",
                    p { class: "text-gray-600", "Contact our experts for more info." }
                    div { class: "mt-4",
                        a { href: "/contact", class: "px-6 py-3 bg-gray-600 text-white font-semibold rounded-md hover:bg-gray-700 transition-colors", "Get in Touch" }
                    }
                }
            }
        }
    }
}
