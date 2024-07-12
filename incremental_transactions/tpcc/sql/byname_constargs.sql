CREATE VIEW cust_agg AS
SELECT c_w_id, c_d_id, ARRAY_AGG(c_id ORDER BY c_first) AS cust_array
FROM (SELECT c.c_id, c.c_w_id, c.c_d_id, c.c_first
      FROM customer AS c
      WHERE c.c_last = 'lastname'
        AND c.c_d_id = 43
        AND c.c_w_id = 44)
GROUP BY c_w_id, c_d_id;

CREATE VIEW cust_byname AS
SELECT c.c_first, c.c_middle, c.c_last, c.c_id, c.c_w_id, c.c_d_id,
    c.c_street_1, c.c_street_2, c.c_city, c.c_state, c.c_zip,
    c.c_phone, c.c_credit, c.c_credit_lim,
    c.c_discount, c.c_balance, c.c_since
FROM customer as c,
     cust_agg as a
WHERE c.c_d_id = 43
  AND c.c_w_id = 44
  AND c.c_id = a.cust_array[(ARRAY_LENGTH(a.cust_array) / 2) + 1];