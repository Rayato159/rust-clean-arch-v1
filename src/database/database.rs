use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[async_trait]
pub trait Database {
    async fn get_db(&self) -> Result<Pool<Postgres>, sqlx::Error>;
}