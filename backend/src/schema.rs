table! {
    accounts (student_id) {
        student_id -> Uuid,
        last_transaction -> Uuid,
        name -> Varchar,
        balance -> Int4,
    }
}
