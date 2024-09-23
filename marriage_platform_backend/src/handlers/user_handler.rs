use axum::{Json, extract::Extension, response::IntoResponse};
use crate::services::user_service::{register_user, login_user};
use sqlx::{Pool, Postgres};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

pub async fn register_user_handler(
    Json(payload): Json<RegisterPayload>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> impl IntoResponse {
    match register_user(&pool, &payload.name, &payload.email, &payload.password).await {
        Ok(token) => (axum::http::StatusCode::OK, Json(json!({ "token": token }))).into_response(),
        Err(err) => (axum::http::StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

pub async fn login_user_handler(
    Json(payload): Json<LoginPayload>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> impl IntoResponse {
    match login_user(&pool, &payload.email, &payload.password).await {
        Ok(token) => (axum::http::StatusCode::OK, Json(json!({ "token": token }))).into_response(),
        Err(err) => (axum::http::StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
