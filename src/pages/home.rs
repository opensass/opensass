use crate::components::features::Features;
use crate::components::hero::Header;
use crate::components::testimonial::Testimonial;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            Header {}
            Features {}
            Testimonial {}
        }
    }
}
