use async_trait::async_trait;

use crate::cockroach::models::cockroach::CockroachNotification;

#[async_trait]
pub trait CockroachMessaging {
    async fn push_notification(&self, cockroach_notification_data: CockroachNotification);
}