use crate::components::footer::links::SocialLinks;
use dioxus::prelude::*;

#[component]
pub fn Bottom() -> Element {
    rsx! {
        div {
            class: "border-t border-themed mt-10 pt-6",
            div {
                class: "container mx-auto px-6 lg:px-16 flex flex-col sm:flex-row items-center justify-between gap-3 space-y-4 sm:space-y-0",
                div {
                    class: "text-sm text-themed-secondary",
                    "© 2026. Designed by ",
                    a {
                        href: "https://github.com/opensass",
                        target: "_blank",
                        class: "text-themed-primary hover:text-themed-secondary transition-colors",
                        "Open SASS"
                    }
                },
                div {
                    class: "flex items-center gap-4",
                    a {
                        href: "/blogs/privacy-policy",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "text-xs font-['Lexend'] text-themed-secondary hover:text-themed-primary transition-colors duration-200",
                        i { class: "fa-solid fa-shield-halved mr-1 text-xs" }
                        "Privacy Policy"
                    }
                    span {
                        class: "text-xs text-themed-secondary",
                        "·"
                    }
                    a {
                        href: "/blogs/terms-of-service",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "text-xs font-['Lexend'] text-themed-secondary hover:text-themed-primary transition-colors duration-200",
                        i { class: "fa-solid fa-file-contract mr-1 text-xs" }
                        "Terms of Service"
                    }
                    SocialLinks {},
                }
            }
        }
    }
}
