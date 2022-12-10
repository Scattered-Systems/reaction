/*
    Appellation: index <routes>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Context;
use axum::extract::Path;
use axum::{routing::get, Extension, Json, Router};
use scsys::prelude::Message;
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new()
        .route("/", get(landing))
        .route("/settings", get(settings))
        .route("/notifications/:id", get(notifications).post(notifications))
}

#[utoipa::path(get, path = "/", responses((status = 200, description = "Landing", body = Json<Value>)))]
/// Define the landing endpoint
pub async fn landing() -> Json<Value> {
    let msg = Message::from("Hello from template-axum-rust");
    Json(json!(msg))
}

/// Implements a notification endpoint
pub async fn notifications(Path(id): Path<usize>) -> Json<Value> {
    let data = json!({ "id": id });
    Json(json!(Message::from(data)))
}

/// Broadcasts the current settings specified by the user for the interface and other technical systems to leverage
pub async fn settings(Extension(ctx): Extension<Context>) -> Json<Value> {
    Json(json!(ctx.settings))
}
