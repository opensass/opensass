pub(crate) mod auth_btns;
pub(crate) mod dropdown;
pub(crate) mod logo;
pub(crate) mod nav_links;

use crate::components::navbar::auth_btns::AuthButtons;
use crate::components::navbar::logo::Logo;
use crate::components::navbar::nav_links::NavLinks;
use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    let mut is_menu_open = use_signal(|| false);

    let toggle_menu = move |_| {
        is_menu_open.set(!is_menu_open());
    };

    rsx! {
        nav {
            class: "bg-gray-100 fixed top-0 w-full z-50 flex items-center justify-between px-4 py-4 shadow-lg",

            Logo {}

            button {
                class: format!(
                    "text-3xl md:hidden transition-transform duration-300 {}",
                    if is_menu_open() { "rotate-90" } else { "rotate-0" }
                ),
                onclick: toggle_menu,
                if is_menu_open() { "✕" } else { "☰" }
            }

            div { class: "hidden md:flex",
                NavLinks {}
            }

            div { class: "hidden md:flex",
                AuthButtons { is_vertical: false }
            }

            div {
                class: format!(
                    "fixed top-0 left-0 bg-white border border-black w-2/5 md:w-auto h-auto p-4 z-50 md:hidden transition-transform transform duration-500 ease-in-out {}",
                    if is_menu_open() { "translate-x-0 opacity-100" } else { "-translate-x-full opacity-0" }
                ),
                NavLinks {}
                AuthButtons { is_vertical: true }
            }
        }
        Outlet::<Route> {}
    }
}
