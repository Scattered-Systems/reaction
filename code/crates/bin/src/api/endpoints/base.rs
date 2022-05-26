use axum;
use serde_json::{json, Value};

pub fn create_route() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(base))
}

pub async fn base() -> axum::Json<Value> {
    let mut cache: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let timestamp: bson::DateTime = chrono::Local::now().into();
    cache.insert(String::from("timestamp"), timestamp.to_string());
    axum::Json(
        json!(cache)
    )
}