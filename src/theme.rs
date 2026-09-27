// Copyright (c) 2024 Open SASS Community
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
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
fn inject_gif_overlay(gif_path: &'static str, gif_ms: i32) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };
    let body = match document.body() {
        Some(b) => b,
        None => return,
    };

    if let Some(existing) = document.get_element_by_id("gif-theme-overlay") {
        let _ = body.remove_child(&existing);
    }

    let overlay = document.create_element("div").unwrap();
    let _ = overlay.set_attribute("id", "gif-theme-overlay");
    let _ = overlay.set_attribute(
        "style",
        "position:fixed;top:0;left:0;right:0;bottom:0;width:100vw;height:100vh;\
         z-index:2147483647;background:#000;\
         display:flex;align-items:center;justify-content:center;\
         opacity:1;transition:opacity 0.35s ease;",
    );

    let img = document.create_element("img").unwrap();
    let _ = img.set_attribute("id", "gif-theme-img");
    let _ = img.set_attribute("src", gif_path);
    let _ = img.set_attribute(
        "style",
        "width:100%;height:100%;object-fit:cover;position:absolute;top:0;left:0;",
    );
    let _ = img.set_attribute("alt", "Theme transition");

    let _ = overlay.append_child(&img);
    let _ = body.append_child(&overlay);

    let freeze_ms = (gif_ms - 80).max(400);
    let freeze_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        if let Some(w) = web_sys::window() {
            if let Some(d) = w.document() {
                if let Some(img_el) = d.get_element_by_id("gif-theme-img") {
                    let _ = img_el.set_attribute("style", "display:none;");
                }

                if let Some(overlay_el) = d.get_element_by_id("gif-theme-overlay") {
                    let _ = overlay_el.set_attribute(
                        "style",
                        "position:fixed;top:0;left:0;right:0;bottom:0;width:100vw;height:100vh;\
                         z-index:2147483647;background:#000;\
                         display:flex;align-items:center;justify-content:center;\
                         opacity:0;transition:opacity 0.35s ease;pointer-events:none;",
                    );
                }

                let remove_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                    if let Some(w2) = web_sys::window() {
                        if let Some(d2) = w2.document() {
                            if let Some(b2) = d2.body() {
                                if let Some(el) = d2.get_element_by_id("gif-theme-overlay") {
                                    let _ = b2.remove_child(&el);
                                }
                            }
                        }
                    }
                }));
                let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
                    remove_closure.as_ref().unchecked_ref(),
                    400,
                );
                remove_closure.forget();
            }
        }
    }));

    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        freeze_closure.as_ref().unchecked_ref(),
        freeze_ms,
    );
    freeze_closure.forget();
}

#[component]
pub fn ThemeToggle() -> Element {
    let theme_ctx = use_theme();
    let mut animating = use_signal(|| false);
    let is_dark = matches!((theme_ctx.theme)(), Theme::Dark | Theme::System);

    let on_toggle_click = {
        let theme_ctx = theme_ctx.clone();
        move |_| {
            if animating() {
                return;
            }

            let current = (theme_ctx.theme)();

            let (gif_path, gif_ms): (&'static str, i32) = match current {
                Theme::Light => ("/assets/gifs/light-to-dark.gif", 3000),
                _ => ("/assets/gifs/dark-to-light.gif", 4000),
            };

            let new_theme = match current {
                Theme::Light => Theme::Dark,
                _ => Theme::Light,
            };

            animating.set(true);

            #[cfg(target_arch = "wasm32")]
            inject_gif_overlay(gif_path, gif_ms);

            let theme_ctx_inner = theme_ctx.clone();
            let switch_ms = gif_ms / 2;
            let switch_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                theme_ctx_inner.set_theme.call(new_theme.clone());
            }));
            #[cfg(target_arch = "wasm32")]
            {
                let window = web_sys::window().unwrap();
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    switch_closure.as_ref().unchecked_ref(),
                    switch_ms,
                );
            }
            switch_closure.forget();

            let mut anim = animating;
            let done_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                anim.set(false);
            }));
            #[cfg(target_arch = "wasm32")]
            {
                let window = web_sys::window().unwrap();
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    done_closure.as_ref().unchecked_ref(),
                    gif_ms + 450,
                );
            }
            done_closure.forget();
        }
    };

    rsx! {
        div { class: "flex items-center justify-center",
            button {
                onclick: on_toggle_click,
                class: "theme-toggle-btn flex items-center justify-between px-1",
                aria_label: "Toggle theme",
                title: if is_dark { "Switch to light mode" } else { "Switch to dark mode" },
                span {
                    class: "absolute top-0.5 left-0.5 w-[22px] h-[22px] rounded-full bg-white shadow transition-transform duration-300",
                    style: if is_dark { "transform: translateX(26px);" } else { "transform: translateX(0);" }
                }
                span {
                    class: "absolute inset-0 flex items-center justify-between px-[6px] text-[11px] z-0 pointer-events-none",
                    i { class: "fas fa-moon text-yellow-400", style: if is_dark { "opacity: 1;" } else { "opacity: 0;" } }
                    i { class: "fas fa-sun text-amber-500", style: if is_dark { "opacity: 0;" } else { "opacity: 1;" } }
                }
            }
        }
    }
}

