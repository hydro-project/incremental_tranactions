CREATE VIEW cust_byname_inlined AS
SELECT c.c_first, c.c_middle, c.c_id,
    c.c_street_1, c.c_street_2, c.c_city, c.c_state, c.c_zip,
    c.c_phone, c.c_credit, c.c_credit_lim,
    c.c_discount, c.c_balance, c.c_since
FROM customer as c,
    (SELECT ARRAY_AGG((c_id + c_w_id + c_d_id) ORDER BY c_first) AS cust_array
FROM (SELECT c.c_id, c.c_w_id, c.c_d_id, c.c_first
      FROM customer AS c
      WHERE c.c_last = 'lastname'
        AND c.c_d_id = 43
        AND c.c_w_id = 44
      ORDER BY c_first)) as a
WHERE (c.c_id + c.c_w_id + c.c_d_id) = a.cust_array[(ARRAY_LENGTH(a.cust_array) / 2) + 1];