use std::borrow::BorrowMut;

use async_trait::async_trait;
use sea_orm::DatabaseConnection;

use super::repositories::CockroachRepository;

use crate::cockroach::entities::cockroach::Cockroach;

#[derive(Clone)]
pub struct CockroachPostgresRepository {
    db: DatabaseConnection,
}

impl CockroachPostgresRepository {
    pub fn new<'a>(db: &'a DatabaseConnection) -> Self {
        CockroachPostgresRepository { db: db.clone() }
    }
}

#[async_trait]
impl CockroachRepository for CockroachPostgresRepository {
    async fn insert_cockroach_data(&self, cockroach_data: Cockroach) {}
}