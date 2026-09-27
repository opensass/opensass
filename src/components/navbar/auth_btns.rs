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

use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[derive(Props, Clone, PartialEq)]
pub struct AuthButtonsProps {
    is_vertical: bool,
}

#[component]
pub fn AuthButtons(props: AuthButtonsProps) -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);

    let button_class = if props.is_vertical {
        "flex flex-col gap-4 mt-4"
    } else {
        "flex flex-row gap-3 items-center"
    };

    let join_class = if is_light {
        "border border-gray-900 px-5 py-2 text-base text-gray-900 hover:bg-gray-100 transition-colors duration-200 rounded"
    } else {
        "border border-gray-400 px-5 py-2 text-base text-gray-200 hover:bg-gray-800 transition-colors duration-200 rounded"
    };

    let explore_class = if is_light {
        "bg-gray-900 text-white px-5 py-2 text-base hover:bg-gray-700 transition-colors duration-200 rounded"
    } else {
        "bg-white text-gray-900 px-5 py-2 text-base hover:bg-gray-200 transition-colors duration-200 rounded"
    };

    rsx! {
        div { class: "{button_class}",
            a {
                class: "{join_class}",
                href: "https://discord.gg/b5JbvHW5nv",
                target: "_blank",
                rel: "noopener noreferrer",
                "Join"
            },
            a {
                class: "{explore_class}",
                href: "https://kit.opensass.org",
                target: "_blank",
                rel: "noopener noreferrer",
                "Explore"
            }
        }
    }
}
