CREATE TABLE accounts (
    student_id uuid NOT NULL,
    last_transaction uuid,
    name VARCHAR NOT NULL,
    time_of_negative DATE,
    transaction_in_negative INTEGER,
    PRIMARY KEY (student_id)
);
