use dioxus::prelude::*;

#[component]
pub fn NanoOG() -> Element {
    rsx! {
        iframe {
            class: "w-full min-h-full",
            src: "https://nanoog.onrender.com",
            scrolling: "auto",
            style: "border: none; width: 100vw; height: 100vh; overflow: hidden; position: fixed; top: 0; left: 0;",
            frame_border: "0",
        }
    }
}