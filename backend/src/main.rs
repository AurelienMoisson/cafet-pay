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
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use uuid::Uuid;

pub mod models;
pub mod schema;

#[database("accounts")]
struct AccountDb(diesel::PgConnection);

#[derive(Debug, Serialize)]
struct Transaction {
    pub time: DateTime<Utc>,
    pub regularization: i64,
    pub products: HashMap<Products, u64>,
    pub reductions: HashMap<u64, u64>,
}

#[derive(Hash, Debug, PartialEq, Eq)]
enum Products {
    Demi,
    UnTiers,
    DeuxTiers,
}

impl Products {
    fn id(&self) -> u64 {
        match self {
            Products::UnTiers => 1,
            Products::Demi => 2,
            Products::DeuxTiers => 3,
        }
    }
}

impl serde::Serialize for Products {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.id())
    }
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

#[derive(Debug, Serialize)]
enum Status {
    #[serde(rename = "ok")]
    Success,
    #[serde(rename = "error")]
    Failure,
}

#[derive(Debug, Serialize)]
struct Query<T: Serialize + std::fmt::Debug> {
    pub status: Status,
    pub response: Option<T>,
    pub reason: Option<String>,
}

impl<T: Serialize + std::fmt::Debug> Query<T> {
    pub fn fail(reason: String) -> Query<T> {
        Query {
            status: Status::Failure,
            reason: Some(reason),
            response: None,
        }
    }
    pub fn succeed(response: T) -> Query<T> {
        Query {
            status: Status::Success,
            reason: None,
            response: Some(response),
        }
    }
}

#[derive(Debug, Serialize)]
struct Balance {
    pub balance: i32,
}

#[get("/account/<id>/balance")]
fn get_balance(conn: AccountDb, id: String) -> Json<Query<Balance>> {
    use diesel::prelude::*;
    use schema::accounts::dsl::*;

    let id: Uuid = match Uuid::parse_str(&id) {
        Ok(i) => i,
        Err(e) => {
            info!("Invalid uuid: {:?}", e);
            return Json(Query::fail("invalid uuid".to_owned()));
        }
    };

    let students: Vec<models::Account> = match accounts.filter(student_id.eq(&id)).load(&*conn) {
        Ok(s) => s,
        Err(e) => {
            warn!("Database failure: {:?}", e);
            return Json(Query::fail("internal".to_owned()));
        }
    };
    if students.len() == 1 {
        Json(Query::succeed(Balance { balance: 0 }))
    } else if students.len() == 0 {
        Json(Query::fail("not found".to_owned()))
    } else {
        warn!("Id {} is linkde to more than one account", id);
        Json(Query::fail("internal".to_owned()))
    }
}

#[get("/account/<id>/negative")]
fn get_since_negative(id: String) -> JsonValue {
    // TODO
    json!({
      "transactionsSinceNegative": 0,
      "timeSinceNegative": 0
    })
}

fn main() {
    rocket::ignite()
        .attach(AccountDb::fairing())
        .mount(
            "/api",
            routes![get_balance, get_transactions, get_since_negative],
        )
        .launch();
}
