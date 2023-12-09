pub mod fcm_messaging;

#[async_trait::async_trait]
pub trait CockroachMessaging {
    async fn push_notification(&self);
}