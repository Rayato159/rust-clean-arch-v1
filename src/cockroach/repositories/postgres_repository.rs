use async_trait::async_trait;

use sea_orm::DatabaseConnection;

use super::repositories::CockroachRepository;

use crate::cockroach::entities::cockroach::Cockroach;

pub struct CockroachPostgresRepository {
    db: DatabaseConnection,
}

impl CockroachPostgresRepository {
    pub fn new(db: DatabaseConnection) -> impl CockroachRepository {
        CockroachPostgresRepository { db }
    }
}

#[async_trait]
impl CockroachRepository for CockroachPostgresRepository {
    async fn insert_cockroach_data(&self, cockroach_data: Cockroach) {}
}