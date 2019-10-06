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

impl<T: Serialize> JsonResponse<T> {
    pub fn into_inner(self) -> T {
        self.response
    }
}

fn succeed<T: Serialize>(response: T) -> JsonResult<T> {
    Ok(Json(JsonResponse {
        status: "ok",
        response,
    }))
}

type JsonResult<T> = Result<Json<JsonResponse<T>>, Json<Error>>;

fn inner_most<T: Serialize>(e: Json<JsonResponse<T>>) -> T {
    e.into_inner().into_inner()
}

#[derive(Debug, Serialize)]
enum ErrorKind {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "notFound")]
    NotFound(String),
    #[serde(rename = "invalidUuid")]
    InvalidUUID,
    #[serde(rename = "unknownProduct")]
    UnknownProduct(i32),
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

#[get("/account/<student_id>/balance")]
fn get_balance(conn: CafetDb, student_id: String) -> JsonResult<Balance> {
    use schema::products::dsl::*;
    let p_prices: HashMap<i32, i16> = match products
        .select((id, price))
        .distinct()
        .load::<(i32, i16)>(&*conn)
    {
        Ok(p) => p.into_iter().collect(),
        Err(e) => {
            warn!("Database failure: {:?}", e);
            return fail(ErrorKind::Internal);
        }
    };
    let transactions = inner_most(get_transactions(conn, student_id)?);
    let mut balance = 0;
    for tx in transactions {
        balance += tx.regularization;
        for (p_id, p_count) in tx.products {
            let p_price = match p_prices.get(&p_id) {
                Some(p) => *p as i32,
                None => return fail(ErrorKind::UnknownProduct(p_id)),
            };
            balance -= p_count * p_price;
        }
    }
    succeed(Balance { balance })
}

#[derive(Serialize, Debug)]
struct SinceNegative {
    time: Option<chrono::NaiveDate>,
    amount_of_transactions: Option<i32>,
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
    id: i32,
}

#[get("/products")]
fn get_products(conn: CafetDb) -> JsonResult<Vec<Product>> {
    use schema::products::dsl::*;
    let p: Vec<Product> = match products
        .select((name, category, price, id))
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

#[post("/products", data = "<new_product>")]
fn post_product(conn: CafetDb, new_product: Json<ProductAddition>) -> JsonResult<NewProduct> {
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

#[derive(Deserialize, Debug)]
struct AccountDetails {
    name: String,
}

#[derive(Serialize, Debug, Queryable)]
struct NewAccount {
    id: Uuid,
    name: String,
}

#[post("/account", data = "<details>")]
fn new_account(conn: CafetDb, details: Json<AccountDetails>) -> JsonResult<NewAccount> {
    use schema::accounts::dsl::*;
    let new_account: Result<Vec<NewAccount>, _> = diesel::insert_into(accounts)
        .values(&(
            name.eq(details.into_inner().name),
            student_id.eq(uuid::Uuid::new_v4()),
        ))
        .returning((student_id, name))
        .get_results(&*conn);
    match new_account {
        Err(e) => {
            warn!("Error in creating account: {:?}", e);
            fail(ErrorKind::Internal)
        }
        Ok(mut new_account) => {
            if new_account.len() != 1 {
                warn!("New account returned {:?}", new_account);
                fail(ErrorKind::Internal)
            } else {
                succeed(new_account.pop().unwrap())
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
                new_account,
                post_product,
            ],
        )
        .launch();
}
