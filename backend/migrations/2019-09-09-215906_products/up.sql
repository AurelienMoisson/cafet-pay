-- Your SQL goes here
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    category VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 't',
    price SMALLINT NOT NULL,
    active_monday BOOLEAN NOT NULL DEFAULT 't',
    active_tuesday BOOLEAN NOT NULL DEFAULT 't',
    active_wednesday BOOLEAN NOT NULL DEFAULT 't',
    active_thursday BOOLEAN NOT NULL DEFAULT 't',
    active_friday BOOLEAN NOT NULL DEFAULT 't',
    active_weekend BOOLEAN NOT NULL DEFAULT 'f'
);

INSERT INTO products (category, name, price, active_thursday)
    VALUES ('sandwich', '1/3', 150, 'f');
INSERT INTO products (category, name, price, active_thursday)
    VALUES ('sandwich', '2/3', 200, 'f');
INSERT INTO products (category, name, price, active_thursday)
    VALUES ('sandwich', '3/3', 250, 'f');

INSERT INTO products (category, name, price)
    VALUES ('boisson', 'Oasis orange', 60);
INSERT INTO products (category, name, price) 
    VALUES ('boisson', 'Oasis tropical', 60);
INSERT INTO products (category, name, price)
    VALUES ('boisson', 'Coca', 60);
INSERT INTO products (category, name, price)
    VALUES ('boisson', 'Coca zero', 60);
INSERT INTO products (category, name, price)
    VALUES ('boisson', 'Coca cherry', 60);
INSERT INTO products (category, name, price)
    VALUES ('boisson', 'Schweppes', 60);
INSERT INTO products (category, name, price)
    VALUES ('boisson', 'Orangina', 60);

INSERT INTO products (category, name, price, active_monday, active_tuesday, active_wednesday, active_friday)
    VALUES ('pizza', 'Orangina', 60, 'f','f','f','f');
