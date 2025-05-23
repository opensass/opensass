use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SocialLink {
    pub icon: Element,
    pub link: &'static str,
}

#[component]
pub fn SocialLinks(social_links: Vec<SocialLink>) -> Element {
    rsx! {
        ul { class: "flex justify-center space-x-4 mt-4",
            for social in social_links.iter() {
                li {
                    a {
                        href: "{social.link}",
                        target: "_blank",
                        div {
                            class: "w-6 h-6 opacity-70 hover:opacity-100 transition-opacity duration-200 rounded border border-gray-700 hover:border-gray-200",
                            {social.icon.clone()}
                        }
                    }
                }
            }
        }
    }
}
