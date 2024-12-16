use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Dropdown() -> Element {
    let mut is_dropdown_open = use_signal(|| false);

    let dropdown_items = vec![
        (
            Route::DropdownItem {
                name: "team".into(),
            },
            "Team",
            "fa-solid fa-people-group",
        ),
        (
            Route::DropdownItem {
                name: "testimonials".into(),
            },
            "Testimonials",
            "fa-solid fa-person",
        ),
        (
            Route::DropdownItem {
                name: "mission".into(),
            },
            "Our Mission",
            "fa-solid fa-person-circle-question",
        ),
    ];

    rsx! {
        div { class: "relative",
            button {
                onclick: move |_| is_dropdown_open.set(!is_dropdown_open()),
                class: "flex items-center text-lg gap-1",
                "About Us"
                div {
                    class: "w-6 h-6",
                    i {
                        width: 30,
                        height: 30,
                        class: "fa-solid fa-arrow-down",
                    }
                }
            }

            if is_dropdown_open() {
                div {
                    class: "absolute top-full left-0 bg-white border border-gray-300 mt-2 py-2 shadow-lg w-48",

                    for (to, label, icon) in dropdown_items {
                        div {
                            class: "flex items-start gap-2 p-2",

                            i {
                                width: 30,
                                height: 30,
                                class: icon,
                            }

                            div {
                                Link {
                                    class: "font-semibold",
                                    to: to,
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
