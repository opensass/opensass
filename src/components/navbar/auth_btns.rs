use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AuthButtonsProps {
    is_vertical: bool,
}

#[component]
pub fn AuthButtons(props: AuthButtonsProps) -> Element {
    let button_class = if props.is_vertical {
        "flex flex-col gap-4"
    } else {
        "flex flex-row gap-4"
    };

    rsx! {
        div { class: "{button_class}",
            button {
                class: "border border-black px-5 py-2 text-lg hover:bg-gray-100",
                "Join"
            }
            Link {
                to: "/donate",
                class: "bg-black text-white px-5 py-2 text-lg hover:bg-gray-900",
                "Donate"
            }
        }
    }
}
