use crate::server::subscriber::model::Subscriber;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribersResponse {
    pub subscribers: Vec<Subscriber>,
}
