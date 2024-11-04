use crate::server::auth::controller::register_user;
use crate::server::auth::response::RegisterUserSchema;
use dioxus::prelude::*;
use regex::Regex;

#[component]
pub fn Register() -> Element {
    // TODO: split into multiple comps
    let navigator = use_navigator();

    let mut name = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let mut error_message = use_signal(|| None::<String>);
    let mut email_valid = use_signal(|| true);
    let mut password_valid = use_signal(|| true);

    let mut show_password = use_signal(|| false);

    let validate_email = |email: &str| {
        let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
        pattern.is_match(email)
    };

    let validate_password = |password: &str| !password.is_empty();

    let handle_email_change = move |e: Event<FormData>| {
        let value = e.value();
        email.set(value.clone());
        email_valid.set(validate_email(&value));
    };

    let handle_password_change = move |e: Event<FormData>| {
        let value = e.value();
        password.set(value.clone());
        password_valid.set(validate_password(&value));
    };

    let handle_register = move |_| {
        let name = name().clone();
        let email = email().clone();
        let password = password().clone();

        if !validate_email(&email) || password.is_empty() {
            error_message.set(Some(
                "Please provide a valid email and password.".to_string(),
            ));
            return;
        }

        spawn(async move {
            match register_user(RegisterUserSchema {
                name,
                email,
                password,
            })
            .await
            {
                Ok(_) => {
                    navigator.push("/login");
                }
                Err(e) => {
                    error_message.set(Some(e.to_string()));
                }
            }
        });
    };

    rsx! {
        div { class: "min-h-screen bg-gradient-to-tr from-gray-400 to-gray-600 flex items-center justify-center",
            div { class: "w-96 bg-white rounded-lg shadow-lg p-8",
                h1 { class: "text-4xl text-gray-800 pb-6 text-center", "Register" }

                if let Some(error) = error_message() {
                    p { class: "mb-3 bg-red-600 text-white px-4 py-3 font-semibold rounded-md text-center text-base", "{error}" }
                }

                div { class: "mb-6",
                    label { class: "text-base text-gray-800", "Name" }
                    input {
                        class: "input w-full px-4 py-3 rounded-lg border border-gray-300 bg-gray-100 mt-2",
                        r#type: "text",
                        placeholder: "Enter your name",
                        value: "{name}",
                        oninput: move |e| name.set(e.value())
                    }
                }

                div { class: "mb-6",
                    label { class: "text-base text-gray-800", "Email" }
                    input {
                        class: "input w-full px-4 py-3 rounded-lg border border-gray-300 bg-gray-100 mt-2",
                        r#type: "text",
                        placeholder: "Enter your email",
                        value: "{email}",
                        oninput: handle_email_change
                    }
                    if !email_valid() {
                        p { class: "text-red-500 text-sm mt-1", "Enter a valid email address" }
                    }
                }

                div { class: "mb-6",
                    label { class: "text-base text-gray-800", "Password" }
                    div { class: "relative",
                        input {
                            class: "input w-full px-4 py-3 rounded-lg border border-gray-300 bg-gray-100 mt-2",
                            r#type: if show_password() { "text" } else { "password" },
                            placeholder: "Enter your password",
                            value: "{password}",
                            oninput: handle_password_change
                        }
                        button {
                            class: "absolute inset-y-0 right-0 pr-3 flex items-center text-gray-600",
                            onclick: move |_| show_password.set(!show_password()),
                            // TODO: Use dioxus free icons crate
                            i { class: format!("fa {}", if show_password() { "fa-eye" } else { "fa-eye-slash" }) }
                        }
                    }
                    if !password_valid() {
                        p { class: "text-red-500 text-sm mt-1", "Password can't be blank" }
                    }
                }

                button {
                    class: "w-full bg-gray-600 hover:bg-gray-700 text-white py-3 rounded-lg text-base font-medium tracking-wide",
                    onclick: handle_register,
                    "Register"
                }

                p { class: "w-full text-center mt-4",
                    span { class: "text-base text-gray-800", "Already have an account? " }
                    a { class: "text-base text-blue-600 hover:underline", href: "/admin/login", "Log In" }
                }
            }
        }
    }
}
