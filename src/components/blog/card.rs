use crate::blog::router_blog::BookRoute as BlogRoute;
use crate::pages::blogs::ArrowRight;
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
            class: "flex flex-col border border-gray-300 rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition transform hover:scale-105",

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
                    class: "text-xs font-semibold text-gray-600 uppercase",
                    "{props.category}"
                }

                h2 {
                    class: "text-lg font-bold text-gray-800",
                    "{props.title}"
                }

                div {
                    class: "justify-between flex",
                    span {
                        class: "text-gray-400",
                        "{props.desc.chars().take(30).collect::<String>()}...",
                    }
                    Link {
                        class: "text-blue-500 inline-flex items-center",
                        to: Route::BlogPost { child: props.route },
                        "Read more"
                        ArrowRight {}
                    }
                }

                div {
                    class: "text-gray-500 text-xs mt-2",
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
            class: "flex flex-col md:flex-row gap-4 p-6 bg-gray-900 border border-gray-700 rounded-lg hover:bg-gray-800 shadow-lg hover:shadow-2xl transition-all duration-300 hover:transform hover:scale-105",
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
                    class: "text-xs font-semibold text-white bg-gradient-to-r from-gray-600 to-gray-900 px-3 py-1 rounded-full shadow-md self-start mb-2 tracking-wide uppercase",
                    "{props.category}"
                }

                h1 {
                    class: "text-2xl font-bold text-white",
                    "{props.title}"
                }
                div {
                    class: "justify-between flex",
                    span {
                        class: "text-gray-400",
                        "{props.desc.chars().take(70).collect::<String>()}...",
                    }
                    Link {
                        class: "text-indigo-500 inline-flex items-center",
                        to: Route::BlogPost { child: props.route },
                        "Read more"
                        ArrowRight {}
                    }
                }
                div {
                    class: "flex justify-between items-center text-gray-500 text-sm",
                    span {
                        "{props.created_at}"
                    }
                    div {
                        class: "flex gap-2",
                        a {
                            href: props.facebook,
                            target: "_blank",
                            class: "text-gray-500 hover:text-white transition duration-200",
                            i {
                                width: 30,
                                height: 30,
                                class: "text-xl fa-brands fa-facebook"
                            }
                        }
                        a {
                            href: props.x,
                            target: "_blank",
                            class: "text-gray-500 hover:text-white transition duration-200",
                            i {
                                width: 30,
                                height: 30,
                                class: "text-xl fa-brands fa-x-twitter"
                            }
                        }
                        a {
                            href: props.linkedin,
                            target: "_blank",
                            class: "text-gray-500 hover:text-white transition duration-200",
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
