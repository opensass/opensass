pub(crate) mod card;
pub(crate) mod header;

use crate::components::blog::card::BlogHomeCard;
use crate::components::common::header::Header;
use crate::components::common::server::fetch_and_store_posts;
use crate::components::common::server::BLOGS;
use crate::components::common::server::CATEGORIES;

use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    let mut cat = use_signal(|| None::<String>);
    let page = use_signal(|| 1);
    let posts_per_page = 3;

    let blogs = BLOGS.read();
    let categories = CATEGORIES.read();

    let _resource = use_resource(use_reactive(
        (&page(), &cat()),
        move |(page, cat)| async move {
            fetch_and_store_posts(page, cat, "".to_string(), posts_per_page).await
        },
    ));

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

                for category in (*categories).clone() {
                    button {
                        class: format!("px-4 py-2 rounded-lg {}",
                            if Some(category.slug.clone()) == cat() { "bg-black text-white" } else { "bg-gray-200 text-black" }),
                        onclick: move |_| cat.set(Some(category.slug.clone())),
                        "{category.title}"
                    }
                }
            }

            div {
                class: "mb-8 grid grid-cols-1 md:grid-cols-3 gap-6",

                for post in blogs.iter().take(posts_per_page.try_into().unwrap()) {
                    BlogHomeCard {
                        title: post.title.clone(),
                        desc: post.desc.clone(),
                        img: post.img.clone(),
                        created_at: post.created_at.clone(),
                        category: post.category_slug.clone(),
                        slug: post.slug.clone(),
                    }
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
