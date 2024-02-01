use std::sync::Arc;

use axum::{
    routing::{get, post}, 
    Router
};

use rust_clean_arch_v1::settings::settings::AppSetting;

use rust_clean_arch_v1::database::{postgres_database::PostgresDatabase, database::Database};

use rust_clean_arch_v1::cockroach::{
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

    let cockroach_usecase = Arc::new(CockroachUsecaseImpl::new(
        Arc::new(CockroachPostgresRepository::new(&db)), 
        Arc::new(CockroachFCMMessaging::new())
    ));

    let cockroach_handler = Arc::new(CockroachAxumHandler::new(
        Arc::clone(&cockroach_usecase)
    ));

    let cockroach_detected_handler = Arc::clone(&cockroach_handler);
    let cockroach_more_handler = Arc::clone(&cockroach_handler);

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/v1/cockroach", post(
            |body| async move { 
                cockroach_detected_handler
                    .cockroach_detected(body).await 
            })
        )
        .route("/v1/cockroach/more-handler", post(
            |body| async move { 
                cockroach_more_handler
                    .cockroach_detected(body).await 
            })
        );

    let app_url = format!("0.0.0.0:{}", settings.server.port);

    let listener = tokio::net::TcpListener::bind(app_url.clone()).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
