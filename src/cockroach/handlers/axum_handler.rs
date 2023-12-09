use async_trait::async_trait;

use axum::response::{Json, IntoResponse};

use crate::cockroach::{
    handlers::CockroachHandler,
    usecases::CockroachUsecase,
    models::cockroach_model::InsertCockroachData
};

pub struct CockroachAxumHandler {
    usecase: Box<dyn CockroachUsecase + Sync>
}

impl CockroachAxumHandler {
    pub fn new(usecase: Box<dyn CockroachUsecase + Sync>) -> impl CockroachHandler {
        CockroachAxumHandler { usecase }
    }
}

#[async_trait]
impl CockroachHandler for CockroachAxumHandler {
    async fn cockroach_detected(&self, Json(cockroach_to_insert): Json<InsertCockroachData>) -> impl IntoResponse {
        self.usecase.cockroach_detected(cockroach_to_insert).await;
    }
}