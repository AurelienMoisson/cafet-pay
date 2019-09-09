table! {
    accounts (student_id) {
        student_id -> Uuid,
        last_transaction -> Uuid,
        name -> Varchar,
        balance -> Int4,
        time_of_negative -> Date,
        transaction_in_negative -> Int4,
    }
}

table! {
    products (id) {
        id -> Int4,
        category -> Varchar,
        name -> Varchar,
        active -> Bool,
        price -> Int2,
        active_monday -> Bool,
        active_tuesday -> Bool,
        active_wednesday -> Bool,
        active_thursday -> Bool,
        active_friday -> Bool,
        active_weekend -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    products,
);
