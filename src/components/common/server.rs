use crate::server::post::controller::get_categories;
use crate::server::post::controller::get_posts;
use crate::server::post::controller::get_trending_posts;
use crate::server::post::model::Category;
use crate::server::post::model::Post;
use crate::server::subscriber::controller::get_subs;
use crate::server::subscriber::model::Subscriber;
use dioxus::prelude::*;
use dioxus_logger::tracing;

pub static BLOGS: GlobalSignal<Vec<Post>> = GlobalSignal::new(Vec::new);
pub static TOTAL_POSTS: GlobalSignal<u64> = GlobalSignal::new(|| 0);
pub static TRENDING_POSTS: GlobalSignal<Vec<Post>> = GlobalSignal::new(Vec::new);
pub static CATEGORIES: GlobalSignal<Vec<Category>> = GlobalSignal::new(Vec::new);
pub static SUBSCRIBERS: GlobalSignal<Vec<Subscriber>> = GlobalSignal::new(Vec::new);

pub async fn fetch_and_store_posts(
    page: u64,
    cat: Option<String>,
    search_query: String,
    limit: u64,
) {
    match get_posts(page, limit, cat, Some(search_query)).await {
        Ok(response) => {
            let posts = response.data.posts;
            let total_count = response.data.count;
            *BLOGS.write() = posts;
            *TOTAL_POSTS.write() = total_count;
        }
        Err(err) => tracing::error!("Failed to fetch posts: {err}"),
    }

    match get_trending_posts().await {
        Ok(trending_response) => {
            *TRENDING_POSTS.write() = trending_response.data.posts;
        }
        Err(err) => tracing::error!("Failed to fetch trending posts: {err}"),
    }

    match get_categories().await {
        Ok(categories_response) => {
            *CATEGORIES.write() = categories_response.data.categories;
        }
        Err(err) => tracing::error!("Failed to fetch categories: {err}"),
    }
}

pub async fn fetch_and_store_subs() {
    match get_subs().await {
        Ok(response) => {
            let subs = response.data.subscribers;
            *SUBSCRIBERS.write() = subs;
        }
        Err(err) => tracing::error!("Failed to fetch subs: {err}"),
    }
}
