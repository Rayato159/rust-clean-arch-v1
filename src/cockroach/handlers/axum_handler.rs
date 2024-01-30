use async_trait::async_trait;

use axum::body::Body;
use axum::response::{IntoResponse, Json, Response};

use super::handlers::CockroachHandler;

use crate::cockroach::usecases::usecases::CockroachUsecase;

use crate::cockroach::models::cockroach::InsertCockroachData;

enum SuccessResponse {
    CockroachDetectedSuccess
}

impl IntoResponse for SuccessResponse {
    fn into_response(self) -> Response {
        Response::new(Body::new("OK".to_string()))
    }
}

#[derive(Clone)]
pub struct CockroachAxumHandler<T> {
    usecase: T
}

impl<T> CockroachAxumHandler<T>
where
    T: CockroachUsecase + Sync + Send
{
    pub fn new(usecase: T) -> CockroachAxumHandler<T> {
        Self { usecase }
    }
}

#[async_trait]
impl<T> CockroachHandler for CockroachAxumHandler<T>
where
    T: CockroachUsecase + Sync + Send,
{
    async fn cockroach_detected(&self, Json(cockroach_to_insert): Json<InsertCockroachData>) -> Box<dyn IntoResponse> {
        self.usecase.cockroach_detected(cockroach_to_insert).await;

        Box::new(SuccessResponse::CockroachDetectedSuccess)
    }
}