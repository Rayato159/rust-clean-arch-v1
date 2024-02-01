use super::database::Database;
use crate::settings::postgres_setting::PostgresSetting;
use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


#[derive(Clone)]
pub struct PostgresDatabase {
    setting: PostgresSetting,
}

impl PostgresDatabase {
    pub fn new(setting: PostgresSetting) -> PostgresDatabase {
        Self { setting }
    }
}

#[async_trait]
impl Database for PostgresDatabase {
    async fn get_db(&self) -> Result<Pool<Postgres>, sqlx::Error> {
        let database_url = self
            .setting
            .clone()
            .url;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url).await?;
    
        Ok(pool)
    }
}