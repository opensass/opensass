use crate::server::auth::controller::get_user_info;
use crate::server::auth::model::User;
use crate::server::post::controller::get_single_post;
use crate::server::post::request::GetSinglePostRequest;
use crate::server::post::response::GetPostResponse;
use dioxus::prelude::*;

#[component]
pub fn Blog(id: String) -> Element {
    let mut post = use_signal(|| None::<GetPostResponse>);
    let mut user_info = use_signal(|| None::<User>);

    let _resource = use_resource(move || {
        let value = id.clone();

        async move {
            if let Ok(response) = get_single_post(GetSinglePostRequest { slug: value }).await {
                post.set(Some(response.data.clone()));

                if let Ok(user_response) = get_user_info(response.data.user).await {
                    user_info.set(Some(user_response.data));
                }
            }
        }
    });

    let mut read_time = 0.;
    if post().is_some() {
        read_time = (post().unwrap().desc.len() as f64 / 7000.0).max(1.0);
    }
    let format_time = format!("{:.2}", read_time);

    rsx! {
        div {
            class: "bg-gray-900 min-h-screen text-white",

            div {
                class: "container mx-auto flex flex-row p-4 gap-6 justify-center items-start",

                div {
                    class: "flex flex-col items-center text-gray-400 space-y-4 md:mr-8",
                    button {
                        class: "hover:text-red-500",
                        "❤️"
                    }
                    button {
                        class: "hover:text-blue-500",
                        "💬"
                    }
                    button {
                        class: "hover:text-yellow-500",
                        "..."
                    }
                }

                div {
                    class: "flex-1 max-w-2xl",

                    if let Some(post) = post() {
                        if let Some(img_url) = &post.img {
                            img { src: "{img_url}", class: "w-full h-64 md:h-80 object-cover rounded-lg mb-6" }
                        }

                        if let Some(user) = user_info() {
                            div {
                                class: "flex items-center mb-4 space-x-4",
                                img {
                                    src: "{user.photo}",
                                    class: "w-12 h-12 rounded-full"
                                }
                                div {
                                    class: "flex flex-col",
                                    span { class: "font-semibold text-lg", "{user.name}" }
                                    span {
                                        class: "text-gray-400 text-sm",
                                        // TODO: Determine the correct formula for calculating the read time.
                                        // Currently, this is a hardcoded approximate value that seems to work.
                                        "{post.created_at.format(\"%B %d, %Y\")} · {format_time} min read"
                                    }
                                }
                            }
                        } else {
                            p { class: "text-gray-400 italic", "Loading author information..." }
                        }

                        h1 {
                            class: "text-3xl md:text-4xl font-bold mb-4 text-center md:text-left",
                            "{post.title}"
                        }

                        span {
                            class: "text-sm text-gray-500 mb-4 block text-center md:text-left",
                            "#{post.category_slug}"
                        }

                        div {
                            class: "text-lg text-gray-300 leading-relaxed",
                            dangerous_inner_html: "{post.desc}",
                        }
                    } else {
                        p { class: "text-gray-400 italic text-center", "Loading post content..." }
                    }
                }
            }
        }
    }
}
