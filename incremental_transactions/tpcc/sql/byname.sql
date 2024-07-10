-- Ignoring byname for now
-- if (byname)
-- {


-- CREATE VIEW ncustomers AS         -- SELECT count(c_id) INTO :namecnt
-- SELECT count(c_id) AS name_cnt    -- FROM customer
-- FROM customer AS c,               -- WHERE c_last=:c_last AND c_d_id=:c_d_id AND c_w_id=:c_w_id;
--      transaction_parameters as t
-- WHERE c.c_last = t.c_last
--   AND c.c_d_id = t.c_d_id
--   AND c.c_w_id = t.c_w_id;

CREATE VIEW cust_enum AS
SELECT c.c_first, c.c_middle, c.c_id,
    c.c_street_1, c.c_street_2, c.c_city, c.c_state, c.c_zip,
    c.c_phone, c.c_credit, c.c_credit_lim,
    c.c_discount, c.c_balance, c.c_since
FROM customer AS c,
     transaction_parameters AS t
WHERE c.c_last = t.c_last
  AND c.c_d_id = t.c_d_id
  AND c.c_w_id = t.c_w_id
ORDER BY c_first
LIMIT 1;

create view cust_max as
SELECT c.c_first, c.c_middle, c.c_id,
    c.c_street_1, c.c_street_2, c.c_city, c.c_state, c.c_zip,
    c.c_phone, c.c_credit, c.c_credit_lim,
    c.c_discount, c.c_balance, c.c_since
FROM customer AS c,
     transaction_parameters AS t
WHERE c.c_last = t.c_last
  AND c.c_d_id = t.c_d_id
  AND c.c_w_id = t.c_w_id
  AND c_first = (select max(c_first) from customer LIMIT 1)
LIMIT 1;

-- CREATE VIEW cust_max AS
-- SELECT c_first, c_last
-- FROM customer
-- ORDER BY c_first
-- LIMIT 1;

-- SELECT c_first, c_middle, c_id,
-- c_street_1, c_street_2, c_city, c_state, c_zip,
-- c_phone, c_credit, c_credit_lim,
-- c_discount, c_balance, c_since


--  DECLARE c_byname CURSOR FOR
--  SELECT c_first, c_middle, c_id,
--  c_street_1, c_street_2, c_city, c_state, c_zip,
--  c_phone, c_credit, c_credit_lim,
--  c_discount, c_balance, c_since
--  FROM customer
--  WHERE c_w_id=:c_w_id AND c_d_id=:c_d_id AND c_last=:c_last
--  ORDER BY c_first;

-- CREATE VIEW median_pos AS
-- SELECT c_first, c_middle, c_id,
--     c_street_1, c_street_2, c_city, c_state, c_zip,
--     c_phone, c_credit, c_credit_lim,
--     c_discount, c_balance, c_since
-- FROM cust_last
-- ORDER BY c_first
-- LIMIT 1 OFFSET (
--     SELECT FLOOR((COUNT(*) - 1) / 2)
--     FROM cust_last
-- );

--    OPEN c_byname;
--
--    if (namecnt%2) namecnt++; --Locate midpoint customer;
--    for (n=0; n<namecnt/2; n++)
--    {
--    EXEC SQL FETCH c_byname
--    INTO :c_first, :c_middle, :c_id,
--    :c_street_1, :c_street_2, :c_city, :c_state, :c_zip,
--    :c_phone, :c_credit, :c_credit_lim,
--    :c_discount, :c_balance, :c_since;
--    }
--    EXEC SQL CLOSE c_byname;
--}
/*
else
{
    EXEC SQL SELECT c_first, c_middle, c_last, c_street_1, c_street_2, c_city, c_state, c_zip,
    c_phone, c_credit, c_credit_lim,
    c_discount, c_balance, c_since
    INTO :c_first, :c_middle, :c_last,
    :c_street_1, :c_street_2, :c_city, :c_state, :c_zip,
    :c_phone, :c_credit, :c_credit_lim,
    :c_discount, :c_balance, :c_since
    FROM customer
    WHERE c_w_id=:c_w_id AND c_d_id=:c_d_id AND c_id=:c_id;
} */