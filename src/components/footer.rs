pub(crate) mod bottom;
pub(crate) mod contact;
pub(crate) mod icon;
pub(crate) mod links;
pub(crate) mod logo;
pub(crate) mod support;

use crate::components::footer::bottom::Bottom;
use crate::components::footer::contact::Contact;
use crate::components::footer::logo::Logo;
use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[component]
pub fn Footer() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);
    let footer_bg = "bg-themed-card";
    rsx! {
        footer {
            class: "text-themed-primary py-10 border-t border-themed {footer_bg}",
            div {
                class: "container mx-auto px-6 lg:px-16 grid gap-8 lg:grid-cols-2 grid-cols-1",
                Logo {},
                Contact {},
            },
            Bottom {},
        }
    }
}