#[component]
pub fn WelcomeScreen() -> Element {
    let mut shown = use_signal(|| false);

    use_effect(move || {
        if shown() {
            return;
        }
        shown.set(true);

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(w) = web_sys::window() {
                if let Some(d) = w.document() {
                    if let Some(ov) = d.get_element_by_id("welcome-gif-overlay") {
                        let _ = ov.set_attribute(
                            "style",
                            "position:fixed;top:0;left:0;right:0;bottom:0;\
                             width:100vw;height:100vh;z-index:2147483647;\
                             background:#000;display:flex;align-items:center;\
                             justify-content:center;opacity:1;\
                             transition:opacity 0.4s ease;",
                        );
                    }
                }
            }

            let freeze_ms = 2920i32;
            let freeze_cl: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                if let Some(w) = web_sys::window() {
                    if let Some(d) = w.document() {
                        if let Some(img) = d.get_element_by_id("welcome-gif-img") {
                            let _ = img.set_attribute("style", "display:none;");
                        }
                        if let Some(ov) = d.get_element_by_id("welcome-gif-overlay") {
                            let _ = ov.set_attribute(
                                "style",
                                "position:fixed;top:0;left:0;right:0;bottom:0;\
                                 width:100vw;height:100vh;z-index:2147483647;\
                                 background:#000;display:flex;align-items:center;\
                                 justify-content:center;\
                                 opacity:0;transition:opacity 0.4s ease;\
                                 pointer-events:none;",
                            );
                        }
                    }
                }
            }));
            if let Some(w) = web_sys::window() {
                let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
                    freeze_cl.as_ref().unchecked_ref(),
                    freeze_ms,
                );
            }
            freeze_cl.forget();

            let hide_cl: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                if let Some(w) = web_sys::window() {
                    if let Some(d) = w.document() {
                        if let Some(ov) = d.get_element_by_id("welcome-gif-overlay") {
                            let _ = ov.set_attribute("style", "display:none !important;");
                        }
                    }
                }
            }));
            if let Some(w) = web_sys::window() {
                let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
                    hide_cl.as_ref().unchecked_ref(),
                    freeze_ms + 450,
                );
            }
            hide_cl.forget();
        }
    });

    rsx! {
        div {
            id: "welcome-gif-overlay",
            style: "position:fixed;top:0;left:0;right:0;bottom:0;width:100vw;height:100vh;\
                    z-index:2147483647;background:#000;\
                    display:flex;align-items:center;justify-content:center;\
                    opacity:0;transition:opacity 0.4s ease;pointer-events:none;",
            img {
                id: "welcome-gif-img",
                src: "/assets/gifs/welcome.gif",
                alt: "Welcome to Open SASS",
                style: "width:100%;height:100%;object-fit:cover;position:absolute;top:0;left:0;",
            }
        }
    }
}
