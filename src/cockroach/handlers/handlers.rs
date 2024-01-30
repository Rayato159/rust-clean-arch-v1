use async_trait::async_trait;

use axum::response::{IntoResponse, Json};

use crate::cockroach::models::cockroach::InsertCockroachData;

#[async_trait]
pub trait CockroachHandler {
    async fn cockroach_detected(&self, Json(cockroach_to_insert): Json<InsertCockroachData>) -> Box<dyn IntoResponse>;
}