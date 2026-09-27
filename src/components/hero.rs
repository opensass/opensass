pub(crate) mod avatars;
pub(crate) mod badge;
pub(crate) mod description;
pub(crate) mod image;
pub(crate) mod title;
pub(crate) mod trust;

use crate::components::hero::badge::HeroBadge;
use crate::components::hero::description::HeroDescription;
use crate::components::hero::image::HeroImage;
use crate::components::hero::title::HeroTitle;
use crate::components::hero::trust::TrustBox;
use crate::components::navbar::auth_btns::AuthButtons;

use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[component]
pub fn Hero() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);
    let section_class = "min-h-screen bg-themed-primary px-6 py-28 md:py-32 flex justify-center flex-col md:flex-row items-center gap-12 w-full";
    rsx! {
        section {
            class: "{section_class}",
            style: "",

            HeroImage {},

            div {
                class: "w-full md:w-1/2 flex flex-col gap-6",
                HeroBadge {},
                HeroTitle {},
                HeroDescription {},

                div {
                    class: "flex flex-col sm:flex-row items-start sm:items-center gap-4 mt-4",
                    TrustBox {},
                    AuthButtons { is_vertical: false },
                }
            }
        }
    }
}
