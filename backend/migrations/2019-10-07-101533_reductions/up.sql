-- Your SQL goes here
CREATE TABLE reductions (
    id SERIAL NOT NULL,
    name VARCHAR NOT NULL,
    amount INTEGER NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE reductions_content (
    idx SERIAL PRIMARY KEY,
    reduction_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL,
    FOREIGN KEY (reduction_id) REFERENCES reductions(id),
    FOREIGN KEY (product_id) REFERENCES products(id)
);
