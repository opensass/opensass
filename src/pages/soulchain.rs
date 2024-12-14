use dioxus::prelude::*;

#[component]
pub fn SoulChain() -> Element {
    rsx! {
        iframe {
            class: "w-full min-h-full",
            src: "https://nbviewer.org/github/opensass/soulchain-paper/blob/main/soulchain.pdf",
            scrolling: "auto",
            style: "border: none; width: 100vw; height: 100vh; overflow: hidden; position: fixed; top: 0; left: 0;",
            frame_border: "0",
        }
    }
}
