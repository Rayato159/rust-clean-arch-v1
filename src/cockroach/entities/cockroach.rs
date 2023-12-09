use chrono::NaiveDateTime;

pub struct Cockroach {
    pub id: Option<i32>,
    pub amount: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}