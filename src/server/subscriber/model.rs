use bson::oid::ObjectId;
use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Subscriber {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub email: String,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}
