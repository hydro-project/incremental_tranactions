package org.dbsp.sqlCompiler.compiler.sql.simple;

import org.dbsp.sqlCompiler.circuit.DBSPCircuit;
import org.dbsp.sqlCompiler.circuit.operator.DBSPMapIndexOperator;
import org.dbsp.sqlCompiler.circuit.operator.DBSPPartitionedRollingAggregateWithWaterlineOperator;
import org.dbsp.sqlCompiler.compiler.DBSPCompiler;
import org.dbsp.sqlCompiler.compiler.StderrErrorReporter;
import org.dbsp.sqlCompiler.compiler.TestUtil;
import org.dbsp.sqlCompiler.compiler.sql.SqlIoTest;
import org.dbsp.sqlCompiler.compiler.visitors.outer.CircuitVisitor;
import org.junit.Assert;
import org.junit.Ignore;
import org.junit.Test;

public class RegressionTests extends SqlIoTest {
    @Test
    public void issue2017() {
        String sql = """
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
                
                CREATE TABLE transaction_parameters (
                    txn_id INT PRIMARY KEY,
                    w_id INT,
                    d_id INT,
                    c_id INT,
                    c_w_id INT,
                    c_d_id INT,
                    c_last VARCHAR(20), -- TODO check
                    h_amount DECIMAL(5,2),
                    h_date TIMESTAMP,
                    datetime_ TIMESTAMP
                );
                
                CREATE VIEW cust_max AS
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
                LIMIT 1;""";
        this.compileRustTestCase(sql);
    }

    @Test
    public void issue1868() {
        String sql = """
                CREATE TABLE example_a (
                    id INT NOT NULL
                );
                
                CREATE TABLE example_b (
                    id INT NOT NULL
                );
                
                CREATE VIEW example_c AS (
                    SELECT COALESCE(example_a.id, 0) - COALESCE(example_b.id, 0)
                    FROM example_a
                         FULL JOIN example_b
                         ON example_a.id = example_b.id
                );""";
        this.compileRustTestCase(sql);
    }

    @Test
    public void issue1898() {
        String sql = """
                create table t(
                    id bigint,
                    part bigint
                );
                
                create view v as
                SELECT
                    id,
                    COUNT(DISTINCT id) FILTER (WHERE id > 100) OVER window_100 AS agg
                FROM
                    t
                WINDOW
                    window_100 AS (PARTITION BY part ORDER BY id RANGE BETWEEN 100 PRECEDING AND CURRENT ROW);""";
        DBSPCompiler compiler = this.testCompiler();
        compiler.options.languageOptions.throwOnError = false;
        compiler.compileStatements(sql);
        TestUtil.assertMessagesContain(compiler, "OVER must be applied to aggregate function");
    }

    @Test
    public void issue1768() {
        String sql = """
                CREATE TABLE transaction (
                    trans_date_time TIMESTAMP NOT NULL LATENESS INTERVAL 1 DAY,
                    cc_num BIGINT NOT NULL,
                    merchant STRING,
                    category STRING,
                    amt FLOAT64,
                    trans_num STRING,
                    unix_time INTEGER NOT NULL,
                    merch_lat FLOAT64 NOT NULL,
                    merch_long FLOAT64 NOT NULL,
                    is_fraud INTEGER
                );
                
                CREATE TABLE demographics (
                    cc_num BIGINT NOT NULL,
                    first STRING,
                    gender STRING,
                    street STRING,
                    city STRING,
                    state STRING,
                    zip INTEGER,
                    lat FLOAT64,
                    long FLOAT64,
                    city_pop INTEGER,
                    job STRING,
                    dob STRING
                );
            
                CREATE VIEW V AS SELECT
                    transaction.cc_num,
                    CASE
                      WHEN dayofweek(trans_date_time) IN(6, 7) THEN true
                      ELSE false
                    END AS is_weekend,
                    CASE
                      WHEN hour(trans_date_time) <= 6 THEN true
                      ELSE false
                    END AS is_night,
                    category,
                    AVG(amt) OVER window_1_day AS avg_spend_pd,
                    AVG(amt) OVER window_7_day AS avg_spend_pw,
                    AVG(amt) OVER window_30_day AS avg_spend_pm,
                    COUNT(*) OVER window_1_day AS trans_freq_24,
                      amt, state, job, unix_time, city_pop, is_fraud
                  FROM transaction
                  JOIN demographics
                  ON transaction.cc_num = demographics.cc_num
                  WINDOW
                    window_1_day AS (PARTITION BY transaction.cc_num ORDER BY unix_time RANGE BETWEEN 86400 PRECEDING AND CURRENT ROW),
                    window_7_day AS (PARTITION BY transaction.cc_num ORDER BY unix_time RANGE BETWEEN 604800 PRECEDING AND CURRENT ROW),
                    window_30_day AS (PARTITION BY transaction.cc_num ORDER BY unix_time RANGE BETWEEN 2592000 PRECEDING AND CURRENT ROW);""";

        DBSPCompiler compiler = this.testCompiler();
        compiler.options.languageOptions.incrementalize = true;
        compiler.compileStatements(sql);
        DBSPCircuit circuit = getCircuit(compiler);
        CircuitVisitor visitor = new CircuitVisitor(new StderrErrorReporter()) {
            int mapIndex = 0;

            @Override
            public void postorder(DBSPMapIndexOperator operator) {
                this.mapIndex++;
            }

            @Override
            public void endVisit() {
                // We expect 9 MapIndex operators instead of 11 if CSE works
                Assert.assertEquals(9, this.mapIndex);
            }
        };
        visitor.apply(circuit);
    }

