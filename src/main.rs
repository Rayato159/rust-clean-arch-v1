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

    let cockroach_usecase = CockroachUsecaseImpl::new(
        CockroachPostgresRepository::new(&db),
        CockroachFCMMessaging::new(),
    );

    let cockroach_handler = CockroachAxumHandler::new(
        cockroach_usecase
    );

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/v1/cockroach", post(
            |body| async move { 
                cockroach_handler.cockroach_detected(body).await 
            })
        );

    let app_url = format!("0.0.0.0:{}", settings.server.port);

    let listener = tokio::net::TcpListener::bind(app_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
