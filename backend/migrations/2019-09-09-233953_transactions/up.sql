-- Your SQL goes here
CREATE TABLE transactions (
    transaction_id uuid NOT NULL,
    student_id uuid NOT NULL,
    regularization INTEGER NOT NULL,
    time timestamptz NOT NULL,
    PRIMARY KEY (transaction_id),
    FOREIGN KEY (student_id) REFERENCES accounts(student_id)
);
