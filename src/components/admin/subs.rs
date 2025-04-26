use crate::components::common::server::fetch_and_store_subs;
use crate::components::common::server::SUBSCRIBERS;
use dioxus::prelude::*;

#[component]
pub fn Subscriptions() -> Element {
    let subs = SUBSCRIBERS.read();

    let _resource = use_server_future(|| async move { fetch_and_store_subs().await })?()
    .unwrap();

    rsx! {
        table { class: "min-w-full bg-white text-center",
            thead { class: "bg-gray-800 text-white",
                tr {
                    th { "Email" }
                    th { "Date" }
                    th { "Actions" }
                }
            }
            tbody {
                for sub in subs.iter() {
                    tr {
                        td { "{sub.email}" }
                        td { "{sub.created_at}" }
                        td {
                            button { class: "bg-red text-white rounded px-2 py-1", "x" }
                        }
                    }
                }
            }
        }
    }
}
