#[cfg(feature = "server")]
use crate::db::get_client;
use crate::server::common::response::SuccessResponse;
use crate::server::subscriber::model::Subscriber;
use bson::{doc, oid::ObjectId};
use chrono::prelude::*;
use dioxus::prelude::*;

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
