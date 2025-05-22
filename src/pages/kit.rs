use dioxus::prelude::*;

#[component]
pub fn Kit() -> Element {
    rsx! {
        iframe {
            class: "w-full min-h-full",
            src: "http://opensass.netlify.app/",
            scrolling: "auto",
            style: "border: none; width: 100vw; height: 100vh; overflow: hidden; position: fixed; top: 0; left: 0;",
            frame_border: "0",
            allow: "fullscreen; geolocation; microphone; camera; clipboard-read; clipboard-write; encrypted-media; autoplay",
            referrerpolicy: "no-referrer",
        }
    }
}
