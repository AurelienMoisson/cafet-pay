-- Your SQL goes here
CREATE TABLE cards (
    student_id uuid NOT NULL,
    card_id INTEGER NOT NULL,
    PRIMARY KEY (card_id),
    FOREIGN KEY (student_id) REFERENCES accounts(student_id)
);
