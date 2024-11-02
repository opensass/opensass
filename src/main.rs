#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
#[cfg(feature = "server")]
use dotenv::dotenv;
use open_sass::router::Route;

fn main() {
    #[cfg(feature = "server")]
    dotenv().ok();

    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
