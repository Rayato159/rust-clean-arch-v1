use super::database::Database;
use crate::settings::postgres_setting::PostgresSetting;
use async_trait::async_trait;
use diesel::pg::PgConnection;
use diesel::prelude::*;


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
    async fn get_db(&self) -> PgConnection {
        let database_url = self
            .setting
            .clone()
            .url;

        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}