use async_trait::async_trait;

pub mod fcm_messaging;

use super::models::cockroach_model::CockroachNotification;

#[async_trait]
pub trait CockroachMessaging {
    async fn push_notification(&self, cockroach_notification_data: CockroachNotification);
}