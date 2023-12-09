use sea_orm::DatabaseConnection;

use crate::cockroach::entities::cockroach::Cockroach;
use crate::cockroach::repositories::CockroachRepository;

pub struct CockroachPostgresRepository {
    db: DatabaseConnection,
}

impl CockroachPostgresRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        CockroachPostgresRepository {db}
    }
}

#[async_trait::async_trait]
impl CockroachRepository for CockroachPostgresRepository {
    async fn select_cockroach_data(&self) -> Vec<Cockroach> {
        vec![]
    }
}