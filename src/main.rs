#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
use open_sass::router::Route;

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
