use async_trait::async_trait;

use crate::cockroach::{
    usecases::CockroachUsecase,
    repositories::CockroachRepository,
    messaging::CockroachMessaging,

    entities::cockroach_entities::Cockroach,
    models::cockroach_model::{
        CockroachNotification,
        InsertCockroachData
    },
};

pub struct CockroachUsecaseImpl {
    repository: Box<dyn CockroachRepository + Sync>,
    messaging: Box<dyn CockroachMessaging + Sync>,
}

impl CockroachUsecaseImpl {
    pub fn new(
        repository: Box<dyn CockroachRepository + Sync>,
        messaging: Box<dyn CockroachMessaging + Sync>,
    ) -> impl CockroachUsecase {
        CockroachUsecaseImpl { 
            repository,
            messaging,
        }
    }
}

#[async_trait]
impl CockroachUsecase for CockroachUsecaseImpl {
    async fn cockroach_detected(&self, cockroach_to_insert: InsertCockroachData) {
        let cockroach_data = Cockroach {
            amount: cockroach_to_insert.amount,
            ..Default::default()
        };

        self.repository.insert_cockroach_data(cockroach_data).await;
        
        let cockroach_notification = CockroachNotification {
            amount: cockroach_to_insert.amount,
            issue_at: chrono::Utc::now().naive_utc(),
        };

        self.messaging.push_notification(cockroach_notification).await;
    }
}