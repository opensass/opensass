use crate::server::post::model::{Category, Post};
use bson::oid::ObjectId;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostsResponse {
    pub posts: Vec<Post>,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
    pub id: ObjectId,
    pub slug: String,
    pub title: String,
    pub desc: String,
    pub img: Option<String>,
    pub category_slug: String,
    pub user_email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPostResponse {
    pub id: ObjectId,
    pub slug: String,
    pub title: String,
    pub desc: String,
    pub img: Option<String>,
    pub views: u64,
    pub category_slug: String,
    pub user_email: String,
    pub category: ObjectId,
    pub user: ObjectId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoriesResponse {
    pub categories: Vec<Category>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingPostsResponse {
    pub posts: Vec<Post>,
}
