-- Your SQL goes here
CREATE TABLE transaction_details (
    id SERIAL PRIMARY KEY,
    transaction_id uuid NOT NULL,
    product_id INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    FOREIGN KEY (transaction_id) REFERENCES transactions(transaction_id),
    FOREIGN KEY (product_id) REFERENCES products(id)
);
