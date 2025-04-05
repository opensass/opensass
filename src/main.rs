#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
use dotenv::dotenv;
use open_sass::router::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dotenv().ok();
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");

    #[cfg(feature = "web")]
    {
        LaunchBuilder::new()
            .with_cfg(server_only! {
                let mut cfg = ServeConfig::builder();

                if !cfg!(debug_assertions) {
                    cfg = cfg.incremental(
                        IncrementalRendererConfig::new()
                            .static_dir(static_dir())
                            .clear_cache(false)
                    );
                }

                cfg.build().expect("Unable to build ServeConfig")
            })
            .launch(App);
    }

    #[cfg(feature = "server")]
    {
        use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
        use axum::http::Method;
        use axum::{Extension, Router};
        use open_sass::db::get_client;
        use std::sync::Arc;
        use tower_http::cors::{Any, CorsLayer};

        #[derive(Clone)]
        #[allow(dead_code)]
        pub struct AppState {
            client: mongodb::Client,
        }

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let client = get_client().await;

                let state = Arc::new(AppState {
                    client: client.clone(),
                });

                let cors = CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

                let app = Router::new()
                    .layer(cors)
                    .layer(Extension(state))
                    .serve_dioxus_application(ServeConfig::new().unwrap(), App);

                let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
                let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            });
    }
}

fn App() -> Element {
    let seo_data = SeoData {
        seo_social_title: "Open SASS - Rust and SASS Community".to_string(),
        seo_meta_description:
            "Join the Rust and SASS community for enhanced tools and collaboration.".to_string(),
        seo_meta_keywords: "Rust, SASS, community, tools, programming".to_string(),
        seo_image: asset!("/assets/logo.webp"),
        seo_url: "https://opensass.org".to_string(),
    };
    rsx! {
        // General Meta Tags
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" },
        document::Meta { name: "description", content: seo_data.seo_meta_description.clone() },
        document::Meta { name: "keywords", content: seo_data.seo_meta_keywords.clone() },

        // Open Graph Meta Tags
        document::Meta { property: "og:title", content: seo_data.seo_social_title.clone() },
        document::Meta { property: "og:description", content: seo_data.seo_meta_description.clone() },
        document::Meta { property: "og:image", content: seo_data.seo_image.clone() },
        document::Meta { property: "og:url", content: seo_data.seo_url.clone() },

        // X Card Meta Tags
        document::Meta { name: "twitter:card", content: "summary_large_image" },
        document::Meta { name: "twitter:title", content: seo_data.seo_social_title.clone() },
        document::Meta { name: "twitter:description", content: seo_data.seo_meta_description.clone() },
        document::Meta { name: "twitter:image", content: seo_data.seo_image.clone() },
        document::Meta { name: "twitter:url", content: seo_data.seo_url.clone() },

        document::Title { "Open SASS" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        document::Script { src: "https://kit.fontawesome.com/62e08d355c.js", defer: true }
        Router::<Route> {}
    }
}

fn static_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("public")
}

#[derive(Clone, PartialEq)]
pub struct SeoData {
    pub seo_social_title: String,
    pub seo_meta_description: String,
    pub seo_meta_keywords: String,
    pub seo_image: Asset,
    pub seo_url: String,
}
