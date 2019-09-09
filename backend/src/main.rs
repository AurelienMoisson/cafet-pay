#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket;
use chrono::{DateTime, Utc};
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[get("/account/transactions")]
fn get_transactions() -> Json<Vec<Transaction>> {
    Json(vec![Transaction {
        time: Utc::now(),
        regularization: 0,
        products: HashMap::new(),
        reductions: HashMap::new(),
    }])
}

#[get("/account/balance")]
fn get_balance() -> JsonValue {
    json!({"balance": 0})
}

fn main() {
    rocket::ignite()
        .mount("/api", routes![get_balance, get_transactions])
        .launch();
}
