use super::entities::cockroach::Cockroach;

pub mod postgres_repository;

#[async_trait::async_trait]
pub trait CockroachRepository {
    async fn select_cockroach_data(&self) -> Vec<Cockroach>;
}