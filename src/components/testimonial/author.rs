use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AuthorProps {
    author_image: Asset,
    author_name: &'static str,
    author_title: &'static str,
    company_logo: Asset,
}

#[component]
pub fn AuthorInfo(props: AuthorProps) -> Element {
    rsx! {
        div { class: "flex items-center justify-center space-x-4",
            img { src: "{props.author_image}", class: "h-12 rounded-full" }
            div { class: "text-left",
                p { class: "text-base font-semibold text-black", "{props.author_name}" }
                p { class: "text-sm text-gray-500", "{props.author_title}" }
            }
            div { class: "w-px h-16 bg-black" }
            img { src: "{props.company_logo}", alt: "Company logo", class: "w-30 h-12" }
        }
    }
}
