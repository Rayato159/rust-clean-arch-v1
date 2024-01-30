use async_trait::async_trait;

use tracing::log::info;

use super::messaging::CockroachMessaging;

use crate::cockroach::models::cockroach::CockroachNotification;

#[derive(Clone)]
pub struct CockroachFCMMessaging;

impl CockroachFCMMessaging {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl CockroachMessaging for CockroachFCMMessaging {
    async fn push_notification(&self, cockroach_notification_data: CockroachNotification) {
        info!("Pushing notification to FCM: {:?}", cockroach_notification_data);
    }
}