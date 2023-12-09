use async_trait::async_trait;

use super::entities::cockroach_entities::Cockroach;

pub mod postgres_repository;

#[async_trait]
pub trait CockroachRepository {
    async fn insert_cockroach_data(&self, cockroach_data: Cockroach);
}