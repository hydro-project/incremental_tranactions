// Automatically-generated file
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unconditional_panic)]
#![allow(non_camel_case_types)]

use crate::datatypes::*;
use ::serde::{Deserialize, Serialize};
use compare::{Compare, Extract};
use core::cmp::Ordering;
use dbsp::declare_tuples;
use dbsp::{
    algebra::{
        AddAssignByRef, AddByRef, DefaultSemigroup, HasZero, MulByRef, NegByRef, Semigroup,
        SemigroupValue, UnimplementedSemigroup, ZRingValue, ZSet, F32, F64,
    },
    circuit::{checkpointer::Checkpoint, Circuit, CircuitConfig, Stream},
    dynamic::{DynData, DynDataTyped},
    indexed_zset,
    operator::{
        time_series::{OrdPartitionedIndexedZSet, RelOffset, RelRange},
        CmpFunc, FilterMap, Fold, Generator, MaxSemigroup, MinSemigroup,
    },
    utils::*,
    zset, DBData, DBSPHandle, DBWeight, Error, MapHandle, NumEntries, OrdIndexedZSet, OrdZSet,
    OutputHandle, Runtime, TypedBox, ZSetHandle,
};
use dbsp_adapters::Catalog;
#[cfg(test)]
use hashing::*;
use json::*;
use pipeline_types::{deserialize_table_record, serialize_table_record};
#[cfg(test)]
use readers::*;
use rust_decimal::Decimal;
use size_of::*;
use sqllib::{
    aggregates::*, array::*, binary::*, casts::*, geopoint::*, interval::*, operators::*,
    string::*, timestamp::*, *,
};
use sqlvalue::*;
#[cfg(test)]
use sqlx::{any::AnyRow, AnyConnection, Row};
use std::{
    collections::BTreeMap,
    convert::identity,
    fmt::{Debug, Formatter, Result as FmtResult},
    marker::PhantomData,
    ops::Neg,
    path::Path,
};

#[derive(Clone)]
pub struct Semigroup1<T0, TS0>(PhantomData<(T0, TS0)>);

impl<T0, TS0> Semigroup<Tup1<T0>> for Semigroup1<T0, TS0>
where
    TS0: Semigroup<T0>,
{
    fn combine(left: &Tup1<T0>, right: &Tup1<T0>) -> Tup1<T0> {
        Tup1::new(TS0::combine(&left.0, &right.0))
    }
}

pub fn circuit(
    cconf: CircuitConfig,
) -> Result<
    (
        DBSPHandle,
        (
            ZSetHandle<
                Tup8<
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Decimal>,
                >,
            >,
            ZSetHandle<Tup2<Option<i32>, Option<Decimal>>>,
            ZSetHandle<
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Decimal>,
                >,
            >,
            ZSetHandle<Tup3<Option<i32>, Option<i32>, Option<i32>>>,
            ZSetHandle<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>,
            ZSetHandle<
                Tup21<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                >,
            >,
            ZSetHandle<
                Tup10<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
            OutputHandle<WSet<Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>>>>,
            OutputHandle<
                WSet<
                    Tup17<
                        Option<String>,
                        Option<String>,
                        Option<String>,
                        Option<i32>,
                        Option<i32>,
                        Option<i32>,
                        Option<String>,
                        Option<String>,
                        Option<String>,
                        Option<String>,
                        Option<String>,
                        Option<String>,
                        Option<String>,
                        Option<Decimal>,
                        Option<Decimal>,
                        Option<Decimal>,
                        Option<Timestamp>,
                    >,
                >,
            >,
        ),
    ),
    Error,
