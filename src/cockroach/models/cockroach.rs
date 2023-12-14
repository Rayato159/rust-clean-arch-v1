use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertCockroachData {
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CockroachNotification {
    pub amount: i32,
    pub issue_at: NaiveDateTime,
}

pub struct SelectCockroachData {
    pub id: i32,
    pub amount: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}