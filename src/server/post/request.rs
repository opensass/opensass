use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreatePostRequest {
    pub slug: String,
    pub title: String,
    pub desc: String,
    pub img: Option<String>,
    pub category_slug: String,
    pub views: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSinglePostRequest {
    pub slug: String,
}
