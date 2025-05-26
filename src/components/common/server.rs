use crate::server::subscriber::controller::get_subs;
use crate::server::subscriber::model::Subscriber;
use dioxus::prelude::*;
use dioxus_logger::tracing;

pub static SUBSCRIBERS: GlobalSignal<Vec<Subscriber>> = GlobalSignal::new(Vec::new);

pub async fn fetch_and_store_subs() {
    match get_subs().await {
        Ok(response) => {
            let subs = response.data.subscribers;
            *SUBSCRIBERS.write() = subs;
        }
        Err(err) => tracing::error!("Failed to fetch subs: {err}"),
    }
}
