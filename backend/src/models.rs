use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Account {
    pub student_id: Uuid,
    pub last_transaction: Option<Uuid>,
    pub name: String,
    pub time_of_negative: Option<chrono::NaiveDate>,
    pub transaction_in_negative: Option<i32>,
}

#[derive(Queryable)]
pub struct ProductModel {
    pub id: i32,
    pub category: String,
    pub name: String,
    pub active: bool,
    pub price: i16,
    pub active_monday: bool,
    pub active_tuesday: bool,
    pub active_wednesday: bool,
    pub active_thursday: bool,
    pub active_friday: bool,
    pub active_weekend: bool,
}
