pub(crate) mod description;
pub(crate) mod heading;
pub(crate) mod image;

use crate::components::hero::description::Description;
use crate::components::hero::heading::Heading;
use crate::components::hero::image::BannerImage;
use crate::components::navbar::auth_btns::AuthButtons;
use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "bg-white py-28 md:py-32 flex justify-center",
            div {
                class: "flex flex-col md:flex-row items-center",
                div {
                    class: "flex flex-col gap-16 px-4",
                    Heading {},
                    Description {},
                    AuthButtons { is_vertical: false },
                }
                BannerImage {}
                }
        }
        Outlet::<Route> {}
    }
}
