-- Your SQL goes here
CREATE TABLE transactions (
    transaction_id uuid NOT NULL,
    student_id uuid NOT NULL,
    regularization INTEGER NOT NULL,
    PRIMARY KEY (transaction_id),
    FOREIGN KEY (student_id) REFERENCES accounts(student_id)
);
