#![allow(unused_imports)]

use crate::server::common::response::SuccessResponse;
use crate::server::post::model::Comment;
use bson::{doc, oid::ObjectId};
use chrono::prelude::*;
use dioxus::prelude::*;
use futures_util::TryStreamExt;
#[cfg(feature = "server")]
use {crate::db::get_client, crate::server::auth::controller::auth};

#[server]
pub async fn get_comments(post_id: String) -> Result<Vec<Comment>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"));
    let comment_collection = db.collection::<Comment>("comments");

    let comment_cursor = comment_collection
        .find(doc! {})
        .sort(doc! { "createdAt": -1 })
        .await
        .map_err(|_| ServerFnError::new("Failed to fetch comments"))?;
    let comments: Vec<Comment> = comment_cursor
        .try_collect::<Vec<Comment>>()
        .await
        .map_err(|_| ServerFnError::new("Error while processing comments"))?
        .into_iter()
        .filter(|comment| {
            match (comment.post.trim().parse::<i32>(), post_id.trim().parse::<i32>()) {
                (Ok(a), Ok(b)) => {a == b},
                _err => {false},
            }
        })
        .collect();
    Ok(comments)
}

#[server]
pub async fn create_comment(
    post_id: String,
    username: String,
    user_email: String,
    pic: Option<String>,
    content: String,
) -> Result<Comment, ServerFnError> {
    if username.trim().is_empty() || user_email.trim().is_empty() || content.trim().is_empty() {
        return Err(ServerFnError::new(
            "All fields except picture are required.",
        ));
    }

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"));

    let comment_collection = db.collection::<Comment>("comments");

    let new_comment = Comment {
        id: ObjectId::new(),
        post: post_id,
        username,
        user_email,
        pic: pic.unwrap_or_else(|| "https://placehold.co/50".to_string()),
        content,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    comment_collection.insert_one(new_comment.clone()).await?;
    Ok(new_comment)
}
