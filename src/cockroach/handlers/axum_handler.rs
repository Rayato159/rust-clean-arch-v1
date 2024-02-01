use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use serde_json::json;

use crate::cockroach::usecases::usecases::CockroachUsecase;

use crate::cockroach::models::cockroach::InsertCockroachData;

#[derive(Clone)]
pub struct CockroachAxumHandler<T> {
    usecase: Arc<T>
}

impl<T> CockroachAxumHandler<T>
where
    T: CockroachUsecase + Clone + Send + Sync + 'static,
{
    pub fn new(usecase: Arc<T>) -> CockroachAxumHandler<T> {
        Self { usecase }
    }

    pub async fn cockroach_detected(&self, Json(cockroach_to_insert): Json<InsertCockroachData>) -> impl IntoResponse {
        self.usecase.cockroach_detected(cockroach_to_insert).await;

        (
            StatusCode::OK,
            Json(json!({
                "message": "insert cockroach success",
            })).into_response()
        )
    }
}