#![allow(unused_imports)]

use crate::server::common::response::SuccessResponse;
use crate::server::post::model::{Category, Comment, Post};
use crate::server::post::request::{CreatePostRequest, GetSinglePostRequest};
use crate::server::post::response::{
    CategoriesResponse, GetPostResponse, PostResponse, PostsResponse, TrendingPostsResponse,
};
use bson::{doc, oid::ObjectId};
use chrono::prelude::*;
use dioxus::prelude::*;
use futures_util::TryStreamExt;
#[cfg(feature = "server")]
use {crate::db::get_client, crate::server::auth::controller::auth};

#[server]
pub async fn get_categories() -> Result<SuccessResponse<CategoriesResponse>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"));
    let category_collection = db.collection::<Category>("categories");

    let categories_cursor = category_collection.find(doc! {}).await?;
    let categories: Vec<Category> = categories_cursor.try_collect().await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: CategoriesResponse { categories },
    })
}

#[server]
pub async fn get_trending_posts() -> Result<SuccessResponse<TrendingPostsResponse>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"));
    let post_collection = db.collection::<Post>("posts");

    let filter = doc! {};
    let sort = doc! { "views": -1 };

    let trending_cursor = post_collection.find(filter).sort(sort).limit(4).await?;

    let trending_posts: Vec<Post> = trending_cursor.try_collect().await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: TrendingPostsResponse {
            posts: trending_posts,
        },
    })
}

#[server]
pub async fn get_posts(
    page: u64,
    limit: u64,
    cat: Option<String>,
    query: Option<String>,
) -> Result<SuccessResponse<PostsResponse>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"));
    let post_collection = db.collection::<Post>("posts");

    let skip = limit * (page - 1);

    let mut filter = doc! {};

    if let Some(category) = cat {
        filter.insert("categorySlug", category);
    }

    if let Some(search_query) = query {
        // MongoDB partial matching, case-insensitive
        filter.insert(
            "$or",
            vec![
                doc! { "title": { "$regex": &search_query, "$options": "i" } },
                doc! { "desc": { "$regex": &search_query, "$options": "i" } },
            ],
        );
    }

    let post_cursor = post_collection
        .find(filter.clone())
        .skip(skip)
        .limit(limit as i64)
        .await?;

    let posts: Vec<Post> = post_cursor.try_collect().await?;
    let count = post_collection.count_documents(filter).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: PostsResponse { posts, count },
    })
}

#[server]
pub async fn create_post(
    req: CreatePostRequest,
    token: String,
) -> Result<SuccessResponse<PostResponse>, ServerFnError> {
    // check if user is authenticated
    let user = auth(token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"));
    let post_collection = db.collection::<Post>("posts");
    let category_collection = db.collection::<Category>("categories");

    let mut category = category_collection
        .find_one(doc! { "categorySlug": &req.category_slug })
        .await?;

    if category.is_none() {
        let new_category = Category {
            id: ObjectId::new(),
            slug: req.category_slug.clone(),
            title: req.category_slug.clone(),
            img: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        category_collection.insert_one(new_category.clone()).await?;

        category = Some(new_category);
    }

    // Create new post
    let new_post = Post {
        id: ObjectId::new(),
        slug: req.slug,
        title: req.title,
        desc: req.desc,
        img: req.img,
        views: req.views,
        user: user.id,
        category: category.unwrap().id,
        category_slug: req.category_slug,
        user_email: user.email.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    post_collection.insert_one(new_post.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: PostResponse {
            id: new_post.id,
            slug: new_post.slug,
            title: new_post.title,
            desc: new_post.desc,
            img: new_post.img,
            category_slug: new_post.category_slug,
            user_email: new_post.user_email,
            created_at: new_post.created_at,
        },
    })
}

#[server]
pub async fn get_single_post(
    params: GetSinglePostRequest,
) -> Result<SuccessResponse<GetPostResponse>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"));
    let post_collection = db.collection::<Post>("posts");

    let filter = doc! { "slug": &params.slug };
    let update = doc! { "$inc": { "views": 1 } };

    let updated_post = post_collection
        .find_one_and_update(filter, update)
        .await
        .map_err(|_| ServerFnError::new("Something went wrong while updating the post"))?
        .ok_or(ServerFnError::new("Post not found"))?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: GetPostResponse {
            id: updated_post.id,
            slug: updated_post.slug,
            title: updated_post.title,
            desc: updated_post.desc,
            img: updated_post.img,
            views: updated_post.views,
            category_slug: updated_post.category_slug,
            user_email: updated_post.user_email,
            category: updated_post.category,
            user: updated_post.user,
            created_at: updated_post.created_at,
            updated_at: updated_post.updated_at,
        },
    })
}

#[server]
pub async fn get_comments(post_id: String) -> Result<Vec<Comment>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"));
    let comment_collection = db.collection::<Comment>("comments");

    let filter = doc! { "post": post_id };

    let comment_cursor = comment_collection
        .find(filter)
        .sort(doc! { "createdAt": -1 })
        .await
        .map_err(|_| ServerFnError::new("Failed to fetch comments"))?;
    let comments: Vec<Comment> = comment_cursor
        .try_collect()
        .await
        .map_err(|_| ServerFnError::new("Error while processing comments"))?;

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
        pic: pic.unwrap_or_else(|| "https://via.placeholder.com/50".to_string()),
        content,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    comment_collection.insert_one(new_comment.clone()).await?;
    Ok(new_comment)
}
