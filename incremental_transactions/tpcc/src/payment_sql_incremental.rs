// Automatically-generated file
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unconditional_panic)]
#![allow(non_camel_case_types)]

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

declare_tuples! {
    Tup16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>,
    Tup21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>,
    Tup14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>,
    Tup15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>,
}

pipeline_types::deserialize_without_context!(
    Tup16, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
pipeline_types::deserialize_without_context!(
    Tup21, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
    T19, T20
);
pipeline_types::deserialize_without_context!(
    Tup14, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13
);
pipeline_types::deserialize_without_context!(
    Tup15, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14
);

sqlvalue::to_sql_row_impl! {
    Tup16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>,
    Tup21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>,
    Tup14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>,
    Tup15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>,
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
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
            OutputHandle<WSet<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>>,
            OutputHandle<
                WSet<
                    Tup7<
                        Option<String>,
                        Option<String>,
                        Option<String>,
                        Option<String>,
                        Option<String>,
                        Option<String>,
                        Option<i32>,
                    >,
                >,
            >,
            OutputHandle<WSet<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>>,
            OutputHandle<
                WSet<
                    Tup7<
                        Option<String>,
                        Option<String>,
                        Option<String>,
                        Option<String>,
                        Option<String>,
                        Option<String>,
                        Option<i32>,
                    >,
                >,
            >,
            OutputHandle<
                WSet<
                    Tup15<
                        Option<String>,
                        Option<String>,
                        Option<String>,
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
                        Option<i32>,
                    >,
                >,
            >,
            OutputHandle<
                WSet<
                    Tup14<
                        Option<String>,
                        Option<String>,
                        Option<String>,
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
            OutputHandle<
                WSet<
                    Tup8<
                        Option<i32>,
                        Option<i32>,
                        Option<i32>,
                        Option<i32>,
                        Option<i32>,
                        Option<Timestamp>,
                        Option<Decimal>,
                        Option<String>,
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

        // CREATE TABLE `TRANSACTION_PARAMETERS` (`TXN_ID` INTEGER PRIMARY KEY, `W_ID` INTEGER, `D_ID` INTEGER, `C_ID` INTEGER, `C_W_ID` INTEGER, `C_D_ID` INTEGER, `H_AMOUNT` DECIMAL(5, 2), `H_DATE` TIMESTAMP, `DATETIME_` TIMESTAMP)
        // DBSPSourceMultisetOperator 332
        // CREATE TABLE `TRANSACTION_PARAMETERS` (`TXN_ID` INTEGER PRIMARY KEY, `W_ID` INTEGER, `D_ID` INTEGER, `C_ID` INTEGER, `C_W_ID` INTEGER, `C_D_ID` INTEGER, `H_AMOUNT` DECIMAL(5, 2), `H_DATE` TIMESTAMP, `DATETIME_` TIMESTAMP)
        let (stream332, handle332) = circuit.add_input_zset::<Tup9<
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<Decimal>,
            Option<Timestamp>,
            Option<Timestamp>,
        >>();

        // rel#66:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition==($0, $3),joinType=inner)
        // DBSPFilterOperator 3342(3114)
        let stream3342: Stream<_, WSet<Tup2<Option<i32>, Option<Decimal>>>> =
            stream67.filter(move |t_1: &Tup2<Option<i32>, Option<Decimal>>| -> bool {
                (!(*t_1).0.is_none())
            });
        // rel#66:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition==($0, $3),joinType=inner)
        // DBSPFilterOperator 3347(3117)
        let stream3347: Stream<
            _,
            WSet<
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
        > = stream332.filter(
            move |t_2: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> bool { (!(*t_2).1.is_none()) },
        );
        // rel#66:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition==($0, $3),joinType=inner)
        // DBSPMapIndexOperator 3352(3120)
        let stream3352: Stream<_, IndexedWSet<Tup1<i32>, Tup2<Option<i32>, Option<Decimal>>>> = stream3342.map_index(move |t_3: &Tup2<Option<i32>, Option<Decimal>>, | ->
        (Tup1<i32>, Tup2<Option<i32>, Option<Decimal>>, ) {
            (Tup1::new(cast_to_i32_i32N((*t_3).0)), Tup2::new((*t_3).0, (*t_3).1.clone()), )
        });
        // rel#66:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition==($0, $3),joinType=inner)
        // DBSPMapIndexOperator 3358(3124)
        let stream3358: Stream<
            _,
            IndexedWSet<
                Tup1<i32>,
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
        > = stream3347.map_index(
            move |t_4: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> (
                Tup1<i32>,
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            ) {
                (
                    Tup1::new(cast_to_i32_i32N((*t_4).1)),
                    Tup9::new(
                        (*t_4).0,
                        (*t_4).1,
                        (*t_4).2,
                        (*t_4).3,
                        (*t_4).4,
                        (*t_4).5,
                        (*t_4).6.clone(),
                        (*t_4).7,
                        (*t_4).8,
                    ),
                )
            },
        );
        // rel#66:LogicalJoin.(left=LogicalTableScan#1,right=LogicalTableScan#3,condition==($0, $3),joinType=inner)
        // DBSPJoinOperator 4490(3130)
        let stream4490: Stream<_, WSet<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>> =
            stream3352.join(
                &stream3358,
                move |t_5: &Tup1<i32>,
                      t_3: &Tup2<Option<i32>, Option<Decimal>>,
                      t_4: &Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >|
                      -> Tup3<Option<i32>, Option<i32>, Option<Decimal>> {
                    Tup3::new(
                        (*t_4).0,
                        (*t_3).0,
                        plus_decimalN_decimalN(
                            cast_to_decimalN_decimalN((*t_3).1.clone().clone(), 13, 2),
                            cast_to_decimalN_decimalN((*t_4).6.clone().clone(), 13, 2),
                        ),
                    )
                },
            );
        // CREATE VIEW `WAREHOUSE_UPDATES` AS
        // SELECT `T`.`TXN_ID`, `W`.`W_ID`, `W`.`W_YTD` + `T`.`H_AMOUNT` AS `W_YTD`
        // FROM `schema`.`WAREHOUSE` AS `W`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `W`.`W_ID` = `T`.`W_ID`
        // DBSPSinkOperator 20135(573)
        let handle20135 = stream4490.output();

        // rel#136:LogicalJoin.(left=LogicalTableScan#71,right=LogicalTableScan#73,condition==($0, $9),joinType=inner)
        // DBSPFilterOperator 3384(3144)
        let stream3384: Stream<
            _,
            WSet<
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
        > = stream49.filter(
            move |t_8: &Tup8<
                Option<i32>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Decimal>,
            >|
                  -> bool { (!(*t_8).0.is_none()) },
        );
        // rel#136:LogicalJoin.(left=LogicalTableScan#71,right=LogicalTableScan#73,condition==($0, $9),joinType=inner)
        // DBSPMapIndexOperator 3394(3150)
        let stream3394: Stream<
            _,
            IndexedWSet<
                Tup1<i32>,
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
        > = stream3384.map_index(
            move |t_10: &Tup8<
                Option<i32>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Decimal>,
            >|
                  -> (
                Tup1<i32>,
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
            ) {
                (
                    Tup1::new(cast_to_i32_i32N((*t_10).0)),
                    Tup8::new(
                        (*t_10).0,
                        (*t_10).1.clone(),
                        (*t_10).2.clone(),
                        (*t_10).3.clone(),
                        (*t_10).4.clone(),
                        (*t_10).5.clone(),
                        (*t_10).6.clone(),
                        (*t_10).7.clone(),
                    ),
                )
            },
        );
        // rel#136:LogicalJoin.(left=LogicalTableScan#71,right=LogicalTableScan#73,condition==($0, $9),joinType=inner)
        // DBSPMapIndexOperator 26980(3154)
        let stream26980: Stream<
            _,
            IndexedWSet<
                Tup1<i32>,
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
        > = stream3347.map_index(
            move |t_11: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> (
                Tup1<i32>,
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            ) {
                (
                    Tup1::new(cast_to_i32_i32N((*t_11).1)),
                    Tup9::new(
                        (*t_11).0,
                        (*t_11).1,
                        (*t_11).2,
                        (*t_11).3,
                        (*t_11).4,
                        (*t_11).5,
                        (*t_11).6.clone(),
                        (*t_11).7,
                        (*t_11).8,
                    ),
                )
            },
        );
        // rel#136:LogicalJoin.(left=LogicalTableScan#71,right=LogicalTableScan#73,condition==($0, $9),joinType=inner)
        // DBSPJoinOperator 26984(3160)
        let stream26984: Stream<
            _,
            WSet<
                Tup7<
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<i32>,
                >,
            >,
        > = stream3394.join(
            &stream26980,
            move |t_12: &Tup1<i32>,
                  t_10: &Tup8<
                Option<i32>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Decimal>,
            >,
                  t_11: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> Tup7<
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<i32>,
            > {
                Tup7::new(
                    (*t_10).2.clone().clone(),
                    (*t_10).3.clone().clone(),
                    (*t_10).4.clone().clone(),
                    (*t_10).5.clone().clone(),
                    (*t_10).6.clone().clone(),
                    (*t_10).1.clone().clone(),
                    (*t_11).0,
                )
            },
        );
        // CREATE VIEW `WAREHOUSE_SELECT` AS
        // SELECT `W`.`W_STREET_1`, `W`.`W_STREET_2`, `W`.`W_CITY`, `W`.`W_STATE`, `W`.`W_ZIP`, `W`.`W_NAME`, `T`.`TXN_ID`
        // FROM `schema`.`WAREHOUSE_STATIC` AS `W`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `W`.`W_ID` = `T`.`W_ID`
        // DBSPSinkOperator 26986(888)
        let handle26986 = stream26984.output();

        // rel#206:LogicalJoin.(left=LogicalTableScan#141,right=LogicalTableScan#143,condition=AND(=($1, $4), =($0, $5)),joinType=inner)
        // DBSPFilterOperator 3426(3174)
        let stream3426: Stream<_, WSet<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>> =
            stream166.filter(
                move |t_15: &Tup3<Option<i32>, Option<i32>, Option<Decimal>>| -> bool {
                    (!or_b_b((*t_15).0.is_none(), (*t_15).1.is_none()))
                },
            );
        // rel#206:LogicalJoin.(left=LogicalTableScan#141,right=LogicalTableScan#143,condition=AND(=($1, $4), =($0, $5)),joinType=inner)
        // DBSPFilterOperator 3431(3177)
        let stream3431: Stream<
            _,
            WSet<
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
        > = stream332.filter(
            move |t_16: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> bool { (!or_b_b((*t_16).1.is_none(), (*t_16).2.is_none())) },
        );
        // rel#206:LogicalJoin.(left=LogicalTableScan#141,right=LogicalTableScan#143,condition=AND(=($1, $4), =($0, $5)),joinType=inner)
        // DBSPMapIndexOperator 3436(3180)
        let stream3436: Stream<
            _,
            IndexedWSet<Tup2<i32, i32>, Tup3<Option<i32>, Option<i32>, Option<Decimal>>>,
        > = stream3426.map_index(
            move |t_17: &Tup3<Option<i32>, Option<i32>, Option<Decimal>>| -> (
                Tup2<i32, i32>,
                Tup3<Option<i32>, Option<i32>, Option<Decimal>>,
            ) {
                (
                    Tup2::new(cast_to_i32_i32N((*t_17).1), cast_to_i32_i32N((*t_17).0)),
                    Tup3::new((*t_17).0, (*t_17).1, (*t_17).2.clone()),
                )
            },
        );
        // rel#206:LogicalJoin.(left=LogicalTableScan#141,right=LogicalTableScan#143,condition=AND(=($1, $4), =($0, $5)),joinType=inner)
        // DBSPMapIndexOperator 3442(3184)
        let stream3442: Stream<
            _,
            IndexedWSet<
                Tup2<i32, i32>,
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
        > = stream3431.map_index(
            move |t_18: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> (
                Tup2<i32, i32>,
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            ) {
                (
                    Tup2::new(cast_to_i32_i32N((*t_18).1), cast_to_i32_i32N((*t_18).2)),
                    Tup9::new(
                        (*t_18).0,
                        (*t_18).1,
                        (*t_18).2,
                        (*t_18).3,
                        (*t_18).4,
                        (*t_18).5,
                        (*t_18).6.clone(),
                        (*t_18).7,
                        (*t_18).8,
                    ),
                )
            },
        );
        // rel#206:LogicalJoin.(left=LogicalTableScan#141,right=LogicalTableScan#143,condition=AND(=($1, $4), =($0, $5)),joinType=inner)
        // DBSPJoinOperator 6268(3190)
        let stream6268: Stream<_, WSet<Tup3<Option<i32>, Option<i32>, Option<Decimal>>>> =
            stream3436.join(
                &stream3442,
                move |t_19: &Tup2<i32, i32>,
                      t_17: &Tup3<Option<i32>, Option<i32>, Option<Decimal>>,
                      t_18: &Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >|
                      -> Tup3<Option<i32>, Option<i32>, Option<Decimal>> {
                    Tup3::new(
                        (*t_17).1,
                        (*t_17).0,
                        plus_decimalN_decimalN(
                            cast_to_decimalN_decimalN((*t_17).2.clone().clone(), 13, 2),
                            cast_to_decimalN_decimalN((*t_18).6.clone().clone(), 13, 2),
                        ),
                    )
                },
            );
        // CREATE VIEW `DISTRICT_UPDATES` AS
        // SELECT `D`.`D_W_ID`, `D`.`D_ID`, `D`.`D_YTD` + `T`.`H_AMOUNT` AS `D_YTD`
        // FROM `schema`.`DISTRICT_YTD` AS `D`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `D`.`D_W_ID` = `T`.`W_ID` AND `D`.`D_ID` = `T`.`D_ID`
        // DBSPSinkOperator 20139(1161)
        let handle20139 = stream6268.output();

        // rel#276:LogicalJoin.(left=LogicalTableScan#211,right=LogicalTableScan#213,condition=AND(=($1, $10), =($0, $11)),joinType=inner)
        // DBSPFilterOperator 3468(3204)
        let stream3468: Stream<
            _,
            WSet<
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
        > = stream120.filter(
            move |t_22: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Decimal>,
            >|
                  -> bool { (!or_b_b((*t_22).0.is_none(), (*t_22).1.is_none())) },
        );
        // rel#276:LogicalJoin.(left=LogicalTableScan#211,right=LogicalTableScan#213,condition=AND(=($1, $10), =($0, $11)),joinType=inner)
        // DBSPMapIndexOperator 3478(3210)
        let stream3478: Stream<
            _,
            IndexedWSet<
                Tup2<i32, i32>,
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
        > = stream3468.map_index(
            move |t_24: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Decimal>,
            >|
                  -> (
                Tup2<i32, i32>,
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
            ) {
                (
                    Tup2::new(cast_to_i32_i32N((*t_24).1), cast_to_i32_i32N((*t_24).0)),
                    Tup9::new(
                        (*t_24).0,
                        (*t_24).1,
                        (*t_24).2.clone(),
                        (*t_24).3.clone(),
                        (*t_24).4.clone(),
                        (*t_24).5.clone(),
                        (*t_24).6.clone(),
                        (*t_24).7.clone(),
                        (*t_24).8.clone(),
                    ),
                )
            },
        );
        // rel#276:LogicalJoin.(left=LogicalTableScan#211,right=LogicalTableScan#213,condition=AND(=($1, $10), =($0, $11)),joinType=inner)
        // DBSPMapIndexOperator 26988(3214)
        let stream26988: Stream<
            _,
            IndexedWSet<
                Tup2<i32, i32>,
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
        > = stream3431.map_index(
            move |t_25: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> (
                Tup2<i32, i32>,
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            ) {
                (
                    Tup2::new(cast_to_i32_i32N((*t_25).1), cast_to_i32_i32N((*t_25).2)),
                    Tup9::new(
                        (*t_25).0,
                        (*t_25).1,
                        (*t_25).2,
                        (*t_25).3,
                        (*t_25).4,
                        (*t_25).5,
                        (*t_25).6.clone(),
                        (*t_25).7,
                        (*t_25).8,
                    ),
                )
            },
        );
        // rel#276:LogicalJoin.(left=LogicalTableScan#211,right=LogicalTableScan#213,condition=AND(=($1, $10), =($0, $11)),joinType=inner)
        // DBSPJoinOperator 26992(3220)
        let stream26992: Stream<
            _,
            WSet<
                Tup7<
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<i32>,
                >,
            >,
        > = stream3478.join(
            &stream26988,
            move |t_26: &Tup2<i32, i32>,
                  t_24: &Tup9<
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
                  t_25: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> Tup7<
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<i32>,
            > {
                Tup7::new(
                    (*t_24).3.clone().clone(),
                    (*t_24).4.clone().clone(),
                    (*t_24).5.clone().clone(),
                    (*t_24).6.clone().clone(),
                    (*t_24).7.clone().clone(),
                    (*t_24).2.clone().clone(),
                    (*t_25).0,
                )
            },
        );
        // CREATE VIEW `DISTRICT_SELECT` AS
        // SELECT `D`.`D_STREET_1`, `D`.`D_STREET_2`, `D`.`D_CITY`, `D`.`D_STATE`, `D`.`D_ZIP`, `D`.`D_NAME`, `T`.`TXN_ID`
        // FROM `schema`.`DISTRICT_STATIC` AS `D`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `D`.`D_W_ID` = `T`.`W_ID` AND `D`.`D_ID` = `T`.`D_ID`
        // DBSPSinkOperator 26994(1508)
        let handle26994 = stream26992.output();

        // rel#346:LogicalJoin.(left=LogicalTableScan#281,right=LogicalTableScan#283,condition=AND(=($2, $25), =($1, $26), =($0, $24)),joinType=inner)
        // DBSPFilterOperator 3510(3234)
        let stream3510: Stream<
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
            move |t_29: &Tup21<
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
                    or_b_b((*t_29).0.is_none(), (*t_29).1.is_none()),
                    (*t_29).2.is_none(),
                ))
            },
        );
        // rel#346:LogicalJoin.(left=LogicalTableScan#281,right=LogicalTableScan#283,condition=AND(=($2, $25), =($1, $26), =($0, $24)),joinType=inner)
        // DBSPFilterOperator 3515(3237)
        let stream3515: Stream<
            _,
            WSet<
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
        > = stream332.filter(
            move |t_30: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> bool {
                (!or_b_b(
                    or_b_b((*t_30).3.is_none(), (*t_30).4.is_none()),
                    (*t_30).5.is_none(),
                ))
            },
        );
        // rel#346:LogicalJoin.(left=LogicalTableScan#281,right=LogicalTableScan#283,condition=AND(=($2, $25), =($1, $26), =($0, $24)),joinType=inner)
        // DBSPMapIndexOperator 3520(3240)
        let stream3520: Stream<
            _,
            IndexedWSet<
                Tup3<i32, i32, i32>,
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
        > = stream3510.map_index(
            move |t_31: &Tup21<
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
                Tup3<i32, i32, i32>,
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
                        cast_to_i32_i32N((*t_31).2),
                        cast_to_i32_i32N((*t_31).1),
                        cast_to_i32_i32N((*t_31).0),
                    ),
                    Tup21::new(
                        (*t_31).0,
                        (*t_31).1,
                        (*t_31).2,
                        (*t_31).3.clone(),
                        (*t_31).4.clone(),
                        (*t_31).5.clone(),
                        (*t_31).6.clone(),
                        (*t_31).7.clone(),
                        (*t_31).8.clone(),
                        (*t_31).9.clone(),
                        (*t_31).10.clone(),
                        (*t_31).11.clone(),
                        (*t_31).12,
                        (*t_31).13.clone(),
                        (*t_31).14.clone(),
                        (*t_31).15.clone(),
                        (*t_31).16.clone(),
                        (*t_31).17.clone(),
                        (*t_31).18,
                        (*t_31).19,
                        (*t_31).20.clone(),
                    ),
                )
            },
        );
        // rel#346:LogicalJoin.(left=LogicalTableScan#281,right=LogicalTableScan#283,condition=AND(=($2, $25), =($1, $26), =($0, $24)),joinType=inner)
        // DBSPMapIndexOperator 3526(3244)
        let stream3526: Stream<
            _,
            IndexedWSet<
                Tup3<i32, i32, i32>,
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
        > = stream3515.map_index(
            move |t_32: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> (
                Tup3<i32, i32, i32>,
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            ) {
                (
                    Tup3::new(
                        cast_to_i32_i32N((*t_32).4),
                        cast_to_i32_i32N((*t_32).5),
                        cast_to_i32_i32N((*t_32).3),
                    ),
                    Tup9::new(
                        (*t_32).0,
                        (*t_32).1,
                        (*t_32).2,
                        (*t_32).3,
                        (*t_32).4,
                        (*t_32).5,
                        (*t_32).6.clone(),
                        (*t_32).7,
                        (*t_32).8,
                    ),
                )
            },
        );
        // rel#346:LogicalJoin.(left=LogicalTableScan#281,right=LogicalTableScan#283,condition=AND(=($2, $25), =($1, $26), =($0, $24)),joinType=inner)
        // DBSPJoinOperator 11982(3250)
        let stream11982: Stream<
            _,
            WSet<
                Tup15<
                    Option<String>,
                    Option<String>,
                    Option<String>,
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
                    Option<i32>,
                >,
            >,
        > = stream3520.join(
            &stream3526,
            move |t_33: &Tup3<i32, i32, i32>,
                  t_31: &Tup21<
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
                  t_32: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> Tup15<
                Option<String>,
                Option<String>,
                Option<String>,
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
                Option<i32>,
            > {
                Tup15::new(
                    (*t_31).3.clone().clone(),
                    (*t_31).4.clone().clone(),
                    (*t_31).5.clone().clone(),
                    (*t_31).6.clone().clone(),
                    (*t_31).7.clone().clone(),
                    (*t_31).8.clone().clone(),
                    (*t_31).9.clone().clone(),
                    (*t_31).10.clone().clone(),
                    (*t_31).11.clone().clone(),
                    (*t_31).13.clone().clone(),
                    (*t_31).14.clone().clone(),
                    (*t_31).15.clone().clone(),
                    (*t_31).16.clone().clone(),
                    (*t_31).12,
                    (*t_32).0,
                )
            },
        );
        // CREATE VIEW `CUSTOMER_SELECT` AS
        // SELECT `C`.`C_FIRST`, `C`.`C_MIDDLE`, `C`.`C_LAST`, `C`.`C_STREET_1`, `C`.`C_STREET_2`, `C`.`C_CITY`, `C`.`C_STATE`, `C`.`C_ZIP`, `C`.`C_PHONE`, `C`.`C_CREDIT`, `C`.`C_CREDIT_LIM`, `C`.`C_DISCOUNT`, `C`.`C_BALANCE`, `C`.`C_SINCE`, `T`.`TXN_ID`
        // FROM `schema`.`CUSTOMER` AS `C`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `C`.`C_W_ID` = `T`.`C_W_ID` AND `C`.`C_D_ID` = `T`.`C_D_ID` AND `C`.`C_ID` = `T`.`C_ID`
        // DBSPSinkOperator 20143(2046)
        let handle20143 = stream11982.output();

        // rel#416:LogicalJoin.(left=LogicalTableScan#351,right=LogicalTableScan#353,condition==($14, $15),joinType=inner)
        // DBSPFilterOperator 20145(3264)
        let stream20145: Stream<
            _,
            WSet<
                Tup15<
                    Option<String>,
                    Option<String>,
                    Option<String>,
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
                    Option<i32>,
                >,
            >,
        > = stream11982.filter(
            move |t_36: &Tup15<
                Option<String>,
                Option<String>,
                Option<String>,
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
                Option<i32>,
            >|
                  -> bool { (!(*t_36).14.is_none()) },
        );
        // rel#416:LogicalJoin.(left=LogicalTableScan#351,right=LogicalTableScan#353,condition==($14, $15),joinType=inner)
        // DBSPFilterOperator 3557(3267)
        let stream3557: Stream<
            _,
            WSet<
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
        > = stream332.filter(
            move |t_37: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> bool { (!(*t_37).0.is_none()) },
        );
        // rel#416:LogicalJoin.(left=LogicalTableScan#351,right=LogicalTableScan#353,condition==($14, $15),joinType=inner)
        // DBSPMapIndexOperator 20148(3270)
        let stream20148: Stream<
            _,
            IndexedWSet<
                Tup1<i32>,
                Tup15<
                    Option<String>,
                    Option<String>,
                    Option<String>,
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
                    Option<i32>,
                >,
            >,
        > = stream20145.map_index(
            move |t_38: &Tup15<
                Option<String>,
                Option<String>,
                Option<String>,
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
                Option<i32>,
            >|
                  -> (
                Tup1<i32>,
                Tup15<
                    Option<String>,
                    Option<String>,
                    Option<String>,
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
                    Option<i32>,
                >,
            ) {
                (
                    Tup1::new(cast_to_i32_i32N((*t_38).14)),
                    Tup15::new(
                        (*t_38).0.clone(),
                        (*t_38).1.clone(),
                        (*t_38).2.clone(),
                        (*t_38).3.clone(),
                        (*t_38).4.clone(),
                        (*t_38).5.clone(),
                        (*t_38).6.clone(),
                        (*t_38).7.clone(),
                        (*t_38).8.clone(),
                        (*t_38).9.clone(),
                        (*t_38).10.clone(),
                        (*t_38).11.clone(),
                        (*t_38).12.clone(),
                        (*t_38).13,
                        (*t_38).14,
                    ),
                )
            },
        );
        // rel#416:LogicalJoin.(left=LogicalTableScan#351,right=LogicalTableScan#353,condition==($14, $15),joinType=inner)
        // DBSPMapIndexOperator 3568(3274)
        let stream3568: Stream<
            _,
            IndexedWSet<
                Tup1<i32>,
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
        > = stream3557.map_index(
            move |t_39: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> (
                Tup1<i32>,
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            ) {
                (
                    Tup1::new(cast_to_i32_i32N((*t_39).0)),
                    Tup9::new(
                        (*t_39).0,
                        (*t_39).1,
                        (*t_39).2,
                        (*t_39).3,
                        (*t_39).4,
                        (*t_39).5,
                        (*t_39).6.clone(),
                        (*t_39).7,
                        (*t_39).8,
                    ),
                )
            },
        );
        // rel#416:LogicalJoin.(left=LogicalTableScan#351,right=LogicalTableScan#353,condition==($14, $15),joinType=inner)
        // DBSPJoinOperator 20152(3280)
        let stream20152: Stream<
            _,
            WSet<
                Tup14<
                    Option<String>,
                    Option<String>,
                    Option<String>,
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
        > = stream20148.join(
            &stream3568,
            move |t_40: &Tup1<i32>,
                  t_38: &Tup15<
                Option<String>,
                Option<String>,
                Option<String>,
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
                Option<i32>,
            >,
                  t_39: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> Tup14<
                Option<String>,
                Option<String>,
                Option<String>,
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
                Tup14::new(
                    (*t_38).0.clone().clone(),
                    (*t_38).1.clone().clone(),
                    (*t_38).2.clone().clone(),
                    (*t_38).3.clone().clone(),
                    (*t_38).4.clone().clone(),
                    (*t_38).5.clone().clone(),
                    (*t_38).6.clone().clone(),
                    (*t_38).7.clone().clone(),
                    (*t_38).8.clone().clone(),
                    (*t_38).9.clone().clone(),
                    (*t_38).10.clone().clone(),
                    (*t_38).11.clone().clone(),
                    plus_decimalN_decimalN(
                        cast_to_decimalN_decimalN((*t_38).12.clone().clone(), 13, 2),
                        cast_to_decimalN_decimalN((*t_39).6.clone().clone(), 13, 2),
                    ),
                    (*t_38).13,
                )
            },
        );
        // CREATE VIEW `CUSTOMER_UPDATE` AS
        // SELECT `C`.`C_FIRST`, `C`.`C_MIDDLE`, `C`.`C_LAST`, `C`.`C_STREET_1`, `C`.`C_STREET_2`, `C`.`C_CITY`, `C`.`C_STATE`, `C`.`C_ZIP`, `C`.`C_PHONE`, `C`.`C_CREDIT`, `C`.`C_CREDIT_LIM`, `C`.`C_DISCOUNT`, `C`.`C_BALANCE` + `T`.`H_AMOUNT` AS `C_BALANCE`, `C`.`C_SINCE`
        // FROM `schema`.`CUSTOMER_SELECT` AS `C`,
        // `schema`.`TRANSACTION_PARAMETERS` AS `T`
        // WHERE `C`.`TXN_ID` = `T`.`TXN_ID`
        // DBSPSinkOperator 20154(2493)
        let handle20154 = stream20152.output();

        // rel#517:LogicalJoin.(left=LogicalTableScan#421,right=LogicalTableScan#423,condition==($0, $15),joinType=inner)
        // DBSPFilterOperator 26996(3297)
        let stream26996: Stream<
            _,
            WSet<
                Tup7<
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<i32>,
                >,
            >,
        > = stream26984.filter(
            move |t_44: &Tup7<
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<i32>,
            >|
                  -> bool { (!(*t_44).6.is_none()) },
        );
        // rel#517:LogicalJoin.(left=LogicalTableScan#421,right=LogicalTableScan#423,condition==($0, $15),joinType=inner)
        // DBSPMapIndexOperator 26999(3300)
        let stream26999: Stream<
            _,
            IndexedWSet<
                Tup1<i32>,
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            >,
        > = stream3557.map_index(
            move |t_45: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >|
                  -> (
                Tup1<i32>,
                Tup9<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                >,
            ) {
                (
                    Tup1::new(cast_to_i32_i32N((*t_45).0)),
                    Tup9::new(
                        (*t_45).0,
                        (*t_45).1,
                        (*t_45).2,
                        (*t_45).3,
                        (*t_45).4,
                        (*t_45).5,
                        (*t_45).6.clone(),
                        (*t_45).7,
                        (*t_45).8,
                    ),
                )
            },
        );
        // rel#517:LogicalJoin.(left=LogicalTableScan#421,right=LogicalTableScan#423,condition==($0, $15),joinType=inner)
        // DBSPMapIndexOperator 27003(3304)
        let stream27003: Stream<
            _,
            IndexedWSet<
                Tup1<i32>,
                Tup7<
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<i32>,
                >,
            >,
        > = stream26996.map_index(
            move |t_46: &Tup7<
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<i32>,
            >|
                  -> (
                Tup1<i32>,
                Tup7<
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<i32>,
                >,
            ) {
                (
                    Tup1::new(cast_to_i32_i32N((*t_46).6)),
                    Tup7::new(
                        (*t_46).0.clone(),
                        (*t_46).1.clone(),
                        (*t_46).2.clone(),
                        (*t_46).3.clone(),
                        (*t_46).4.clone(),
                        (*t_46).5.clone(),
                        (*t_46).6,
                    ),
                )
            },
        );
        // rel#517:LogicalJoin.(left=LogicalTableScan#421,right=LogicalTableScan#423,condition==($0, $15),joinType=inner)
        // DBSPJoinFlatmapOperator 27007(3310)
        let stream27007: Stream<
            _,
            WSet<
                Tup16<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<i32>,
                >,
            >,
        > = stream26999.join_flatmap(
            &stream27003,
            move |t_47: &Tup1<i32>,
                  t_45: &Tup9<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
            >,
                  t_46: &Tup7<
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<i32>,
            >|
                  -> Option<
                Tup16<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<i32>,
                >,
            > {
                let tmp: Tup16<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<i32>,
                > = Tup16::new(
                    (*t_45).0,
                    (*t_45).1,
                    (*t_45).2,
                    (*t_45).3,
                    (*t_45).4,
                    (*t_45).5,
                    (*t_45).6.clone(),
                    (*t_45).7,
                    (*t_45).8,
                    (*t_46).0.clone(),
                    (*t_46).1.clone(),
                    (*t_46).2.clone(),
                    (*t_46).3.clone(),
                    (*t_46).4.clone(),
                    (*t_46).5.clone(),
                    (*t_46).6,
                );
                (if (!tmp.0.is_none()) {
                    Some(tmp)
                } else {
                    None::<
                        Tup16<
                            Option<i32>,
                            Option<i32>,
                            Option<i32>,
                            Option<i32>,
                            Option<i32>,
                            Option<i32>,
                            Option<Decimal>,
                            Option<Timestamp>,
                            Option<Timestamp>,
                            Option<String>,
                            Option<String>,
                            Option<String>,
                            Option<String>,
                            Option<String>,
                            Option<String>,
                            Option<i32>,
                        >,
                    >
                })
            },
        );
        // rel#520:LogicalJoin.(left=LogicalJoin#517,right=LogicalTableScan#427,condition==($0, $22),joinType=inner)
        // DBSPFilterOperator 27009(3313)
        let stream27009: Stream<
            _,
            WSet<
                Tup7<
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<i32>,
                >,
            >,
        > = stream26992.filter(
            move |t_50: &Tup7<
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<i32>,
            >|
                  -> bool { (!(*t_50).6.is_none()) },
        );
        // rel#520:LogicalJoin.(left=LogicalJoin#517,right=LogicalTableScan#427,condition==($0, $22),joinType=inner)
        // DBSPMapIndexOperator 27012(3316)
        let stream27012: Stream<
            _,
            IndexedWSet<
                Tup1<i32>,
                Tup16<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<i32>,
                >,
            >,
        > = stream27007.map_index(
            move |t_51: &Tup16<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<i32>,
            >|
                  -> (
                Tup1<i32>,
                Tup16<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Decimal>,
                    Option<Timestamp>,
                    Option<Timestamp>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<i32>,
                >,
            ) {
                (
                    Tup1::new(cast_to_i32_i32N((*t_51).0)),
                    Tup16::new(
                        (*t_51).0,
                        (*t_51).1,
                        (*t_51).2,
                        (*t_51).3,
                        (*t_51).4,
                        (*t_51).5,
                        (*t_51).6.clone(),
                        (*t_51).7,
                        (*t_51).8,
                        (*t_51).9.clone(),
                        (*t_51).10.clone(),
                        (*t_51).11.clone(),
                        (*t_51).12.clone(),
                        (*t_51).13.clone(),
                        (*t_51).14.clone(),
                        (*t_51).15,
                    ),
                )
            },
        );
        // rel#520:LogicalJoin.(left=LogicalJoin#517,right=LogicalTableScan#427,condition==($0, $22),joinType=inner)
        // DBSPMapIndexOperator 27016(3320)
        let stream27016: Stream<
            _,
            IndexedWSet<
                Tup1<i32>,
                Tup7<
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<i32>,
                >,
            >,
        > = stream27009.map_index(
            move |t_52: &Tup7<
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<i32>,
            >|
                  -> (
                Tup1<i32>,
                Tup7<
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<i32>,
                >,
            ) {
                (
                    Tup1::new(cast_to_i32_i32N((*t_52).6)),
                    Tup7::new(
                        (*t_52).0.clone(),
                        (*t_52).1.clone(),
                        (*t_52).2.clone(),
                        (*t_52).3.clone(),
                        (*t_52).4.clone(),
                        (*t_52).5.clone(),
                        (*t_52).6,
                    ),
                )
            },
        );
        // rel#520:LogicalJoin.(left=LogicalJoin#517,right=LogicalTableScan#427,condition==($0, $22),joinType=inner)
        // DBSPJoinOperator 27020(3326)
        let stream27020: Stream<
            _,
            WSet<
                Tup8<
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<i32>,
                    Option<Timestamp>,
                    Option<Decimal>,
                    Option<String>,
                >,
            >,
        > = stream27012.join(
            &stream27016,
            move |t_53: &Tup1<i32>,
                  t_51: &Tup16<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Decimal>,
                Option<Timestamp>,
                Option<Timestamp>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<i32>,
            >,
                  t_52: &Tup7<
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<i32>,
            >|
                  -> Tup8<
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<Timestamp>,
                Option<Decimal>,
                Option<String>,
            > {
                Tup8::new(
                    (*t_51).5,
                    (*t_51).4,
                    (*t_51).3,
                    (*t_51).2,
                    (*t_51).1,
                    (*t_51).8,
                    (*t_51).6.clone().clone(),
                    concat_sN_sN((*t_51).14.clone().clone(), (*t_52).5.clone().clone()),
                )
            },
        );
        // CREATE VIEW `HISTORY_INSERT` AS
        // SELECT `T`.`C_D_ID` AS `H_C_D_ID`, `T`.`C_W_ID` AS `H_C_W_ID`, `T`.`C_ID` AS `H_C_ID`, `T`.`D_ID` AS `H_D_ID`, `T`.`W_ID` AS `H_W_ID`, `T`.`DATETIME_` AS `H_DATE`, `T`.`H_AMOUNT` AS `H_AMOUNT`, CONCAT(`W`.`W_NAME`, `D`.`D_NAME`) AS `H_DATA`
        // FROM `schema`.`TRANSACTION_PARAMETERS` AS `T`,
        // `schema`.`WAREHOUSE_SELECT` AS `W`,
        // `schema`.`DISTRICT_SELECT` AS `D`
        // WHERE `T`.`TXN_ID` = `W`.`TXN_ID` AND `T`.`TXN_ID` = `D`.`TXN_ID`
        // DBSPSinkOperator 27022(3091)
        let handle27022 = stream27020.output();

        Ok((
            handle49,
            handle67,
            handle120,
            handle143,
            handle166,
            handle279,
            handle332,
            handle20135,
            handle26986,
            handle20139,
            handle26994,
            handle20143,
            handle20154,
            handle27022,
        ))
    })?;
    Ok((circuit, streams))
}