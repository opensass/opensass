use crate::server::post::controller::{create_comment, get_comments};
use dioxus::prelude::*;

#[component]
pub fn CommentsSection(post_id: String) -> Element {
    let mut comments = use_signal(|| vec![]);
    let mut username = use_signal(|| "".to_string());
    let mut user_email = use_signal(|| "".to_string());
    let mut pic = use_signal(|| "".to_string());
    let mut content = use_signal(|| "".to_string());
    let mut error_message = use_signal(|| None::<String>);
    let mut success_message = use_signal(|| None::<String>);
    let post_id = use_signal(|| post_id);

    let _resource = use_resource(move || async move {
        match get_comments(post_id()).await {
            Ok(fetched_comments) => comments.set(fetched_comments),
            Err(_) => eprintln!("Failed to fetch comments."),
        }
    });

    let submit_comment = move |_| {
        spawn(async move {
            match create_comment(
                post_id(),
                username().clone(),
                user_email().clone(),
                Some(pic().clone()),
                content().clone(),
            )
            .await
            {
                Ok(new_comment) => {
                    let mut current_comments = comments();
                    current_comments.insert(0, new_comment);
                    comments.set(current_comments);
                    username.set("".to_string());
                    user_email.set("".to_string());
                    pic.set("".to_string());
                    content.set("".to_string());
                    success_message.set(Some("Comment added successfully.".to_string()));
                }
                Err(_) => error_message.set(Some("Failed to add comment.".to_string())),
            }
        });
    };

    rsx! {
        div { class: "max-w-2xl mx-auto p-4 rounded-lg shadow",
            h2 { class: "text-xl font-semibold mb-4", "Comments" }

            div { class: "text-gray-800 mb-6",
                input {
                    class: "w-full p-2 border rounded mb-2",
                    placeholder: "Username",
                    value: "{username}",
                    required: true,
                    oninput: move |e| username.set(e.value())
                }
                input {
                    class: "w-full p-2 border rounded mb-2",
                    placeholder: "Email",
                    value: "{user_email}",
                    required: true,
                    oninput: move |e| user_email.set(e.value())
                }
                input {
                    class: "w-full p-2 border rounded mb-2",
                    placeholder: "Picture URL (optional)",
                    value: "{pic}",
                    oninput: move |e| pic.set(e.value())
                }
                textarea {
                    class: "w-full p-2 border rounded mb-2",
                    placeholder: "Your comment...",
                    value: "{content}",
                    required: true,
                    rows: "3",
                    oninput: move |e| content.set(e.value())
                }
                button {
                    class: "bg-blue-500 px-4 py-2 rounded hover:bg-blue-600 transition",
                    onclick: submit_comment,
                    "Comment"
                }
            }

            div { class: "space-y-4",
                for comment in comments.iter() {
                    div { class: "p-4 rounded shadow flex",
                        img { class: "w-10 h-10 rounded-full mr-4", src: "{comment.pic}", alt: "Profile Picture" }
                        div {
                            p { class: "font-semibold", "@{comment.username}" }
                            p { class: "text-sm", "{comment.created_at.format(\"%b %d, %Y\")}" }
                            p { class: "mt-1", "{comment.content}" }
                        }
                    }
                }
            }
        }
    }
}
