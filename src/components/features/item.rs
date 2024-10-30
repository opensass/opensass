use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ItemProps {
    icon: &'static str,
    title: &'static str,
    description: &'static str,
}

#[component]
pub fn FeatureItem(props: ItemProps) -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            img { src: props.icon, alt: format!("{} icon", "{props.title}"), class: "w-12 h-12" }
            h3 { class: "text-2xl font-bold leading-snug text-black", "{props.title}" }
            p { class: "text-lg text-black leading-relaxed", "{props.description}" }
        }
    }
}
