use async_trait::async_trait;

use super::models::cockroach_model::InsertCockroachData;

pub mod usecase_impl;

#[async_trait]
pub trait CockroachUsecase {
    async fn cockroach_detected(&self, cockroach_to_insert: InsertCockroachData);
}