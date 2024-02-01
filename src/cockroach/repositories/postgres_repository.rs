use std::sync::Arc;
use async_trait::async_trait;
use super::repositories::CockroachRepository;
use crate::{cockroach::entities::cockroach::Cockroach, database::database::Database};

#[derive(Clone)]
pub struct CockroachPostgresRepository<T> 
where
    T: Database + Send + Sync + 'static,
{
    db: Arc<T>,
}

impl<T> CockroachPostgresRepository<T> 
where
    T: Database + Send + Sync + 'static,
{
    pub fn new(db: Arc<T>) -> CockroachPostgresRepository<T> {
        Self { db }
    }
}

#[async_trait]
impl<T> CockroachRepository for CockroachPostgresRepository<T> 
where
    T: Database + Send + Sync + 'static,
{
    async fn insert_cockroach_data(&self, cockroach_data: &Cockroach) {
        let conn = self.db.get_db().await;
    }
}