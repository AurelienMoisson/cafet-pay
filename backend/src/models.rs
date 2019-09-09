use diesel::Queryable;
//use rocket_contrib::uuid::Uuid;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Account {
    pub student_id: Uuid,
    pub last_transaction: Uuid,
    pub name: String,
    pub balance: i32,
}
