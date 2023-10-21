use std::time::Duration;

use axum::{extract::State, http::StatusCode, Json};
use jwt_authorizer::JwtClaims;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::time::sleep;
use tracing::info;

use super::{auth::User, state::Counter};

#[derive(Debug, Serialize)]
pub struct HelloResponse {
    pub(crate) message: String,
    #[serde(rename = "ageIsEven")]
    pub(crate) age_is_even: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub enum Gender {
    Female,
    Male,
    Diverse,
}

#[derive(Debug, Deserialize)]
pub struct HelloRequest {
    name: String,
    age: Option<u8>,
    gender: Option<Gender>,
}

pub(crate) async fn get_hello(
    JwtClaims(user): JwtClaims<User>,
) -> Result<Json<HelloResponse>, (StatusCode, String)> {
    sleep(Duration::from_millis(100)).await;
    Ok(Json(HelloResponse {
        age_is_even: None,
        message: format!("Hello {} from JSON API", user.preferred_username()),
    }))
}

pub(crate) async fn post_hello(
    State(counter): State<Counter>,
    Json(payload): Json<HelloRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    sleep(Duration::from_millis(100)).await;
    info!("Aufruf mit {:?}", payload);
    let value = counter.inc_and_get();
    Ok(Json(json!({ "counter": value })))
}
