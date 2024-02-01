use async_trait::async_trait;
use crate::cockroach::models::cockroach::InsertCockroachData;

#[async_trait]
pub trait CockroachUsecase {
    async fn cockroach_detected(&self, cockroach_to_insert: &InsertCockroachData);
}