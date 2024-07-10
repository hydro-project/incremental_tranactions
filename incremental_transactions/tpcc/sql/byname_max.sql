CREATE VIEW cust_max_order_by AS
SELECT c.c_first, c.c_middle, c.c_id,
    c.c_street_1, c.c_street_2, c.c_city, c.c_state, c.c_zip,
    c.c_phone, c.c_credit, c.c_credit_lim,
    c.c_discount, c.c_balance, c.c_since
FROM customer AS c,
     transaction_parameters AS t
WHERE c.c_last = t.c_last
  AND c.c_d_id = t.c_d_id
  AND c.c_w_id = t.c_w_id
ORDER BY c_first DESC
LIMIT 1;