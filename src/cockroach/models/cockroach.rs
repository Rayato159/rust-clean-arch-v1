use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertCockroachData {
    pub amount: i32,
}

#[derive(Default, Debug)]
pub struct CockroachNotification {
    pub amount: i32,
    pub notified_at: NaiveDateTime,
}