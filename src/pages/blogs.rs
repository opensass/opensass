#![allow(non_snake_case)]

use crate::blog::router_blog::BookRoute as BlogRoute;
use crate::components::blog::card::BlogCard;
use crate::components::blog::header::Header;
use crate::components::footer::Footer;
use std::collections::HashSet;

use dioxus::prelude::*;

#[component]
pub fn Blogs() -> Element {
    let mut page = use_signal(|| 1);
    let cat = use_signal(|| None::<String>);
    let mut search_query = use_signal(String::new);
    let posts_per_page = 6;

    let mut filtered_posts = use_signal(Vec::new);

    use_effect(move || {
        let filtered = BlogRoute::static_routes()
            .into_iter()
            .rev()
            .filter(|route| {
                let raw_title = &route.page().title;

                if raw_title.contains("[draft]") {
                    return false;
                }

                let items = raw_title.splitn(11, " |---| ").collect::<Vec<_>>();

                if let [_, _title, category, _slug, _, _, _img, ..] = items.as_slice() {
                    match cat() {
                        Some(ref selected) => category.trim() == selected.trim(),
                        None => true,
                    }
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        filtered_posts.set(filtered);
    });

    let paginated_posts = use_memo(move || {
        let posts = filtered_posts();
        let start = (page() - 1) * posts_per_page;
        let end = (start + posts_per_page).min(posts.len());
        posts[start..end].to_vec()
    });

    let total_filtered = filtered_posts().len();
    let total_pages = (total_filtered as f64 / posts_per_page as f64).ceil() as usize;
    let has_prev = page() > 1;
    let has_next = page() < total_pages;

    let mut post_data = vec![];

    for route in BlogRoute::static_routes().into_iter().rev() {
        let raw_title = &route.page().title;

        if raw_title.contains("[draft]") {
            continue;
        }

        let items = raw_title.splitn(11, " |---| ").collect::<Vec<_>>();
        let [_, title, _category, slug, _, _, img, ..] = items.as_slice() else {
            continue;
        };

        post_data.push((title.to_string(), slug.to_string(), img.to_string()));
    }

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
                        for route in paginated_posts() {
                            BlogPostItem { route }
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
                            CategoriesList {cat, page}
                        }
                    }


                    div {
                        h2 { class: "text-2xl font-bold mb-4", "Search" }
                        input {
                            class: "w-full p-2 rounded-lg text-black",
                            placeholder: "Search...",
                            value: "{search_query()}",
                            oninput: move |e| {
                                search_query.set(e.value().clone());
                                let query = search_query().to_lowercase();

                                let posts = BlogRoute::static_routes()
                                    .into_iter()
                                    .rev()
                                    .filter(|route| {
                                        let title = route.page().title.to_lowercase();
                                        title.contains(&query)
                                    })
                                    .collect::<Vec<_>>();

                                filtered_posts.set(posts);
                            }
                        }
                    }

                    div {
                        h2 {
                            class: "text-xl font-bold mb-4 text-white",
                            "Trending"
                        }

                        for post in post_data[..5] {
                                a {
                                    href: "/blogs/{post.1}",
                                    class: "hover:bg-gray-800 rounded-lg p-4 flex items-start gap-4 mb-4 p-4",

                                    img {
                                        src: "{post.2}",
                                        alt: "{post.0}",
                                        class: "w-16 h-16 object-cover rounded-md",
                                        loading: "lazy",

                                    }

                                    div {
                                        class: "flex flex-col",

                                        span {
                                            class: "text-base font-semibold text-white hover:underline leading-tight",
                                            "{post.0}"
                                        }

                                        // div {
                                        //     class: "text-xs text-gray-400 mt-1",
                                        //     span {
                                        //         "{post.created_at.format(\"%b %d %Y\")} · {post.views} Views"
                                        //     }
                                        // }
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

#[component]
fn CategoriesList(cat: Signal<Option<String>>, page: Signal<usize>) -> Element {
    let mut unique_categories = HashSet::new();
    let mut category_items = vec![];

    for route in BlogRoute::static_routes().into_iter().rev() {
        let raw_title = &route.page().title;

        if raw_title.contains("[draft]") {
            continue;
        }

        let items = raw_title.splitn(11, " |---| ").collect::<Vec<_>>();
        let [_, _, category, ..] = items.as_slice() else {
            continue;
        };

        let category = category.to_string();

        if unique_categories.insert(category.clone()) {
            category_items.push(category);
        }
    }

    rsx! { for item in category_items {

        button {
            class: "flex flex-col items-center rounded-lg p-4 hover:bg-gray-800 transition text-gray-300 hover:text-white",
            onclick: move |_| { cat.set(Some(item.clone())); page.set(1); },

            div {
                class: "w-12 h-12 rounded-full bg-gray-600 mb-2",
            }

            span {
                class: "text-lg font-medium",
                "{item}"
            }
        }
    } }
}

pub(crate) fn ArrowRight() -> Element {
    rsx! {
        svg {
            class: "w-4 h-4 ml-1",
            stroke_linejoin: "round",
            stroke: "currentColor",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke_linecap: "round",
            path { d: "M5 12h14M12 5l7 7-7 7" }
        }
    }
}

#[component]
fn BlogPostItem(route: BlogRoute) -> Element {
    let raw_title = &route.page().title;

    if raw_title.contains("[draft]") {
        return rsx! {};
    }

    let items = raw_title.splitn(11, " |---| ").collect::<Vec<_>>();
    let [_, title, category, slug, date, description, img, facebook, x, linkedin] =
        items.as_slice()
    else {
        panic!("Invalid post structure:");
    };

    rsx! {
        BlogCard {
            title: title,
            desc: description,
            route: route,
            img: Some(img.to_string()),
            created_at: date,
            category: category,
            slug: slug,
            facebook: facebook,
            x: x,
            linkedin: linkedin,
        }
    }
}
