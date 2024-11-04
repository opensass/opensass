#![allow(unused_imports)]

#[cfg(feature = "server")]
use crate::db::get_client;
use crate::server::common::response::SuccessResponse;
use crate::server::subscriber::model::Subscriber;
use crate::server::subscriber::response::SubscribersResponse;
use bson::{doc, oid::ObjectId};
use chrono::prelude::*;
use dioxus::prelude::*;
use futures_util::TryStreamExt;

#[server]
pub async fn subscribe_user(email: String) -> Result<SuccessResponse<Subscriber>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"));
    let subscribers_collection = db.collection::<Subscriber>("subscribers");

    if subscribers_collection
        .find_one(doc! { "email": &email })
        .await?
        .is_some()
    {
        return Err(ServerFnError::new("Email is already subscribed"));
    }

    let new_subscriber = Subscriber {
        id: ObjectId::new(),
        email,
        created_at: Utc::now(),
    };

    subscribers_collection
        .insert_one(new_subscriber.clone())
        .await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: new_subscriber,
    })
}

#[server]
pub async fn get_subs() -> Result<SuccessResponse<SubscribersResponse>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"));
    let subscribers_collection = db.collection::<Subscriber>("subscribers");

    let subscribers_cursor = subscribers_collection.find(doc! {}).await?;

    let subscribers: Vec<Subscriber> = subscribers_cursor.try_collect().await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: SubscribersResponse { subscribers },
    })
}
