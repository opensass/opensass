use crate::blog::router_blog::BookRoute as BlogRoute;
use crate::pages::blogs::ArrowRight;
use crate::router::Route;
use chrono::prelude::*;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq, Debug)]
pub struct BlogHomeCardProps {
    pub title: String,
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
}

#[component]
pub fn BlogHomeCard(props: BlogHomeCardProps) -> Element {
    // let formatted_date = props
    //     .created_at
    //     .parse::<DateTime<Utc>>()
    //     .expect("Invalid date format")
    //     .format("%b %d, %Y")
    //     .to_string();

    let read_time = (props.desc.len() as f64 / 7000.0).max(1.0);
    let format_time = format!("{:.2}", read_time);

    rsx! {
        a {
            href: "/blog/{props.slug}",
            class: "flex flex-col border border-gray-300 rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition transform hover:scale-105",

            if let Some(img_url) = &props.img {
                img {
                    src: "{img_url}",
                    alt: "{props.title}",
                    class: "w-full h-48 object-cover"
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
                    class: "text-gray-600 text-sm",
                    dangerous_inner_html: "{props.desc.chars().take(30).collect::<String>()}...",
                }

                div {
                    class: "text-gray-500 text-xs mt-2",
                    // TODO: Determine the correct formula for calculating the read time.
                    // Currently, this is a hardcoded approximate value that seems to work.
                    "{props.created_at} · {format_time} min read"
                }

                a {
                    href: "/blog/{props.slug}",
                    class: "text-blue-500 hover:underline mt-4",
                    "Read more →"
                }
            }
        }
    }
}

#[component]
pub fn BlogCard(props: BlogCardProps) -> Element {
    // let formatted_date = props
    //     .created_at
    //     .parse::<DateTime<Utc>>()
    //     .expect("Invalid date format")
    //     .format("%b %d, %Y")
    //     .to_string();

    let read_time = (props.desc.len() as f64 / 7000.0).max(1.0);
    let format_time = format!("{:.2}", read_time);

    rsx! {
        div {
            class: "flex flex-col md:flex-row gap-4 p-6 bg-gray-900 border border-gray-700 rounded-lg hover:bg-gray-800 shadow-lg hover:shadow-2xl transition-all duration-300 hover:transform hover:scale-105",
            if let Some(img_url) = &props.img {
                div {
                    class: "w-full md:w-1/3 h-48 rounded-lg overflow-hidden",
                    img {
                        src: "{img_url}",
                        alt: "{props.title}",
                        class: "object-cover w-full h-full transition-transform duration-300 hover:scale-110"
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
                    class: "text-gray-400",
                    dangerous_inner_html: "{props.desc.chars().take(30).collect::<String>()}...",
                }
                div {
                    class: "flex justify-between items-center text-gray-500 text-sm mt-4",
                    span {
                        // TODO: Determine the correct formula for calculating the read time.
                        // Currently, this is a hardcoded approximate value that seems to work.
                        "{props.created_at} · {format_time} min read"
                    }
                    Link {
                        class: "text-indigo-500 inline-flex items-center mt-4",
                        to: Route::BlogPost { child: props.route },
                        "Read more"
                        ArrowRight {}
                    }
                    div {
                        class: "flex gap-2",
                        a {
                            href: "#",
                            class: "text-gray-500 hover:text-white transition duration-200",
                            i {
                                width: 30,
                                height: 30,
                                class: "fa-brands fa-facebook"
                            }
                        }
                        a {
                            href: "#",
                            class: "text-gray-500 hover:text-white transition duration-200",
                            i {
                                width: 30,
                                height: 30,
                                class: "fa-brands fa-x-twitter"
                            }
                        }
                        a {
                            href: "#",
                            class: "text-gray-500 hover:text-white transition duration-200",
                            i {
                                width: 30,
                                height: 30,
                                class: "fa-brands fa-linkedin"
                            }
                        }
                    }
                }
            }
        }
    }
}
