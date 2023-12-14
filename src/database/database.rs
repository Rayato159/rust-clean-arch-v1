use sea_orm::{
    DatabaseConnection,
    DbErr
};

#[async_trait::async_trait]
pub trait Database {
    async fn get_db(&self) -> Result<DatabaseConnection, DbErr>;
}