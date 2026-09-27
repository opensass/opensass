// Copyright (c) 2024 Open SASS Community
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#![allow(non_snake_case)]
#![allow(unused)]

mod blog;
mod components;
#[cfg(feature = "server")]
mod db;
mod pages;
mod router;
mod server;
mod theme;

use crate::router::Route;
use crate::theme::WelcomeScreen;

use ::theme::dioxus::ThemeProvider;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dotenv::dotenv;

#[cfg(feature = "server")]
use {
    axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    axum::http::Method,
    axum::{Extension, Router},
    open_sass::db::get_client,
    std::sync::Arc,
    tower_http::cors::{Any, CorsLayer},
};

#[cfg(not(feature = "web"))]
#[derive(Clone)]
pub struct AppState {
    client: mongodb::Client,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const OPEN_SASS_JSON_LD: &str = r#"{"@context":"https://schema.org","@type":"WebSite","name":"Open SASS","url":"https://opensass.org","description":"Open SASS gives you everything you need to create, deploy, and scale full-stack applications using Rust and WebAssembly.","potentialAction":{"@type":"SearchAction","target":{"@type":"EntryPoint","urlTemplate":"https://opensass.org/blogs?search={search_term_string}"},"query-input":"required name=search_term_string"},"publisher":{"@type":"Organization","name":"Open SASS","url":"https://opensass.org","logo":{"@type":"ImageObject","url":"https://opensass.org/assets/og-image.jpg"},"sameAs":["https://x.com/opensassorg","https://github.com/opensass"]}}"#;

fn static_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("public")
}

#[cfg(feature = "web")]
fn main() {
    dotenv().ok();
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting client");
    dioxus::launch(App);
}

