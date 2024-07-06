/*
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    amount INTEGER NOT NULL,
    price REAL NOT NULL
);

INSERT INTO 
    products
VALUES
    ('Two-doors wardrobe', 100, 800),
    ('Dining table', 1000, 560),
    ('Towel holder', 10000, 25.5),
    ('Computer desk', 350, 320.5),
    ('Chair', 3000, 210.64),
    ('Single bed', 750, 460);
*/

SELECT MAX(price) AS price, MIN(price) AS price
FROM products;