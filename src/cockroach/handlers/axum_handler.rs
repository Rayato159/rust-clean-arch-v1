use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use serde_json::json;

use crate::cockroach::usecases::usecases::CockroachUsecase;

use crate::cockroach::models::cockroach::InsertCockroachData;

#[derive(Clone)]
pub struct CockroachAxumHandler<T> {
    usecase: T
}

impl<T> CockroachAxumHandler<T>
where
    T: CockroachUsecase + Clone + Send + Sync + 'static,
{
    pub fn new(usecase: T) -> CockroachAxumHandler<T> {
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