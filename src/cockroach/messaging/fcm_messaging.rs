use async_trait::async_trait;

use tracing::log::info;

use crate::cockroach::{
    messaging::CockroachMessaging,
    models::cockroach_model::CockroachNotification
};

pub struct CockroachFCMMessaging;

impl CockroachFCMMessaging {
    pub fn new() -> impl CockroachMessaging {
        CockroachFCMMessaging
    }
}

#[async_trait]
impl CockroachMessaging for CockroachFCMMessaging {
    async fn push_notification(&self, cockroach_notification_data: CockroachNotification) {
        info!("Pushing notification to FCM: {:?}", cockroach_notification_data);
    }
}