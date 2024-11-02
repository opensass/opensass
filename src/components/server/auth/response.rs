use crate::components::server::auth::model::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SuccessResponse<T> {
    pub status: String,
    pub data: T,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserResponse {
    pub user: User,
}
