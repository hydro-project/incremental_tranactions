-- gettimestamp(datetime);

/* UPDATE warehouse SET w_ytd = w_ytd + :h_amount
WHERE w_id=:w_id; */
CREATE VIEW warehouse_updates AS SELECT t.txn_id, w.w_id, (w.w_ytd + t.h_amount) AS w_ytd FROM warehouse AS w, transaction_parameters AS t WHERE w.w_id=t.w_id;

/* SELECT w_street_1, w_street_2, w_city, w_state, w_zip, w_name
INTO :w_street_1, :w_street_2, :w_city, :w_state, :w_zip, :w_name
FROM warehouse
WHERE w_id=:w_id; */
CREATE VIEW warehouse_select AS SELECT w.w_street_1, w.w_street_2, w.w_city, w.w_state, w.w_zip, w.w_name, t.txn_id FROM warehouse_static AS w, transaction_parameters AS t WHERE w.w_id=t.w_id;

/* UPDATE district SET d_ytd = d_ytd + :h_amount
WHERE d_w_id=:w_id AND d_id=:d_id; */
CREATE VIEW district_updates AS SELECT d.d_w_id, d.d_id, (d.d_ytd + t.h_amount) AS d_ytd FROM district_ytd AS d, transaction_parameters AS t WHERE d.d_w_id=t.w_id AND d.d_id=t.d_id;

/* SELECT d_street_1, d_street_2, d_city, d_state, d_zip, d_name
INTO :d_street_1, :d_street_2, :d_city, :d_state, :d_zip, :d_name
FROM district
WHERE d_w_id=:w_id AND d_id=:d_id; */
CREATE VIEW district_select AS SELECT d.d_street_1, d.d_street_2, d.d_city, d.d_state, d.d_zip, d.d_name, t.txn_id FROM district_static AS d, transaction_parameters AS t WHERE d.d_w_id=t.w_id AND d.d_id=t.d_id;

-- Ignoring byname for now
/* if (byname)
{
    SELECT count(c_id) INTO :namecnt
    FROM customer
    WHERE c_last=:c_last AND c_d_id=:c_d_id AND c_w_id=:c_w_id;

    DECLARE c_byname CURSOR FOR
    SELECT c_first, c_middle, c_id,
    c_street_1, c_street_2, c_city, c_state, c_zip,
    c_phone, c_credit, c_credit_lim,
    c_discount, c_balance, c_since
    FROM customer
    WHERE c_w_id=:c_w_id AND c_d_id=:c_d_id AND c_last=:c_last
    ORDER BY c_first;

    OPEN c_byname;

    if (namecnt%2) namecnt++; --Locate midpoint customer;
    for (n=0; n<namecnt/2; n++)
    {
    EXEC SQL FETCH c_byname
    INTO :c_first, :c_middle, :c_id,
    :c_street_1, :c_street_2, :c_city, :c_state, :c_zip,
    :c_phone, :c_credit, :c_credit_lim,
    :c_discount, :c_balance, :c_since;
    }
    EXEC SQL CLOSE c_byname;
}
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
-- Using the simpler by-id else branch for now
CREATE VIEW customer_select AS SELECT c.c_first, c.c_middle, c.c_last, c.c_street_1, c.c_street_2, c.c_city, c.c_state, c.c_zip, c.c_phone, c.c_credit, c.c_credit_lim, c.c_discount, c.c_balance, c.c_since, t.txn_id FROM customer AS c, transaction_parameters AS t WHERE c.c_w_id=t.c_w_id AND c.c_d_id=t.c_d_id AND c.c_id=t.c_id;

-- Ignoring the BC credit branch for now, using the simpler else branch
/* c_balance += h_amount;
c_credit[2]='\0';
if (strstr(c_credit, "BC") )
{
    EXEC SQL SELECT c_data INTO :c_data
    FROM customer
    WHERE c_w_id=:c_w_id AND c_d_id=:c_d_id AND c_id=:c_id;
    sprintf(c_new_data,"| %4d %2d %4d %2d %4d $%7.2f %12c %24c",
    c_id,c_d_id,c_w_id,d_id,w_id,h_amount
    h_date, h_data);
    strncat(c_new_data,c_data,500-strlen(c_new_data));
    EXEC SQL UPDATE customer
    SET c_balance = :c_balance, c_data = :c_new_data
    WHERE c_w_id = :c_w_id AND c_d_id = :c_d_id AND
    c_id = :c_id;
}
else
{
    EXEC SQL UPDATE customer SET c_balance = :c_balance
    WHERE c_w_id = :c_w_id AND c_d_id = :c_d_id AND
    c_id = :c_id;
} */
-- TODO: Create a new customer record with the updated balance
CREATE VIEW customer_update AS SELECT c.c_first, c.c_middle, c.c_last, c.c_street_1, c.c_street_2, c.c_city, c.c_state, c.c_zip, c.c_phone, c.c_credit, c.c_credit_lim, c.c_discount, (c.c_balance + t.h_amount) as c_balance, c.c_since FROM customer_select AS c, transaction_parameters as t WHERE c.txn_id=t.txn_id;

/* strncpy(h_data,w_name,10);
h_data[10]='\0';
strncat(h_data,d_name,10);
h_data[20]=' ';
h_data[21]=' ';
h_data[22]=' ';
h_data[23]=' ';
EXEC SQL INSERT INTO history (h_c_d_id, h_c_w_id, h_c_id, h_d_id,
h_w_id, h_date, h_amount, h_data)
VALUES (:c_d_id, :c_w_id, :c_id, :d_id,
:w_id, :datetime, :h_amount, :h_data); */
CREATE VIEW history_insert AS SELECT t.c_d_id AS h_c_d_id, t.c_w_id AS h_c_w_id, t.c_id AS h_c_id, t.d_id AS h_d_id, t.w_id AS h_w_id, t.datetime_ AS h_date, t.h_amount AS h_amount, CONCAT(w.w_name, d.d_name) AS h_data FROM transaction_parameters AS t, warehouse_select AS w, district_select AS d WHERE t.txn_id=w.txn_id AND t.txn_id=d.txn_id;