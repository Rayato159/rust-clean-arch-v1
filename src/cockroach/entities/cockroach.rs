use chrono::NaiveDateTime;

#[derive(Debug, Default, Clone)]
pub struct Cockroach {
    pub id: Option<i32>,
    pub amount: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}