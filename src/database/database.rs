use async_trait::async_trait;
use diesel::PgConnection;

#[async_trait]
pub trait Database {
    async fn get_db(&self) -> PgConnection;
}