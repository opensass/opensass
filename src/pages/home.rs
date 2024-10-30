use crate::components::hero::Header;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            Header {}
        }
    }
}
