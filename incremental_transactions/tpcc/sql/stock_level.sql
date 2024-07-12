/*
EXEC SQL SELECT d_next_o_id INTO :o_id
FROM district
WHERE d_w_id=:w_id AND d_id=:d_id;
EXEC SQL SELECT COUNT(DISTINCT (s_i_id)) INTO :stock_count
FROM order_line, stock
WHERE ol_w_id=:w_id AND
ol_d_id=:d_id AND ol_o_id<:o_id AND
ol_o_id>=:o_id-20 AND s_w_id=:w_id AND
s_i_id=ol_i_id AND s_quantity < :threshold;
*/


SELECT d_next_o_id AS o_id_max, d_next_o_id-20 AS o_id_min
FROM district
WHERE d_w_id=:w_id AND d_id=:d_id;

CREATE VIEW stock_level_district AS
SELECT COUNT(DISTINCT (s_i_id)) AS stock_count
FROM order_line, stock
WHERE ol_w_id=44 AND -- set warehouse here
ol_d_id=43 AND ol_o_id<o_id_max AND -- set district here
ol_o_id>=o_id_min AND s_w_id=44 AND -- set warehouse here
s_i_id=ol_i_id AND s_quantity < 100; -- set threshold here