/* UPDATE warehouse SET w_ytd = w_ytd + :h_amount
WHERE w_id=:w_id; */
CREATE VIEW warehouse_updates AS SELECT t.txn_id, w.w_id, (w.w_ytd + t.h_amount) AS w_ytd FROM warehouse AS w, transaction_parameters AS t WHERE w.w_id=t.w_id;