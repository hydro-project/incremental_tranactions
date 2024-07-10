CREATE TABLE transaction_parameters (
    txn_id INT PRIMARY KEY,
    w_id INT, -- warehouse key
    d_id INT, -- district key, [1..10]
    c_id INT, -- customer key
    o_ol_cnt INT, -- # order lines
);

-- int neword()
-- {
--  EXEC SQL WHENEVER NOT FOUND GOTO sqlerr;
--  EXEC SQL WHENEVER SQLERROR GOTO sqlerr;
--  gettimestamp(datetime);

-- row in WAREHOUSE table is selected, warehouse tax rate (W_TAX) is retrieved
-- row in CUSTOMER table is selected, customer discount (C_DISCOUNT),
--                                             last name (C_LAST), and
--                                             credit status (C_CREDIT) retrieved
CREATE VIEW customer_select AS SELECT c.c_discount, c.c_last, c.c_credit, w.w_tax   --  EXEC SQL SELECT c_discount, c_last, c_credit, w_tax 
FROM customer AS c,                                                                 --  INTO :c_discount, :c_last, :c_credit, :w_tax
     warehouse AS w,                                                                --  FROM customer, warehouse
     transaction_parameters AS t                                                    --  WHERE w_id = :w_id AND c_w_id = w_id AND
WHERE w.w_id = t.w_id                                                               --  c_d_id = :d_id AND c_id = :c_id;
  AND c.c_w_id = t.w.id
  AND c.c_d_id = t.d_id
  AND c.c_id = t.c_id;   

-- row in DISTRICT table selected, district tax rate (D_TAX),
--                                          current order number (D_NEXT_O_ID) retrieved
CREATE VIEW district_select AS SELECT d.d_next_o_id, d.d_tax                        --  EXEC SQL SELECT d_next_o_id, d_tax INTO :d_next_o_id, :d_tax
FROM district AS d,                                                                 --  FROM district
     transaction_parameters AS t                                                    --  WHERE d_id = :d_id AND d_w_id = :w_id;
WHERE d.d_id = t.d_id
  AND d.d_w_id = t.w_id;                                     

-- update district order number [+1]
CREATE VIEW district_updates AS                                                     --  EXEC SQL UPDATE district SET d_next_o_id = :d_next_o_id + 1
SELECT t.txn_id, d.d_id, d.d_w_id, (d.d_next_o_id + 1) AS d_next_o_id               --  WHERE d_id = :d_id AND d_w_id = :w_id;
FROM district AS d,                                                                 --  o_id=d_next_o_id;
     transaction_parameters AS t
WHERE d.d_id = t.d_id,
  AND d.d_w_id = t.w_id;

-- update order s.t. o_id = d_next_o_id,
--                   o_all_local = { 1 if only home order-lines, 0 otherwise }
--                     i.e., ol_supply_w_id = o_w_id ; TODO need to get this from another view?
CREATE VIEW order_update AS                                                       --  EXEC SQL INSERT INTO ORDERS (o_id,  o_d_id, o_w_id,
SELECT t.txn_id, u.d_next_o_id AS o_id, t.d_id AS o_d_id, t.w_id AS o_w_id,       --                               o_c_id, o_entry_d, o_ol_cnt,
       t.c_id AS o_c_id, t.datettime AS ts, t.ol_cnt AS o_ol_cnt                  --                               o_all_local)
FROM orders AS o,                                                                 --                       VALUES (:o_id, :d_id,  :w_id,
     district_updates AS u,                                                       --                               :c_id,  :datetime, :o_ol_cnt,
     transaction_parameters AS t                                                  --                               :o_all_local);
WHERE u.txn_id = t.txn_id

CREATE VIEW new_order_update AS                                                   --  EXEC SQL INSERT INTO NEW_ORDER (no_o_id, no_d_id, no_w_id)
SELECT t.txn_id, u.d_next_o_id as o_id, t.d_id, t.w_id                            --  VALUES (:o_id, :d_id, :w_id);
FROM orders AS o,
    district_updates AS u,
    transaction_parameters AS t
WHERE u.txn_id = t.txn_id

 for (ol_number=1; ol_number<=o_ol_cnt; ol_number++) 
 {
    ol_supply_w_id=atol(supware[ol_number-1]);
    if (ol_supply_w_id != w_id) o_all_local=0; -- set o_all_local to 0 if any supplier not local
    ol_i_id=atol(itemid[ol_number-1]);
    ol_quantity=atol(qty[ol_number-1]);
    EXEC SQL WHENEVER NOT FOUND GOTO invaliditem;

    EXEC SQL SELECT i_price, i_name , i_data  -- select item
    INTO :i_price, :i_name, :i_data
    FROM item

    WHERE i_id = :ol_i_id;
    price[ol_number-1] = i_price;
    strncpy(iname[ol_number-1],i_name,24);
    EXEC SQL WHENEVER NOT FOUND GOTO sqlerr;

    EXEC SQL SELECT s_quantity, s_data, 
    s_dist_01, s_dist_02, s_dist_03, s_dist_04, s_dist_05
    s_dist_06, s_dist_07, s_dist_08, s_dist_09, s_dist_10
    INTO :s_quantity, :s_data, 
    :s_dist_01, :s_dist_02, :s_dist_03, :s_dist_04, :s_dist_05
    :s_dist_06, :s_dist_07, :s_dist_08, :s_dist_09, :s_dist_10
    FROM stock
    WHERE s_i_id = :ol_i_id AND s_w_id = :ol_supply_w_id;
    pick_dist_info(ol_dist_info, ol_w_id); -- pick correct s_dist_xx
    stock[ol_number-1] = s_quantity;
    if ( (strstr(i_data,"original") != NULL) &&
         (strstr(s_data,"original") != NULL) ) 
        bg[ol_number-1] = 'B';
    else
        bg[ol_number-1] = 'G';
    
    -- txn always succeeds, unless it's a not-found item
    if (s_quantity > ol_quantity)          -- XXX only if it exceeds by 10 or more? 2.4.2.2
        s_quantity = s_quantity - ol_quantity;
    else
        s_quantity = s_quantity - ol_quantity + 91;

    EXEC SQL UPDATE stock SET s_quantity = :s_quantity
    WHERE s_i_id = :ol_i_id
    AND s_w_id = :ol_supply_w_id;

    ol_amount = ol_quantity * i_price * (1+w_tax+d_tax) * (1-c_discount);
    amt[ol_number-1]=ol_amount;
    total += ol_amount;
    EXEC SQL INSERT 
    INTO order_line (ol_o_id, ol_d_id, ol_w_id, ol_number,
    ol_i_id, ol_supply_w_id,
    ol_quantity, ol_amount, ol_dist_info) 
    VALUES (:o_id, :d_id, :w_id, :ol_number,
    :ol_i_id, :ol_supply_w_id, 
    :ol_quantity, :ol_amount, :ol_dist_info);
 } / *End Order Lines*/

--  EXEC SQL COMMIT WORK;
--  return(0);
-- 
-- invaliditem:
--  EXEC SQL ROLLBACK WORK;
--  printf("Item number is not valid");
--  return(0);
-- 
-- sqlerr:
--  error();
-- }