use crate::server::post::controller::get_posts;
use crate::server::post::model::Post;
use chrono::prelude::*;
use dioxus::prelude::*;
use dioxus_logger::tracing;

static BLOGS: GlobalSignal<Vec<Post>> = GlobalSignal::new(Vec::new);
static TOTAL_POSTS: GlobalSignal<u64> = GlobalSignal::new(|| 0);

#[component]
pub fn Blogs() -> Element {
    let mut page = use_signal(|| 1);
    let mut cat = use_signal(|| None::<String>);
    let posts_per_page = 4;

    // TODO: This can cause infinite rerender, fix it.
    let resource = use_resource(use_reactive(
        (&page(), &cat()),
        move |(page, cat)| async move { fetch_and_store_posts(page, cat, posts_per_page).await },
    ));

    let blogs = BLOGS.read();
    let total_posts = TOTAL_POSTS.read();
    let total_pages = (*total_posts / posts_per_page as u64).max(1) + 1;

    let has_prev = page() > 1;
    let has_next = page() * posts_per_page < *total_posts;

    rsx! {
        div {
            class: "flex flex-col items-center p-8 bg-black min-h-screen",
            h1 {
                class: "text-4xl font-bold text-white p-4 rounded mb-8",
                "{cat().unwrap_or_else(|| String::from(\"All\"))} Blog"
            }
            div {
                class: "my-8",
                // h1 { class: "text-3xl font-bold text-white text-center mb-4", "Recent Posts" }
                div {
                    class: "flex flex-col gap-8",
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
                // Pagination
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
        }
    }
}

async fn fetch_and_store_posts(page: u64, cat: Option<String>, limit: u64) {
    match get_posts(page, limit, cat).await {
        Ok(response) => {
            let posts = response.data.posts;
            let total_count = response.data.count;
            *BLOGS.write() = posts;
            *TOTAL_POSTS.write() = total_count;
        }
        Err(err) => tracing::error!("Failed to fetch posts: {err}"),
    }
}

#[derive(Props, Clone, PartialEq, Debug)]
struct BlogCardProps {
    title: String,
    desc: String,
    img: Option<String>,
    created_at: String,
    category: String,
    slug: String,
}

#[component]
fn BlogCard(props: BlogCardProps) -> Element {
    let formatted_date = DateTime::parse_from_rfc3339(&props.created_at)
        .unwrap_or_else(|_| Utc::now().into())
        .format("%b %d, %Y")
        .to_string();

    rsx! {
        a {
            href: "/blogs/{props.slug}",
            class: "flex flex-col md:flex-row gap-4 p-6 bg-gray-900 border border-gray-700 rounded-lg shadow-lg hover:shadow-2xl transition-all duration-300 hover:transform hover:scale-105",
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
                p {
                    class: "text-gray-400",
                    "{props.desc.chars().take(20).collect::<String>()}..."
                }
                div {
                    class: "flex justify-between items-center text-gray-500 text-sm mt-4",
                    span {
                        "{formatted_date} • 3 min read"
                    }
                    div {
                        class: "flex gap-2",
                        // TODO: Replace with dioxus free icons crate
                        a {
                            href: "#",
                            class: "text-gray-500 hover:text-white transition duration-200",
                            i { class: "fab fa-facebook" }
                        }
                        a {
                            href: "#",
                            class: "text-gray-500 hover:text-white transition duration-200",
                            i { class: "fab fa-twitter" }
                        }
                        a {
                            href: "#",
                            class: "text-gray-500 hover:text-white transition duration-200",
                            i { class: "fas fa-link" }
                        }
                    }
                }
            }
        }
    }
}
