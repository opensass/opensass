use dioxus::prelude::*;

#[component]
pub fn AIBook() -> Element {
    rsx! {
        iframe {
            class: "w-full min-h-full",
            // TODO: Deploy app
            src: "https://aibook-8syx.onrender.com/",
            scrolling: "auto",
            style: "border: none; width: 100vw; height: 100vh; overflow: hidden; position: fixed; top: 0; left: 0;",
            frame_border: "0",
        }
    }
}
