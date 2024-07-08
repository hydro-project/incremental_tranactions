-- define Person table
CREATE TABLE Person
(
    name    VARCHAR NOT NULL,
    age     INT,
    present BOOLEAN
);
CREATE VIEW Adult AS SELECT Person.name FROM Person WHERE Person.age > 18;
-- extension of example to include chaining of views
CREATE VIEW NumAdult AS SELECT COUNT(*) AS count FROM Adult;