    @Test
    public void missingCast() {
        String sql = """
                create table TRANSACTION (unix_time BIGINT LATENESS 0);
                """;
        this.compileRustTestCase(sql);
    }

    @Test @Ignore("Calcite decorrelator fails")
    public void issue1956() {
        String sql = """
                CREATE TABLE auctions (
                  id INT PRIMARY KEY,
                  seller INT,
                  item TEXT
                );
                
                CREATE TABLE bids (
                  id INT PRIMARY KEY,
                  buyer INT,
                  auction_id INT,
                  amount INT
                );
                
                CREATE VIEW V AS SELECT id, (SELECT array_agg(buyer) FROM (
                  SELECT buyer FROM bids WHERE auction_id = auctions.id
                  ORDER BY buyer LIMIT 10
                )) FROM auctions;""";
        this.compileRustTestCase(sql);
    }

    @Test
    public void issue1957() {
        String sql = """
                CREATE TABLE warehouse (
                   id INT PRIMARY KEY,
                   parentId INT
                );
                
                CREATE VIEW V AS SELECT\s
                  id,
                  (SELECT ARRAY_AGG(id) FROM (
                    SELECT id FROM warehouse WHERE parentId = warehouse.id
                    ORDER BY id LIMIT 10
                  )) AS first_children
                FROM warehouse;""";
        this.compileRustTestCase(sql);
    }

    @Test
    public void issue1793() {
        String sql = """
                CREATE TABLE transaction_demographics (
                    trans_date_time TIMESTAMP,
                    cc_num BIGINT NOT NULL,
                    category STRING,
                    amt FLOAT64,
                    unix_time INTEGER NOT NULL LATENESS 86400,
                    first STRING,
                    state STRING,
                    job STRING,
                    city_pop INTEGER,
                    is_fraud BOOLEAN
                );
            
                CREATE VIEW V AS SELECT
                    cc_num,
                    CASE
                      WHEN dayofweek(trans_date_time) IN(6, 7) THEN true
                      ELSE false
                    END AS is_weekend,
                    CASE
                      WHEN hour(trans_date_time) <= 6 THEN true
                      ELSE false
                    END AS is_night,
                    category,
                    AVG(amt) OVER window_1_day AS avg_spend_pd,
                    AVG(amt) OVER window_7_day AS avg_spend_pw,
                    AVG(amt) OVER window_30_day AS avg_spend_pm,
                    COUNT(*) OVER window_1_day AS trans_freq_24,
                      amt, state, job, unix_time, city_pop, is_fraud
                  FROM transaction_demographics
                  WINDOW
                    window_1_day AS (PARTITION BY cc_num ORDER BY unix_time RANGE BETWEEN 86400 PRECEDING AND CURRENT ROW),
                    window_7_day AS (PARTITION BY cc_num ORDER BY unix_time RANGE BETWEEN 604800 PRECEDING AND CURRENT ROW),
                    window_30_day AS (PARTITION BY cc_num ORDER BY unix_time RANGE BETWEEN 2592000 PRECEDING AND CURRENT ROW);""";
        DBSPCompiler compiler = this.testCompiler();
        compiler.options.languageOptions.incrementalize = true;
        compiler.compileStatements(sql);
        DBSPCircuit circuit = getCircuit(compiler);
        CircuitVisitor visitor = new CircuitVisitor(new StderrErrorReporter()) {
            int count = 0;

            @Override
            public void postorder(DBSPPartitionedRollingAggregateWithWaterlineOperator operator) {
                this.count++;
            }

            @Override
            public void endVisit() {
                Assert.assertEquals(3, this.count);
            }
        };
        visitor.apply(circuit);
        this.compileRustTestCase(sql);
    }
}