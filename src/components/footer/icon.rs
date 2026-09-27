use dioxus::prelude::*;

#[component]
pub fn SocialIcon(href: &'static str, icon: Element) -> Element {
    rsx! {
        li {
            a {
                href: "{href}",
                target: "_blank",
                class: "p-2 rounded-full text-themed-primary bg-themed-card transition-colors hover:text-themed-secondary shadow-sm",
                {icon}
            }
        }
    }
}
