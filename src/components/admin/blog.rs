use crate::components::common::server::fetch_and_store_posts;
use crate::components::common::server::BLOGS;
use dioxus::prelude::*;

#[component]
pub fn BlogList() -> Element {
    let blogs = BLOGS.read();

    let _resource =
        use_server_future(
            || async move { fetch_and_store_posts(1, None, "".to_string(), 100).await },
        )?()
        .unwrap();

    rsx! {
        table { class: "min-w-full bg-white text-center",
            thead { class: "bg-gray-800 text-white",
                tr {
                    th { "Title" }
                    th { "Slug" }
                    th { "Views" }
                    th { "Actions" }
                }
            }
            tbody {
                for blog in blogs.iter() {
                    tr {
                        td { "{blog.title}" }
                        td { "{blog.slug}" }
                        td { "{blog.views}" }
                        td {
                            button { class: "bg-red text-white rounded px-2 py-1", "x" }
                        }
                    }
                }
            }
        }
    }
}
