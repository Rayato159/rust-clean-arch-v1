use crate::cockroach::messaging::CockroachMessaging;

pub struct CockroachFCMMessaging;

impl CockroachFCMMessaging {
    pub fn new() -> Self {
        CockroachFCMMessaging
    }
}

#[async_trait::async_trait]
impl CockroachMessaging for CockroachFCMMessaging {
    async fn push_notification(&self) {
    }
}