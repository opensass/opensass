use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    title: &'static str,
    subtitle: &'static str,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    rsx! {
        div { class: "max-w-[600px] mb-20 justify-center text-center",
            h2 { class: "text-themed-primary text-4xl md:text-5xl font-bold leading-tight mt-4 mb-6",
                "{props.title}"
            }
            p { class: "text-themed-secondary text-lg leading-relaxed mb-8",
                "{props.subtitle}"
            }
        }
    }
}
