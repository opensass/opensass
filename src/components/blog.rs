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
            class: "flex flex-col items-center p-4 bg-themed-primary min-h-screen justify-center",
            Header {
                title: "Latest Insights",
                subtitle: "Explore our latest posts, expert tips, and updates on everything Open SASS."
            }
            div {
                class: "flex gap-4 mb-8",

                button {
                    class: format!("px-4 py-2 rounded-lg {}",
                        if cat().is_none() { "bg-themed-card text-themed-primary border border-themed" } else { "bg-themed-secondary text-themed-secondary border border-themed" }),
                    onclick: move |_| cat.set(None),
                    "All"
                }
                CategoriesList { cat }
            }

            div {
                class: "mb-8 grid grid-cols-1 md:grid-cols-3 gap-6",
                for route in BlogRoute::static_routes().into_iter().rev().filter(|r| {
                    let title = &r.page().title;
                    if title.contains("[draft]") {
                        return false;
                    }
                    let clean = title.replace(" |---| |---| ", " |---|  |---| ").replace(" |---| |---| ", " |---|  |---| ");
                    let items = clean.splitn(11, " |---| ").collect::<Vec<_>>();
                    items.get(2).map(|c| c.trim()) != Some("legal")
                }).take(3) {
                    BlogHomePostItem { route, cat }
                }
            }
            Link {
                to: "/blogs",
                class: "px-4 py-2 rounded-lg bg-themed-secondary text-themed-primary hover:bg-themed-card transition-colors border border-themed",
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

        if *category == "legal" {
            continue;
        }

        let category = category.to_string();

        if unique_categories.insert(category.clone()) {
            category_items.push(category);
        }
    }

    rsx! { for item in category_items {
    button {
        class: format!("px-4 py-2 rounded-lg {}",
            if Some(item.clone()) == cat() { "bg-themed-card text-themed-primary border border-themed" } else { "bg-themed-secondary text-themed-secondary border border-themed" }),
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

    let clean_title = raw_title
        .replace(" |---| |---| ", " |---|  |---| ")
        .replace(" |---| |---| ", " |---|  |---| ");
    let items = clean_title.splitn(11, " |---| ").collect::<Vec<_>>();

    let title = items.get(1).unwrap_or(&"").trim().to_string();
    let category = items.get(2).unwrap_or(&"").trim().to_string();
    let slug = items.get(3).unwrap_or(&"").trim().to_string();
    let date = items.get(4).unwrap_or(&"").trim().to_string();
    let description = items.get(5).unwrap_or(&"").trim().to_string();
    let img = items.get(6).unwrap_or(&"").trim().to_string();

    if category == "legal" {
        return rsx! {};
    }

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
