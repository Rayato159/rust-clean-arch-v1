use chrono::NaiveDateTime;

pub struct SelectCockroachData {
    pub id: i32,
    pub amount: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}