#[cfg(not(feature = "web"))]
#[tokio::main]
async fn main() {
    use tokio::net::TcpListener;

    dotenv().ok();
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting server");

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    let client = get_client().await;

    let state = Arc::new(AppState {
        client: client.clone(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let router = axum::Router::new()
        .route(
            "/robots.txt",
            axum::routing::get(|| async {
                let content = std::fs::read_to_string("assets/robots.txt").unwrap_or_default();
                (
                    axum::http::StatusCode::OK,
                    [("content-type", "text/plain")],
                    content,
                )
            }),
        )
        .route(
            "/sitemap.xml",
            axum::routing::get(|| async {
                let content = std::fs::read_to_string("assets/sitemap.xml").unwrap_or_default();
                (
                    axum::http::StatusCode::OK,
                    [("content-type", "application/xml")],
                    content,
                )
            }),
        )
        .route(
            "/llms.txt",
            axum::routing::get(|| async {
                let content = std::fs::read_to_string("assets/llms.txt").unwrap_or_default();
                (
                    axum::http::StatusCode::OK,
                    [("content-type", "text/plain")],
                    content,
                )
            }),
        )
        .layer(cors)
        .layer(Extension(state))
        .serve_dioxus_application(
            ServeConfig::builder()
                .incremental(IncrementalRendererConfig::new().static_dir("static"))
                .build()
                .unwrap(),
            App,
        )
        .into_make_service();
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", r#type: "image/x-icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        document::Link { rel: "stylesheet", href: "/assets/fontawesome/fa-local.css" }
        document::Link { rel: "stylesheet", href: "/assets/fonts/fonts.css" }

        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1, maximum-scale=1" }
        document::Meta { name: "description", content: "Open SASS gives you everything you need to create, deploy, and scale full-stack applications using Rust and WebAssembly." }
        document::Meta { name: "robots", content: "index, follow" }
        document::Meta { name: "keywords", content: "Open SASS, Rust, SASS, WebAssembly, WASM, full-stack, community, tools, programming, open-source" }
        document::Meta { name: "author", content: "Mahmoud Harmouch" }
        document::Meta { name: "copyright", content: "© 2024 Open SASS Community. All rights reserved." }
        document::Meta { name: "revisit-after", content: "7 days" }
        document::Meta { name: "language", content: "English" }
        document::Meta { name: "rating", content: "General" }
        document::Meta { name: "designer", content: "Mahmoud Harmouch" }
        document::Meta { name: "reply-to", content: "oss@opensass.org" }
        document::Meta { name: "target", content: "all" }
        document::Meta { name: "audience", content: "all" }
        document::Meta { name: "mobile-web-app-capable", content: "yes" }

        document::Meta { property: "og:title", content: "Open SASS - Rust and WebAssembly Community" }
        document::Meta { property: "og:description", content: "Open SASS gives you everything you need to create, deploy, and scale full-stack applications using Rust and WebAssembly." }
        document::Meta { property: "og:image", content: "https://opensass.org/assets/og-image.jpg" }
        document::Meta { property: "og:url", content: "https://opensass.org/" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:site_name", content: "Open SASS | Rust and WebAssembly Community" }
        document::Meta { property: "og:locale", content: "en_US" }
        document::Meta { property: "og:image:width", content: "1200" }
        document::Meta { property: "og:image:height", content: "630" }

        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "Open SASS - Rust and WebAssembly Community" }
        document::Meta { name: "twitter:description", content: "Open SASS gives you everything you need to create, deploy, and scale full-stack applications using Rust and WebAssembly." }
        document::Meta { name: "twitter:image", content: "https://opensass.org/assets/og-image.jpg" }
        document::Meta { name: "twitter:site", content: "@opensassorg" }
        document::Meta { name: "twitter:creator", content: "@opensassorg" }
        document::Meta { name: "twitter:url", content: "https://opensass.org/" }

        document::Meta { name: "msapplication-TileColor", content: "#ffffff" }
        document::Meta { name: "msapplication-TileImage", content: "/assets/ms-icon-144x144.png" }
        document::Meta { name: "theme-color", content: "#ffffff" }
        document::Meta { name: "pinterest-rich-pin", content: "true" }

        document::Title { "Open SASS | Rust and WebAssembly Community" }

        document::Link { rel: "apple-touch-icon", sizes: "57x57", href: "/assets/apple-icon-57x57.png" }
        document::Link { rel: "apple-touch-icon", sizes: "60x60", href: "/assets/apple-icon-60x60.png" }
        document::Link { rel: "apple-touch-icon", sizes: "72x72", href: "/assets/apple-icon-72x72.png" }
        document::Link { rel: "apple-touch-icon", sizes: "76x76", href: "/assets/apple-icon-76x76.png" }
        document::Link { rel: "apple-touch-icon", sizes: "114x114", href: "/assets/apple-icon-114x114.png" }
        document::Link { rel: "apple-touch-icon", sizes: "120x120", href: "/assets/apple-icon-120x120.png" }
        document::Link { rel: "apple-touch-icon", sizes: "144x144", href: "/assets/apple-icon-144x144.png" }
        document::Link { rel: "apple-touch-icon", sizes: "152x152", href: "/assets/apple-icon-152x152.png" }
        document::Link { rel: "apple-touch-icon", sizes: "180x180", href: "/assets/apple-icon-180x180.png" }
        document::Link { rel: "icon", r#type: "image/png", sizes: "192x192", href: "/assets/android-icon-192x192.png" }
        document::Link { rel: "icon", r#type: "image/png", sizes: "32x32", href: "/assets/favicon-32x32.png" }
        document::Link { rel: "icon", r#type: "image/png", sizes: "96x96", href: "/assets/favicon-96x96.png" }
        document::Link { rel: "icon", r#type: "image/png", sizes: "16x16", href: "/assets/favicon-16x16.png" }
        document::Link { rel: "canonical", href: "https://opensass.org/" }

        document::Script {
            r#type: "application/ld+json",
            dangerous_inner_html: "{OPEN_SASS_JSON_LD}"
        }

        ThemeProvider {
            WelcomeScreen {}
            Router::<Route> {}
        }
    }
}
