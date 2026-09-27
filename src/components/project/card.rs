use dioxus::prelude::*;

#[derive(Clone, Props, PartialEq)]
pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
    pub link: Option<&'static str>,
    pub tech_stack: Vec<&'static str>,
    pub image: &'static str,
}

#[component]
pub fn ProjectCard(project: Project) -> Element {
    rsx! {
        div { class: "shadow-lg rounded-lg overflow-hidden hover:shadow-2xl transition-shadow duration-300 bg-themed-card text-themed-primary",
            img {
                src: "{project.image}",
                alt: "{project.title} image",
                class: "w-full h-48 object-cover",
                loading: "lazy",

            }

            div { class: "p-6",
                h3 { class: "text-2xl font-semibold mb-3", "{project.title}" }

                p { class: "text-themed-secondary mb-4", "{project.description}" }

                div { class: "flex flex-wrap items-center mb-4",
                    for tech in &project.tech_stack {
                        span {
                            class: "text-sm font-medium border border-themed text-themed-secondary bg-themed-card rounded-full px-2 py-1 mr-2 mb-2",
                            "{tech}"
                        }
                    }
                }
                if let Some(link) = &project.link {
                    a {
                        href: "{link}",
                        target: "_blank",
                        class: "flex items-center text-blue-500 hover:text-blue-700 text-sm font-semibold",
                        "Learn more ",
                        i { class: "ml-2 text-md fa-solid fa-globe" }
                    }
                }
            }
        }
    }
}
