use crate::blog::router_blog::BookRoute as BlogRoute;
use crate::router::Route;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq, Debug)]
pub struct BlogHomeCardProps {
    pub title: String,
    pub route: BlogRoute,
    pub desc: String,
    pub img: Option<String>,
    pub created_at: String,
    pub category: String,
    pub slug: String,
}

#[derive(Props, Clone, PartialEq, Debug)]
pub struct BlogCardProps {
    pub title: String,
    pub route: BlogRoute,
    pub desc: String,
    pub img: Option<String>,
    pub created_at: String,
    pub category: String,
    pub slug: String,
    pub facebook: String,
    pub x: String,
    pub linkedin: String,
}

#[component]
pub fn BlogHomeCard(props: BlogHomeCardProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col border border-themed rounded-lg shadow-lg overflow-hidden bg-themed-card hover:shadow-xl transition transform hover:scale-105",

            if let Some(img_url) = &props.img {
                img {
                    src: "{img_url}",
                    alt: "{props.title}",
                    class: "w-full h-48 object-cover",
                    loading: "lazy",

                }
            }

            div {
                class: "p-4 flex flex-col gap-2",

                div {
                    class: "text-xs font-semibold text-themed-secondary uppercase",
                    "{props.category}"
                }

                h2 {
                    class: "text-lg font-bold text-themed-primary",
                    "{props.title}"
                }

                div {
                    class: "justify-between flex",
                    span {
                        class: "text-themed-secondary",
                        "{props.desc.chars().take(30).collect::<String>()}...",
                    }
                    Link {
                        class: "text-blue-500 inline-flex items-center",
                        to: Route::BlogPost { child: props.route },
                        "Read more"
                        i { class: "ml-2 text-sm fa-solid fa-arrow-right" }
                    }
                }

                div {
                    class: "text-themed-secondary text-xs mt-2",
                    "{props.created_at}"
                }
            }
        }
    }
}

#[component]
pub fn BlogCard(props: BlogCardProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col md:flex-row gap-4 p-6 bg-themed-card border border-themed rounded-lg hover:bg-themed-secondary shadow-lg hover:shadow-2xl transition-all duration-300 hover:transform hover:scale-105",
            if let Some(img_url) = &props.img {
                div {
                    class: "w-full md:w-1/3 h-48 rounded-lg overflow-hidden",
                    img {
                        src: "{img_url}",
                        alt: "{props.title}",
                        class: "object-cover w-full h-full transition-transform duration-300 hover:scale-110",
                        loading: "lazy",

                    }
                }
            }
            div {
                class: "flex-1 flex flex-col justify-between gap-4",

                div {
                    class: "text-xs font-semibold text-themed-primary bg-themed-secondary px-3 py-1 rounded-full shadow-md self-start mb-2 tracking-wide uppercase",
                    "{props.category}"
                }

                h1 {
                    class: "text-2xl font-bold text-themed-primary",
                    "{props.title}"
                }
                div {
                    class: "justify-between flex",
                    span {
                        class: "text-themed-secondary",
                        "{props.desc.chars().take(70).collect::<String>()}...",
                    }
                    Link {
                        class: "text-indigo-500 inline-flex items-center",
                        to: Route::BlogPost { child: props.route },
                        "Read more"
                        i { class: "ml-2 text-sm fa-solid fa-arrow-right" }
                    }
                }
                div {
                    class: "flex justify-between items-center text-themed-secondary text-sm",
                    span {
                        "{props.created_at}"
                    }
                    div {
                        class: "flex gap-2",
                        a {
                            href: props.facebook,
                            target: "_blank",
                            class: "text-themed-secondary hover:text-themed-primary transition duration-200",
                            i {
                                width: 30,
                                height: 30,
                                class: "text-xl fa-brands fa-facebook"
                            }
                        }
                        a {
                            href: props.x,
                            target: "_blank",
                            class: "text-themed-secondary hover:text-themed-primary transition duration-200",
                            i {
                                width: 30,
                                height: 30,
                                class: "text-xl fa-brands fa-x-twitter"
                            }
                        }
                        a {
                            href: props.linkedin,
                            target: "_blank",
                            class: "text-themed-secondary hover:text-themed-primary transition duration-200",
                            i {
                                width: 30,
                                height: 30,
                                class: "text-xl fa-brands fa-linkedin"
                            }
                        }
                    }
                }
            }
        }
    }
}
