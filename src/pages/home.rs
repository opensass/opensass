use crate::components::features::Features;
use crate::components::hero::Hero;
use crate::components::team::Team;
use crate::components::testimonial::Testimonial;
use dioxus::prelude::*;
use gloo::events::EventListener;
use gloo::utils::window;
use std::rc::Rc;

#[component]
pub fn Home() -> Element {
    let mut header_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let mut visible = use_signal(|| false);
    let top_offset = 10.0;

    use_effect(move || {
        // TODO: Fix me
        let listener = EventListener::new(&window(), "scroll", move |_| {
            let scroll_position = window().scroll_x().unwrap_or_default();
            visible.set(scroll_position > top_offset);
        });

        (move || drop(listener))()
    });

    let scroll_to_top = move |_| async move {
        if let Some(header) = header_element() {
            let _ = header.scroll_to(ScrollBehavior::Smooth).await;
        }
    };

    rsx! {
        div {
            onmounted: move |cx| header_element.set(Some(cx.data())),
            Hero {}
            Features {}
            Testimonial {}
            Team {}

            if !visible() {
                div {
                    class: "fixed bottom-4 right-4 bg-gray-500 text-white p-3 rounded-full cursor-pointer hover:bg-gray-600 transition duration-300 ease-in-out",
                    onclick: scroll_to_top,
                    svg {
                        class: "w-6 h-6",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M5 10l7-7m0 0l7 7m-7-7v18"
                        }
                    }
                }
            }
        }
    }
}
