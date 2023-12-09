use axum::{
    routing::get,
    Router,
};

pub mod database;
pub mod settings;

use database::postgres_database::PostgresDatabase;
use database::Database;

use settings::AppSetting;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let settings = AppSetting::new();

    let db = PostgresDatabase::new(settings.database);

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
