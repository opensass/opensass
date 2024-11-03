use crate::server::auth::controller::login_user;
use crate::server::auth::response::LoginUserSchema;
use crate::server::post::controller::create_post;
use crate::server::post::request::CreatePostRequest;
use dioxus::prelude::*;
use regex::Regex;

fn extract_token(cookie_str: &str) -> Option<String> {
    let re = Regex::new(r"token=([^;]+)").unwrap();

    if let Some(captures) = re.captures(cookie_str) {
        return Some(captures[1].to_string());
    }

    None
}

#[component]
pub fn CreateBlog() -> Element {
    let mut title = use_signal(|| "".to_string());
    let mut desc = use_signal(|| "".to_string());
    let mut category_slug = use_signal(|| "".to_string());
    let mut slug = use_signal(|| "".to_string());
    let mut img = use_signal(|| "".to_string());
    let mut views = use_signal(|| 0u64);

    let mut title_valid = use_signal(|| true);
    let mut desc_valid = use_signal(|| true);
    let mut category_valid = use_signal(|| true);
    let mut slug_valid = use_signal(|| true);
    let mut img_valid = use_signal(|| true);
    let mut views_valid = use_signal(|| true);

    let mut error_message = use_signal(|| None::<String>);
    let navigator = use_navigator();

    let validate_title = |title: &str| !title.trim().is_empty();
    let validate_desc = |desc: &str| !desc.trim().is_empty();
    let validate_category_slug = |category_slug: &str| !category_slug.trim().is_empty();
    let validate_slug = |slug: &str| Regex::new(r"^[a-zA-Z0-9-]+$").unwrap().is_match(slug);
    let validate_img = |img: &str| img.is_empty() || img.starts_with("http");
    let validate_views = |views: u64| views >= 0;

    let handle_title_change = move |e: Event<FormData>| {
        let value = e.value();
        title.set(value.clone());
        title_valid.set(validate_title(&value));
    };

    let handle_desc_change = move |e: Event<FormData>| {
        let value = e.value();
        desc.set(value.clone());
        desc_valid.set(validate_desc(&value));
    };

    let handle_category_change = move |e: Event<FormData>| {
        let value = e.value();
        category_slug.set(value.clone());
        category_valid.set(validate_category_slug(&value));
    };

    let handle_slug_change = move |e: Event<FormData>| {
        let value = e.value();
        slug.set(value.clone());
        slug_valid.set(validate_slug(&value));
    };

    let handle_img_change = move |e: Event<FormData>| {
        let value = e.value();
        img.set(value.clone());
        img_valid.set(validate_img(&value));
    };

    let handle_views_change = move |e: Event<FormData>| {
        let value = e.value();
        if let Ok(parsed_views) = value.parse::<u64>() {
            views.set(parsed_views);
            views_valid.set(validate_views(parsed_views));
        }
    };

    let handle_submit = move |_| {
        if !validate_title(&title())
            || !validate_desc(&desc())
            || !validate_category_slug(&category_slug())
            || !validate_slug(&slug())
            || !validate_img(&img())
            || !validate_views(views())
        {
            error_message.set(Some("Please fill all fields correctly.".to_string()));
            return;
        }

        // TODO: replace with actual token retrieval
        spawn(async move {
            let mut token = "auth_token".to_string();
            match login_user(LoginUserSchema {
                email: std::env::var("BLOG_ADMIN")
                    .expect("BLOG_ADMIN must be set")
                    .to_string(),
                password: std::env::var("BLOG_PWD")
                    .expect("BLOG_PWD must be set")
                    .to_string(),
            })
            .await
            {
                Ok(data) => match extract_token(&data.data.token) {
                    Some(tok) => token = tok,
                    None => println!("Token not found"),
                },
                Err(e) => {
                    error_message.set(Some(e.to_string()));
                }
            }
            let request_data = CreatePostRequest {
                title: title().clone(),
                desc: desc().clone(),
                category_slug: category_slug().clone(),
                slug: slug().clone(),
                img: Some(img().clone()),
                views: views(),
            };

            // spawn(async move {
            match create_post(request_data, token).await {
                Ok(_) => {
                    navigator.push("/blogs");
                }
                Err(err) => {
                    error_message.set(Some(format!("Error creating post: {}", err)));
                }
            }
            // });
        });
    };

    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen bg-gray-600 text-white p-8",
            h1 { class: "text-3xl font-bold mb-8", "Create New Post" }

            div { class: "w-full max-w-lg bg-gray-800 p-8 rounded-lg shadow-lg",

                if let Some(error) = error_message() {
                    p { class: "text-center mb-3 bg-red-600 text-white px-4 py-3 font-semibold rounded-md", "{error}" }
                }

                div { class: "mb-4",
                    label { class: "text-base text-gray-200", "Title" }
                    input {
                        class: "w-full p-3 rounded border bg-gray-700 mt-2",
                        r#type: "text",
                        placeholder: "Enter post title",
                        value: "{title}",
                        oninput: handle_title_change,
                    }
                    if !title_valid() {
                        p { class: "text-red-500 text-sm mt-1", "Title cannot be empty" }
                    }
                }

                div { class: "mb-4",
                    label { class: "text-base text-gray-200", "Description" }
                    textarea {
                        class: "w-full p-3 rounded border bg-gray-700 mt-2",
                        placeholder: "Enter post description",
                        value: "{desc}",
                        oninput: handle_desc_change,
                    }
                    if !desc_valid() {
                        p { class: "text-red-500 text-sm mt-1", "Description cannot be empty" }
                    }
                }

                div { class: "mb-4",
                    label { class: "text-base text-gray-200", "Category Slug" }
                    input {
                        class: "w-full p-3 rounded border bg-gray-700 mt-2",
                        r#type: "text",
                        placeholder: "Enter category slug",
                        value: "{category_slug}",
                        oninput: handle_category_change,
                    }
                    if !category_valid() {
                        p { class: "text-red-500 text-sm mt-1", "Category Slug cannot be empty" }
                    }
                }

                div { class: "mb-4",
                    label { class: "text-base text-gray-200", "Slug" }
                    input {
                        class: "w-full p-3 rounded border bg-gray-700 mt-2",
                        r#type: "text",
                        placeholder: "Enter post slug",
                        value: "{slug}",
                        oninput: handle_slug_change,
                    }
                    if !slug_valid() {
                        p { class: "text-red-500 text-sm mt-1", "Slug must contain only letters, numbers, or hyphens" }
                    }
                }

                div { class: "mb-4",
                    label { class: "text-base text-gray-200", "Image URL (optional)" }
                    input {
                        class: "w-full p-3 rounded border bg-gray-700 mt-2",
                        r#type: "text",
                        placeholder: "Enter image URL",
                        value: "{img}",
                        oninput: handle_img_change,
                    }
                    if !img_valid() {
                        p { class: "text-red-500 text-sm mt-1", "Image URL must start with http" }
                    }
                }

                div { class: "mb-4",
                    label { class: "text-base text-gray-200", "Views" }
                    input {
                        class: "w-full p-3 rounded border bg-gray-700 mt-2",
                        r#type: "number",
                        placeholder: "Enter number of views",
                        value: "{views}",
                        oninput: handle_views_change,
                    }
                    if !views_valid() {
                        p { class: "text-red-500 text-sm mt-1", "Views must be a non-negative number" }
                    }
                }

                button {
                    class: "w-full p-3 bg-gray-600 rounded hover:bg-gray-700 transition",
                    onclick: handle_submit,
                    "Create Post"
                }
            }
        }
    }
}
