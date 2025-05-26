use crate::components::admin::sidebar::Sidebar;
use crate::components::admin::sidebar::Tab;
use crate::components::admin::subs::Subscriptions;
use crate::server::auth::controller::about_me;

use dioxus::prelude::*;
use gloo::storage::SessionStorage;
use gloo::storage::Storage;

#[component]
pub fn AdminPanel() -> Element {
    let navigator = use_navigator();
    let mut user_token = use_signal(|| "".to_string());
    let active_tab = use_signal(|| Tab::Subscriptions);

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

    rsx! {
        div { class: "flex min-h-screen bg-gray-100 text-gray-900",
            Sidebar { active_tab }
            div { class: "flex-1 p-8",
                div { class: "flex justify-between items-center border-b pb-4 mb-6 shadow-sm",
                    h1 { class: "text-3xl font-semibold text-gray-800", "Admin Panel" }
                    div { class: "w-10 h-10 rounded-full overflow-hidden border-2 border-gray-500 shadow-lg",
                        img {
                            src: "path_to_profile_image",
                            class: "w-full h-full object-cover",
                            alt: "Profile Picture",
                            loading: "lazy",

                        }
                    }
                }

                match active_tab() {
                    Tab::Subscriptions => rsx! { Subscriptions {} },
                }
            }
        }
    }
}
