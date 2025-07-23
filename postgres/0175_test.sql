CREATE TABLE IF NOT EXISTS test_person (
    personId INT PRIMARY KEY,
    lastName VARCHAR,
    firstName VARCHAR
);

CREATE TABLE IF NOT EXISTS test_address (
    addressId INT PRIMARY KEY,
    personId INT,
    city VARCHAR,
    state VARCHAR
);

TRUNCATE TABLE test_person, test_address CASCADE;

INSERT INTO test_person (personId, lastName, firstName) VALUES
(1, 'Wang', 'Allen'),
(2, 'Alice', 'Bob');

INSERT INTO test_address (addressId, personId, city, state) VALUES
(1, 2, 'New York City', 'New York'),
(2, 3, 'Leetcode', 'California');

DO $$
DECLARE
    rec record;
    counter int := 0;
BEGIN
    FOR rec IN
        SELECT p.firstName, p.lastName, a.city, a.state
        FROM test_person p
        LEFT JOIN test_address a ON p.personId = a.personId
        ORDER BY p.personId
    LOOP
        counter := counter + 1;
        CASE counter
            WHEN 1 THEN
                ASSERT rec.firstName = 'Allen', 'Wrong first name 1';
                ASSERT rec.lastName = 'Wang', 'Wrong last name 1';
                ASSERT rec.city IS NULL, 'Expected NULL city';
                ASSERT rec.state IS NULL, 'Expected NULL state';
            WHEN 2 THEN
                ASSERT rec.firstName = 'Bob', 'Wrong first name 2';
                ASSERT rec.lastName = 'Alice', 'Wrong last name 2';
                ASSERT rec.city = 'New York City', 'Wrong city';
                ASSERT rec.state = 'New York', 'Wrong state';
        END CASE;
    END LOOP;
    
    ASSERT counter = 2, 'Wrong number of rows';
    
    RAISE NOTICE 'Test passed successfully!';
END $$;
