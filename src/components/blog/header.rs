use crate::server::subscriber::controller::subscribe_user;
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut feedback_message = use_signal(|| None::<String>);

    let mut handle_subscribe = move |_| {
        let email_value = email().clone();

        if email_value.is_empty() {
            feedback_message.set(Some("Please enter a valid email.".to_string()));
            return;
        }

        feedback_message.set(None);

        spawn(async move {
            match subscribe_user(email_value.clone()).await {
                Ok(_) => feedback_message.set(Some("Subscription successful!".to_string())),
                Err(_) => {
                    feedback_message.set(Some("Failed to subscribe. Please try again.".to_string()))
                }
            }
        });
    };

    rsx! {
        div { class: "text-white flex flex-col items-center",
            // TODO: move it to a separate navbar comp and use a router outlet
            nav { class: "bg-gray-800  w-full flex justify-between items-center p-6 shadow-md",
                div { class: "text-2xl font-bold flex items-center",
                    img {
                        src: asset!("/assets/logo.webp"),
                        alt: "Logo",
                        class: "h-16 mr-2",
                        loading: "lazy",

                    }
                    span { "Open SASS" }
                }
                Link {
                    to: "/",
                    class: "text-white bg-gray-700 rounded-lg hover:bg-white hover:text-black  px-4 py-2",
                    "Go Back →"
                }
            }

            main { class: "flex flex-col items-center mt-16",
                h1 { class: "text-4xl font-bold", "Open SASS Blog" }
                p { class: "text-gray-300 mt-4 text-center max-w-lg",
                    "Explore our recent posts and stay updated on all things Open SASS."
                }

                div { class: "mt-8 flex flex-col items-center",
                    form { class: "flex border border-black rounded overflow-hidden",
                        onsubmit: move |e| {
                            e.stop_propagation();
                            handle_subscribe(());
                        },
                        input {
                            r#type: "email",
                            placeholder: "Enter your email",
                            class: "p-4 text-gray-700 placeholder-gray-500 focus:outline-none",
                            value: "{email}",
                            oninput: move |e| email.set(e.value()),
                            required: true
                        }
                        button {
                            r#type: "submit",
                            class: "text-white bg-gray-700 hover:bg-white hover:text-black  px-6 py-4",
                            "Subscribe"
                        }
                    }
                    if let Some(message) = feedback_message() {
                        p { class: "text-gray-700 mt-4", "{message}" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BlogHeader() -> Element {
    rsx! {
        div { class: "text-white flex flex-col items-center",
            nav { class: "bg-gray-800  w-full flex justify-between items-center p-6 shadow-md",
                div { class: "text-2xl font-bold flex items-center",
                    img {
                        src: asset!("/assets/logo.webp"),
                        alt: "Logo",
                        class: "h-16 mr-2",
                        loading: "lazy",

                    }
                    span { "Open SASS" }
                }
                Link {
                    to: "/blogs",
                    class: "text-white bg-gray-700 rounded-lg hover:bg-white hover:text-black  px-4 py-2",
                    "Go Back →"
                }
            }
        }
    }
}
