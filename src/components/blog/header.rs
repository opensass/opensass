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

use crate::components::footer::Footer;
use crate::router::Route;
use crate::server::subscriber::controller::subscribe_user;
use crate::theme::ThemeToggle;
use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn Header() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut feedback_message = use_signal(|| None::<String>);
    let mut is_menu_open = use_signal(|| false);
    let mut is_scrolled = use_signal(|| false);
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);

    use_effect(move || {
        let window = web_sys::window().expect("no global `window` exists");
        let mut is_scrolled_inner = is_scrolled;

        let closure = Closure::wrap(Box::new(move || {
            let window = web_sys::window().expect("no global `window` exists");
            let scroll_y = window.scroll_y().unwrap_or(0.0);
            is_scrolled_inner.set(scroll_y > 50.0);
        }) as Box<dyn FnMut()>);

        window
            .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .expect("failed to add scroll event listener");

        closure.forget();
    });

    let mut handle_subscribe = move |_| {
        let email_value = email().clone();

        if email_value.is_empty() {
            feedback_message.set(Some("Please enter a valid email.".to_string()));
            return;
        }

        feedback_message.set(None);

        spawn(async move {
            match subscribe_user(email_value.clone()).await {
                Ok(_) => feedback_message.set(Some("Subscription successful!".to_string())),
                Err(_) => {
                    feedback_message.set(Some("Failed to subscribe. Please try again.".to_string()))
                }
            }
        });
    };

    let nav_bg = "nav-themed shadow-md";
    let link_color = "text-themed-secondary hover:text-themed-accent";
    let title_color = "text-themed-primary";
    let hamburger_color = "text-themed-primary";
    let back_btn_class = "text-themed-secondary bg-themed-card border border-themed hover:bg-themed-secondary hover:text-themed-primary px-4 py-2 rounded-lg text-sm transition-colors duration-200 flex items-center gap-2";
    let hero_text_color = "text-themed-primary";
    let hero_sub_color = "text-themed-secondary";
    let input_class = "p-4 text-themed-primary placeholder-gray-500 focus:outline-none border-r border-themed bg-themed-primary";
    let form_class = "flex border border-themed rounded overflow-hidden";
    let btn_class =
        "text-white bg-green-600 hover:bg-green-700 px-6 py-4 transition-colors duration-200";

    let blur_class = if is_scrolled() {
        "backdrop-blur-md"
    } else {
        ""
    };

    let menu_icon_class = format!(
        "fa {}",
        if is_menu_open() {
            "fa-times"
        } else {
            "fa-bars"
        }
    );

    rsx! {
        div {
            class: format!("flex flex-col items-center {}", if is_light { "text-gray-900" } else { "text-white" }),

            header {
                class: format!("fixed top-0 left-0 right-0 w-full transition-all duration-300 {}", blur_class),
                style: "z-index: 1024; max-width: 100vw;",

                div {
                    class: format!(
                        "{} flex justify-between items-center w-full max-w-[1260px] \
                         mx-auto px-4 py-3 rounded-xl transition-colors duration-300 \
                         border border-transparent",
                        nav_bg
                    ),
                    style: if is_light { "border-color: var(--border-color);" } else { "" },

                    div { class: "flex items-center gap-2",
                        img {
                            src: asset!("/assets/logo.webp"),
                            alt: "Open SASS Logo",
                            class: "w-8 h-8 object-contain shrink-0",
                            loading: "lazy",
                        }
                        span {
                            class: format!("text-base font-bold hidden sm:block {}", title_color),
                            "Open SASS"
                        }
                    }

                    div { class: "flex items-center gap-3 shrink-0",
                        div { class: "hidden md:flex items-center gap-3",
                            ThemeToggle {}
                            Link {
                                to: Route::Home {},
                                class: "{back_btn_class}",
                                i { class: "fa-solid fa-arrow-left text-xs" }
                                "Go Back"
                            }
                        }

                        div { class: "flex md:hidden items-center gap-2",
                            ThemeToggle {}
                            button {
                                class: format!("p-2 {}", hamburger_color),
                                onclick: move |_| is_menu_open.set(!is_menu_open()),
                                aria_expanded: "{is_menu_open()}",
                                aria_label: "Toggle menu",
                                i { class: "{menu_icon_class}" }
                            }
                        }
                    }
                }

                if is_menu_open() {
                    nav {
                        class: format!("{} md:hidden w-full py-3 px-4 transition-colors duration-300", nav_bg),
                        aria_label: "Mobile Blog Navigation",
                        ul { class: "flex flex-col gap-3",
                            li { class: "mt-2",
                                Link {
                                    to: Route::Home {},
                                    class: "{back_btn_class}",
                                    onclick: move |_| is_menu_open.set(false),
                                    i { class: "fa-solid fa-arrow-left text-xs" }
                                    "Go Back"
                                }
                            }
                        }
                    }
                }
            }

            main { class: "flex flex-col items-center mt-32 px-4 pb-16",
                h1 { class: format!("text-4xl font-bold text-center {}", hero_text_color), "Open SASS Blog" }
                p { class: format!("mt-4 text-center max-w-lg {}", hero_sub_color),
                    "Explore our recent posts and stay updated on all things Open SASS."
                }

                div { class: "mt-8 flex flex-col items-center",
                    form {
                        class: "{form_class}",
                        onsubmit: move |e| {
                            e.stop_propagation();
                            handle_subscribe(());
                        },
                        input {
                            r#type: "email",
                            placeholder: "Enter your email",
                            class: "{input_class}",
                            value: "{email}",
                            oninput: move |e| email.set(e.value()),
                            required: true
                        }
                        button {
                            r#type: "submit",
                            class: "{btn_class}",
                            "Subscribe"
                        }
                    }
                    if let Some(message) = feedback_message() {
                        p { class: "text-green-500 mt-4 text-sm", "{message}" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BlogHeader() -> Element {
    let mut is_menu_open = use_signal(|| false);
    let mut is_scrolled = use_signal(|| false);
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);

    use_effect(move || {
        let window = web_sys::window().expect("no global `window` exists");
        let mut is_scrolled_inner = is_scrolled;

        let closure = Closure::wrap(Box::new(move || {
            let window = web_sys::window().expect("no global `window` exists");
            let scroll_y = window.scroll_y().unwrap_or(0.0);
            is_scrolled_inner.set(scroll_y > 50.0);
        }) as Box<dyn FnMut()>);

        window
            .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .expect("failed to add scroll event listener");

        closure.forget();
    });

    let nav_bg = "nav-themed shadow-md";
    let link_color = "text-themed-secondary hover:text-themed-accent";
    let title_color = "text-themed-primary";
    let hamburger_color = "text-themed-primary";
    let back_btn_class = "text-themed-secondary bg-themed-card border border-themed hover:bg-themed-secondary hover:text-themed-primary px-4 py-2 rounded-lg text-sm transition-colors duration-200 flex items-center gap-2";

    let header_class = format!(
        "w-full fixed top-0 left-0 right-0 transition-all duration-300 {}",
        if is_scrolled() {
            "backdrop-blur-md"
        } else {
            ""
        }
    );

    let menu_icon_class = format!(
        "fa {}",
        if is_menu_open() {
            "fa-times"
        } else {
            "fa-bars"
        }
    );

    rsx! {
        header {
            class: "{header_class}",
            style: "z-index: 1024;",

            div {
                class: format!(
                    "{} flex justify-between items-center w-full max-w-[1260px] mx-auto px-4 py-3 rounded-xl relative transition-colors duration-300 border border-transparent mt-2",
                    nav_bg
                ),
                style: if is_light { "border-color: var(--border-color);" } else { "" },

                div { class: "flex items-center gap-2",
                    img {
                        src: asset!("/assets/logo.webp"),
                        alt: "Open SASS Logo",
                        class: "w-8 h-8 object-contain shrink-0",
                        loading: "lazy",
                    }
                    span {
                        class: format!("text-base font-bold hidden sm:block {}", title_color),
                        "Open SASS"
                    }
                }

                nav {
                    class: "hidden md:flex items-center gap-6",
                    aria_label: "Blog Navigation",
                    a {
                        href: "/#home",
                        class: format!("text-sm uppercase whitespace-nowrap transition-colors duration-200 {}", link_color),
                        i { class: "fa-solid fa-house-chimney mr-1.5", aria_hidden: "true" }
                        "Home"
                    }
                    a {
                        href: "/#blog",
                        class: format!("text-sm uppercase whitespace-nowrap transition-colors duration-200 {}", link_color),
                        i { class: "fa-solid fa-newspaper mr-1.5", aria_hidden: "true" }
                        "Blog"
                    }
                }

                div { class: "flex items-center gap-3",
                    div { class: "hidden md:flex items-center gap-3",
                        ThemeToggle {}
                        Link {
                            to: Route::Home {},
                            class: "{back_btn_class}",
                            i { class: "fa-solid fa-arrow-left text-xs" }
                            "Go Back"
                        }
                    }

                    div { class: "flex md:hidden items-center gap-2",
                        ThemeToggle {}
                        button {
                            class: format!("p-2 {}", hamburger_color),
                            onclick: move |_| is_menu_open.set(!is_menu_open()),
                            aria_expanded: "{is_menu_open()}",
                            aria_label: "Toggle menu",
                            i { class: "{menu_icon_class}" }
                        }
                    }
                }
            }

            if is_menu_open() {
                nav {
                    class: format!("{} md:hidden w-full py-3 px-4 transition-colors duration-300", nav_bg),
                    aria_label: "Mobile Blog Navigation",
                    ul { class: "flex flex-col gap-3",
                        li {
                            a {
                                href: "/#home",
                                class: format!("flex items-center text-sm uppercase transition-colors duration-200 {}", link_color),
                                onclick: move |_| is_menu_open.set(false),
                                i { class: "fa-solid fa-house-chimney mr-2", aria_hidden: "true" }
                                "Home"
                            }
                        }
                        li {
                            a {
                                href: "/#blog",
                                class: format!("flex items-center text-sm uppercase transition-colors duration-200 {}", link_color),
                                onclick: move |_| is_menu_open.set(false),
                                i { class: "fa-solid fa-newspaper mr-2", aria_hidden: "true" }
                                "Blog"
                            }
                        }
                        li { class: "mt-2",
                            Link {
                                to: Route::Home {},
                                class: "{back_btn_class}",
                                onclick: move |_| is_menu_open.set(false),
                                i { class: "fa-solid fa-arrow-left text-xs" }
                                "Go Back"
                            }
                        }
                    }
                }
            }
        }
    }
}
