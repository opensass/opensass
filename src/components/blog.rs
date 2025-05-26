pub(crate) mod card;
pub(crate) mod code;
pub(crate) mod header;

use crate::blog::router_blog::BookRoute as BlogRoute;
use crate::components::blog::card::BlogHomeCard;
use crate::components::common::header::Header;
use std::collections::HashSet;

use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    let mut cat = use_signal(|| None::<String>);

    rsx! {
        section {
            id: "blog",
            class: "flex flex-col items-center p-4 bg-white min-h-screen justify-center",
            Header {
                title: "Latest Insights",
                subtitle: "Explore our latest posts, expert tips, and updates on everything Open SASS."
            }
            div {
                class: "flex gap-4 mb-8",

                button {
                    class: format!("px-4 py-2 rounded-lg {}",
                        if cat().is_none() { "bg-black text-white" } else { "bg-gray-200 text-black" }),
                    onclick: move |_| cat.set(None),
                    "All"
                }
                CategoriesList { cat }
            }

            div {
                class: "mb-8 grid grid-cols-1 md:grid-cols-3 gap-6",
                for route in BlogRoute::static_routes().into_iter().rev().take(3) {
                    BlogHomePostItem { route, cat }
                }
            }
            Link {
                to: "/blogs",
                class: "px-4 py-2 rounded-lg bg-gray-200 text-black hover:text-white hover:bg-black",
                "Go To Blog"
            }
        }
    }
}

#[component]
fn CategoriesList(cat: Signal<Option<String>>) -> Element {
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
        class: format!("px-4 py-2 rounded-lg {}",
            if Some(item.clone()) == cat() { "bg-black text-white" } else { "bg-gray-200 text-black" }),
        onclick: move |_| cat.set(Some(item.clone())),
        "{item}"
    } } }
}

#[component]
fn BlogHomePostItem(route: BlogRoute, cat: Signal<Option<String>>) -> Element {
    let raw_title = &route.page().title;

    if raw_title.contains("[draft]") {
        return rsx! {};
    }

    let items = raw_title.splitn(11, " |---| ").collect::<Vec<_>>();
    let [_, title, category, slug, date, description, img, ..] = items.as_slice() else {
        panic!("Invalid post structure:");
    };

    if Some(category.to_string()) == cat() {
        return rsx! {
            BlogHomeCard {
                title: title,
                desc: description,
                route: route,
                img: Some(img.to_string()),
                created_at: date,
                category: category,
                slug: slug,
            }
        };
    }
    if cat().is_none() {
        return rsx! {
            BlogHomeCard {
                title: title,
                desc: description,
                route: route,
                img: Some(img.to_string()),
                created_at: date,
                category: category,
                slug: slug,
            }
        };
    }
    rsx! {}
}
