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
    pub products: HashMap<i32, i32>,
    pub reductions: HashMap<i32, i32>,
}

#[derive(Debug, Queryable)]
struct TransactionData {
    pub id: uuid::Uuid,
    pub time: DateTime<Utc>,
    pub regularization: i32,
}

#[get("/account/<id>/transactions")]
fn get_transactions(conn: CafetDb, id: String) -> JsonResult<Vec<Transaction>> {
    use schema::transactions::dsl::*;
    let id: Uuid = match Uuid::parse_str(&id) {
        Ok(i) => i,
        Err(e) => {
            info!("Invalid uuid: {:?}", e);
            return fail(ErrorKind::InvalidUUID);
        }
    };
    let transaction_ids: Vec<TransactionData> = match transactions
        .filter(student_id.eq(id))
        .select((transaction_id, time, regularization))
        .distinct()
        .load(&*conn)
    {
        Ok(d) => d,
        Err(e) => {
            warn!("Failure in getting transactions: {:?}", e);
            return fail(ErrorKind::Internal);
        }
    };
    let mut results = Vec::with_capacity(transaction_ids.len());
    for data in transaction_ids {
        use schema::transaction_details::dsl::*;
        let product_pairs: Vec<(i32, i32)> = match transaction_details
            .filter(transaction_id.eq(data.id))
            .select((product_id, amount))
            .distinct()
            .load(&*conn)
        {
            Ok(products) => products,
            Err(e) => {
                warn!("Error retrieving transaction details: {:?}", e);
                return fail(ErrorKind::Internal);
            }
        };
        let reductions = HashMap::new();
        let mut products = HashMap::new();
        for (prod_id, prod_amount) in product_pairs {
            products.insert(prod_id, prod_amount);
        }
        results.push(Transaction {
            time: data.time,
            regularization: data.regularization,
            products,
            reductions,
        })
    }
    succeed(results)
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

#[derive(Debug, Deserialize)]
pub struct ProductAddition {
    name: String,
    category: String,
    price: i16,
    days_active: Vec<chrono::Weekday>,
}

#[derive(Debug, Serialize, Queryable)]
pub struct NewProduct {
    name: String,
    id: i32,
}

#[put("/products", data = "<new_product>")]
fn put_product(conn: CafetDb, new_product: Json<ProductAddition>) -> JsonResult<NewProduct> {
    use schema::products::dsl::*;
    let new_product = new_product.into_inner();
    let mon = new_product.days_active.contains(&chrono::Weekday::Mon);
    let tue = new_product.days_active.contains(&chrono::Weekday::Tue);
    let wed = new_product.days_active.contains(&chrono::Weekday::Wed);
    let thu = new_product.days_active.contains(&chrono::Weekday::Thu);
    let fri = new_product.days_active.contains(&chrono::Weekday::Fri);
    let we = new_product.days_active.contains(&chrono::Weekday::Sat)
        || new_product.days_active.contains(&chrono::Weekday::Sun);
    let addition = diesel::insert_into(products)
        .values(&(
            name.eq(new_product.name),
            category.eq(new_product.category),
            price.eq(new_product.price),
            active_monday.eq(mon),
            active_tuesday.eq(tue),
            active_wednesday.eq(wed),
            active_thursday.eq(thu),
            active_friday.eq(fri),
            active_weekend.eq(we),
        ))
        .returning((name, id))
        .get_results(&*conn);
    match addition {
        Err(err) => {
            warn!("Failed to insert new product: {:?}", err);
            fail(ErrorKind::Internal)
        }
        Ok(mut new) => {
            if new.len() == 1 {
                succeed(new.pop().unwrap())
            } else {
                warn!("Insert product returned multiple or none {:?}", new);
                fail(ErrorKind::Internal)
            }
        }
    }
}

fn main() {
    rocket::ignite()
        .attach(CafetDb::fairing())
        .mount(
            "/api",
            routes![
                get_balance,
                get_transactions,
                get_since_negative,
                get_products,
                put_product,
            ],
        )
        .launch();
}
