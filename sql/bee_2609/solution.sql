/*
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64)
);

INSERT INTO categories (name)
VALUES
    ('wood'),
    ('luxury'),
    ('vintage'),
    ('modern'),
    ('super luxury');


CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    amount INTEGER NOT NULL,
    price REAL NOT NULL,
    id_categories INTEGER REFERENCES categories(id) ON DELETE CASCADE
);

INSERT INTO products (name, amount, price, id_categories)
VALUES
    ('Two-doors wardrobe', 100, 800, 1),
    ('Dining table', 1000, 560, 3),
    ('Towel holder', 10000, 25.5, 4),
    ('Computer desk', 350, 320.5, 2),
    ('Chair', 3000, 210.64, 4),
    ('Single bed', 750, 460, 1);
*/

SELECT 
    categories.name AS name,
    SUM(products.amount) AS sum
FROM 
    products
JOIN 
    categories
ON 
    products.id_categories = categories.id
GROUP BY
    categories.name;
