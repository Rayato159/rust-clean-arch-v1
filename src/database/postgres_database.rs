use super::database::Database;
use crate::settings::postgres_setting::PostgresSetting;
use tracing::log::info;

use sea_orm::{
    Database as SeaORMDatabase,
    DatabaseConnection,
    DbErr
};

pub struct PostgresDatabase {
    setting: PostgresSetting,
}

impl PostgresDatabase {
    pub fn new(setting: PostgresSetting) -> Self {
        Self {setting}
    }
}

#[async_trait::async_trait]
impl Database for PostgresDatabase {
    async fn get_db(&self) -> Result<DatabaseConnection, DbErr> {
        let db = SeaORMDatabase::connect(&self.setting.url).await?;
        
        info!("Connected to Postgres database.");

        Ok(db)
    }
}