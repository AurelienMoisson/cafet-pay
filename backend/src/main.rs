#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use uuid::Uuid;

pub mod models;
pub mod schema;

#[database("cafet_db")]
struct CafetDb(diesel::PgConnection);

#[derive(Debug, Serialize)]
struct Error {
    status: &'static str,
    reason: ErrorKind,
}

fn fail<T: Serialize>(reason: ErrorKind) -> JsonResult<T> {
    Err(Json(Error {
        status: "error",
        reason,
    }))
}

#[derive(Debug, Serialize)]
struct JsonResponse<T: Serialize> {
    status: &'static str,
    response: T,
}

fn succeed<T: Serialize>(response: T) -> JsonResult<T> {
    Ok(Json(JsonResponse {
        status: "ok",
        response,
    }))
}

type JsonResult<T> = Result<Json<JsonResponse<T>>, Json<Error>>;

#[derive(Debug, Serialize)]
enum ErrorKind {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "notFound")]
    NotFound(String),
    #[serde(rename = "invalidUuid")]
    InvalidUUID,
}

fn get_student(conn: CafetDb, id: Uuid) -> Result<models::Account, ErrorKind> {
    use schema::accounts::dsl::*;

    let mut students: Vec<models::Account> = match accounts.filter(student_id.eq(&id)).load(&*conn)
    {
        Ok(s) => s,
        Err(e) => {
            warn!("Database failure: {:?}", e);
            return Err(ErrorKind::Internal);
        }
    };
    if students.len() == 1 {
        Ok(students.pop().unwrap())
    } else if students.len() == 0 {
        Err(ErrorKind::NotFound(format!("{}", id)))
    } else {
        warn!("Id {} is linked to more than one account", id);
        Err(ErrorKind::Internal)
    }
}

#[derive(Debug, Serialize)]
struct Balance {
    pub balance: i32,
}

#[get("/account/<id>/balance")]
fn get_balance(conn: CafetDb, id: String) -> JsonResult<Balance> {
    let id: Uuid = match Uuid::parse_str(&id) {
        Ok(i) => i,
        Err(e) => {
            info!("Invalid uuid: {:?}", e);
            return fail(ErrorKind::InvalidUUID);
        }
    };
    match get_student(conn, id) {
        Ok(s) => succeed(Balance { balance: s.balance }),
        Err(e) => fail(e),
    }
}

#[derive(Serialize, Debug)]
struct SinceNegative {
    time: chrono::NaiveDate,
    amount_of_transactions: i32,
}

#[get("/account/<id>/negative")]
fn get_since_negative(conn: CafetDb, id: String) -> JsonResult<SinceNegative> {
    let id: Uuid = match Uuid::parse_str(&id) {
        Ok(i) => i,
        Err(e) => {
            info!("Invalid uuid: {:?}", e);
            return fail(ErrorKind::InvalidUUID);
        }
    };
    match get_student(conn, id) {
        Err(e) => fail(e),
        Ok(st) => succeed(SinceNegative {
            time: st.time_of_negative,
            amount_of_transactions: st.transaction_in_negative,
        }),
    }
}

#[derive(Debug, Serialize)]
struct Transaction {
    pub time: DateTime<Utc>,
    pub regularization: i32,
    pub products: HashMap<i32, u64>,
    pub reductions: HashMap<i32, u64>,
}

#[get("/account/<id>/transactions")]
fn get_transactions(id: String) -> Json<Vec<Transaction>> {
    //TODO
    Json(vec![Transaction {
        time: Utc::now(),
        regularization: 0,
        products: HashMap::new(),
        reductions: HashMap::new(),
    }])
}

#[derive(Serialize, Debug, Queryable)]
pub struct Product {
    name: String,
    category: String,
    price: i16,
}

#[get("/products")]
fn get_products(conn: CafetDb) -> JsonResult<Vec<Product>> {
    use schema::products::dsl::*;
    let p: Vec<Product> = match products
        .select((name, category, price))
        .distinct()
        .load::<Product>(&*conn)
    {
        Ok(p) => p,
        Err(e) => {
            warn!("Database failure: {:?}", e);
            return fail(ErrorKind::Internal);
        }
    };
    succeed(p)
}

#[derive(Deserialize)]
struct ProductAddition {
    pub name: String,
    pub price: u64,
}
#[derive(Serialize)]
struct ProductResponse {
    pub id: u32,
}

#[put("/products")]
fn add_new_product(conn: CafetDb, new_product: ProductAddition) {}

fn main() {
    rocket::ignite()
        .attach(CafetDb::fairing())
        .mount(
            "/api",
            routes![
                get_balance,
                get_transactions,
                get_since_negative,
                get_products
            ],
        )
        .launch();
}
