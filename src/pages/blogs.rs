use crate::components::blog::card::BlogCard;
use crate::components::blog::header::Header;
use crate::components::common::server::fetch_and_store_posts;
use crate::components::common::server::BLOGS;
use crate::components::common::server::CATEGORIES;
use crate::components::common::server::TOTAL_POSTS;
use crate::components::common::server::TRENDING_POSTS;
use crate::components::footer::Footer;

use dioxus::prelude::*;

#[component]
pub fn Blogs() -> Element {
    let mut page = use_signal(|| 1);
    let mut cat = use_signal(|| None::<String>);
    let mut search_query = use_signal(|| String::new());
    let posts_per_page = 4;

    // TODO: This can cause infinite rerender, fix it.
    let _resource = use_resource(use_reactive(
        (&page(), &cat(), &search_query()),
        move |(page, cat, search_query)| async move {
            fetch_and_store_posts(page, cat, search_query, posts_per_page).await
        },
    ));

    let blogs = BLOGS.read();
    let trending_posts = TRENDING_POSTS.read();
    let categories = CATEGORIES.read();
    let total_posts = TOTAL_POSTS.read();
    let total_pages = (*total_posts / posts_per_page as u64).max(1) + 1;

    let has_prev = page() > 1;
    let has_next = page() * posts_per_page < *total_posts;

    rsx! {
        div {
            class: "bg-gray-900",

            Header {}

            div {
                class: "flex flex-col md:flex-row items-start p-4 md:p-8",

                div {
                    class: "flex-1 flex flex-col items-center",
                    h1 {
                        class: "text-4xl font-bold text-white p-4 rounded mb-8",
                        "{cat().unwrap_or_else(|| String::from(\"All\"))} Blog"
                    }
                    div {
                        class: "my-8 flex flex-col gap-8",
                        if blogs.is_empty() {
                            p { class: "text-gray-400", "No posts available." }
                        } else {
                            for post in blogs.iter() {
                                BlogCard {
                                    title: post.title.clone(),
                                    desc: post.desc.clone(),
                                    img: post.img.clone(),
                                    created_at: post.created_at.clone(),
                                    category: post.category_slug.clone(),
                                    slug: post.slug.clone(),
                                }
                            }
                        }
                    }

                    div {
                        class: "flex justify-center mt-8 gap-2",
                        if has_prev {
                            button {
                                class: "px-4 py-2 text-white bg-gray-700 rounded-lg hover:bg-white hover:text-black transition-all duration-300",
                                onclick: move |_| { page.set(page() - 1); },
                                "Previous"
                            }
                        }
                        for p in 1..=total_pages {
                            button {
                                class: {
                                    format!("px-4 py-2 rounded-lg transition-all duration-300 {}",
                                        if page() == p { "bg-white text-black font-bold" } else { "bg-gray-700 text-white hover:bg-white hover:text-black" }
                                    )
                                },
                                onclick: move |_| { page.set(p); },
                                "{p}"
                            }
                        }
                        if has_next {
                            button {
                                class: "px-4 py-2 text-white bg-gray-700 rounded-lg hover:bg-white hover:text-black transition-all duration-300",
                                onclick: move |_| { page.set(page() + 1); },
                                "Next"
                            }
                        }
                    }
                }
                div {
                    class: "w-full md:w-1/3 p-4 text-white flex flex-col gap-8",

                    div {
                        h2 {
                            class: "text-2xl font-bold mb-4 text-white",
                            "Categories"
                        }

                        div {
                            class: "grid grid-cols-2 gap-4",

                            for category in (*categories).clone() {
                                button {
                                    class: "flex flex-col items-center rounded-lg p-4 hover:bg-gray-800 transition text-gray-300 hover:text-white",
                                    onclick: move |_| { cat.set(Some(category.slug.clone())); page.set(1); },

                                    if let Some(img_url) = category.img.clone() {
                                        img {
                                            src: img_url,
                                            alt: category.title.clone(),
                                            class: "w-12 h-12 rounded-full mb-2",
                                        }
                                    } else {
                                        div {
                                            class: "w-12 h-12 rounded-full bg-gray-600 mb-2",
                                        }
                                    }

                                    span {
                                        class: "text-lg font-medium",
                                        "{category.title}"
                                    }
                                }
                            }
                        }
                    }


                    div {
                        h2 { class: "text-2xl font-bold mb-4", "Search" }
                        input {
                            class: "w-full p-2 rounded-lg text-black",
                            placeholder: "Search...",
                            value: "{search_query()}",
                            oninput: move |e| search_query.set(e.value().clone())
                        }
                    }

                    div {
                        h2 {
                            class: "text-xl font-bold mb-4 text-white",
                            "Trending"
                        }

                        for post in trending_posts.iter() {
                            a {
                                href: "/blog/{post.slug}",
                                class: "hover:bg-gray-800 rounded-lg p-4 flex items-start gap-4 mb-4 p-4",

                                img {
                                    src: "{post.img.as_deref().unwrap_or(\"/default-thumbnail.jpg\")}",
                                    alt: "{post.title}",
                                    class: "w-16 h-16 object-cover rounded-md"
                                }

                                div {
                                    class: "flex flex-col",

                                    span {
                                        class: "text-base font-semibold text-white hover:underline leading-tight",
                                        "{post.title}"
                                    }

                                    div {
                                        class: "text-xs text-gray-400 mt-1",
                                        span {
                                            "{post.created_at.format(\"%b %d %Y\")} · {post.views} Views"
                                        }
                                    }
                                }
                            }
                        }
                    }

                }
            }
            Footer {}
        }
    }
}
