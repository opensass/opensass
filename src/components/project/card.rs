use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaGlobe;
use dioxus_free_icons::Icon;

#[derive(Clone, Props, PartialEq)]
pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
    pub link: Option<&'static str>,
    pub tech_stack: Vec<&'static str>,
}

#[component]
pub fn ProjectCard(project: Project) -> Element {
    rsx! {
        div { class: "bg-white shadow-lg rounded-lg p-6 hover:shadow-2xl transition-shadow duration-300",
            h3 { class: "text-2xl font-semibold text-gray-800 mb-3", "{project.title}" }
            p { class: "text-gray-600 mb-4", "{project.description}" }

            div { class: "flex flex-wrap items-center mb-4",
                for tech in &project.tech_stack {
                    span { class: "text-sm font-medium text-gray-500 bg-gray-200 rounded-full px-2 py-1 mr-2 mb-2", "{tech}" }
                }
            }

            if let Some(link) = &project.link {
                a {
                    href: "{link}",
                    target: "_blank",
                    class: "flex text-blue-500 hover:text-blue-700 text-sm font-semibold",
                    "Learn more "
                    Icon { icon: FaGlobe }
                }
            }
        }
    }
}
