table! {
    accounts (student_id) {
        student_id -> Uuid,
        last_transaction -> Nullable<Uuid>,
        name -> Varchar,
        time_of_negative -> Nullable<Date>,
        transaction_in_negative -> Nullable<Int4>,
    }
}

table! {
    cards (card_id) {
        student_id -> Uuid,
        card_id -> Int4,
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
    reductions (id) {
        id -> Int4,
        name -> Varchar,
        amount -> Int4,
    }
}

table! {
    reductions_content (idx) {
        idx -> Int4,
        reduction_id -> Int4,
        product_id -> Int4,
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

joinable!(cards -> accounts (student_id));
joinable!(reductions_content -> products (product_id));
joinable!(reductions_content -> reductions (reduction_id));
joinable!(transaction_details -> products (product_id));
joinable!(transaction_details -> transactions (transaction_id));
joinable!(transactions -> accounts (student_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    cards,
    products,
    reductions,
    reductions_content,
    transaction_details,
    transactions,
);
