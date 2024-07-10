-- define Person table
CREATE TABLE Person
(
    name    VARCHAR NOT NULL,
    age     INT,
    present BOOLEAN
);
CREATE VIEW Adult AS SELECT Person.name, Person.present FROM Person WHERE Person.age > 18;
-- extension of example to include chaining of views
CREATE VIEW NumAdult AS SELECT present, COUNT(*) AS count FROM Adult GROUP BY present;
CREATE VIEW Derived AS SELECT 1 as test, count FROM NumAdult WHERE present = 1;