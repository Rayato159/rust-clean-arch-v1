use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use std::{error::Error, result::Result, sync::Arc};
use super::repositories::CockroachRepository;
use crate::cockroach::entities::cockroach::Cockroach;

#[derive(Clone)]
pub struct CockroachPostgresRepository {
    db: Arc<Pool<Postgres>>,
}

impl CockroachPostgresRepository {
    pub fn new(db: Arc<Pool<Postgres>>) -> CockroachPostgresRepository {
        Self { db }
    }
}

#[async_trait]
impl CockroachRepository for CockroachPostgresRepository
where {
    async fn insert_cockroach_data(&self, cockroach_data: &Cockroach) -> Result<Cockroach, Box<dyn Error>> {
        let conn = &self.db;

        Ok(Cockroach {
            id: Some(1),
            amount: 1,
            ..Default::default()
        })
    }
}