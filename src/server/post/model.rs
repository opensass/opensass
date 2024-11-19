use bson::oid::ObjectId;
use bson::serde_helpers::chrono_datetime_as_bson_datetime;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Category {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub slug: String,
    pub title: String,
    pub img: Option<String>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub category: ObjectId,
    pub slug: String,
    pub title: String,
    pub desc: String,
    pub img: Option<String>,
    pub views: u64,
    #[serde(rename = "categorySlug")]
    pub category_slug: String,
    #[serde(rename = "userEmail")]
    pub user_email: String,
    pub user: ObjectId,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Comment {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub post: ObjectId,
    // Allow anonymous users to comment, no relation with Users table
    pub username: String,
    #[serde(rename = "userEmail")]
    pub user_email: String,
    pub pic: String,
    pub content: String,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
