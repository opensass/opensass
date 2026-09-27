pub(crate) mod accordion;
pub(crate) mod item;

use crate::components::common::header::Header;
use crate::components::faq::accordion::Accordion;
use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[component]
pub fn Faq() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);
    let text_color = if is_light {
        "text-gray-600"
    } else {
        "text-gray-400"
    };
    rsx! {
        section { id: "faq", class: if is_light { "py-16 bg-gray-100 min-h-screen flex items-center justify-center" } else { "py-16 min-h-screen flex items-center justify-center" }, style: if is_light { "" } else { "background: var(--bg-secondary);" },
            div { class: "container mx-auto px-4",
                Header {
                    title: "Got Questions?",
                    subtitle: "Explore our FAQs for insights and guidance on all things Open SASS."
                }
                div { class: "max-w-2xl mx-auto border-2 rounded",
                    Accordion {}
                }
                div { class: "text-center mt-8",
                    p { class: "{text_color}", "Contact our experts for more info." }
                    div { class: "mt-4",
                        a { href: "/contact", class: "px-6 py-3 bg-gray-600 text-white font-semibold rounded-md hover:bg-gray-700 transition-colors", "Get in Touch" }
                    }
                }
            }
        }
    }
}
