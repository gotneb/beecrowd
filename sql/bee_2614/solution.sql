/*
CREATE TABLE customers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    street VARCHAR(64) NOT NULL,
    city VARCHAR(64) NOT NULL
);

CREATE TABLE rentals (
    id SERIAL,
    rentals_date DATE NOT NULL,
    id_customers INTEGER REFERENCES customers(id) ON DELETE CASCADE
);

INSERT INTO 
    customers (name, street, city)
VALUES
    ('Giovanna Goncalves Oliveira', 'Rua Mato Grosso', 'Canoas'),
    ('Kauã Azevedo Ribeiro', 'Travessa Ibiá', 'Uberlândia'),
    ('Rebeca Barbosa Santos', 'Rua Observatório Meteorológico', 'Salvador'),
    ('Sarah Carvalho Correia', 'Rua Antônio Carlos da Silva', 'Apucarana'),
    ('João Almeida Lima', 'Rua Rio Taiuva', 'Ponta Grossa'),
    ('Diogo Melo Dias', 'Rua Duzentos e Cinqüenta', 'Várzea Grande');

INSERT INTO 
    rentals (rentals_date, id_customers)
VALUES
    ('2016-09-10', 3),
    ('2016-09-10', 1),
    ('2016-02-08', 4),
    ('2016-02-09', 2),
    ('2016-02-03', 6),
    ('2016-04-04', 4);
*/

SELECT 
    customers.name, rentals.rentals_date
FROM 
    customers
JOIN 
    rentals
ON 
    customers.id = rentals.id_customers
WHERE 
    rentals.rentals_date > '2016-09-01' 
    AND 
    rentals.rentals_date < '2016-10-01';