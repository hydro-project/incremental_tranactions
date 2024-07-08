-- Warehouse table
/* W_ID    2*W unique IDs  W Warehouses are populated
W_NAME  variable text, size 10
W_STREET_1  variable text, size 20
W_STREET_2 variable text, size 20
W_CITY variable text, size 20
W_STATE fixed text, size 2
W_ZIP   fixed text, size 9
W_TAX   signed numeric(4,4)   Sales tax
W_YTD   signed numeric(12,2)    Year to date balance */
CREATE TABLE warehouse_static (
    w_id INT PRIMARY KEY,
    w_name VARCHAR(10),
    w_street_1 VARCHAR(20),
    w_street_2 VARCHAR(20),
    w_city VARCHAR(20),
    w_state CHAR(2),
    w_zip CHAR(9),
    w_tax DECIMAL(4,4)
);
CREATE TABLE warehouse (
    w_id INT PRIMARY KEY,
    w_ytd DECIMAL(12,2)
);

-- District table
/* Field Name   Field Definition    Comments
D_ID    20 unique IDs   10 are populated per warehouse
D_W_ID  2*W unique IDs
D_NAME  variable text, size 10
D_STREET_1  variable text, size 20
D_STREET_2  variable text, size 20
D_CITY  variable text, size 20
D_STATE fixed text, size 2
D_ZIP   fixed text, size 9
D_TAX   signed numeric(4,4) Sales tax
D_YTD   signed numeric(12,2)    Year to date balance
D_NEXT_O_ID 10,000,000 unique IDs   Next available Order number
Primary Key: (D_W_ID, D_ID)
D_W_ID Foreign Key, references W_ID */
CREATE TABLE district_static (
    d_id INT,
    d_w_id INT,
    d_name VARCHAR(10),
    d_street_1 VARCHAR(20),
    d_street_2 VARCHAR(20),
    d_city VARCHAR(20),
    d_state CHAR(2),
    d_zip CHAR(9),
    d_tax DECIMAL(4,4),
    PRIMARY KEY (d_w_id, d_id),
    FOREIGN KEY (d_w_id) REFERENCES warehouse(w_id)
);
CREATE TABLE district_next_id (
    d_id INT,
    d_w_id INT,
    d_next_o_id INT,
    PRIMARY KEY (d_w_id, d_id),
    FOREIGN KEY (d_w_id) REFERENCES warehouse(w_id)
);
CREATE TABLE district_ytd (
    d_id INT,
    d_w_id INT,
    d_ytd DECIMAL(12,2),
    PRIMARY KEY (d_w_id, d_id),
    FOREIGN KEY (d_w_id) REFERENCES warehouse(w_id)
);

-- Customer table
/* Field Name	Field Definition	Comments
C_ID	96,000 unique IDs	3,000 are populated per district
C_D_ID	20 unique IDs
C_W_ID	2*W unique IDs
C_FIRST	variable text, size 16
C_MIDDLE	fixed text, size 2
C_LAST	variable text, size 16
C_STREET_1	variable text, size 20
C_STREET_2	variable text, size 20
C_CITY	variable text, size 20
C_STATE	fixed text, size 2
C_ZIP	fixed text, size 9
C_PHONE	fixed text, size 16
C_SINCE	date and time
C_CREDIT	fixed text, size 2	"GC"=good, "BC"=bad
C_CREDIT_LIM	signed numeric(12, 2)
C_DISCOUNT	signed numeric(4, 4)
C_BALANCE	signed numeric(12, 2)
C_YTD_PAYMENT	signed numeric(12, 2)
C_PAYMENT_CNT	numeric(4)
C_DELIVERY_CNT	numeric(4)
C_DATA	variable text, size 500	Miscellaneous information
Primary Key: (C_W_ID, C_D_ID, C_ID)
(C_W_ID, C_D_ID) Foreign Key, references (D_W_ID, D_ID) */
CREATE TABLE customer (
    c_id INT,
    c_d_id INT,
    c_w_id INT,
    c_first VARCHAR(16),
    c_middle CHAR(2),
    c_last VARCHAR(16),
    c_street_1 VARCHAR(20),
    c_street_2 VARCHAR(20),
    c_city VARCHAR(20),
    c_state CHAR(2),
    c_zip CHAR(9),
    c_phone CHAR(16),
    c_since TIMESTAMP,
    c_credit CHAR(2),
    c_credit_lim DECIMAL(12,2),
    c_discount DECIMAL(4,4),
    c_balance DECIMAL(12,2),
    c_ytd_payment DECIMAL(12,2),
    c_payment_cnt INT,
    c_delivery_cnt INT,
    c_data VARCHAR(500),
    PRIMARY KEY (c_w_id, c_d_id, c_id),
    FOREIGN KEY (c_w_id, c_d_id) REFERENCES district(d_w_id, d_id)
);