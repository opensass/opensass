// Copyright (c) 2026 Open SASS Community
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

pub(crate) mod auth_btns;
pub(crate) mod dropdown;
pub(crate) mod logo;
pub(crate) mod nav_links;

use crate::components::navbar::auth_btns::AuthButtons;
use crate::components::navbar::logo::Logo;
use crate::components::navbar::nav_links::NavLinks;
use crate::router::Route;
use crate::theme::ThemeToggle;
use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[component]
pub fn NavBar() -> Element {
    let mut is_menu_open = use_signal(|| false);
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);

    let toggle_menu = move |_| {
        is_menu_open.set(!is_menu_open());
    };

    let nav_bg = if is_light {
        "nav-themed shadow-md"
    } else {
        "nav-themed shadow-lg"
    };

    let hamburger_color = if is_light {
        "text-gray-800"
    } else {
        "text-white"
    };

    let drawer_bg = if is_light {
        "bg-white border border-gray-200"
    } else {
        "bg-gray-900 border border-gray-800"
    };

    rsx! {
        nav {
            class: format!(
                "fixed top-0 w-full z-50 flex items-center justify-between px-4 py-3 transition-colors duration-300 {}",
                nav_bg
            ),

            Logo {}

            button {
                class: format!(
                    "text-2xl md:hidden transition-transform duration-300 {}",
                    if is_menu_open() { format!("rotate-90 {}", hamburger_color) } else { format!("rotate-0 text-themed-primary") }
                ),
                onclick: toggle_menu,
                aria_label: "Toggle menu",
                if is_menu_open() { "✕" } else { "☰" }
            }

            div { class: "hidden md:flex items-center",
                NavLinks {}
            }

            div { class: "hidden md:flex items-center gap-3",
                ThemeToggle {}
                AuthButtons { is_vertical: false }
            }

            div {
                class: format!(
                    "fixed top-0 left-0 w-2/5 md:w-auto h-auto p-4 z-50 md:hidden transition-transform transform duration-500 ease-in-out {} {}",
                    drawer_bg,
                    if is_menu_open() { "translate-x-0 opacity-100" } else { "-translate-x-full opacity-0" }
                ),
                NavLinks {}
                div { class: "mt-4",
                    ThemeToggle {}
                }
                AuthButtons { is_vertical: true }
            }
        }
        Outlet::<Route> {}
    }
}
