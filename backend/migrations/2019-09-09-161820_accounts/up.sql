CREATE TABLE accounts (
    student_id uuid NOT NULL,
    last_transaction uuid NOT NULL,
    name VARCHAR NOT NULL,
    balance INTEGER NOT NULL,
    time_of_negative DATE NOT NULL,
    transaction_in_negative INTEGER NOT NULL,
    PRIMARY KEY (student_id)
);
