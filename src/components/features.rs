pub(crate) mod grid;
pub(crate) mod item;

use crate::components::common::header::Header;
use crate::components::features::grid::Grid;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
struct Feature {
    icon: Element,
    title: &'static str,
    description: &'static str,
}

#[component]
pub fn Features() -> Element {
    let features = vec![
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "fa-solid fa-headset",
            }},
            title: "Seamless Integration with Wasm Frameworks",
            description: "Easily integrate SASS components within your wasm app, leveraging the power of a robust back-end for your full-stack applications.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "fa-solid fa-book-bible",
            }},
            title: "Extensive Template Library",
            description: "Access a rich library of pre-built templates designed to kickstart your SASS projects and streamline your development process.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "fa-solid fa-diagram-project",
            }},
            title: "Customizable Components",
            description: "Utilize flexible and customizable components that adapt to your project needs, making SASS development efficient and enjoyable.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "fa-solid fa-keyboard",
            }},
            title: "Community-Driven Development",
            description: "Join a passionate community of developers sharing insights, resources, and support to elevate your SASS projects in Rust.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "fa-solid fa-book",
            }},
            title: "Comprehensive Documentation",
            description: "Explore detailed documentation and guides that help you navigate through SASS development, ensuring a smooth learning curve.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "fa-solid fa-people-group",
            }},
            title: "Collaborative Tools for Teams",
            description: "Leverage built-in tools for team collaboration, making it easier to work on projects together while maintaining code integrity.",
        },
    ];

    rsx! {
        section { id: "features", class: "bg-gray-100 py-28 px-16 md:px-4 font-roboto flex min-h-screen justify-center",
            div { class: "max-w-[1312px] mx-auto",
                Header {
                    title: "Discover the Open SASS Community",
                    subtitle: "Join a dynamic community of open-source Rust developers leveraging SASS to create and innovate together."
                }
                Grid { features: features }
            }
        }
    }
}
