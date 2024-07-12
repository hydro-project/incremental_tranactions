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

        // rel#119:LogicalFilter.(input=LogicalTableScan#1,condition=AND(=($1, 43), =($2, 44)))
        // DBSPFilterOperator 398
        let stream398: Stream<
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
                wrap_bool(and_bN_bN(
                    eq_i32N_i32((*t_1).1, 43i32),
                    eq_i32N_i32((*t_1).2, 44i32),
                ))
            },
        );
        // rel#121:LogicalFilter.(input=LogicalTableScan#1,condition=AND(=($5, 'lastname'), =($1, 43), =($2, 44)))
        // DBSPFilterOperator 476
        let stream476: Stream<
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
            move |t_2: &Tup21<
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
                wrap_bool(and_bN_bN(
                    and_bN_bN(
                        eq_sN_s((*t_2).5.clone(), String::from("lastname")),
                        eq_i32N_i32((*t_2).1, 43i32),
                    ),
                    eq_i32N_i32((*t_2).2, 44i32),
                ))
            },
        );
        // rel#123:LogicalProject.(input=LogicalFilter#121,exprs=[CAST(44):INTEGER, CAST(43):INTEGER, $0, $3])
        // DBSPMapOperator 1344(531)
        let stream1344: Stream<
            _,
            WSet<Tup4<Option<i32>, Option<i32>, Option<i32>, Option<String>>>,
        > = stream476.map(
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
                  -> Tup4<Option<i32>, Option<i32>, Option<i32>, Option<String>> {
                Tup4::new(Some(44i32), Some(43i32), (*t_3).0, (*t_3).3.clone())
            },
        );
        // rel#125:LogicalAggregate.(input=LogicalProject#123,group={0, 1},CUST_ARRAY=ARRAY_AGG($2) WITHIN GROUP ([3]))
        // DBSPMapIndexOperator 1392(614)
        let stream1392: Stream<
            _,
            IndexedWSet<
                Tup2<Option<i32>, Option<i32>>,
                Tup4<Option<i32>, Option<i32>, Option<i32>, Option<String>>,
            >,
        > = stream1344.map_index(
            move |t_4: &Tup4<Option<i32>, Option<i32>, Option<i32>, Option<String>>| -> (
                Tup2<Option<i32>, Option<i32>>,
                Tup4<Option<i32>, Option<i32>, Option<i32>, Option<String>>,
            ) {
                (
                    Tup2::new((*t_4).0, (*t_4).1),
                    Tup4::new((*t_4).0, (*t_4).1, (*t_4).2, (*t_4).3.clone()),
                )
            },
        );
        // rel#125:LogicalAggregate.(input=LogicalProject#123,group={0, 1},CUST_ARRAY=ARRAY_AGG($2) WITHIN GROUP ([3]))
        // DBSPStreamAggregateOperator 5102(619)
        let stream5102: Stream<
            _,
            IndexedWSet<Tup2<Option<i32>, Option<i32>>, Tup1<Vec<Option<i32>>>>,
        > = stream1392.stream_aggregate(Fold::<
            _,
            _,
            Semigroup1<Vec<Option<i32>>, ConcatSemigroup<Vec<Option<i32>>>>,
            _,
            _,
        >::with_output(
            Tup1::new(vec![]),
            move |t_22: &mut Tup1<Vec<Option<i32>>>,
                  t_5: &Tup4<Option<i32>, Option<i32>, Option<i32>, Option<String>>,
                  t_24: Weight| {
                (*t_22) = Tup1::new(array_agg(&mut (*t_22).0, (*t_5).2, t_24, false))
            },
            move |t_23: Tup1<Vec<Option<i32>>>| -> Tup1<Vec<Option<i32>>> { Tup1::new(t_23.0) },
        ));
        // rel#125:LogicalAggregate.(input=LogicalProject#123,group={0, 1},CUST_ARRAY=ARRAY_AGG($2) WITHIN GROUP ([3]))
        // DBSPMapOperator 5104(644)
        let stream5104: Stream<_, WSet<Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>>>> = stream5102.map(move |t_7: (&Tup2<Option<i32>, Option<i32>>, &Tup1<Vec<Option<i32>>>, ), | ->
        Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>> {
            Tup3::new((*t_7.0).0, (*t_7.0).1, (*t_7.1).0.clone())
        });
        // rel#127:LogicalJoin.(left=LogicalFilter#119,right=LogicalAggregate#125,condition==($0, ITEM($23, +(/(ARRAY_LENGTH($23), 2), 1))),joinType=inner)
        // DBSPMapIndexOperator 914
        let stream914: Stream<
            _,
            IndexedWSet<
                Tup0,
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
        > = stream398.map_index(
            move |t_8: &Tup21<
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
                Tup0,
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
                    Tup0::new(),
                    Tup21::new(
                        (*t_8).0,
                        (*t_8).1,
                        (*t_8).2,
                        (*t_8).3.clone(),
                        (*t_8).4.clone(),
                        (*t_8).5.clone(),
                        (*t_8).6.clone(),
                        (*t_8).7.clone(),
                        (*t_8).8.clone(),
                        (*t_8).9.clone(),
                        (*t_8).10.clone(),
                        (*t_8).11.clone(),
                        (*t_8).12,
                        (*t_8).13.clone(),
                        (*t_8).14.clone(),
                        (*t_8).15.clone(),
                        (*t_8).16.clone(),
                        (*t_8).17.clone(),
                        (*t_8).18,
                        (*t_8).19,
                        (*t_8).20.clone(),
                    ),
                )
            },
        );
        // rel#127:LogicalJoin.(left=LogicalFilter#119,right=LogicalAggregate#125,condition==($0, ITEM($23, +(/(ARRAY_LENGTH($23), 2), 1))),joinType=inner)
        // DBSPMapIndexOperator 5109(937)
        let stream5109: Stream<_, IndexedWSet<Tup0, Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>>>> = stream5104.map_index(move |t_9: &Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>>, | ->
        (Tup0, Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>>, ) {
            (Tup0::new(), Tup3::new((*t_9).0, (*t_9).1, (*t_9).2.clone()), )
        });
        // rel#127:LogicalJoin.(left=LogicalFilter#119,right=LogicalAggregate#125,condition==($0, ITEM($23, +(/(ARRAY_LENGTH($23), 2), 1))),joinType=inner)
        // DBSPStreamJoinOperator 5113(947)
        let stream5113: Stream<
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
        > = stream914.stream_join(
            &stream5109,
            move |t_11: &Tup0,
                  t_8: &Tup21<
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
                  t_9: &Tup3<Option<i32>, Option<i32>, Vec<Option<i32>>>|
                  -> Tup24<
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
            > {
                Tup24::new(
                    (*t_8).0,
                    (*t_8).1,
                    (*t_8).2,
                    (*t_8).3.clone(),
                    (*t_8).4.clone(),
                    (*t_8).5.clone(),
                    (*t_8).6.clone(),
                    (*t_8).7.clone(),
                    (*t_8).8.clone(),
                    (*t_8).9.clone(),
                    (*t_8).10.clone(),
                    (*t_8).11.clone(),
                    (*t_8).12,
                    (*t_8).13.clone(),
                    (*t_8).14.clone(),
                    (*t_8).15.clone(),
                    (*t_8).16.clone(),
                    (*t_8).17.clone(),
                    (*t_8).18,
                    (*t_8).19,
                    (*t_8).20.clone(),
                    (*t_9).0,
                    (*t_9).1,
                    (*t_9).2.clone(),
                )
            },
        );
        // rel#127:LogicalJoin.(left=LogicalFilter#119,right=LogicalAggregate#125,condition==($0, ITEM($23, +(/(ARRAY_LENGTH($23), 2), 1))),joinType=inner)
        // DBSPFilterOperator 5118(949)
        let stream5118: Stream<
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
        > = stream5113.filter(
            move |t_10: &Tup24<
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
                  -> bool {
                wrap_bool(eq_i32N_i32N(
                    (*t_10).0,
                    index__N_(
                        (*t_10).23.clone(),
                        cast_to_u_i32(plus_i32_i32(
                            div_i32_i32(cardinality((*t_10).23.clone()), 2i32),
                            1i32,
                        )) - 1,
                    ),
                ))
            },
        );
        // rel#129:LogicalProject.(input=LogicalJoin#127,exprs=[$3, $4, $5, $0, CAST(44):INTEGER, CAST(43):INTEGER, $6, $7, $8, $9, $10, $11, $13, $14, $15, $16, $12])
        // DBSPMapOperator 5121(1074)
        let stream5121: Stream<
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
        > = stream5118.map(
            move |t_13: &Tup24<
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
                    (*t_13).3.clone(),
                    (*t_13).4.clone(),
                    (*t_13).5.clone(),
                    (*t_13).0,
                    Some(44i32),
                    Some(43i32),
                    (*t_13).6.clone(),
                    (*t_13).7.clone(),
                    (*t_13).8.clone(),
                    (*t_13).9.clone(),
                    (*t_13).10.clone(),
                    (*t_13).11.clone(),
                    (*t_13).13.clone(),
                    (*t_13).14.clone(),
                    (*t_13).15.clone(),
                    (*t_13).16.clone(),
                    (*t_13).12,
                )
            },
        );
        // CREATE VIEW `CUST_BYNAME_INLINED` AS
        // SELECT `C`.`C_FIRST`, `C`.`C_MIDDLE`, `C`.`C_LAST`, `C`.`C_ID`, `C`.`C_W_ID`, `C`.`C_D_ID`, `C`.`C_STREET_1`, `C`.`C_STREET_2`, `C`.`C_CITY`, `C`.`C_STATE`, `C`.`C_ZIP`, `C`.`C_PHONE`, `C`.`C_CREDIT`, `C`.`C_CREDIT_LIM`, `C`.`C_DISCOUNT`, `C`.`C_BALANCE`, `C`.`C_SINCE`
        // FROM `schema`.`CUSTOMER` AS `C`,
        // (SELECT `EXPR$0`.`C_W_ID`, `EXPR$0`.`C_D_ID`, ARRAY_AGG(`EXPR$0`.`C_ID` ORDER BY `EXPR$0`.`C_FIRST`) AS `CUST_ARRAY`
        // FROM (SELECT `C`.`C_ID`, `C`.`C_W_ID`, `C`.`C_D_ID`, `C`.`C_FIRST`
        // FROM `schema`.`CUSTOMER` AS `C`
        // WHERE `C`.`C_LAST` = 'lastname' AND `C`.`C_D_ID` = 43 AND `C`.`C_W_ID` = 44) AS `EXPR$0`
        // GROUP BY `EXPR$0`.`C_W_ID`, `EXPR$0`.`C_D_ID`) AS `A`
        // WHERE `C`.`C_D_ID` = 43 AND `C`.`C_W_ID` = 44 AND `C`.`C_ID` = `A`.`CUST_ARRAY`[ARRAY_LENGTH(`A`.`CUST_ARRAY`) / 2 + 1]
        // DBSPSinkOperator 5124(1120)
        let handle5124 = stream5121.output();

        Ok((
            handle49, handle67, handle120, handle143, handle166, handle279, handle337, handle5124,
        ))
    })?;
    Ok((circuit, streams))
}
