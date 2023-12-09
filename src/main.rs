use axum::{
    routing::get,
    Router,
};

pub mod database;
pub mod settings;
pub mod cockroach;

use settings::AppSetting;

use database::{postgres_database::PostgresDatabase, Database};

use cockroach::{
    repositories::postgres_repository::CockroachPostgresRepository,
    messaging::fcm_messaging::CockroachFCMMessaging,
    usecases::usecase_impl::CockroachUsecaseImpl,
    handlers::axum_handler::CockroachAxumHandler,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let settings = AppSetting::new();

    let db = PostgresDatabase::new(settings.database)
        .get_db()
        .await
        .unwrap();

    let cockroach_repository = CockroachPostgresRepository::new(db);
    let cockroach_messaging = CockroachFCMMessaging::new();

    let cockroach_usecase = CockroachUsecaseImpl::new(
        Box::new(cockroach_repository),
        Box::new(cockroach_messaging),
    );

    let cockroach_handler = CockroachAxumHandler::new(
        Box::new(cockroach_usecase)
    );

    let app = Router::new()
        .route("/", get(|| async { "OK" }));

    let app_url = format!("0.0.0.0:{}", settings.server.port);

    let listener = tokio::net::TcpListener::bind(app_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
