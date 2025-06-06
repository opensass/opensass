use crate::blog::router_blog::BookRoute as BlogRoute;
use crate::components::blog::header::BlogHeader;
use crate::components::comments::CommentsSection;
use crate::components::footer::Footer;
use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    let path: Route = use_route();
    let slug_from_url: String = path
        .to_string()
        .rsplit('/')
        .next()
        .unwrap_or("")
        .to_string();
    let mut blog_info = use_signal(|| None::<(String, String, String, String, String, String)>);
    let mut post_id = use_signal(String::new);

    let mut blog_post = use_signal(|| None::<BlogRoute>);

    use_effect(move || {
        let post = BlogRoute::static_routes()
            .into_iter()
            .rev()
            .find(|route| {
                let raw_title = &route.page().title;
                if raw_title.contains("[draft]") {
                    return false;
                }

                let items = raw_title.splitn(11, " |---| ").collect::<Vec<_>>();
                let [_id, _title, _category, slug, ..] = items.as_slice() else {
                    return false;
                };

                slug == &slug_from_url
            });

        blog_post.set(post);
    });

    use_effect(move || {
        if let Some(route) = blog_post() {
            let raw_title = &route.page().title;
            let items = raw_title.splitn(11, " |---| ").collect::<Vec<_>>();

            if let [id, title, category, slug, date, description, img, ..] = items.as_slice() {
                blog_info.set(Some((
                    title.to_string(),
                    category.to_string(),
                    slug.to_string(),
                    date.to_string(),
                    description.to_string(),
                    img.to_string(),
                )));
                post_id.set(id.to_string());
            } else {
                blog_info.set(None);
                post_id.set(String::new());
            }
        } else {
            blog_info.set(None);
            post_id.set(String::new());
        }
    });

    rsx! {
        div {
            class: "bg-gray-900 min-h-screen text-white",
            BlogHeader {}
            div {
                class: "container mx-auto flex flex-row p-4 gap-6 justify-center items-start",

                div {
                    class: "flex-1 max-w-3xl",

                    if let Some(post) = blog_info() {
                        img { src: "{post.5}", class: "w-full h-64 md:h-80 object-cover rounded-lg mb-6" }

                        div {
                            class: "flex items-center mb-4 space-x-4",
                            img {
                                src: "https://avatars.githubusercontent.com/u/62179149?s=400&u=be78b13411b4e94aac03546fcbc9fb611afc473c&v=4",
                                class: "w-12 h-12 rounded-full",
                                loading: "lazy",

                            }
                            div {
                                class: "flex flex-col",
                                span { class: "font-semibold text-lg", "Mahmoud Harmouch" }
                                span {
                                    class: "text-gray-400 text-sm",
                                    "{post.3}"
                                }
                            }
                        }

                        h1 {
                            class: "text-3xl md:text-4xl font-bold mb-4 text-center md:text-left",
                            "{post.0}"
                        }

                        span {
                            class: "text-sm text-gray-500 mb-4 block text-center md:text-left",
                            "#{post.2}"
                        }

                        div {
                            class: "no-tailwind",
                            Outlet::<Route> {}
                        }
                    } else {
                        p { class: "text-gray-400 italic text-center", "Loading post content..." }
                    }
                    CommentsSection { post_id: post_id }
                }
            }
            Footer {}
        }
    }
}
