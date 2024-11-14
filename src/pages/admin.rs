use crate::components::admin::blog::BlogList;
use crate::components::admin::input::InputField;
use crate::components::admin::sidebar::Sidebar;
use crate::components::admin::sidebar::Tab;
use crate::components::admin::subs::Subscriptions;
use crate::server::auth::controller::about_me;
use crate::server::post::controller::create_post;
use crate::server::post::request::CreatePostRequest;

use dioxus::prelude::*;
use gloo::storage::SessionStorage;
use gloo::storage::Storage;
use regex::Regex;

#[component]
pub fn AdminPanel() -> Element {
    let navigator = use_navigator();
    let mut user_token = use_signal(|| "".to_string());
    let active_tab = use_signal(|| Tab::AddBlog);
    let mut error_message = use_signal(|| None::<String>);

    let title = use_signal(|| "".to_string());
    let desc = use_signal(|| "".to_string());
    let category_slug = use_signal(|| "".to_string());
    let slug = use_signal(|| "".to_string());
    let img = use_signal(|| "".to_string());

    let title_valid = use_signal(|| true);
    let desc_valid = use_signal(|| true);
    let category_valid = use_signal(|| true);
    let slug_valid = use_signal(|| true);
    let img_valid = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            let token: String = SessionStorage::get("jwt").unwrap_or_default();
            if token.is_empty() {
                navigator.push("/admin/login");
            } else {
                match about_me(token.clone()).await {
                    Ok(data) => {
                        let user = data.data.user;
                        if user.role == "admin" {
                            user_token.set(token.clone());
                        } else {
                            navigator.push("/admin/login");
                        }
                    }
                    Err(_) => {
                        navigator.push("/admin/login");
                    }
                }
            }
        });
    });

    let validate_title = |title: &str| !title.trim().is_empty();
    let validate_desc = |desc: &str| !desc.trim().is_empty();
    let validate_category_slug = |category_slug: &str| !category_slug.trim().is_empty();
    let validate_slug = |slug: &str| Regex::new(r"^[a-zA-Z0-9-]+$").unwrap().is_match(slug);
    let validate_img = |img: &str| img.is_empty() || img.starts_with("http");

    let handle_submit = move |e: Event<FormData>| {
        e.stop_propagation();
        if !validate_title(&(title().clone()))
            || !validate_desc(&desc().clone())
            || !validate_category_slug(&category_slug().clone())
            || !validate_slug(&slug().clone())
            || !validate_img(&img().clone())
        {
            error_message.set(Some("Please fill all fields correctly.".to_string()));
            return;
        }

        spawn(async move {
            let request_data = CreatePostRequest {
                title: title().clone(),
                desc: desc().clone(),
                category_slug: category_slug().clone(),
                slug: slug().clone(),
                img: Some(img().clone()),
                views: 0,
            };
            if !user_token().is_empty() {
                match create_post(request_data, user_token()).await {
                    Ok(_) => {
                        error_message.set(Some("Post created successfully".to_string()));
                    }
                    Err(err) => {
                        error_message.set(Some(format!("Error creating post: {}", err)));
                    }
                }
            }
        });
    };

    rsx! {
        div { class: "flex min-h-screen bg-gray-100 text-gray-900",
            Sidebar { active_tab: active_tab.clone() }
            div { class: "flex-1 p-8",
                div { class: "flex justify-between items-center border-b pb-4 mb-6 shadow-sm",
                    h1 { class: "text-3xl font-semibold text-gray-800", "Admin Panel" }
                    div { class: "w-10 h-10 rounded-full overflow-hidden border-2 border-gray-500 shadow-lg",
                        img { src: "path_to_profile_image", class: "w-full h-full object-cover", alt: "Profile Picture" }
                    }
                }

                match active_tab() {
                    Tab::AddBlog => rsx! { div { class: "max-w-2xl bg-white p-8 rounded-lg shadow-xl",
                        h2 { class: "text-2xl font-semibold mb-4 text-gray-800", "Create New Blog Post" },
                        if let Some(error) = (error_message)() {
                            p { class: "text-center mb-4 bg-red-500 text-white px-4 py-2 rounded-lg shadow-md", "{error}" }
                        }
                        form { class: "space-y-4" ,
                            onsubmit: handle_submit,
                            InputField { label: "Title", value: title, is_valid: title_valid, validator: |v| !v.trim().is_empty() },
                            InputField { label: "Description", value: desc, is_valid: desc_valid, validator: |v| !v.trim().is_empty() },
                            InputField { label: "Category Slug", value: category_slug, is_valid: category_valid, validator: |v| !v.trim().is_empty() },
                            InputField { label: "Slug", value: slug, is_valid: slug_valid, validator: |v| Regex::new(r"^[a-zA-Z0-9-]+$").unwrap().is_match(v) },
                            InputField { label: "Image URL (optional)", value: img, is_valid: img_valid, validator: |v| v.is_empty() || v.starts_with("http") },
                            button {
                                class: "w-full bg-blue-600 text-white p-3 rounded-lg mt-4 hover:bg-blue-700 transition",
                                r#type: "submit",
                                "Publish Post"
                            }
                        }
                    }},
                    Tab::BlogList => rsx! { BlogList {} },
                    Tab::Subscriptions => rsx! { Subscriptions {} },
                }
            }
        }
    }
}
