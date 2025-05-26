use bson::oid::ObjectId;
use bson::serde_helpers::chrono_datetime_as_bson_datetime;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Comment {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub post: String,
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
