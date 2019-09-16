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

table! {
    transaction_details (id) {
        id -> Int4,
        transaction_id -> Uuid,
        product_id -> Int4,
        amount -> Int4,
    }
}

table! {
    transactions (transaction_id) {
        transaction_id -> Uuid,
        student_id -> Uuid,
        regularization -> Int4,
        time -> Timestamptz,
    }
}

joinable!(transaction_details -> products (product_id));
joinable!(transaction_details -> transactions (transaction_id));
joinable!(transactions -> accounts (student_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    products,
    transaction_details,
    transactions,
);
