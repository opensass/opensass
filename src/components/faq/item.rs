use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AccordionItemProps {
    question: &'static str,
    answer: &'static str,
    is_active: bool,
    onclick: EventHandler,
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    let display_class = if props.is_active { "block" } else { "hidden" };
    let title_class = if props.is_active {
        "flex justify-between items-center py-4 px-5 bg-gray-600 text-white rounded-md cursor-pointer"
    } else {
        "flex justify-between items-center py-4 px-5 bg-gray-100 text-gray-700 rounded-md cursor-pointer hover:bg-gray-200"
    };

    rsx! {
        div { class: "mb-4  border-2 rounded",
            div { class: "{title_class}", onclick: move |_| props.onclick.call(()),
                span { class: "font-semibold text-lg", "{props.question}" }
                span {
                    // TODO: add icon here instead of "+"
                    class: "transform transition-transform duration-200",
                    style: format!("rotate: {}deg;", if props.is_active { 180 } else { 0 }),
                    "+"
                }
            }
            p { class: "mt-2 px-5 pb-4 text-gray-600 {display_class}", "{props.answer}" }
        }
    }
}