> {
    let (circuit, streams) = Runtime::init_circuit(cconf, |circuit| {
        // CREATE TABLE `WAREHOUSE_STATIC` (`W_ID` INTEGER PRIMARY KEY, `W_NAME` VARCHAR(10), `W_STREET_1` VARCHAR(20), `W_STREET_2` VARCHAR(20), `W_CITY` VARCHAR(20), `W_STATE` CHAR(2), `W_ZIP` CHAR(9), `W_TAX` DECIMAL(4, 4))
        // DBSPSourceMultisetOperator 49
        // CREATE TABLE `WAREHOUSE_STATIC` (`W_ID` INTEGER PRIMARY KEY, `W_NAME` VARCHAR(10), `W_STREET_1` VARCHAR(20), `W_STREET_2` VARCHAR(20), `W_CITY` VARCHAR(20), `W_STATE` CHAR(2), `W_ZIP` CHAR(9), `W_TAX` DECIMAL(4, 4))
        let (stream49, handle49) = circuit.add_input_zset::<Tup8<
            Option<i32>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<Decimal>,
        >>();

        // CREATE TABLE `WAREHOUSE` (`W_ID` INTEGER PRIMARY KEY, `W_YTD` DECIMAL(12, 2))
        // DBSPSourceMultisetOperator 67
        // CREATE TABLE `WAREHOUSE` (`W_ID` INTEGER PRIMARY KEY, `W_YTD` DECIMAL(12, 2))
        let (stream67, handle67) = circuit.add_input_zset::<Tup2<Option<i32>, Option<Decimal>>>();

        // CREATE TABLE `DISTRICT_STATIC` (`D_ID` INTEGER, `D_W_ID` INTEGER, `D_NAME` VARCHAR(10), `D_STREET_1` VARCHAR(20), `D_STREET_2` VARCHAR(20), `D_CITY` VARCHAR(20), `D_STATE` CHAR(2), `D_ZIP` CHAR(9), `D_TAX` DECIMAL(4, 4), PRIMARY KEY (`D_W_ID`, `D_ID`))
        // DBSPSourceMultisetOperator 120
        // CREATE TABLE `DISTRICT_STATIC` (`D_ID` INTEGER, `D_W_ID` INTEGER, `D_NAME` VARCHAR(10), `D_STREET_1` VARCHAR(20), `D_STREET_2` VARCHAR(20), `D_CITY` VARCHAR(20), `D_STATE` CHAR(2), `D_ZIP` CHAR(9), `D_TAX` DECIMAL(4, 4), PRIMARY KEY (`D_W_ID`, `D_ID`))
        let (stream120, handle120) = circuit.add_input_zset::<Tup9<
            Option<i32>,
            Option<i32>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<Decimal>,
        >>();

        // CREATE TABLE `DISTRICT_NEXT_ID` (`D_ID` INTEGER, `D_W_ID` INTEGER, `D_NEXT_O_ID` INTEGER, PRIMARY KEY (`D_W_ID`, `D_ID`))
        // DBSPSourceMultisetOperator 143
        // CREATE TABLE `DISTRICT_NEXT_ID` (`D_ID` INTEGER, `D_W_ID` INTEGER, `D_NEXT_O_ID` INTEGER, PRIMARY KEY (`D_W_ID`, `D_ID`))
        let (stream143, handle143) =
            circuit.add_input_zset::<Tup3<Option<i32>, Option<i32>, Option<i32>>>();

        // CREATE TABLE `DISTRICT_YTD` (`D_ID` INTEGER, `D_W_ID` INTEGER, `D_YTD` DECIMAL(12, 2), PRIMARY KEY (`D_W_ID`, `D_ID`))
        // DBSPSourceMultisetOperator 166
        // CREATE TABLE `DISTRICT_YTD` (`D_ID` INTEGER, `D_W_ID` INTEGER, `D_YTD` DECIMAL(12, 2), PRIMARY KEY (`D_W_ID`, `D_ID`))
        let (stream166, handle166) =
            circuit.add_input_zset::<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>();

        // CREATE TABLE `CUSTOMER` (`C_ID` INTEGER, `C_D_ID` INTEGER, `C_W_ID` INTEGER, `C_FIRST` VARCHAR(16), `C_MIDDLE` CHAR(2), `C_LAST` VARCHAR(16), `C_STREET_1` VARCHAR(20), `C_STREET_2` VARCHAR(20), `C_CITY` VARCHAR(20), `C_STATE` CHAR(2), `C_ZIP` CHAR(9), `C_PHONE` CHAR(16), `C_SINCE` TIMESTAMP, `C_CREDIT` CHAR(2), `C_CREDIT_LIM` DECIMAL(12, 2), `C_DISCOUNT` DECIMAL(4, 4), `C_BALANCE` DECIMAL(12, 2), `C_YTD_PAYMENT` DECIMAL(12, 2), `C_PAYMENT_CNT` INTEGER, `C_DELIVERY_CNT` INTEGER, `C_DATA` VARCHAR(500), PRIMARY KEY (`C_W_ID`, `C_D_ID`, `C_ID`))
        // DBSPSourceMultisetOperator 279
        // CREATE TABLE `CUSTOMER` (`C_ID` INTEGER, `C_D_ID` INTEGER, `C_W_ID` INTEGER, `C_FIRST` VARCHAR(16), `C_MIDDLE` CHAR(2), `C_LAST` VARCHAR(16), `C_STREET_1` VARCHAR(20), `C_STREET_2` VARCHAR(20), `C_CITY` VARCHAR(20), `C_STATE` CHAR(2), `C_ZIP` CHAR(9), `C_PHONE` CHAR(16), `C_SINCE` TIMESTAMP, `C_CREDIT` CHAR(2), `C_CREDIT_LIM` DECIMAL(12, 2), `C_DISCOUNT` DECIMAL(4, 4), `C_BALANCE` DECIMAL(12, 2), `C_YTD_PAYMENT` DECIMAL(12, 2), `C_PAYMENT_CNT` INTEGER, `C_DELIVERY_CNT` INTEGER, `C_DATA` VARCHAR(500), PRIMARY KEY (`C_W_ID`, `C_D_ID`, `C_ID`))
        let (stream279, handle279) = circuit.add_input_zset::<Tup21<
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<Timestamp>,
            Option<String>,
            Option<Decimal>,
            Option<Decimal>,
            Option<Decimal>,
            Option<Decimal>,
            Option<i32>,
            Option<i32>,
            Option<String>,
        >>();

        // CREATE TABLE `TRANSACTION_PARAMETERS` (`TXN_ID` INTEGER PRIMARY KEY, `W_ID` INTEGER, `D_ID` INTEGER, `C_ID` INTEGER, `C_W_ID` INTEGER, `C_D_ID` INTEGER, `C_LAST` VARCHAR(16), `H_AMOUNT` DECIMAL(5, 2), `H_DATE` TIMESTAMP, `DATETIME_` TIMESTAMP)
        // DBSPSourceMultisetOperator 337
        // CREATE TABLE `TRANSACTION_PARAMETERS` (`TXN_ID` INTEGER PRIMARY KEY, `W_ID` INTEGER, `D_ID` INTEGER, `C_ID` INTEGER, `C_W_ID` INTEGER, `C_D_ID` INTEGER, `C_LAST` VARCHAR(16), `H_AMOUNT` DECIMAL(5, 2), `H_DATE` TIMESTAMP, `DATETIME_` TIMESTAMP)
        let (stream337, handle337) = circuit.add_input_zset::<Tup10<
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<String>,
            Option<Decimal>,
            Option<Timestamp>,
            Option<Timestamp>,
        >>();

        // rel#83:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=($5, $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPFilterOperator 1949(1863)
        let stream1949: Stream<
            _,
            WSet<
                Tup21<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                >,
            >,
        > = stream279.filter(
            move |t_1: &Tup21<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Timestamp>,
                Option<String>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<i32>,
                Option<i32>,
                Option<String>,
            >|
                  -> bool {
                (!or_b_b(
                    or_b_b((*t_1).1.is_none(), (*t_1).2.is_none()),
                    (*t_1).5.is_none(),
                ))
            },
        );
        // rel#83:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=($5, $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPFilterOperator 1954(1866)
        let stream1954: Stream<
            _,
            WSet<
                Tup10<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
        > = stream337.filter(
            move |t_2: &Tup10<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> bool {
                (!or_b_b(
                    or_b_b((*t_2).4.is_none(), (*t_2).5.is_none()),
                    (*t_2).6.is_none(),
                ))
            },
        );
        // rel#83:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=($5, $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPMapIndexOperator 1959(1869)
        let stream1959: Stream<
            _,
            IndexedWSet<
                Tup3<String, i32, i32>,
                Tup21<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                >,
            >,
        > = stream1949.map_index(
            move |t_3: &Tup21<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Timestamp>,
                Option<String>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<i32>,
                Option<i32>,
                Option<String>,
            >|
                  -> (
                Tup3<String, i32, i32>,
                Tup21<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                >,
            ) {
                (
                    Tup3::new(
                        cast_to_s_sN((*t_3).5.clone(), 16, false),
                        cast_to_i32_i32N((*t_3).1),
                        cast_to_i32_i32N((*t_3).2),
                    ),
                    Tup21::new(
                        (*t_3).0,
                        (*t_3).1,
                        (*t_3).2,
                        (*t_3).3.clone(),
                        (*t_3).4.clone(),
                        (*t_3).5.clone(),
                        (*t_3).6.clone(),
                        (*t_3).7.clone(),
                        (*t_3).8.clone(),
                        (*t_3).9.clone(),
                        (*t_3).10.clone(),
                        (*t_3).11.clone(),
                        (*t_3).12,
                        (*t_3).13.clone(),
                        (*t_3).14.clone(),
                        (*t_3).15.clone(),
                        (*t_3).16.clone(),
                        (*t_3).17.clone(),
                        (*t_3).18,
                        (*t_3).19,
                        (*t_3).20.clone(),
                    ),
                )
            },
        );
        // rel#83:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=($5, $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPMapIndexOperator 1965(1873)
        let stream1965: Stream<
            _,
            IndexedWSet<
                Tup3<String, i32, i32>,
                Tup10<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
        > = stream1954.map_index(
            move |t_4: &Tup10<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> (
                Tup3<String, i32, i32>,
                Tup10<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            ) {
                (
                    Tup3::new(
                        cast_to_s_sN((*t_4).6.clone(), 16, false),
                        cast_to_i32_i32N((*t_4).5),
                        cast_to_i32_i32N((*t_4).4),
                    ),
                    Tup10::new(
                        (*t_4).0,
                        (*t_4).1,
                        (*t_4).2,
                        (*t_4).3,
                        (*t_4).4,
                        (*t_4).5,
                        (*t_4).6.clone(),
                        (*t_4).7.clone(),
                        (*t_4).8,
                        (*t_4).9,
                    ),
                )
            },
        );
        // rel#83:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition=AND(=($5, $27), =($1, $26), =($2, $25)),joinType=inner)
        // DBSPJoinOperator 3633(1879)
        let stream3633: Stream<
            _,
            WSet<Tup4<Option<i32>, Option<i32>, Option<i32>, Option<String>>>,
        > = stream1959.join(
            &stream1965,
            move |t_5: &Tup3<String, i32, i32>,
                  t_3: &Tup21<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Timestamp>,
                Option<String>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<i32>,
                Option<i32>,
                Option<String>,
            >,
                  t_4: &Tup10<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> Tup4<Option<i32>, Option<i32>, Option<i32>, Option<String>> {
                Tup4::new((*t_3).2, (*t_3).1, (*t_3).0, (*t_3).3.clone().clone())
            },
        );
        // rel#87:LogicalAggregate.(input=LogicalProject#85,group={0, 1},CUST_ARRAY=ARRAY_AGG($2) WITHIN GROUP ([3]))
        // DBSPMapIndexOperator 3635(1882)
        let stream3635: Stream<
            _,
            IndexedWSet<
                Tup2<Option<i32>, Option<i32>>,
                Tup4<Option<i32>, Option<i32>, Option<i32>, Option<String>>,
            >,
        > = stream3633.map_index(
            move |t_8: &Tup4<Option<i32>, Option<i32>, Option<i32>, Option<String>>| -> (
                Tup2<Option<i32>, Option<i32>>,
                Tup4<Option<i32>, Option<i32>, Option<i32>, Option<String>>,
            ) {
                (
                    Tup2::new((*t_8).0, (*t_8).1),
                    Tup4::new((*t_8).0, (*t_8).1, (*t_8).2, (*t_8).3.clone()),
                )
            },
        );
        // rel#87:LogicalAggregate.(input=LogicalProject#85,group={0, 1},CUST_ARRAY=ARRAY_AGG($2) WITHIN GROUP ([3]))
        // DBSPAggregateOperator 15422(1886)
        let stream15422: Stream<
            _,
            IndexedWSet<Tup2<Option<i32>, Option<i32>>, Tup1<Vec<Option<i32>>>>,
        > = stream3635.aggregate(Fold::<
            _,
            _,
            Semigroup1<Vec<Option<i32>>, ConcatSemigroup<Vec<Option<i32>>>>,
            _,
            _,
        >::with_output(
            Tup1::new(vec![]),
            move |t_33: &mut Tup1<Vec<Option<i32>>>,
                  t_9: &Tup4<Option<i32>, Option<i32>, Option<i32>, Option<String>>,
                  t_35: Weight| {
                (*t_33) = Tup1::new(array_agg(&mut (*t_33).0, (*t_9).2, t_35, false))
            },
            move |t_34: Tup1<Vec<Option<i32>>>| -> Tup1<Vec<Option<i32>>> { Tup1::new(t_34.0) },
        ));
        // rel#87:LogicalAggregate.(input=LogicalProject#85,group={0, 1},CUST_ARRAY=ARRAY_AGG($2) WITHIN GROUP ([3]))
        // DBSPMapOperator 15424(1888)
        let stream15424: Stream<_, WSet<Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>>>> = stream15422.map(move |t_11: (&Tup2<Option<i32>, Option<i32>>, &Tup1<Vec<Option<i32>>>, ), | ->
        Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>> {
            Tup3::new((*t_11.0).0, (*t_11.0).1, (*t_11.1).0.clone())
        });
        // CREATE VIEW `CUST_AGG` AS
        // SELECT `EXPR$0`.`C_W_ID`, `EXPR$0`.`C_D_ID`, ARRAY_AGG(`EXPR$0`.`C_ID` ORDER BY `EXPR$0`.`C_FIRST`) AS `CUST_ARRAY`
        // FROM (SELECT `C`.`C_ID`, `C`.`C_W_ID`, `C`.`C_D_ID`, `C`.`C_FIRST`
        // FROM `schema`.`CUSTOMER` AS `C`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `C`.`C_LAST` = `T`.`C_LAST` AND `C`.`C_D_ID` = `T`.`C_D_ID` AND `C`.`C_W_ID` = `T`.`C_W_ID`) AS `EXPR$0`
        // GROUP BY `EXPR$0`.`C_W_ID`, `EXPR$0`.`C_D_ID`
        // DBSPSinkOperator 15429(925)
        let handle15429 = stream15424.output();

        // rel#186:LogicalJoin.(left=LogicalTableScan#90,right=LogicalTableScan#92,condition=AND(=($2, $21), =($1, $22), =($0, ITEM($23, +(/(ARRAY_LENGTH($23), 2), 1)))),joinType=inner)
        // DBSPFilterOperator 2008(1904)
        let stream2008: Stream<
            _,
            WSet<
                Tup21<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                >,
            >,
        > = stream279.filter(
            move |t_12: &Tup21<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Timestamp>,
                Option<String>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<i32>,
                Option<i32>,
                Option<String>,
            >|
                  -> bool { (!or_b_b((*t_12).1.is_none(), (*t_12).2.is_none())) },
        );
        // rel#186:LogicalJoin.(left=LogicalTableScan#90,right=LogicalTableScan#92,condition=AND(=($2, $21), =($1, $22), =($0, ITEM($23, +(/(ARRAY_LENGTH($23), 2), 1)))),joinType=inner)
        // DBSPFilterOperator 15431(1907)
        let stream15431: Stream<_, WSet<Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>>>> =
            stream15424.filter(
                move |t_13: &Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>>| -> bool {
                    (!or_b_b((*t_13).0.is_none(), (*t_13).1.is_none()))
                },
            );
        // rel#186:LogicalJoin.(left=LogicalTableScan#90,right=LogicalTableScan#92,condition=AND(=($2, $21), =($1, $22), =($0, ITEM($23, +(/(ARRAY_LENGTH($23), 2), 1)))),joinType=inner)
        // DBSPMapIndexOperator 2018(1910)
        let stream2018: Stream<
            _,
            IndexedWSet<
                Tup2<i32, i32>,
                Tup21<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                >,
            >,
        > = stream2008.map_index(
            move |t_14: &Tup21<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Timestamp>,
                Option<String>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<i32>,
                Option<i32>,
                Option<String>,
            >|
                  -> (
                Tup2<i32, i32>,
                Tup21<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                >,
            ) {
                (
                    Tup2::new(cast_to_i32_i32N((*t_14).2), cast_to_i32_i32N((*t_14).1)),
                    Tup21::new(
                        (*t_14).0,
                        (*t_14).1,
                        (*t_14).2,
                        (*t_14).3.clone(),
                        (*t_14).4.clone(),
                        (*t_14).5.clone(),
                        (*t_14).6.clone(),
                        (*t_14).7.clone(),
                        (*t_14).8.clone(),
                        (*t_14).9.clone(),
                        (*t_14).10.clone(),
                        (*t_14).11.clone(),
                        (*t_14).12,
                        (*t_14).13.clone(),
                        (*t_14).14.clone(),
                        (*t_14).15.clone(),
                        (*t_14).16.clone(),
                        (*t_14).17.clone(),
                        (*t_14).18,
                        (*t_14).19,
                        (*t_14).20.clone(),
                    ),
                )
            },
        );
        // rel#186:LogicalJoin.(left=LogicalTableScan#90,right=LogicalTableScan#92,condition=AND(=($2, $21), =($1, $22), =($0, ITEM($23, +(/(ARRAY_LENGTH($23), 2), 1)))),joinType=inner)
        // DBSPMapIndexOperator 15434(1914)
        let stream15434: Stream<
            _,
            IndexedWSet<Tup2<i32, i32>, Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>>>,
        > = stream15431.map_index(
            move |t_15: &Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>>| -> (
                Tup2<i32, i32>,
                Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>>,
            ) {
                (
                    Tup2::new(cast_to_i32_i32N((*t_15).0), cast_to_i32_i32N((*t_15).1)),
                    Tup3::new((*t_15).0, (*t_15).1, (*t_15).2.clone()),
                )
            },
        );
        // rel#186:LogicalJoin.(left=LogicalTableScan#90,right=LogicalTableScan#92,condition=AND(=($2, $21), =($1, $22), =($0, ITEM($23, +(/(ARRAY_LENGTH($23), 2), 1)))),joinType=inner)
        // DBSPJoinFlatmapOperator 15438(1920)
        let stream15438: Stream<
            _,
            WSet<
                Tup24<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<i32>,
                    Option<i32>,
                    Vec<Option<i32>>,
                >,
            >,
        > = stream2018.join_flatmap(
            &stream15434,
            move |t_17: &Tup2<i32, i32>,
                  t_14: &Tup21<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Timestamp>,
                Option<String>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<i32>,
                Option<i32>,
                Option<String>,
            >,
                  t_15: &Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>>|
                  -> Option<
                Tup24<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<i32>,
                    Option<i32>,
                    Vec<Option<i32>>,
                >,
            > {
                let tmp: Tup24<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<i32>,
                    Option<i32>,
                    Vec<Option<i32>>,
                > = Tup24::new(
                    (*t_14).0,
                    (*t_14).1,
                    (*t_14).2,
                    (*t_14).3.clone(),
                    (*t_14).4.clone(),
                    (*t_14).5.clone(),
                    (*t_14).6.clone(),
                    (*t_14).7.clone(),
                    (*t_14).8.clone(),
                    (*t_14).9.clone(),
                    (*t_14).10.clone(),
                    (*t_14).11.clone(),
                    (*t_14).12,
                    (*t_14).13.clone(),
                    (*t_14).14.clone(),
                    (*t_14).15.clone(),
                    (*t_14).16.clone(),
                    (*t_14).17.clone(),
                    (*t_14).18,
                    (*t_14).19,
                    (*t_14).20.clone(),
                    (*t_15).0,
                    (*t_15).1,
                    (*t_15).2.clone(),
                );
                (if wrap_bool(eq_i32N_i32N(
                    tmp.0,
                    index__N_(
                        tmp.23.clone(),
                        cast_to_u_i32(plus_i32_i32(
                            div_i32_i32(cardinality(tmp.23.clone()), 2i32),
                            1i32,
                        )) - 1,
                    ),
                )) {
                    Some(tmp)
                } else {
                    None::<
                        Tup24<
                            Option<i32>,
                            Option<i32>,
                            Option<i32>,
                            Option<String>,
                            Option<String>,
                            Option<String>,
                            Option<String>,
                            Option<String>,
                            Option<String>,
                            Option<String>,
                            Option<String>,
                            Option<String>,
                            Option<Timestamp>,
                            Option<String>,
                            Option<Decimal>,
                            Option<Decimal>,
                            Option<Decimal>,
                            Option<Decimal>,
                            Option<i32>,
                            Option<i32>,
                            Option<String>,
                            Option<i32>,
                            Option<i32>,
                            Vec<Option<i32>>,
                        >,
                    >
                })
            },
        );
        // rel#189:LogicalJoin.(left=LogicalJoin#186,right=LogicalTableScan#96,condition=true,joinType=inner)
        // DBSPMapIndexOperator 15440(1923)
        let stream15440: Stream<
            _,
            IndexedWSet<
                Tup0,
                Tup24<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<i32>,
                    Option<i32>,
                    Vec<Option<i32>>,
                >,
            >,
        > = stream15438.map_index(
            move |t_19: &Tup24<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Timestamp>,
                Option<String>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<i32>,
                Option<i32>,
                Vec<Option<i32>>,
            >|
                  -> (
                Tup0,
                Tup24<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<i32>,
                    Option<i32>,
                    Vec<Option<i32>>,
                >,
            ) {
                (
                    Tup0::new(),
                    Tup24::new(
                        (*t_19).0,
                        (*t_19).1,
                        (*t_19).2,
                        (*t_19).3.clone(),
                        (*t_19).4.clone(),
                        (*t_19).5.clone(),
                        (*t_19).6.clone(),
                        (*t_19).7.clone(),
                        (*t_19).8.clone(),
                        (*t_19).9.clone(),
                        (*t_19).10.clone(),
                        (*t_19).11.clone(),
                        (*t_19).12,
                        (*t_19).13.clone(),
                        (*t_19).14.clone(),
                        (*t_19).15.clone(),
                        (*t_19).16.clone(),
                        (*t_19).17.clone(),
                        (*t_19).18,
                        (*t_19).19,
                        (*t_19).20.clone(),
                        (*t_19).21,
                        (*t_19).22,
                        (*t_19).23.clone(),
                    ),
                )
            },
        );
        // rel#189:LogicalJoin.(left=LogicalJoin#186,right=LogicalTableScan#96,condition=true,joinType=inner)
        // DBSPMapIndexOperator 2045(1927)
        let stream2045: Stream<
            _,
            IndexedWSet<
                Tup0,
                Tup10<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
        > = stream337.map_index(
            move |t_20: &Tup10<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> (
                Tup0,
                Tup10<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            ) {
                (
                    Tup0::new(),
                    Tup10::new(
                        (*t_20).0,
                        (*t_20).1,
                        (*t_20).2,
                        (*t_20).3,
                        (*t_20).4,
                        (*t_20).5,
                        (*t_20).6.clone(),
                        (*t_20).7.clone(),
                        (*t_20).8,
                        (*t_20).9,
                    ),
                )
            },
        );
        // rel#189:LogicalJoin.(left=LogicalJoin#186,right=LogicalTableScan#96,condition=true,joinType=inner)
        // DBSPJoinOperator 15444(1933)
        let stream15444: Stream<
            _,
            WSet<
                Tup17<
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Decimal>,
                    Option<Timestamp>,
                >,
            >,
        > = stream15440.join(
            &stream2045,
            move |t_22: &Tup0,
                  t_19: &Tup24<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Timestamp>,
                Option<String>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<i32>,
                Option<i32>,
                Vec<Option<i32>>,
            >,
                  t_20: &Tup10<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> Tup17<
                Option<String>,
                Option<String>,
                Option<String>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Timestamp>,
            > {
                Tup17::new(
                    (*t_19).3.clone().clone(),
                    (*t_19).4.clone().clone(),
                    (*t_19).5.clone().clone(),
                    (*t_19).0,
                    (*t_19).2,
                    (*t_19).1,
                    (*t_19).6.clone().clone(),
                    (*t_19).7.clone().clone(),
                    (*t_19).8.clone().clone(),
                    (*t_19).9.clone().clone(),
                    (*t_19).10.clone().clone(),
                    (*t_19).11.clone().clone(),
                    (*t_19).13.clone().clone(),
                    (*t_19).14.clone().clone(),
                    (*t_19).15.clone().clone(),
                    (*t_19).16.clone().clone(),
                    (*t_19).12,
                )
            },
        );
        // CREATE VIEW `CUST_BYNAME` AS
        // SELECT `C`.`C_FIRST`, `C`.`C_MIDDLE`, `C`.`C_LAST`, `C`.`C_ID`, `C`.`C_W_ID`, `C`.`C_D_ID`, `C`.`C_STREET_1`, `C`.`C_STREET_2`, `C`.`C_CITY`, `C`.`C_STATE`, `C`.`C_ZIP`, `C`.`C_PHONE`, `C`.`C_CREDIT`, `C`.`C_CREDIT_LIM`, `C`.`C_DISCOUNT`, `C`.`C_BALANCE`, `C`.`C_SINCE`
        // FROM `schema`.`CUSTOMER` AS `C`,
        // `schema`.`CUST_AGG` AS `A`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `C`.`C_W_ID` = `A`.`C_W_ID` AND `C`.`C_D_ID` = `A`.`C_D_ID` AND `C`.`C_ID` = `A`.`CUST_ARRAY`[ARRAY_LENGTH(`A`.`CUST_ARRAY`) / 2 + 1]
        // DBSPSinkOperator 15446(1800)
        let handle15446 = stream15444.output();

        Ok((
            handle49,
            handle67,
            handle120,
            handle143,
            handle166,
            handle279,
            handle337,
            handle15429,
            handle15446,
        ))
    })?;
    Ok((circuit, streams))
}
