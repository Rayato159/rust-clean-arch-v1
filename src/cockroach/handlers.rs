use async_trait::async_trait;

use axum::response::{Json, Response};

use super::models::cockroach_model::InsertCockroachData;

pub mod axum_handler;

#[async_trait]
pub trait CockroachHandler {
    async fn cockroach_detected(&self, Json(cockroach_to_insert): Json<InsertCockroachData>) -> impl IntoResponse;
}