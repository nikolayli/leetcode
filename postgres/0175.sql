SELECT 
    p.firstName AS "firstName",
    p.lastName AS "lastName",
    a.city AS "city",
    a.state AS "state"
FROM 
    Person p
LEFT JOIN 
    Address a ON p.personId = a.personId;
