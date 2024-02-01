use std::sync::Arc;

use async_trait::async_trait;

use super::usecases::CockroachUsecase;

use crate::cockroach::{
    repositories::repositories::CockroachRepository,
    messaging::messaging::CockroachMessaging,

    entities::cockroach::Cockroach,
    models::cockroach::{
        CockroachNotification,
        InsertCockroachData
    },
};

#[derive(Clone)]
pub struct CockroachUsecaseImpl<T, U> {
    repository: Arc<T>,
    messaging: Arc<U>,
}

impl<T, U> CockroachUsecaseImpl<T, U> 
where
    T: CockroachRepository + Clone + Send + Sync + 'static,
    U: CockroachMessaging + Clone + Send + Sync + 'static,
{
    pub fn new(
        repository: Arc<T>,
        messaging: Arc<U>,
    ) -> CockroachUsecaseImpl<T, U> {
        Self { 
            repository,
            messaging,
        }
    }
}

#[async_trait]
impl<T, U> CockroachUsecase for CockroachUsecaseImpl<T, U> 
where
    T: CockroachRepository + Clone + Send + Sync + 'static,
    U: CockroachMessaging + Clone + Send + Sync + 'static,
{
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