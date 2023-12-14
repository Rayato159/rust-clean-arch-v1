use async_trait::async_trait;

use crate::cockroach::entities::cockroach::Cockroach;

#[async_trait]
pub trait CockroachRepository {
    async fn insert_cockroach_data(&self, cockroach_data: Cockroach);
}