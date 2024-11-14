use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ItemProps {
    icon: Element,
    title: &'static str,
    description: &'static str,
}

#[component]
pub fn FeatureItem(props: ItemProps) -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { class: "w-12 h-12", {props.icon} }
            h3 { class: "text-2xl font-bold leading-snug text-black", "{props.title}" }
            p { class: "text-lg text-black leading-relaxed", "{props.description}" }
        }
    }
}
